use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result,
    handlers::auth::get_claims_full,
    broadcast::WsMessage,
    AppState
};

// 离线消息限制
const MAX_OFFLINE_MESSAGES_PER_USER: i64 = 100;

/// 发送私聊消息
pub async fn send_direct_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(receiver_id): Path<String>,
    Json(req): Json<DirectMessageRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    if req.content.is_empty() || req.content.len() > 5000 {
        return Err(crate::error::AppError::BadRequest("消息内容无效".to_string()));
    }
    
    // 检查接收者是否存在并获取在线状态
    let receiver_info: Option<(String, i64)> = sqlx::query_as(
        "SELECT id, online FROM users WHERE id = ?"
    )
    .bind(&receiver_id)
    .fetch_optional(&state.db)
    .await?;
    
    let (receiver_id, is_online) = match receiver_info {
        Some(info) => info,
        None => return Err(crate::error::AppError::BadRequest("用户不存在".to_string())),
    };
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let msg_type = req.msg_type.clone().unwrap_or_else(|| "text".to_string());
    
    // 获取发送者头像
    let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    // 【关键修复】无论用户是否在线，都存储消息到数据库
    // 检查离线消息数量限制
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM direct_messages WHERE receiver_id = ? AND read = 0"
    )
    .bind(&receiver_id)
    .fetch_one(&state.db)
    .await?;
    
    if count >= MAX_OFFLINE_MESSAGES_PER_USER {
        // 清理最旧的消息
        sqlx::query(
            "DELETE FROM direct_messages WHERE receiver_id = ? AND read = 0 ORDER BY created_at ASC LIMIT 10"
        )
        .bind(&receiver_id)
        .execute(&state.db)
        .await?;
    }
    
    // 存储消息（无论用户是否在线）
    sqlx::query(
        "INSERT INTO direct_messages (id, sender_id, receiver_id, content, type, file_name, file_size, read, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?)"
    )
    .bind(&id)
    .bind(&claims.sub)
    .bind(&receiver_id)
    .bind(&req.content)
    .bind(&msg_type)
    .bind(&req.file_name)
    .bind(req.file_size.unwrap_or(0))
    .bind(&now)
    .execute(&state.db)
    .await?;
    
    let msg_data = json!({
        "id": id,
        "senderId": claims.sub,
        "senderNickname": claims.nickname,
        "senderAvatar": avatar,
        "receiverId": receiver_id,
        "content": req.content,
        "msgType": msg_type,
        "fileName": req.file_name,
        "fileSize": req.file_size,
        "createdAt": now
    });
    
    // 如果接收者在线，尝试实时推送
    if is_online == 1 {
        let _ = state.broadcast.broadcast_to_user(
            &receiver_id, WsMessage {
                event: "direct_message".into(),
                data: msg_data.clone()
            }
        );
    }
    
    // 同时发回给发送者（用于多设备同步）
    let _ = state.broadcast.broadcast_to_user(
        &claims.sub, WsMessage {
            event: "direct_message".into(),
            data: msg_data
        }
    );
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "receiverId": receiver_id,
            "content": req.content,
            "msgType": msg_type,
            "createdAt": now,
            "delivered": is_online == 1
        }
    })))
}

#[derive(Debug, serde::Deserialize)]
pub struct DirectMessageRequest {
    pub content: String,
    pub msg_type: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
}

