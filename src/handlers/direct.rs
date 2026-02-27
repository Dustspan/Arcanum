use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result,
    handlers::auth::get_claims_full,
    broadcast::WsMessage,
    AppState
};

/// 发送私聊消息 - 不保存到数据库，仅通过WebSocket实时传递
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
    
    let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    // 发送给接收者
    let _ = state.broadcast.broadcast_to_user(
        &receiver_id, WsMessage {
        event: "direct_message".into(),
        data: json!({
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
        })
    });
    
    // 同时发回给发送者（用于多设备同步）
    let _ = state.broadcast.broadcast_to_user(
        &claims.sub, WsMessage {
        event: "direct_message".into(),
        data: json!({
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
        })
    });
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "receiverId": receiver_id,
            "content": req.content,
            "msgType": msg_type,
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

/// 获取私聊消息列表 - 返回空列表（不保存历史）
pub async fn get_direct_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(_user_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let _claims = get_claims_full(&headers, &state).await?;
    
    // 不保存历史，返回空列表
    Ok(Json(json!({
        "success": true,
        "data": [],
        "message": "私聊消息不保存历史记录"
    })))
}

/// 获取私聊会话列表 - 返回空列表（不保存历史）
pub async fn get_conversations(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let _claims = get_claims_full(&headers, &state).await?;
    
    // 不保存历史，返回空列表
    Ok(Json(json!({
        "success": true,
        "data": []
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
