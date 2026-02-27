use axum::{extract::{State, Path, Multipart}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result, 
    models::SendMessageRequest, 
    handlers::auth::get_claims_full, 
    utils::{check_permission, is_muted, check_rate_limit},
    ws::WsMessage,
    AppState
};

const MAX_MSG_LEN: usize = 5000;

pub async fn send_message(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<SendMessageRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查禁言
    if is_muted(&state.db, &claims.sub).await? {
        return Err(crate::error::AppError::BadRequest("你已被禁言".to_string()));
    }
    
    // 检查速率限制
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
    
    // 获取发送者头像
    let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub).fetch_optional(&state.db).await?
        .flatten();
    
    // 广播消息
    let _ = state.tx.send(WsMessage { 
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
    
    // 检查权限：管理员或频道所有者
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&group_id).fetch_optional(&state.db).await?;
    let owner = owner.ok_or(crate::error::AppError::NotFound)?;
    
    if claims.role != "admin" && owner != claims.sub {
        // 检查是否有删除消息权限
        check_permission(&claims, "message_delete")?;
    }
    
    sqlx::query("DELETE FROM messages WHERE group_id = ?").bind(&group_id).execute(&state.db).await?;
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
    
    // 检查禁言
    if is_muted(&state.db, &claims.sub).await? {
        return Err(crate::error::AppError::BadRequest("你已被禁言".to_string()));
    }
    
    // 检查速率限制
    if !check_rate_limit(&state.db, &claims.sub, "file_upload", &state.config).await? {
        return Err(crate::error::AppError::BadRequest("上传太快，请稍后再试".to_string()));
    }
    
    // 检查是否是频道成员
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    while let Some(field) = multipart.next_field().await.map_err(|e| 
        crate::error::AppError::BadRequest(format!("上传失败: {}", e))
    )? {
        let content_type = field.content_type().map(|s| s.to_string()).unwrap_or_default();
        let file_name = field.file_name().map(|s| s.to_string()).unwrap_or_else(|| "file".to_string());
        
        // 检查文件类型
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
        
        // 转换为base64存储
        let base64 = if is_image {
            format!("data:{};base64,{}", content_type, base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data))
        } else {
            // 文本文件直接存储内容
            String::from_utf8(data.to_vec()).unwrap_or_else(|_| "".to_string())
        };
        
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let msg_type = if is_image { "image" } else { "file" };
        
        sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, file_name, file_size, burn_after, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?)")
            .bind(&id).bind(&claims.sub).bind(&group_id).bind(&base64).bind(msg_type)
            .bind(&file_name).bind(data.len() as i64).bind(&now)
            .execute(&state.db).await?;
        
        // 获取发送者头像
        let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
            .bind(&claims.sub).fetch_optional(&state.db).await?
            .flatten();
        
        // 直接广播消息 - 不需要前端再通过WebSocket发送
        let _ = state.tx.send(WsMessage { 
            event: "message".into(), 
            data: json!({
                "id": id, 
                "groupId": group_id, 
                "senderId": claims.sub, 
                "senderNickname": claims.nickname,
                "senderAvatar": avatar, 
                "content": base64, 
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
                "content": base64,
                "msgType": msg_type,
                "fileName": file_name,
                "fileSize": data.len(),
                "createdAt": now
            }
        })));
    }
    
    Err(crate::error::AppError::BadRequest("未找到文件".to_string()))
}
