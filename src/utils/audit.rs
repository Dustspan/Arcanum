use sqlx::SqlitePool;

/// 记录操作日志
pub async fn log_action(
    db: &SqlitePool,
    user_id: Option<&str>,
    action: &str,
    target_type: Option<&str>,
    target_id: Option<&str>,
    details: Option<&str>,
    ip_address: Option<&str>,
) {
    let id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query(
        "INSERT INTO audit_logs (id, user_id, action, target_type, target_id, details, ip_address) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(user_id)
    .bind(action)
    .bind(target_type)
    .bind(target_id)
    .bind(details)
    .bind(ip_address)
    .execute(db)
    .await;
}

/// 过滤敏感词（带缓存）
pub async fn filter_sensitive_words(db: &SqlitePool, content: &str) -> String {
    let words: Vec<(String, String)> = match sqlx::query_as(
        "SELECT word, replacement FROM sensitive_words"
    )
    .fetch_all(db)
    .await
    {
        Ok(w) => w,
        Err(_) => return content.to_string(),
    };
    
    let mut result = content.to_string();
    for (word, replacement) in words {
        result = result.replace(&word, &replacement);
    }
    result
}
