use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result, 
    models::CreateUserRequest, 
    utils::{generate_uid, hash_password, is_valid_uid}, 
    handlers::auth::{get_claims, check_admin},
    AppState,
};

pub async fn create_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Json(req): Json<CreateUserRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    if req.password.len() < 6 { 
        return Err(crate::error::AppError::BadRequest("密码至少6位".to_string())); 
    }
    
    let uid = match req.uid {
        Some(ref uid) if !uid.is_empty() => {
            if !is_valid_uid(uid) { 
                return Err(crate::error::AppError::BadRequest("UID格式无效".to_string())); 
            }
            let exists: Option<String> = sqlx::query_scalar("SELECT uid FROM users WHERE uid = ?")
                .bind(uid).fetch_optional(&state.db).await?;
            if exists.is_some() { 
                return Err(crate::error::AppError::BadRequest("UID已存在".to_string())); 
            }
            uid.clone()
        }
        _ => {
            let mut uid = generate_uid();
            for _ in 0..100 {
                let exists: Option<String> = sqlx::query_scalar("SELECT uid FROM users WHERE uid = ?")
                    .bind(&uid).fetch_optional(&state.db).await?;
                if exists.is_none() { break; }
                uid = generate_uid();
            }
            uid
        }
    };
    
    let id = uuid::Uuid::new_v4().to_string();
    let hash = hash_password(&req.password)?;
    
    sqlx::query("INSERT INTO users (id, uid, nickname, password_hash, role, account_status, token_version, online) VALUES (?, ?, ?, ?, 'member', 'active', 0, 0)")
        .bind(&id).bind(&uid).bind(&req.nickname).bind(&hash)
        .execute(&state.db).await?;
    
    Ok(Json(json!({"success":true,"data":{"uid":uid,"nickname":req.nickname,"password":req.password}})))
}

pub async fn list_users(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    let users: Vec<(String, String, String, String, String, i64, Option<String>)> = 
        sqlx::query_as("SELECT id, uid, nickname, role, account_status, online, last_ip FROM users ORDER BY created_at DESC")
            .fetch_all(&state.db).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": users.iter().map(|u| json!({
            "id": u.0, "uid": u.1, "nickname": u.2, "role": u.3, 
            "status": u.4, "online": u.5 == 1, "lastIp": u.6
        })).collect::<Vec<_>>()
    })))
}

pub async fn delete_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    if uid == claims.uid { 
        return Err(crate::error::AppError::BadRequest("不能删除自己".to_string())); 
    }
    
    sqlx::query("DELETE FROM messages WHERE sender_id = (SELECT id FROM users WHERE uid = ?)")
        .bind(&uid).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_members WHERE user_id = (SELECT id FROM users WHERE uid = ?)")
        .bind(&uid).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM users WHERE uid = ? AND role != 'admin'")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn ban_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    if uid == claims.uid { 
        return Err(crate::error::AppError::BadRequest("不能封禁自己".to_string())); 
    }
    
    sqlx::query("UPDATE users SET account_status = 'banned', token_version = token_version + 1, online = 0 WHERE uid = ? AND role != 'admin'")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn unban_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    sqlx::query("UPDATE users SET account_status = 'active' WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn kick_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    if uid == claims.uid { 
        return Err(crate::error::AppError::BadRequest("不能踢出自己".to_string())); 
    }
    
    sqlx::query("UPDATE users SET token_version = token_version + 1, online = 0 WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}
