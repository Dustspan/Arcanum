use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{error::Result, models::{EnterGroupRequest, CreateGroupRequest}, handlers::auth::{get_claims, get_claims_full}, utils::hash_password, utils::check_permission, AppState};

pub async fn enter_by_name(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<EnterGroupRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    if req.name.is_empty() { return Err(crate::error::AppError::BadRequest("请输入频道名".to_string())); }
    
    let group: Option<(String, String)> = sqlx::query_as("SELECT id, cipher_hash FROM groups WHERE name = ?")
        .bind(&req.name).fetch_optional(&state.db).await?;
    let group = group.ok_or(crate::error::AppError::NotFound)?;
    
    if !verify_cipher(&req.name, &group.1)? { return Err(crate::error::AppError::Auth("频道不存在".to_string())); }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group.0).bind(&claims.sub).fetch_optional(&state.db).await?;
    
    if member.is_none() {
        let member_id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO group_members (id, group_id, user_id) VALUES (?, ?, ?)")
            .bind(&member_id).bind(&group.0).bind(&claims.sub).execute(&state.db).await?;
    }
    
    Ok(Json(json!({"success":true,"data":{"id":group.0,"name":req.name}})))
}

fn verify_cipher(name: &str, hash: &str) -> Result<bool> {
    use argon2::{password_hash::PasswordHash, Argon2, password_hash::PasswordVerifier};
    let parsed = PasswordHash::new(hash).map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
    Ok(Argon2::default().verify_password(name.as_bytes(), &parsed).is_ok())
}

pub async fn create_group(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<CreateGroupRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "group_create")?;
    
    if req.name.is_empty() { return Err(crate::error::AppError::BadRequest("频道名不能为空".to_string())); }
    
    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM groups WHERE name = ?").bind(&req.name).fetch_optional(&state.db).await?;
    if exists.is_some() { return Err(crate::error::AppError::BadRequest("频道已存在".to_string())); }
    
    let hash = hash_password(&req.name)?;
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO groups (id, name, cipher_hash, owner_id) VALUES (?, ?, ?, ?)")
        .bind(&id).bind(&req.name).bind(&hash).bind(&claims.sub).execute(&state.db).await?;
    
    let member_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO group_members (id, group_id, user_id) VALUES (?, ?, ?)")
        .bind(&member_id).bind(&id).bind(&claims.sub).execute(&state.db).await?;
    
    Ok(Json(json!({"success":true,"data":{"id":id,"name":req.name}})))
}

pub async fn list_my_groups(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    let groups: Vec<(String, String)> = sqlx::query_as(
        "SELECT g.id, g.name FROM groups g JOIN group_members gm ON g.id = gm.group_id WHERE gm.user_id = ?"
    ).bind(&claims.sub).fetch_all(&state.db).await?;
    Ok(Json(json!({"success":true,"data":groups.iter().map(|g|json!({"id":g.0,"name":g.1})).collect::<Vec<_>>()})))
}

pub async fn list_all_groups(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "group_view")?;
    
    let groups: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT g.id, g.name, g.owner_id, g.created_at FROM groups g ORDER BY g.created_at DESC"
    ).fetch_all(&state.db).await?;
    
    let mut result = Vec::new();
    for g in groups {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM group_members WHERE group_id = ?")
            .bind(&g.0).fetch_one(&state.db).await?;
        result.push(json!({"id":g.0,"name":g.1,"ownerId":g.2,"memberCount":count,"createdAt":g.3}));
    }
    
    Ok(Json(json!({"success":true,"data":result})))
}

pub async fn delete_group(State(state): State<AppState>, headers: HeaderMap, Path(id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "group_delete")?;
    
    sqlx::query("DELETE FROM messages WHERE group_id = ?").bind(&id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_members WHERE group_id = ?").bind(&id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM groups WHERE id = ?").bind(&id).execute(&state.db).await?;
    
    Ok(Json(json!({"success":true})))
}
