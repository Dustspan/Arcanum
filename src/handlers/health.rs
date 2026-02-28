use axum::{extract::State, Json, http::HeaderMap};
use serde_json::json;
use sqlx::SqlitePool;
use std::time::Instant;
use crate::AppState;
use crate::db;
use crate::handlers::auth::get_claims_full;
use crate::utils::check_permission;

pub struct SystemStats {
    pub start_time: Instant,
}

impl SystemStats {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
    
    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

impl Default for SystemStats {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SystemStats {
    fn clone(&self) -> Self {
        Self {
            start_time: self.start_time,
        }
    }
}

/// 健康检查 - 检查所有系统组件状态
pub async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut status = "healthy";
    let mut components = serde_json::Map::new();
    
    // 检查数据库连接
    let db_status = check_database(&state.db).await;
    if db_status != "ok" {
        status = "degraded";
    }
    components.insert("database".to_string(), json!({
        "status": db_status,
        "type": "sqlite"
    }));
    
    // 广播系统状态
    components.insert("broadcast".to_string(), json!({
        "status": "ok",
        "active_groups": state.broadcast.active_groups(),
        "active_users": state.broadcast.active_users()
    }));
    
    // 缓存状态
    let cache_size = state.cache.cache_size().await;
    components.insert("cache".to_string(), json!({
        "status": "ok",
        "size": cache_size
    }));
    
    // 存储状态
    let storage_status = match state.storage.get_storage_usage() {
        Ok(usage) => {
            if usage.usage_percent() > 90.0 {
                status = "degraded";
            }
            json!({
                "status": if usage.usage_percent() > 90.0 { "warning" } else { "ok" },
                "usage_percent": format!("{:.1}%", usage.usage_percent()),
                "used": usage.format_size(usage.total_size),
                "available": usage.format_size(usage.available)
            })
        }
        Err(_) => {
            status = "degraded";
            json!({"status": "error"})
        }
    };
    components.insert("storage".to_string(), storage_status);
    
    // 系统信息
    let uptime = state.stats.uptime_secs();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;
    
    Json(json!({
        "status": status,
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": format!("{}h {}m {}s", hours, minutes, seconds),
        "uptime_secs": uptime,
        "components": components,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// 简单健康检查（用于负载均衡器）
pub async fn health_simple() -> &'static str {
    "OK"
}

async fn check_database(pool: &SqlitePool) -> &'static str {
    match sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(pool)
        .await
    {
        Ok(_) => "ok",
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            "error"
        }
    }
}

/// 获取系统统计数据（权限模块化）
pub async fn get_statistics(
    State(state): State<AppState>,
    headers: HeaderMap
) -> crate::error::Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查是否有任何管理权限
    let has_any_admin_perm = claims.role == "admin" || 
        check_permission(&claims, "user_view").is_ok() ||
        check_permission(&claims, "group_view").is_ok() ||
        check_permission(&claims, "message_delete").is_ok();
    
    if !has_any_admin_perm {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 根据权限返回不同的统计数据
    let can_view_users = claims.role == "admin" || check_permission(&claims, "user_view").is_ok();
    let can_view_groups = claims.role == "admin" || check_permission(&claims, "group_view").is_ok();
    let can_view_messages = claims.role == "admin" || check_permission(&claims, "message_delete").is_ok();
    let can_view_storage = claims.role == "admin" || check_permission(&claims, "file_upload").is_ok();
    
    let mut result = json!({
        "success": true,
        "data": {}
    });
    
    let data = result.get_mut("data").unwrap().as_object_mut().unwrap();
    
    // 用户统计
    if can_view_users {
        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&state.db).await?;
        let online_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE online = 1")
            .fetch_one(&state.db).await?;
        let banned_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE account_status = 'banned'")
            .fetch_one(&state.db).await?;
        
        data.insert("users".to_string(), json!({
            "total": total_users,
            "online": online_users,
            "banned": banned_users
        }));
    }
    
    // 频道统计
    if can_view_groups {
        let total_groups: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM groups")
            .fetch_one(&state.db).await?;
        let active_groups: i64 = sqlx::query_scalar("SELECT COUNT(DISTINCT group_id) FROM group_members")
            .fetch_one(&state.db).await?;
        
        data.insert("groups".to_string(), json!({
            "total": total_groups,
            "active": active_groups
        }));
    }
    
