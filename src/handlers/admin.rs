use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use crate::{error::Result, handlers::auth::{get_claims, check_admin}, AppState};

pub async fn list_banned_ips(
    State(state): State<AppState>, 
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
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

pub async fn unban_ip(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(ip): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_admin(&claims)?;
    
    sqlx::query("DELETE FROM ip_bans WHERE ip = ?")
        .bind(&ip).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}
