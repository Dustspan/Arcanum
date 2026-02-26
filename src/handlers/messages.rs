use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{error::Result, models::SendMessageRequest, handlers::auth::get_claims_full, AppState};

const MAX_MSG_LEN: usize = 5000;

pub async fn send_message(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<SendMessageRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    if req.content.is_empty() { return Err(crate::error::AppError::BadRequest("消息不能为空".to_string())); }
    if req.content.len() > MAX_MSG_LEN { return Err(crate::error::AppError::BadRequest("消息太长".to_string())); }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&req.group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let burn = req.burn_after.unwrap_or(0);
    
    sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, burn_after, created_at) VALUES (?, ?, ?, ?, 'text', ?, ?)")
        .bind(&id).bind(&claims.sub).bind(&req.group_id).bind(&req.content).bind(burn).bind(&now)
        .execute(&state.db).await?;
    
    Ok(Json(json!({"success":true,"data":{"id":id,"senderId":claims.sub,"senderNickname":claims.nickname,"content":req.content,"burnAfter":burn,"createdAt":now}})))
}

pub async fn get_messages(State(state): State<AppState>, headers: HeaderMap, Path(group_id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    let messages: Vec<(String, String, String, String, i64, String)> = 
        sqlx::query_as("SELECT m.id, m.sender_id, u.nickname, m.content, m.burn_after, m.created_at FROM messages m JOIN users u ON m.sender_id = u.id WHERE m.group_id = ? ORDER BY m.created_at ASC")
            .bind(&group_id).fetch_all(&state.db).await?;
    
    Ok(Json(json!({"success":true,"data":messages.iter().map(|m|json!({"id":m.0,"senderId":m.1,"senderNickname":m.2,"content":m.3,"burnAfter":m.4,"createdAt":m.5})).collect::<Vec<_>>()})))
}

pub async fn clear_messages(State(state): State<AppState>, headers: HeaderMap, Path(group_id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&group_id).fetch_optional(&state.db).await?;
    let owner = owner.ok_or(crate::error::AppError::NotFound)?;
    
    if claims.role != "admin" && owner != claims.sub { return Err(crate::error::AppError::Forbidden); }
    
    sqlx::query("DELETE FROM messages WHERE group_id = ?").bind(&group_id).execute(&state.db).await?;
    Ok(Json(json!({"success":true})))
}
