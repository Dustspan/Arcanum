use axum::{extract::{State, Path, Query}, http::HeaderMap, Json};
use serde_json::json;
use crate::{error::Result, handlers::auth::get_claims_full, utils::{check_permission, log_action}, AppState};

pub async fn list_banned_ips(
    State(state): State<AppState>, 
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "ip_ban")?;
    
    let ips: Vec<(String, String, Option<String>, String)> = 
        sqlx::query_as("SELECT id, ip, reason, created_at FROM ip_bans ORDER BY created_at DESC")
            .fetch_all(&state.db).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": ips.iter().map(|ip| json!({
            "id": ip.0, "ip": ip.1, "reason": ip.2, "createdAt": ip.3
        })).collect::<Vec<_>>()
    })))
}

pub async fn ban_ip(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(ip): Path<String>,
    Json(req): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "ip_ban")?;
    
    let reason = req.get("reason").and_then(|v| v.as_str()).unwrap_or("");
    
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT OR IGNORE INTO ip_bans (id, ip, reason, banned_by) VALUES (?, ?, ?, ?)")
        .bind(&id).bind(&ip).bind(reason).bind(&claims.sub)
        .execute(&state.db).await?;
    
    log_action(&state.db, Some(&claims.sub), "ban_ip", Some("ip"), Some(&ip), Some(reason), None).await;
    
    Ok(Json(json!({"success": true})))
}

pub async fn unban_ip(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(ip): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "ip_ban")?;
    
    sqlx::query("DELETE FROM ip_bans WHERE ip = ?")
        .bind(&ip).execute(&state.db).await?;
    
    log_action(&state.db, Some(&claims.sub), "unban_ip", Some("ip"), Some(&ip), None, None).await;
    
    Ok(Json(json!({"success": true})))
}

// 敏感词管理
pub async fn list_sensitive_words(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "admin")?;
    
    let words: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT id, word, replacement FROM sensitive_words ORDER BY word"
    )
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": words.iter().map(|w| json!({
            "id": w.0,
            "word": w.1,
            "replacement": w.2
        })).collect::<Vec<_>>()
    })))
}

#[derive(Debug, serde::Deserialize)]
pub struct AddSensitiveWordRequest {
    pub word: String,
    pub replacement: Option<String>,
}

pub async fn add_sensitive_word(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AddSensitiveWordRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "admin")?;
    
    let id = uuid::Uuid::new_v4().to_string();
    let replacement = req.replacement.unwrap_or_else(|| "***".to_string());
    
    sqlx::query("INSERT INTO sensitive_words (id, word, replacement) VALUES (?, ?, ?)")
        .bind(&id).bind(&req.word).bind(&replacement)
        .execute(&state.db)
        .await?;
    
    log_action(&state.db, Some(&claims.sub), "add_sensitive_word", Some("word"), Some(&id), Some(&req.word), None).await;
    
    Ok(Json(json!({"success": true})))
}

pub async fn delete_sensitive_word(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "admin")?;
    
    sqlx::query("DELETE FROM sensitive_words WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;
    
    log_action(&state.db, Some(&claims.sub), "delete_sensitive_word", Some("word"), Some(&id), None, None).await;
    
    Ok(Json(json!({"success": true})))
}

// 操作日志
pub async fn list_audit_logs(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<AuditLogQuery>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "admin")?;
    
    let limit = params.limit.unwrap_or(100).min(500);
    
    let logs: Vec<(String, Option<String>, String, Option<String>, Option<String>, Option<String>, Option<String>, String)> = 
        sqlx::query_as(r#"
            SELECT id, user_id, action, target_type, target_id, details, ip_address, created_at
            FROM audit_logs
            ORDER BY created_at DESC
            LIMIT ?
        "#)
        .bind(limit)
        .fetch_all(&state.db)
        .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": logs.iter().map(|l| json!({
            "id": l.0,
            "userId": l.1,
            "action": l.2,
            "targetType": l.3,
            "targetId": l.4,
            "details": l.5,
            "ipAddress": l.6,
            "createdAt": l.7
        })).collect::<Vec<_>>()
    })))
}

#[derive(Debug, serde::Deserialize)]
pub struct AuditLogQuery {
    pub limit: Option<i64>,
}
