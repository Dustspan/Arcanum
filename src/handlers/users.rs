use axum::{extract::{State, Path, Multipart}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result, 
    models::{CreateUserRequest, GrantPermissionRequest, MuteUserRequest}, 
    utils::{generate_uid, hash_password, is_valid_uid, check_permission, grant_permission, revoke_permission, get_user_permissions}, 
    AppState,
};

pub async fn create_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Json(req): Json<CreateUserRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_create")?;
    
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
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_view")?;
    
    let users: Vec<(String, String, String, String, String, Option<String>, i64, Option<String>, Option<String>)> = 
        sqlx::query_as("SELECT id, uid, nickname, role, account_status, muted_until, online, last_ip, avatar FROM users ORDER BY created_at DESC")
            .fetch_all(&state.db).await?;
    
    let mut result = Vec::new();
    for u in users {
        let permissions = get_user_permissions(&state.db, &u.0).await.unwrap_or_default();
        result.push(json!({
            "id": u.0, 
            "uid": u.1, 
            "nickname": u.2, 
            "role": u.3, 
            "status": u.4,
            "mutedUntil": u.5,
            "online": u.6 == 1, 
            "lastIp": u.7,
            "avatar": u.8,
            "permissions": permissions
        }));
    }
    
    Ok(Json(json!({"success": true, "data": result})))
}

pub async fn delete_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_kick")?;
    
    if uid == claims.uid { 
        return Err(crate::error::AppError::BadRequest("不能删除自己".to_string())); 
    }
    
    sqlx::query("DELETE FROM messages WHERE sender_id = (SELECT id FROM users WHERE uid = ?)")
        .bind(&uid).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_members WHERE user_id = (SELECT id FROM users WHERE uid = ?)")
        .bind(&uid).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM user_permissions WHERE user_id = (SELECT id FROM users WHERE uid = ?)")
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
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_ban")?;
    
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
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_ban")?;
    
    sqlx::query("UPDATE users SET account_status = 'active' WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn kick_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_kick")?;
    
    if uid == claims.uid { 
        return Err(crate::error::AppError::BadRequest("不能踢出自己".to_string())); 
    }
    
    sqlx::query("UPDATE users SET token_version = token_version + 1, online = 0 WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn mute_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>,
    Json(req): Json<MuteUserRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_mute")?;
    
    if uid == claims.uid { 
        return Err(crate::error::AppError::BadRequest("不能禁言自己".to_string())); 
    }
    
    let muted_until = chrono::Utc::now() + chrono::Duration::minutes(req.duration_minutes);
    sqlx::query("UPDATE users SET muted_until = ? WHERE uid = ? AND role != 'admin'")
        .bind(muted_until.to_rfc3339())
        .bind(&uid)
        .execute(&state.db).await?;
    
    Ok(Json(json!({"success": true, "data": {"mutedUntil": muted_until.to_rfc3339()}})))
}

pub async fn unmute_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_mute")?;
    
    sqlx::query("UPDATE users SET muted_until = NULL WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn grant_user_permission(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>,
    Json(req): Json<GrantPermissionRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "permission_grant")?;
    
    let user_id: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE uid = ?")
        .bind(&uid).fetch_optional(&state.db).await?;
    
    let user_id = user_id.ok_or_else(|| crate::error::AppError::BadRequest("用户不存在".to_string()))?;
    
    grant_permission(&state.db, &user_id, &req.permission_name, &claims.sub).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn revoke_user_permission(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>,
    Json(req): Json<GrantPermissionRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "permission_grant")?;
    
    let user_id: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE uid = ?")
        .bind(&uid).fetch_optional(&state.db).await?;
    
    let user_id = user_id.ok_or_else(|| crate::error::AppError::BadRequest("用户不存在".to_string()))?;
    
    revoke_permission(&state.db, &user_id, &req.permission_name).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn list_permissions(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims(&headers, &state.config)?;
    check_permission(&claims, "permission_grant")?;
    
    let perms: Vec<(String, String, Option<String>)> = sqlx::query_as("SELECT id, name, description FROM permissions ORDER BY name")
        .fetch_all(&state.db).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": perms.iter().map(|p| json!({
            "id": p.0,
            "name": p.1,
            "description": p.2
        })).collect::<Vec<_>>()
    })))
}

pub async fn upload_avatar(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>> {
    let claims = super::auth::get_claims_full(&headers, &state).await?;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| 
        crate::error::AppError::BadRequest(format!("上传失败: {}", e))
    )? {
        let content_type = field.content_type().map(|s| s.to_string()).unwrap_or_default();
        
        if !content_type.starts_with("image/") {
            continue;
        }
        
        let data = field.bytes().await.map_err(|e| 
            crate::error::AppError::BadRequest(format!("读取失败: {}", e))
        )?;
        
        if data.len() > state.config.max_file_size {
            return Err(crate::error::AppError::BadRequest("文件太大".to_string()));
        }
        
        // 转换为base64
        let base64 = base64_encode(&data, &content_type);
        
        sqlx::query("UPDATE users SET avatar = ? WHERE id = ?")
            .bind(&base64).bind(&claims.sub)
            .execute(&state.db).await?;
        
        return Ok(Json(json!({"success": true, "data": {"avatar": base64}})));
    }
    
    Err(crate::error::AppError::BadRequest("未找到文件".to_string()))
}

fn base64_encode(data: &[u8], content_type: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    format!("data:{};base64,{}", content_type, general_purpose::STANDARD.encode(data))
}