/// 获取离线消息并在获取后标记为已读
pub async fn get_direct_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 只能查看自己相关的消息
    if user_id != claims.sub {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 获取未读消息
    let messages: Vec<(String, String, String, Option<String>, String, String, Option<String>, Option<i64>, String)> = sqlx::query_as(r#"
        SELECT id, sender_id, receiver_id, content, type, file_name, file_size, created_at
        FROM direct_messages 
        WHERE receiver_id = ? AND read = 0
        ORDER BY created_at ASC
    "#)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    // 标记为已读
    if !messages.is_empty() {
        sqlx::query("UPDATE direct_messages SET read = 1 WHERE receiver_id = ? AND read = 0")
            .bind(&claims.sub)
            .execute(&state.db)
            .await?;
    }
    
    // 获取发送者信息
    let mut result = Vec::new();
    for msg in messages {
        let sender_nickname: Option<String> = sqlx::query_scalar("SELECT nickname FROM users WHERE id = ?")
            .bind(&msg.1)
            .fetch_optional(&state.db)
            .await?
            .flatten();
        
        let sender_avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
            .bind(&msg.1)
            .fetch_optional(&state.db)
            .await?
            .flatten();
        
        result.push(json!({
            "id": msg.0,
            "senderId": msg.1,
            "senderNickname": sender_nickname,
            "senderAvatar": sender_avatar,
            "receiverId": msg.2,
            "content": msg.3,
            "msgType": msg.4,
            "fileName": msg.5,
            "fileSize": msg.6,
            "createdAt": msg.8
        }));
    }
    
    Ok(Json(json!({
        "success": true,
        "data": result
    })))
}

/// 获取私聊会话列表
pub async fn get_conversations(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取有消息往来的用户
    let conversations: Vec<(String, String, Option<String>, i64, String)> = sqlx::query_as(r#"
        SELECT u.id, u.nickname, u.avatar, u.online, MAX(dm.created_at) as last_msg_time
        FROM direct_messages dm
        JOIN users u ON (u.id = dm.sender_id OR u.id = dm.receiver_id)
        WHERE (dm.sender_id = ? OR dm.receiver_id = ?) AND u.id != ?
        GROUP BY u.id
        ORDER BY last_msg_time DESC
    "#)
    .bind(&claims.sub)
    .bind(&claims.sub)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": conversations.iter().map(|c| json!({
            "userId": c.0,
            "nickname": c.1,
            "avatar": c.2,
            "online": c.3 == 1,
            "lastMessageTime": c.4
        })).collect::<Vec<_>>()
    })))
}

/// 添加好友
pub async fn add_friend(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(friend_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    if friend_id == claims.sub {
        return Err(crate::error::AppError::BadRequest("不能添加自己为好友".to_string()));
    }
    
    // 检查用户是否存在
    let friend: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE id = ?")
        .bind(&friend_id)
        .fetch_optional(&state.db)
        .await?;
    
    if friend.is_none() {
        return Err(crate::error::AppError::BadRequest("用户不存在".to_string()));
    }
    
    // 检查是否已经是好友或已发送请求
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT id FROM friendships WHERE user_id = ? AND friend_id = ?"
    )
    .bind(&claims.sub).bind(&friend_id)
    .fetch_optional(&state.db)
    .await?;
    
    if existing.is_some() {
        return Err(crate::error::AppError::BadRequest("已经是好友或请求已发送".to_string()));
    }
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO friendships (id, user_id, friend_id, status) VALUES (?, ?, ?, 'pending')")
        .bind(&id).bind(&claims.sub).bind(&friend_id)
        .execute(&state.db)
        .await?;
    
    // 通知对方
    let _ = state.broadcast.broadcast_to_user(
        &friend_id, WsMessage {
            event: "friend_request".into(),
            data: json!({
                "from": claims.nickname,
                "fromId": claims.sub
            })
        }
    );
    
    Ok(Json(json!({"success": true, "message": "好友请求已发送"})))
}

/// 接受好友请求
pub async fn accept_friend(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(friend_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 更新状态
    let result = sqlx::query("UPDATE friendships SET status = 'accepted' WHERE user_id = ? AND friend_id = ? AND status = 'pending'")
        .bind(&friend_id).bind(&claims.sub)
        .execute(&state.db)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::BadRequest("好友请求不存在".to_string()));
    }
    
    // 创建反向关系
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO friendships (id, user_id, friend_id, status) VALUES (?, ?, ?, 'accepted')")
        .bind(&id).bind(&claims.sub).bind(&friend_id)
        .execute(&state.db)
        .await?;
    
    Ok(Json(json!({"success": true, "message": "已添加好友"})))
}

/// 获取好友列表
pub async fn get_friends(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let friends: Vec<(String, String, Option<String>, i64)> = sqlx::query_as(r#"
        SELECT u.id, u.nickname, u.avatar, u.online
        FROM friendships f
        JOIN users u ON f.friend_id = u.id
        WHERE f.user_id = ? AND f.status = 'accepted'
    "#)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": friends.iter().map(|f| json!({
            "id": f.0,
            "nickname": f.1,
            "avatar": f.2,
            "online": f.3 == 1
        })).collect::<Vec<_>>()
    })))
}

/// 获取好友请求列表
pub async fn get_friend_requests(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let requests: Vec<(String, String, String, Option<String>)> = sqlx::query_as(r#"
        SELECT f.id, u.id, u.nickname, u.avatar
        FROM friendships f
        JOIN users u ON f.user_id = u.id
        WHERE f.friend_id = ? AND f.status = 'pending'
    "#)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": requests.iter().map(|r| json!({
            "requestId": r.0,
            "userId": r.1,
            "nickname": r.2,
            "avatar": r.3
        })).collect::<Vec<_>>()
    })))
}