    // 消息统计
    if can_view_messages {
        let total_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
            .fetch_one(&state.db).await?;
        let today_messages: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM messages WHERE date(created_at) = date('now')"
        )
        .fetch_one(&state.db).await?;
        
        data.insert("messages".to_string(), json!({
            "total": total_messages,
            "today": today_messages
        }));
    }
    
    // 文件统计
    if can_view_storage {
        let total_files: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE type IN ('image', 'file')")
            .fetch_one(&state.db).await?;
        let total_file_size: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(file_size), 0) FROM messages WHERE type IN ('image', 'file')")
            .fetch_one(&state.db).await?;
        
        data.insert("files".to_string(), json!({
            "total": total_files,
            "totalSize": total_file_size
        }));
    }
    
    // 私聊统计（仅管理员）
    if claims.role == "admin" {
        let total_direct_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM direct_messages")
            .fetch_one(&state.db).await?;
        let total_friendships: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM friendships WHERE status = 'accepted'")
            .fetch_one(&state.db).await?;
        
        data.insert("direct".to_string(), json!({
            "messages": total_direct_messages,
            "friendships": total_friendships
        }));
    }
    
    // 系统信息（所有人可见）
    let uptime = state.stats.uptime_secs();
    data.insert("system".to_string(), json!({
        "uptime": uptime,
        "activeBroadcastGroups": state.broadcast.active_groups(),
        "activeBroadcastUsers": state.broadcast.active_users(),
        "cacheSize": state.cache.cache_size().await
    }));
    
    Ok(Json(result))
}

/// 获取存储详情（需要 file_upload 权限或管理员）
pub async fn get_storage_info(
    State(state): State<AppState>,
    headers: HeaderMap
) -> crate::error::Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    if claims.role != "admin" {
        check_permission(&claims, "file_upload")?;
    }
    
    let usage = state.storage.get_storage_usage()?;
    let db_stats = db::get_db_stats(&state.db).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "storage": {
                "total": usage.format_size(usage.total_size),
                "max": usage.format_size(usage.max_size),
                "available": usage.format_size(usage.available),
                "usagePercent": format!("{:.1}%", usage.usage_percent()),
                "filesCount": usage.files_count,
                "breakdown": {
                    "images": usage.format_size(usage.images_size),
                    "files": usage.format_size(usage.files_size),
                    "avatars": usage.format_size(usage.avatars_size)
                }
            },
            "database": {
                "users": db_stats.users,
                "groups": db_stats.groups,
                "messages": db_stats.messages,
                "directMessages": db_stats.direct_messages,
                "files": db_stats.files,
                "totalFileSize": db_stats.total_file_size
            },
            "limits": {
                "maxFileSize": "5MB",
                "maxTotalStorage": "200MB",
                "messageRetentionDays": db::MESSAGE_RETENTION_DAYS,
                "pinnedMessageRetentionDays": db::PINNED_MESSAGE_RETENTION_DAYS,
                "maxMessagesPerGroup": db::MAX_MESSAGES_PER_GROUP
            }
        }
    })))
}

/// 手动触发清理（仅管理员）
pub async fn trigger_cleanup(
    State(state): State<AppState>,
    headers: HeaderMap
) -> crate::error::Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 仅管理员可以手动清理
    if claims.role != "admin" {
        return Err(crate::error::AppError::Forbidden);
    }
    
    let stats = db::cleanup_expired_data(&state.db).await?;
    
    // 清理孤立文件
    let files: Vec<String> = sqlx::query_scalar(
        "SELECT content FROM messages WHERE type IN ('image', 'file')"
    )
    .fetch_all(&state.db)
    .await?;
    
    let orphaned = state.storage.cleanup_orphaned_files(&files)?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "messagesDeleted": stats.messages_deleted,
            "pinnedDeleted": stats.pinned_deleted,
            "overflowDeleted": stats.overflow_deleted,
            "directDeleted": stats.direct_deleted,
            "logsDeleted": stats.logs_deleted,
            "orphanedFilesDeleted": orphaned
        }
    })))
}
