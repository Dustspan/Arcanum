use axum::{extract::{State, Path, Multipart}, http::HeaderMap, Json};
use serde_json::json;
use sqlx::SqlitePool;
use crate::{
    error::{Result, AppError},
    handlers::auth::{get_claims, get_claims_full},
    models::User,
    utils::{check_permission, hash_password},
    AppState
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserRequest {
    pub uid: Option<String>,
    pub nickname: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GrantPermissionRequest {
    pub permission_name: String,
}

pub async fn create_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Json(req): Json<CreateUserRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_create")?;
    
    if req.nickname.is_empty() || req.password.is_empty() {
        return Err(AppError::BadRequest("昵称和密码不能为空".to_string()));
    }
    
    let uid = req.uid.clone().unwrap_or_else(|| {
        format!("U{}", rand::random::<u32>() % 900000 + 100000)
    });
    
    let id = uuid::Uuid::new_v4().to_string();
    let hash = hash_password(&req.password)?;
    
    let result = sqlx::query(
        "INSERT INTO users (id, uid, nickname, password_hash, role, account_status, token_version, online) 
         VALUES (?, ?, ?, ?, 'member', 'active', 0, 0)"
    )
    .bind(&id).bind(&uid).bind(&req.nickname).bind(&hash)
    .execute(&state.db).await;
    
    match result {
        Ok(_) => Ok(Json(json!({"success": true, "data": {"uid": uid, "nickname": req.nickname}}))),
        Err(sqlx::Error::Database(e)) if e.message().contains("UNIQUE") => {
            Err(AppError::BadRequest("UID已存在".to_string()))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn list_users(
    State(state): State<AppState>, 
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_view")?;
    
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&state.db).await?;
    
    Ok(Json(json!({"success": true, "data": users})))
}

pub async fn delete_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_ban")?;
    
    // 不能删除自己
    if uid == claims.uid {
        return Err(AppError::BadRequest("不能删除自己".to_string()));
    }
    
    // 检查目标用户角色
    let target_role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE uid = ?")
        .bind(&uid)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    if target_role.as_deref() == Some("admin") {
        return Err(AppError::BadRequest("不能删除管理员".to_string()));
    }
    
    let result = sqlx::query("DELETE FROM users WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(Json(json!({"success": true})))
}

pub async fn ban_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_ban")?;
    
    // 不能封禁自己
    if uid == claims.uid {
        return Err(AppError::BadRequest("不能封禁自己".to_string()));
    }
    
    // 检查目标用户角色
    let target_role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE uid = ?")
        .bind(&uid)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    if target_role.as_deref() == Some("admin") {
        return Err(AppError::BadRequest("不能封禁管理员".to_string()));
    }
    
    let result = sqlx::query("UPDATE users SET account_status = 'banned', token_version = token_version + 1 WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(Json(json!({"success": true})))
}

pub async fn unban_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
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
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_kick")?;
    
    // 不能踢出自己
    if uid == claims.uid {
        return Err(AppError::BadRequest("不能踢出自己".to_string()));
    }
    
    // 检查目标用户角色
    let target_role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE uid = ?")
        .bind(&uid)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    if target_role.as_deref() == Some("admin") {
        return Err(AppError::BadRequest("不能踢出管理员".to_string()));
    }
    
    let result = sqlx::query("UPDATE users SET token_version = token_version + 1 WHERE uid = ?")
        .bind(&uid).execute(&state.db).await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(Json(json!({"success": true})))
}

#[derive(Debug, serde::Deserialize)]
pub struct MuteRequest {
    pub duration_minutes: i64,
}

pub async fn mute_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>,
    Json(req): Json<MuteRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "user_mute")?;
    
    // 不能禁言自己
    if uid == claims.uid {
        return Err(AppError::BadRequest("不能禁言自己".to_string()));
    }
    
    // 检查目标用户角色
    let target_role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE uid = ?")
        .bind(&uid)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    if target_role.as_deref() == Some("admin") {
        return Err(AppError::BadRequest("不能禁言管理员".to_string()));
    }
    
    let muted_until = chrono::Utc::now() + chrono::Duration::minutes(req.duration_minutes);
    let result = sqlx::query("UPDATE users SET muted_until = ? WHERE uid = ?")
        .bind(muted_until.to_rfc3339())
        .bind(&uid)
        .execute(&state.db).await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(Json(json!({"success": true, "data": {"mutedUntil": muted_until.to_rfc3339()}})))
}

pub async fn unmute_user(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
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
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "permission_grant")?;
    
    // 获取目标用户信息
    let target_info: Option<(String, String)> = sqlx::query_as(
        "SELECT id, role FROM users WHERE uid = ?"
    )
    .bind(&uid)
    .fetch_optional(&state.db)
    .await?;
    
    let (user_id, target_role) = target_info
        .ok_or_else(|| AppError::BadRequest("用户不存在".to_string()))?;
    
    // 安全检查：不能修改自己的权限
    if user_id == claims.sub {
        return Err(AppError::BadRequest("不能修改自己的权限".to_string()));
    }
    
    // 安全检查：不能修改管理员的权限
    if target_role == "admin" {
        return Err(AppError::BadRequest("不能修改管理员的权限".to_string()));
    }
    
    // 安全检查：非管理员不能授予敏感权限
    let sensitive_perms = ["permission_grant", "user_ban", "user_kick"];
    if claims.role != "admin" && sensitive_perms.contains(&req.permission_name.as_str()) {
        return Err(AppError::BadRequest("您没有权限授予此权限".to_string()));
    }
    
    grant_permission(&state.db, &user_id, &req.permission_name, &claims.sub).await?;
    
    // 使缓存失效
    state.cache.invalidate(&user_id).await;
    
    Ok(Json(json!({"success": true})))
}

