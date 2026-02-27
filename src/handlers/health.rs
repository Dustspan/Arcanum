use axum::{extract::State, Json};
use serde_json::json;
use sqlx::SqlitePool;
use std::time::Instant;
use crate::AppState;

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
        "active_groups": state.broadcast.active_groups()
    }));
    
    // 缓存状态
    let cache_size = state.cache.cache_size().await;
    components.insert("cache".to_string(), json!({
        "status": "ok",
        "size": cache_size
    }));
    
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
