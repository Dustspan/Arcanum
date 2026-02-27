use sqlx::SqlitePool;
use crate::config::Config;
use crate::error::{AppError, Result};

/// 检查并更新速率限制
/// 返回是否允许操作
pub async fn check_rate_limit(
    pool: &SqlitePool, 
    user_id: &str, 
    action_type: &str,
    config: &Config
) -> Result<bool> {
    let now = chrono::Utc::now();
    let now_str = now.to_rfc3339();
    let window_start = now - chrono::Duration::seconds(config.rate_limit_window_secs);
    
    // 获取当前计数
    let current: Option<(i64, String)> = sqlx::query_as(
        "SELECT count, window_start FROM rate_limits WHERE user_id = ? AND action_type = ?"
    )
    .bind(user_id)
    .bind(action_type)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    
    match current {
        Some((count, window_start_str)) => {
            // 检查窗口是否过期
            if let Ok(window_dt) = chrono::DateTime::parse_from_rfc3339(&window_start_str) {
                let window_dt = window_dt.with_timezone(&chrono::Utc);
                if window_dt < window_start {
                    // 窗口过期，重置计数
                    sqlx::query(
                        "UPDATE rate_limits SET count = 1, window_start = ? WHERE user_id = ? AND action_type = ?"
                    )
                    .bind(&now_str)
                    .bind(user_id)
                    .bind(action_type)
                    .execute(pool)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                    return Ok(true);
                }
            }
            
            // 检查是否超过限制
            if count >= config.rate_limit_messages as i64 {
                return Ok(false);
            }
            
            // 增加计数
            sqlx::query(
                "UPDATE rate_limits SET count = count + 1 WHERE user_id = ? AND action_type = ?"
            )
            .bind(user_id)
            .bind(action_type)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            
            Ok(true)
        }
        None => {
            // 创建新记录
            let id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO rate_limits (id, user_id, action_type, count, window_start) VALUES (?, ?, ?, 1, ?)"
            )
            .bind(&id)
            .bind(user_id)
            .bind(action_type)
            .bind(&now_str)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            
            Ok(true)
        }
    }
}
