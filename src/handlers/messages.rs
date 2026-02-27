use axum::{extract::{State, Path, Multipart}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result, 
    models::SendMessageRequest, 
    handlers::auth::get_claims_full, 
    utils::{check_permission, is_muted, check_rate_limit},
    broadcast::WsMessage,
    AppState
};

const MAX_MSG_LEN: usize = 5000;

pub async fn send_message(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<SendMessageRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    if is_muted(&state.db, &claims.sub).await? {
        return Err(crate::error::AppError::BadRequest("你已被禁言".to_string()));
    }
    
    if !check_rate_limit(&state.db, &claims.sub, "message", &state.config).await? {
        return Err(crate::error::AppError::BadRequest("发送太快，请稍后再试".to_string()));
    }
    
    if req.content.is_empty() { return Err(crate::error::AppError::BadRequest("消息不能为空".to_string())); }
    if req.content.len() > MAX_MSG_LEN { return Err(crate::error::AppError::BadRequest("消息太长".to_string())); }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&req.group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let burn = req.burn_after.unwrap_or(0);
    let msg_type = req.msg_type.unwrap_or_else(|| "text".to_string());
    
    sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, file_name, file_size, burn_after, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(&claims.sub).bind(&req.group_id).bind(&req.content).bind(&msg_type)
        .bind(&req.file_name).bind(req.file_size.unwrap_or(0)).bind(burn).bind(&now)
        .execute(&state.db).await?;
    
    let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub).fetch_optional(&state.db).await?
        .flatten();
    
    let _ = state.broadcast.broadcast_to_group(&req.group_id, WsMessage { 
        event: "message".into(), 
        data: json!({
            "id": id, 
            "groupId": req.group_id, 
            "senderId": claims.sub, 
            "senderNickname": claims.nickname,
            "senderAvatar": avatar, 
            "content": req.content, 
            "msgType": msg_type,
            "fileName": req.file_name, 
            "fileSize": req.file_size, 
            "burnAfter": burn, 
            "createdAt": now
        })
    });
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "senderId": claims.sub,
            "senderNickname": claims.nickname,
            "senderAvatar": avatar,
            "content": req.content,
            "msgType": msg_type,
            "fileName": req.file_name,
            "fileSize": req.file_size,
            "burnAfter": burn,
            "createdAt": now
        }
    })))
}

pub async fn get_messages(State(state): State<AppState>, headers: HeaderMap, Path(group_id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    let messages: Vec<(String, String, String, String, String, Option<String>, i64, i64, String, Option<String>)> = 
        sqlx::query_as(r#"
            SELECT m.id, m.sender_id, u.nickname, m.content, m.type, m.file_name, m.file_size, m.burn_after, m.created_at, u.avatar
            FROM messages m 
            JOIN users u ON m.sender_id = u.id 
            WHERE m.group_id = ? 
            ORDER BY m.created_at ASC
        "#)
            .bind(&group_id).fetch_all(&state.db).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": messages.iter().map(|m| json!({
            "id": m.0,
            "senderId": m.1,
            "senderNickname": m.2,
            "senderAvatar": m.9,
            "content": m.3,
            "msgType": m.4,
            "fileName": m.5,
            "fileSize": m.6,
            "burnAfter": m.7,
            "createdAt": m.8
        })).collect::<Vec<_>>()
    })))
}

pub async fn clear_messages(State(state): State<AppState>, headers: HeaderMap, Path(group_id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&group_id).fetch_optional(&state.db).await?;
    let owner = owner.ok_or(crate::error::AppError::NotFound)?;
    
    if claims.role != "admin" && owner != claims.sub {
        check_permission(&claims, "message_delete")?;
    }
    
    sqlx::query("DELETE FROM messages WHERE group_id = ?").bind(&group_id).execute(&state.db).await?;
    Ok(Json(json!({"success": true})))
}

