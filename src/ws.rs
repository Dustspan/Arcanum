use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures_util::{SinkExt, StreamExt};
use crate::{models::Claims, broadcast::WsMessage, AppState, utils::verify_token};

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsQuery>,
) -> crate::error::Result<impl IntoResponse> {
    // 验证 token
    let claims = verify_token(&query.token, &state.config)?;
    
    // 检查用户状态
    let user_status: Option<(String, i64)> = sqlx::query_as(
        "SELECT account_status, token_version FROM users WHERE id = ?"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;
    
    match user_status {
        Some((status, ver)) => {
            if status == "banned" {
                return Err(crate::error::AppError::Banned);
            }
            if ver != claims.token_version {
                return Err(crate::error::AppError::Kicked);
            }
        }
        None => return Err(crate::error::AppError::Kicked),
    }
    
    // 设置在线状态
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
    
    // 推送离线私聊消息
    let offline_messages: Vec<(String, String, String, String, Option<String>, Option<String>, Option<i64>, String)> = match sqlx::query_as(
        "SELECT id, sender_id, receiver_id, content, type, file_name, file_size, created_at FROM direct_messages WHERE receiver_id = ? AND read = 0 ORDER BY created_at ASC"
    )
    .bind(&user_id)
    .fetch_all(&state.db)
    .await
    {
        Ok(msgs) => msgs,
        Err(_) => Vec::new(),
    };
    
    if !offline_messages.is_empty() {
        let msg_count = offline_messages.len();
        
        // 标记为已读
        let _ = sqlx::query("UPDATE direct_messages SET read = 1 WHERE receiver_id = ? AND read = 0")
            .bind(&user_id)
            .execute(&state.db)
            .await;
        
        // 推送离线消息
        for msg in &offline_messages {
            // 获取发送者信息
            let sender_nickname: Option<String> = sqlx::query_scalar("SELECT nickname FROM users WHERE id = ?")
                .bind(&msg.1)
                .fetch_optional(&state.db)
                .await
                .ok()
                .flatten();
            
            let sender_avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
                .bind(&msg.1)
                .fetch_optional(&state.db)
                .await
                .ok()
                .flatten();
            
            let offline_msg = WsMessage {
                event: "direct_message".into(),
                data: serde_json::json!({
                    "id": msg.0,
                    "senderId": msg.1,
                    "senderNickname": sender_nickname,
                    "senderAvatar": sender_avatar,
                    "receiverId": msg.2,
                    "content": msg.3,
                    "msgType": msg.4,
                    "fileName": msg.5,
                    "fileSize": msg.6,
                    "createdAt": msg.7,
                    "offline": true
                })
            };
            
            let mut s = sender.lock().await;
            if let Ok(json) = serde_json::to_string(&offline_msg) {
                let _ = s.send(Message::Text(json)).await;
            }
        }
        
        tracing::info!("Pushed {} offline messages to {}", msg_count, nickname);
    }
    
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
    
    // 订阅用户自己的消息通道（用于私聊）
    let mut user_rx = state.broadcast.subscribe_user(&user_id);
    
    // 订阅全局通道
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
            // 检查用户私聊消息
            match user_rx.try_recv() {
                Ok(m) => {
                    let json = serde_json::to_string(&m).unwrap();
                    let mut s = sender.lock().await;
                    if s.send(Message::Text(json)).await.is_err() { return; }
                    continue;
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(_) => {}
            }
            
            // 检查全局消息
            match global_rx.try_recv() {
                Ok(m) => {
                    let json = serde_json::to_string(&m).unwrap();
                    let mut s = sender.lock().await;
                    if s.send(Message::Text(json)).await.is_err() { return; }
                    continue;
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(_) => {}
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
    
    // 清理用户的广播通道
    state.broadcast.remove_user(&user_id);
    
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
                    "isTyping": d.is_typing.unwrap_or(false)
                })
            });
        }
    }
}
