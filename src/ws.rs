use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{AppState, error::AppError, models::Claims, utils::verify_token, utils::is_muted, utils::check_rate_limit};
use crate::broadcast::WsMessage;

#[derive(Debug, Deserialize)]
pub struct WsQuery { pub token: String }

pub async fn ws_handler(
    ws: WebSocketUpgrade, 
    State(state): State<AppState>, 
    Query(q): Query<WsQuery>
) -> Result<Response, AppError> {
    let claims = verify_token(&q.token, &state.config).map_err(|_| AppError::Unauthorized)?;
    
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
    
    sqlx::query("UPDATE users SET online = 1 WHERE id = ?")
        .bind(&claims.sub).execute(&state.db).await.ok();
    
    Ok(ws.on_upgrade(move |s| handle(s, state, claims)))
}

async fn handle(socket: WebSocket, state: AppState, claims: Claims) {
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(sender));
    let user_id = claims.sub.clone();
    let nickname = claims.nickname.clone();
    let token_version = claims.token_version;
    
    tracing::info!("WS connected: {}", nickname);
    
    // 获取用户加入的频道列表
    let groups: Vec<String> = match sqlx::query_scalar(
        "SELECT group_id FROM group_members WHERE user_id = ?"
    )
    .bind(&user_id)
    .fetch_all(&state.db)
    .await
    {
        Ok(g) => g,
        Err(_) => return,
    };
    
    // 订阅所有频道
    let mut group_receivers: Vec<(String, tokio::sync::broadcast::Receiver<WsMessage>)> = groups
        .iter()
        .map(|gid| (gid.clone(), state.broadcast.subscribe(gid)))
        .collect();
    
    // 也订阅全局通道
    let mut global_rx = state.broadcast.subscribe_global();
    
    let state2 = state.clone();
    let user_id2 = user_id.clone();
    let nickname2 = nickname.clone();
    let token_version2 = token_version;
    let sender2 = sender.clone();
    
    // 接收客户端消息
    let recv = async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // 处理 ping
                if text.contains(r#""event":"ping""#) {
                    let mut s = sender2.lock().await;
                    let _ = s.send(Message::Text(r#"{"event":"pong"}"#.to_string())).await;
                    continue;
                }
                
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
    
    // 发送消息到客户端
    let send = async {
        loop {
            // 检查全局消息
            match global_rx.try_recv() {
                Ok(m) => {
                    let json = serde_json::to_string(&m).unwrap();
                    let mut s = sender.lock().await;
                    if s.send(Message::Text(json)).await.is_err() { break; }
                    continue;
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(_) => break,
            }
            
            // 检查各频道消息
            let mut got_message = false;
            for (_gid, rx) in &mut group_receivers {
                match rx.try_recv() {
                    Ok(m) => {
                        let json = serde_json::to_string(&m).unwrap();
                        let mut s = sender.lock().await;
                        if s.send(Message::Text(json)).await.is_err() { 
                            return;
                        }
                        got_message = true;
                        break;
                    }
                    Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                    Err(_) => {}
                }
            }
            
            if !got_message {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        }
    };
    
    tokio::select! { _ = recv => {}, _ = send => {} }
    
    // 设置离线
    sqlx::query("UPDATE users SET online = 0 WHERE id = ?")
        .bind(&user_id).execute(&state.db).await.ok();
    
    tracing::info!("WS disconnected: {}", nickname);
}

async fn handle_msg(state: &AppState, user_id: &str, nickname: &str, msg: WsMessage) {
    // 处理 ping
    if msg.event == "ping" {
        let _ = state.broadcast.broadcast_global(WsMessage { 
            event: "pong".into(), 
            data: serde_json::json!({}) 
        });
        return;
    }
    
    // 处理输入状态
    if msg.event == "typing" {
        #[derive(serde::Deserialize)]
        struct TypingData { 
            #[serde(rename = "groupId")]
            group_id: String,
            is_typing: Option<bool>
        }
        
        if let Ok(d) = serde_json::from_value::<TypingData>(msg.data.clone()) {
            let _ = state.broadcast.broadcast_to_group(&d.group_id, WsMessage {
                event: "typing".into(),
                data: serde_json::json!({
                    "groupId": d.group_id,
                    "userId": user_id,
                    "nickname": nickname,
                    "isTyping": d.is_typing.unwrap_or(true)
                })
            });
        }
        return;
    }
    
    if msg.event != "message" { return; }
    
    #[derive(serde::Deserialize)]
    struct MsgData { 
        group_id: String, 
        content: String, 
        burn_after: Option<i64>,
        msg_type: Option<String>,
        file_name: Option<String>,
        file_size: Option<i64>,
        reply_to: Option<String>,
    }
    
    let Ok(d) = serde_json::from_value::<MsgData>(msg.data) else { return };
    if d.content.is_empty() || d.content.len() > 5000 { return; }
    
    if is_muted(&state.db, user_id).await.unwrap_or(false) {
        return;
    }
    
    if !check_rate_limit(&state.db, user_id, "message", &state.config).await.unwrap_or(false) {
        return;
    }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&d.group_id).bind(user_id).fetch_optional(&state.db).await.ok().flatten();
    if member.is_none() { return; }
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let burn = d.burn_after.unwrap_or(0);
    let msg_type = d.msg_type.unwrap_or_else(|| "text".to_string());
    let reply_to = d.reply_to.clone();
    
    if sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, file_name, file_size, burn_after, reply_to, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(user_id).bind(&d.group_id).bind(&d.content).bind(&msg_type)
        .bind(&d.file_name).bind(d.file_size.unwrap_or(0)).bind(burn).bind(&reply_to).bind(&now)
        .execute(&state.db).await.is_ok() {
        
        let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
            .bind(user_id).fetch_optional(&state.db).await.ok().flatten();
        
        // 获取引用消息的信息
        let reply_info: Option<(String, String)> = if let Some(ref_msg_id) = &reply_to {
            sqlx::query_as("SELECT m.content, u.nickname FROM messages m JOIN users u ON m.sender_id = u.id WHERE m.id = ?")
                .bind(ref_msg_id)
                .fetch_optional(&state.db)
                .await
                .ok()
                .flatten()
        } else {
            None
        };
        
        let _ = state.broadcast.broadcast_to_group(&d.group_id, WsMessage { 
            event: "message".into(), 
            data: serde_json::json!({
                "id": id, 
                "groupId": d.group_id, 
                "senderId": user_id, 
                "senderNickname": nickname,
                "senderAvatar": avatar, 
                "content": d.content, 
                "msgType": msg_type,
                "fileName": d.file_name, 
                "fileSize": d.file_size, 
                "burnAfter": burn,
                "replyTo": reply_to,
                "replyInfo": reply_info.as_ref().map(|(content, nick)| serde_json::json!({
                    "content": content,
                    "senderNickname": nick
                })),
                "createdAt": now
            })
        });
    }
}