/// 撤回消息 - 用户只能撤回自己发送的消息，且在时间限制内
pub async fn recall_message(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取消息信息
    let msg: Option<(String, String, String)> = sqlx::query_as(
        "SELECT sender_id, group_id, created_at FROM messages WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;
    
    let msg = msg.ok_or_else(|| crate::error::AppError::NotFound)?;
    
    // 检查是否是消息发送者
    if msg.0 != claims.sub {
        // 如果不是发送者，检查是否有删除权限
        check_permission(&claims, "message_delete")?;
    } else {
        // 如果是发送者，检查时间限制（2分钟内可撤回）
        if let Ok(created_at) = chrono::DateTime::parse_from_rfc3339(&msg.2) {
            let elapsed = chrono::Utc::now() - created_at.with_timezone(&chrono::Utc);
            if elapsed.num_minutes() > 2 {
                return Err(crate::error::AppError::BadRequest("消息发送超过2分钟，无法撤回".to_string()));
            }
        }
    }
    
    // 删除消息
    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(&id).execute(&state.db).await?;
    
    // 广播撤回通知
    let _ = state.broadcast.broadcast_to_group(&msg.1, WsMessage {
        event: "message_recall".into(),
        data: json!({
            "id": id,
            "groupId": msg.1
        })
    });
    
    Ok(Json(json!({"success": true})))
}

pub async fn delete_message(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "message_delete")?;
    
    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(&id).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn upload_file(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(group_id): Path<String>,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "file_upload")?;
    
    if is_muted(&state.db, &claims.sub).await? {
        return Err(crate::error::AppError::BadRequest("你已被禁言".to_string()));
    }
    
    if !check_rate_limit(&state.db, &claims.sub, "file_upload", &state.config).await? {
        return Err(crate::error::AppError::BadRequest("上传太快，请稍后再试".to_string()));
    }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    while let Some(field) = multipart.next_field().await.map_err(|e| 
        crate::error::AppError::BadRequest(format!("上传失败: {}", e))
    )? {
        let content_type = field.content_type().map(|s| s.to_string()).unwrap_or_default();
        let file_name = field.file_name().map(|s| s.to_string()).unwrap_or_else(|| "file".to_string());
        
        let is_image = content_type.starts_with("image/");
        let is_text = content_type.starts_with("text/") || file_name.to_lowercase().ends_with(".txt");
        
        if !is_image && !is_text {
            return Err(crate::error::AppError::BadRequest("只支持图片和文本文件".to_string()));
        }
        
        let data = field.bytes().await.map_err(|e| 
            crate::error::AppError::BadRequest(format!("读取失败: {}", e))
        )?;
        
        if data.len() > state.config.max_file_size {
            return Err(crate::error::AppError::BadRequest("文件太大".to_string()));
        }
        
        // 使用文件存储而不是base64
        let file_url = if is_image {
            state.storage.save_image(&data, &content_type)
                .map_err(|e| crate::error::AppError::Internal(format!("保存失败: {}", e)))?
        } else {
            state.storage.save_file(&data, &file_name)
                .map_err(|e| crate::error::AppError::Internal(format!("保存失败: {}", e)))?
        };
        
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let msg_type = if is_image { "image" } else { "file" };
        
        sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, file_name, file_size, burn_after, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?)")
            .bind(&id).bind(&claims.sub).bind(&group_id).bind(&file_url).bind(msg_type)
            .bind(&file_name).bind(data.len() as i64).bind(&now)
            .execute(&state.db).await?;
        
        let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
            .bind(&claims.sub).fetch_optional(&state.db).await?
            .flatten();
        
        let _ = state.broadcast.broadcast_to_group(&group_id, WsMessage { 
            event: "message".into(), 
            data: json!({
                "id": id, 
                "groupId": group_id, 
                "senderId": claims.sub, 
                "senderNickname": claims.nickname,
                "senderAvatar": avatar, 
                "content": file_url, 
                "msgType": msg_type,
                "fileName": file_name, 
                "fileSize": data.len(), 
                "burnAfter": 0, 
                "createdAt": now
            })
        });
        
        return Ok(Json(json!({
            "success": true,
            "data": {
                "id": id,
                "groupId": group_id,
                "senderId": claims.sub,
                "senderNickname": claims.nickname,
                "senderAvatar": avatar,
                "content": file_url,
                "msgType": msg_type,
                "fileName": file_name,
                "fileSize": data.len(),
                "createdAt": now
            }
        })));
    }
    
    Err(crate::error::AppError::BadRequest("未找到文件".to_string()))
}
