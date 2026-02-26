use axum::{extract::State, http::header, Json};
use serde_json::json;
use crate::{
    error::Result, 
    models::LoginRequest, 
    utils::{verify_password, generate_token, verify_token, extract_ip},
    AppState
};

pub async fn login(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>> {
    let ip = extract_ip(&headers);
    
    // 检查IP封禁
    let ip_banned: Option<String> = sqlx::query_scalar("SELECT ip FROM ip_bans WHERE ip = ?")
        .bind(&ip).fetch_optional(&state.db).await?;
    if ip_banned.is_some() {
        return Err(crate::error::AppError::IpBanned);
    }
    
    // 查找用户
    let user: Option<(String, String, String, String, String, i64)> = 
        sqlx::query_as("SELECT id, uid, nickname, password_hash, account_status, token_version FROM users WHERE uid = ?")
            .bind(&req.uid).fetch_optional(&state.db).await?;
    
    let user = user.ok_or_else(|| crate::error::AppError::Auth("用户不存在".to_string()))?;
    
    if user.4 == "banned" {
        return Err(crate::error::AppError::Banned);
    }
    
    if !verify_password(&req.password, &user.3)? {
        return Err(crate::error::AppError::Auth("密码错误".to_string()));
    }
    
    // 更新 token_version 和在线状态
    let new_version = user.5 + 1;
    sqlx::query("UPDATE users SET token_version = ?, online = 1, last_ip = ? WHERE id = ?")
        .bind(new_version).bind(&ip).bind(&user.0)
        .execute(&state.db).await?;
    
    // 获取角色
    let role: String = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
        .bind(&user.0).fetch_one(&state.db).await?;
    
    let token = generate_token(&user.0, &user.1, &user.2, &role, new_version, &state.config)?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "token": token,
            "user": {
                "id": user.0,
                "uid": user.1,
                "nickname": user.2,
                "role": role
            }
        }
    })))
}

pub async fn logout(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    
    sqlx::query("UPDATE users SET token_version = token_version + 1, online = 0 WHERE id = ?")
        .bind(&claims.sub).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn me(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let user: Option<(String, String, String, String)> = 
        sqlx::query_as("SELECT id, uid, nickname, role FROM users WHERE id = ? AND account_status = 'active'")
            .bind(&claims.sub).fetch_optional(&state.db).await?;
    
    let user = user.ok_or(crate::error::AppError::Kicked)?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": user.0,
            "uid": user.1,
            "nickname": user.2,
            "role": user.3
        }
    })))
}

pub fn get_claims(headers: &axum::http::HeaderMap, config: &crate::config::Config) -> Result<crate::models::Claims> {
    let auth = headers.get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(crate::error::AppError::Unauthorized)?;
    let token = auth.strip_prefix("Bearer ")
        .ok_or(crate::error::AppError::Unauthorized)?;
    verify_token(token, config)
}

pub async fn get_claims_full(
    headers: &axum::http::HeaderMap, 
    state: &AppState
) -> Result<crate::models::Claims> {
    let claims = get_claims(headers, &state.config)?;
    
    // 检查IP封禁
    let ip = extract_ip(headers);
    let ip_banned: Option<String> = sqlx::query_scalar("SELECT ip FROM ip_bans WHERE ip = ?")
        .bind(&ip).fetch_optional(&state.db).await?;
    if ip_banned.is_some() {
        return Err(crate::error::AppError::IpBanned);
    }
    
    // 检查用户状态
    let user: Option<(String, i64)> = sqlx::query_as(
        "SELECT account_status, token_version FROM users WHERE id = ?"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;
    
    match user {
        Some((status, version)) => {
            if status == "banned" {
                return Err(crate::error::AppError::Banned);
            }
            if version != claims.token_version {
                return Err(crate::error::AppError::Kicked);
            }
            Ok(claims)
        }
        None => Err(crate::error::AppError::Kicked),
    }
}

pub fn check_admin(claims: &crate::models::Claims) -> Result<()> {
    if claims.role != "admin" { Err(crate::error::AppError::Forbidden) } else { Ok(()) }
}
