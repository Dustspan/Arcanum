use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use crate::{AppState, error::AppError, models::Claims, utils::verify_token};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage { pub event: String, pub data: serde_json::Value }

#[derive(Debug, Deserialize)]
pub struct WsQuery { pub token: String }

pub async fn ws_handler(
    ws: WebSocketUpgrade, 
    State(state): State<AppState>, 
    Query(q): Query<WsQuery>
) -> Result<Response, AppError> {
    let claims = verify_token(&q.token, &state.config).map_err(|_| AppError::Unauthorized)?;
    
    // 验证用户
    let user: Option<(String, i64)> = sqlx::query_as(
        "SELECT account_status, token_version FROM users WHERE id = ?"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| AppError::Unauthorized)?;
    
    match user {
        Some((status, version)) => {
            if status == "banned" { return Err(AppError::Banned); }
            if version != claims.token_version { return Err(AppError::Kicked); }
        }
        None => return Err(AppError::Kicked),
    }
    
    // 设置在线状态
    sqlx::query("UPDATE users SET online = 1 WHERE id = ?")
        .bind(&claims.sub).execute(&state.db).await.ok();
    
    Ok(ws.on_upgrade(move |s| handle(s, state, claims)))
}

async fn handle(socket: WebSocket, state: AppState, claims: Claims) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();
    let user_id = claims.sub.clone();
    let nickname = claims.nickname.clone();
    let token_version = claims.token_version;
    
    tracing::info!("WS connected: {}", nickname);
    
    let state2 = state.clone();
    let user_id2 = user_id.clone();
    let nickname2 = nickname.clone();
    let token_version2 = token_version;
    
    let recv = async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // 验证用户状态
                let valid: Option<(String, i64)> = sqlx::query_as(
                    "SELECT account_status, token_version FROM users WHERE id = ?"
                )
                .bind(&user_id2)
                .fetch_optional(&state2.db)
                .await
                .ok()
                .flatten();
                
                match valid {
                    Some((status, ver)) => {
                        if status == "banned" || ver != token_version2 {
                            break;
                        }
                    }
                    None => break,
                }
                
                if let Ok(m) = serde_json::from_str::<WsMessage>(&text) {
                    handle_msg(&state2, &user_id2, &nickname2, m).await;
                }
            }
        }
    };
    
    let send = async {
        while let Ok(m) = rx.recv().await {
            let json = serde_json::to_string(&m).unwrap();
            if sender.send(Message::Text(json)).await.is_err() { break; }
        }
    };
    
    tokio::select! { _ = recv => {}, _ = send => {} }
    
    // 设置离线
    sqlx::query("UPDATE users SET online = 0 WHERE id = ?")
        .bind(&user_id).execute(&state.db).await.ok();
    
    tracing::info!("WS disconnected: {}", nickname);
}

async fn handle_msg(state: &AppState, user_id: &str, nickname: &str, msg: WsMessage) {
    if msg.event != "message" { return; }
    
    #[derive(serde::Deserialize)]
    struct MsgData { group_id: String, content: String, burn_after: Option<i64> }
    
    let Ok(d) = serde_json::from_value::<MsgData>(msg.data) else { return };
    if d.content.is_empty() || d.content.len() > 5000 { return; }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&d.group_id).bind(user_id).fetch_optional(&state.db).await.ok().flatten();
    if member.is_none() { return; }
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let burn = d.burn_after.unwrap_or(0);
    
    if sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, burn_after, created_at) VALUES (?, ?, ?, ?, 'text', ?, ?)")
        .bind(&id).bind(user_id).bind(&d.group_id).bind(&d.content).bind(burn).bind(&now)
        .execute(&state.db).await.is_ok() {
        let _ = state.tx.send(WsMessage { event: "message".into(), data: serde_json::json!({
            "id": id, "groupId": d.group_id, "senderId": user_id, "senderNickname": nickname,
            "content": d.content, "burnAfter": burn, "createdAt": now
        })});
    }
}
