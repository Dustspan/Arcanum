use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result,
    handlers::auth::get_claims_full,
    broadcast::WsMessage,
    AppState
};

// ==================== 数据结构 ====================

#[derive(Debug, serde::Deserialize)]
pub struct DirectMessageRequest {
    pub content: String,
    pub msg_type: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
}

// ==================== 发送私聊消息 ====================

pub async fn send_direct_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(receiver_id): Path<String>,
    Json(req): Json<DirectMessageRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 验证消息内容
    if req.content.trim().is_empty() {
        return Err(crate::error::AppError::BadRequest("消息内容不能为空".to_string()));
    }
    if req.content.len() > 5000 {
        return Err(crate::error::AppError::BadRequest("消息内容过长".to_string()));
    }
    
    // 检查接收者是否存在
    let receiver_info: Option<(String, String, Option<String>, i64)> = sqlx::query_as(
        "SELECT id, nickname, avatar, online FROM users WHERE id = ?"
    )
    .bind(&receiver_id)
    .fetch_optional(&state.db)
    .await?;
    
    let (receiver_id, _receiver_nickname, _receiver_avatar, is_online) = match receiver_info {
        Some(info) => info,
        None => return Err(crate::error::AppError::BadRequest("用户不存在".to_string())),
    };
    
    // 不能给自己发消息
    if receiver_id == claims.sub {
        return Err(crate::error::AppError::BadRequest("不能给自己发消息".to_string()));
    }
    
    // 生成消息ID和时间
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let msg_type = req.msg_type.clone().unwrap_or_else(|| "text".to_string());
    
    // 获取发送者头像
    let sender_avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    // 【核心】始终存储消息到数据库
    sqlx::query(
        r#"INSERT INTO direct_messages 
           (id, sender_id, receiver_id, content, type, file_name, file_size, read, created_at) 
           VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?)"#
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
    
    // 构建消息对象
    let msg = json!({
        "id": id,
        "senderId": claims.sub,
        "senderNickname": claims.nickname,
        "senderAvatar": sender_avatar,
        "receiverId": receiver_id,
        "content": req.content,
        "msgType": msg_type,
        "fileName": req.file_name,
        "fileSize": req.file_size,
        "createdAt": now
    });
    
    // 推送给接收者（如果在线）
    if is_online == 1 {
        let _ = state.broadcast.broadcast_to_user(
            &receiver_id, 
            WsMessage {
                event: "direct_message".into(),
                data: msg.clone()
            }
        );
    }
    
    // 推送给发送者（多设备同步）
    let _ = state.broadcast.broadcast_to_user(
        &claims.sub, 
        WsMessage {
            event: "direct_message".into(),
            data: msg.clone()
        }
    );
    
    Ok(Json(json!({
        "success": true,
        "data": msg
    })))
}

// ==================== 获取私聊消息历史 ====================

pub async fn get_direct_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(other_user_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取与某用户的所有消息（最近100条）
    let messages: Vec<(String, String, String, String, Option<String>, Option<String>, Option<i64>, i64, String)> = sqlx::query_as(
        r#"SELECT id, sender_id, receiver_id, content, type, file_name, file_size, read, created_at 
           FROM direct_messages 
           WHERE (sender_id = ? AND receiver_id = ?) OR (sender_id = ? AND receiver_id = ?)
           ORDER BY created_at DESC LIMIT 100"#
    )
    .bind(&claims.sub)
    .bind(&other_user_id)
    .bind(&other_user_id)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    // 标记接收的消息为已读
    sqlx::query(
        "UPDATE direct_messages SET read = 1 WHERE receiver_id = ? AND sender_id = ? AND read = 0"
    )
    .bind(&claims.sub)
    .bind(&other_user_id)
    .execute(&state.db)
    .await?;
    
    // 获取对方用户信息
    let other_user: Option<(String, Option<String>, i64)> = sqlx::query_as(
        "SELECT nickname, avatar, online FROM users WHERE id = ?"
    )
    .bind(&other_user_id)
    .fetch_optional(&state.db)
    .await?;
    
    let (other_nickname, other_avatar, other_online) = other_user
        .map(|(n, a, o)| (n, a, o == 1))
        .unwrap_or(("未知用户".to_string(), None, false));
    
    // 获取当前用户头像
    let my_avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    // 构建消息列表
    let mut result = Vec::new();
    for msg in messages {
        let is_sender = msg.1 == claims.sub;
        let sender_nickname = if is_sender { claims.nickname.clone() } else { other_nickname.clone() };
        let sender_avatar = if is_sender { my_avatar.clone() } else { other_avatar.clone() };
        
        result.push(json!({
            "id": msg.0,
            "senderId": msg.1,
            "receiverId": msg.2,
            "content": msg.3,
            "msgType": msg.4,
            "fileName": msg.5,
            "fileSize": msg.6,
            "read": msg.7 == 1,
            "createdAt": msg.8,
            "isSender": is_sender,
            "senderNickname": sender_nickname,
            "senderAvatar": sender_avatar
        }));
    }
    
    // 反转顺序（最新的在最后）
    result.reverse();
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "messages": result,
            "otherUser": {
                "id": other_user_id,
                "nickname": other_nickname,
                "avatar": other_avatar,
                "online": other_online
            }
        }
    })))
}

// ==================== 获取会话列表 ====================

pub async fn get_conversations(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取所有有消息往来的用户
    let conversations: Vec<(String, String, Option<String>, i64, String)> = sqlx::query_as(
        r#"SELECT 
            u.id, 
            u.nickname, 
            u.avatar, 
            u.online,
            MAX(dm.created_at) as last_msg_time
           FROM direct_messages dm
           JOIN users u ON (u.id = dm.sender_id OR u.id = dm.receiver_id)
           WHERE (dm.sender_id = ? OR dm.receiver_id = ?) AND u.id != ?
           GROUP BY u.id
           ORDER BY last_msg_time DESC"#
    )
    .bind(&claims.sub)
    .bind(&claims.sub)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    // 获取每个会话的未读消息数
    let mut result = Vec::new();
    for conv in conversations {
        let unread_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM direct_messages WHERE receiver_id = ? AND sender_id = ? AND read = 0"
        )
        .bind(&claims.sub)
        .bind(&conv.0)
        .fetch_one(&state.db)
        .await?;
        
        result.push(json!({
            "userId": conv.0,
            "nickname": conv.1,
            "avatar": conv.2,
            "online": conv.3 == 1,
            "lastMessageTime": conv.4,
            "unreadCount": unread_count
        }));
    }
    
    Ok(Json(json!({
        "success": true,
        "data": result
    })))
}

// ==================== 好友系统 ====================

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

pub async fn accept_friend(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(friend_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
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

pub async fn get_friends(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let friends: Vec<(String, String, Option<String>, i64)> = sqlx::query_as(
        "SELECT u.id, u.nickname, u.avatar, u.online FROM friendships f JOIN users u ON f.friend_id = u.id WHERE f.user_id = ? AND f.status = 'accepted'"
    )
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

pub async fn get_friend_requests(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let requests: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT f.id, u.id, u.nickname, u.avatar FROM friendships f JOIN users u ON f.user_id = u.id WHERE f.friend_id = ? AND f.status = 'pending'"
    )
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