pub async fn revoke_user_permission(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(uid): Path<String>,
    Json(req): Json<GrantPermissionRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "permission_grant")?;
    
    // 获取目标用户信息
    let target_info: Option<(String, String)> = sqlx::query_as(
        "SELECT id, role FROM users WHERE uid = ?"
    )
    .bind(&uid)
    .fetch_optional(&state.db)
    .await?;
    
    let (user_id, target_role) = target_info
        .ok_or_else(|| AppError::BadRequest("用户不存在".to_string()))?;
    
    // 安全检查：不能修改自己的权限
    if user_id == claims.sub {
        return Err(AppError::BadRequest("不能修改自己的权限".to_string()));
    }
    
    // 安全检查：不能修改管理员的权限
    if target_role == "admin" {
        return Err(AppError::BadRequest("不能修改管理员的权限".to_string()));
    }
    
    revoke_permission(&state.db, &user_id, &req.permission_name).await?;
    
    // 使缓存失效
    state.cache.invalidate(&user_id).await;
    
    Ok(Json(json!({"success": true})))
}

pub async fn list_permissions(
    State(state): State<AppState>, 
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "permission_grant")?;
    
    let perms: Vec<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, description FROM permissions ORDER BY name"
    )
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
    let claims = get_claims_full(&headers, &state).await?;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| 
        AppError::BadRequest(format!("上传失败: {}", e))
    )? {
        let content_type = field.content_type().map(|s| s.to_string()).unwrap_or_default();
        
        if !content_type.starts_with("image/") {
            continue;
        }
        
        let data = field.bytes().await.map_err(|e| 
            AppError::BadRequest(format!("读取失败: {}", e))
        )?;
        
        if data.len() > state.config.max_file_size {
            return Err(AppError::BadRequest("文件太大".to_string()));
        }
        
        let avatar_url = state.storage.save_avatar(&data, &content_type)
            .map_err(|e| AppError::Internal(format!("保存失败: {}", e)))?;
        
        sqlx::query("UPDATE users SET avatar = ? WHERE id = ?")
            .bind(&avatar_url).bind(&claims.sub)
            .execute(&state.db).await?;
        
        return Ok(Json(json!({"success": true, "data": {"avatar": avatar_url}})));
    }
    
    Err(AppError::BadRequest("未找到文件".to_string()))
}

pub async fn get_user_info(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let _claims = get_claims(&headers, &state.config)?;
    
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = ? OR uid = ?")
        .bind(&id).bind(&id)
        .fetch_optional(&state.db)
        .await?;
    
    match user {
        Some(u) => Ok(Json(json!({"success": true, "data": u}))),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpdateProfileRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    
    if !req.nickname.is_empty() {
        sqlx::query("UPDATE users SET nickname = ? WHERE id = ?")
            .bind(&req.nickname).bind(&claims.sub)
            .execute(&state.db).await?;
    }
    
    Ok(Json(json!({"success": true})))
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: String,
}

pub async fn change_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ChangePasswordRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 验证旧密码
    let hash: Option<String> = sqlx::query_scalar("SELECT password_hash FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?
        .flatten();
    
    let hash = hash.ok_or(AppError::NotFound)?;
    
    if !crate::utils::verify_password(&req.old_password, &hash)? {
        return Err(AppError::Auth("旧密码错误".to_string()));
    }
    
    // 更新密码
    let new_hash = hash_password(&req.new_password)?;
    sqlx::query("UPDATE users SET password_hash = ?, token_version = token_version + 1 WHERE id = ?")
        .bind(&new_hash).bind(&claims.sub)
        .execute(&state.db).await?;
    
    Ok(Json(json!({"success": true, "message": "密码已更新，请重新登录"})))
}

#[derive(Debug, serde::Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// 辅助函数
async fn grant_permission(
    pool: &SqlitePool, 
    user_id: &str, 
    permission_name: &str,
    granted_by: &str
) -> Result<()> {
    let perm_id: Option<String> = sqlx::query_scalar("SELECT id FROM permissions WHERE name = ?")
        .bind(permission_name)
        .fetch_optional(pool)
        .await?;
    
    let perm_id = perm_id.ok_or_else(|| AppError::BadRequest("权限不存在".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT OR IGNORE INTO user_permissions (id, user_id, permission_id, granted_by) VALUES (?, ?, ?, ?)"
    )
    .bind(&id).bind(user_id).bind(&perm_id).bind(granted_by)
    .execute(pool).await?;
    
    Ok(())
}

async fn revoke_permission(
    pool: &SqlitePool, 
    user_id: &str, 
    permission_name: &str
) -> Result<()> {
    sqlx::query(r#"
        DELETE FROM user_permissions 
        WHERE user_id = ? AND permission_id = (
            SELECT id FROM permissions WHERE name = ?
        )
    "#)
    .bind(user_id).bind(permission_name)
    .execute(pool).await?;
    
    Ok(())
}
