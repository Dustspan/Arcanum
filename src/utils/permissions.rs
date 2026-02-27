use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::Claims;

/// 检查用户是否拥有指定权限
pub async fn has_permission(pool: &SqlitePool, user_id: &str, permission_name: &str) -> Result<bool> {
    // 管理员拥有所有权限
    let role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    if role.as_deref() == Some("admin") {
        return Ok(true);
    }
    
    // 检查用户权限
    let has: Option<i64> = sqlx::query_scalar(r#"
        SELECT 1 FROM user_permissions up
        JOIN permissions p ON up.permission_id = p.id
        WHERE up.user_id = ? AND p.name = ?
    "#)
    .bind(user_id)
    .bind(permission_name)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(has.is_some())
}

/// 获取用户所有权限
pub async fn get_user_permissions(pool: &SqlitePool, user_id: &str) -> Result<Vec<String>> {
    // 管理员拥有所有权限
    let role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    if role.as_deref() == Some("admin") {
        let perms: Vec<String> = sqlx::query_scalar("SELECT name FROM permissions")
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        return Ok(perms);
    }
    
    let perms: Vec<String> = sqlx::query_scalar(r#"
        SELECT p.name FROM user_permissions up
        JOIN permissions p ON up.permission_id = p.id
        WHERE up.user_id = ?
    "#)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(perms)
}

/// 授予用户权限
pub async fn grant_permission(pool: &SqlitePool, user_id: &str, permission_name: &str, granted_by: &str) -> Result<()> {
    let perm_id: Option<String> = sqlx::query_scalar("SELECT id FROM permissions WHERE name = ?")
        .bind(permission_name)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    let perm_id = perm_id.ok_or_else(|| AppError::BadRequest("权限不存在".to_string()))?;
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT OR IGNORE INTO user_permissions (id, user_id, permission_id, granted_by) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(user_id)
        .bind(&perm_id)
        .bind(granted_by)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(())
}

/// 撤销用户权限
pub async fn revoke_permission(pool: &SqlitePool, user_id: &str, permission_name: &str) -> Result<()> {
    sqlx::query(r#"
        DELETE FROM user_permissions 
        WHERE user_id = ? AND permission_id = (SELECT id FROM permissions WHERE name = ?)
    "#)
    .bind(user_id)
    .bind(permission_name)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(())
}

/// 检查用户是否被禁言
pub async fn is_muted(pool: &SqlitePool, user_id: &str) -> Result<bool> {
    let muted_until: Option<String> = sqlx::query_scalar("SELECT muted_until FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    if let Some(until) = muted_until {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&until) {
            return Ok(dt > chrono::Utc::now());
        }
    }
    
    Ok(false)
}

/// 检查Claims是否拥有权限
pub fn check_permission(claims: &Claims, permission: &str) -> Result<()> {
    if claims.role == "admin" || claims.permissions.contains(&permission.to_string()) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

/// 检查是否为管理员
pub fn check_admin(claims: &Claims) -> Result<()> {
    if claims.role != "admin" { 
        Err(AppError::Forbidden) 
    } else { 
        Ok(()) 
    }
}
