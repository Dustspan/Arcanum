use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result,
    handlers::auth::get_claims_full,
    broadcast::WsMessage,
    AppState
};

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
    
    // 检查接收者是否存在
    let receiver: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE id = ?")
        .bind(&receiver_id)
        .fetch_optional(&state.db)
        .await?;
    
    if receiver.is_none() {
        return Err(crate::error::AppError::BadRequest("用户不存在".to_string()));
    }
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let msg_type = req.msg_type.unwrap_or_else(|| "text".to_string());
    
    sqlx::query("INSERT INTO direct_messages (id, sender_id, receiver_id, content, type, file_name, file_size, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(&claims.sub).bind(&receiver_id)
        .bind(&req.content).bind(&msg_type)
        .bind(&req.file_name).bind(req.file_size.unwrap_or(0))
        .bind(&now)
        .execute(&state.db)
        .await?;
    
    let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    // 发送给接收者
    let _ = state.broadcast.broadcast_to_user(&receiver_id, WsMessage {
        event: "direct_message".into(),
        data: json!({
            "id": id,
            "senderId": claims.sub,
            "senderNickname": claims.nickname,
            "senderAvatar": avatar,
            "receiverId": receiver_id,
            "content": req.content,
            "msgType": msg_type,
            "createdAt": now
        })
    });
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "receiverId": receiver_id,
            "content": req.content,
            "createdAt": now
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

/// 获取私聊消息列表
pub async fn get_direct_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let messages: Vec<(String, String, String, String, String, Option<String>, i64, i64, String)> = sqlx::query_as(r#"
        SELECT dm.id, dm.sender_id, u.nickname, dm.content, dm.type, dm.file_name, dm.file_size, dm.read, dm.created_at
        FROM direct_messages dm
        JOIN users u ON dm.sender_id = u.id
        WHERE (dm.sender_id = ? AND dm.receiver_id = ?) OR (dm.sender_id = ? AND dm.receiver_id = ?)
        ORDER BY dm.created_at ASC
        LIMIT 100
    "#)
    .bind(&claims.sub).bind(&user_id)
    .bind(&user_id).bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    // 标记为已读
    sqlx::query("UPDATE direct_messages SET read = 1 WHERE sender_id = ? AND receiver_id = ? AND read = 0")
        .bind(&user_id).bind(&claims.sub)
        .execute(&state.db)
        .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": messages.iter().map(|m| json!({
            "id": m.0,
            "senderId": m.1,
            "senderNickname": m.2,
            "content": m.3,
            "msgType": m.4,
            "fileName": m.5,
            "fileSize": m.6,
            "read": m.7 == 1,
            "createdAt": m.8
        })).collect::<Vec<_>>()
    })))
}

/// 获取私聊会话列表
pub async fn get_conversations(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let conversations: Vec<(String, String, Option<String>, String, i64)> = sqlx::query_as(r#"
        SELECT u.id, u.nickname, u.avatar, dm.content, dm.read
        FROM (
            SELECT 
                CASE WHEN sender_id = ? THEN receiver_id ELSE sender_id END as other_user,
                MAX(created_at) as latest_time
            FROM direct_messages
            WHERE sender_id = ? OR receiver_id = ?
            GROUP BY other_user
        ) latest
        JOIN users u ON u.id = latest.other_user
        JOIN direct_messages dm ON (
            (dm.sender_id = ? AND dm.receiver_id = latest.other_user) OR
            (dm.sender_id = latest.other_user AND dm.receiver_id = ?)
        ) AND dm.created_at = latest.latest_time
        ORDER BY latest.latest_time DESC
    "#)
    .bind(&claims.sub)
    .bind(&claims.sub).bind(&claims.sub)
    .bind(&claims.sub).bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    // 获取未读数
    let unread_counts: Vec<(String, i64)> = sqlx::query_as(r#"
        SELECT sender_id, COUNT(*) as count
        FROM direct_messages
        WHERE receiver_id = ? AND read = 0
        GROUP BY sender_id
    "#)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    let unread_map: std::collections::HashMap<String, i64> = unread_counts.into_iter().collect();
    
    Ok(Json(json!({
        "success": true,
        "data": conversations.iter().map(|c| {
            let unread = unread_map.get(&c.0).copied().unwrap_or(0);
            json!({
                "userId": c.0,
                "nickname": c.1,
                "avatar": c.2,
                "lastMessage": c.3.chars().take(50).collect::<String>(),
                "unread": unread
            })
        }).collect::<Vec<_>>()
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
    let _ = state.broadcast.broadcast_to_user(&friend_id, WsMessage {
        event: "friend_request".into(),
        data: json!({
            "from": claims.nickname,
            "fromId": claims.sub
        })
    });
    
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
    
    let friends: Vec<(String, String, Option<String>)> = sqlx::query_as(r#"
        SELECT u.id, u.nickname, u.avatar
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
            "avatar": f.2
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
