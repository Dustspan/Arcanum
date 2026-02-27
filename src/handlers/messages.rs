use axum::{extract::{State, Path, Multipart}, http::HeaderMap, Json};
use serde_json::json;
use crate::{
    error::Result, 
    models::SendMessageRequest, 
    handlers::auth::get_claims_full, 
    utils::{check_permission, is_muted, check_rate_limit},
    broadcast::WsMessage,
    AppState
};

const MAX_MSG_LEN: usize = 5000;

/// 解析消息中的@提及
fn parse_mentions(content: &str) -> Vec<String> {
    let mut mentions = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '@' {
            let mut name = String::new();
            i += 1;
            // 收集@后面的用户名（允许中文、英文、数字、下划线）
            while i < chars.len() {
                let c = chars[i];
                // 检查是否是中文字符 (Unicode范围)
                let is_chinese = (c >= '\u{4E00}' && c <= '\u{9FFF}') || 
                                 (c >= '\u{3400}' && c <= '\u{4DBF}');
                if c.is_alphanumeric() || c == '_' || is_chinese {
                    name.push(c);
                    i += 1;
                } else {
                    break;
                }
            }
            if !name.is_empty() && name.len() <= 20 {
                mentions.push(name);
            }
        } else {
            i += 1;
        }
    }
    
    mentions
}

pub async fn send_message(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<SendMessageRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    if is_muted(&state.db, &claims.sub).await? {
        return Err(crate::error::AppError::BadRequest("你已被禁言".to_string()));
    }
    
    if !check_rate_limit(&state.db, &claims.sub, "message", &state.config).await? {
        return Err(crate::error::AppError::BadRequest("发送太快，请稍后再试".to_string()));
    }
    
    if req.content.is_empty() { return Err(crate::error::AppError::BadRequest("消息不能为空".to_string())); }
    if req.content.len() > MAX_MSG_LEN { return Err(crate::error::AppError::BadRequest("消息太长".to_string())); }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&req.group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let burn = req.burn_after.unwrap_or(0);
    let msg_type = req.msg_type.unwrap_or_else(|| "text".to_string());
    let reply_to = req.reply_to.clone();
    
    sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, file_name, file_size, burn_after, reply_to, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(&claims.sub).bind(&req.group_id).bind(&req.content).bind(&msg_type)
        .bind(&req.file_name).bind(req.file_size.unwrap_or(0)).bind(burn).bind(&reply_to).bind(&now)
        .execute(&state.db).await?;
    
    let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
        .bind(&claims.sub).fetch_optional(&state.db).await?
        .flatten();
    
    // 获取引用消息的信息
    let reply_info: Option<(String, String, String)> = if let Some(ref_msg_id) = &reply_to {
        sqlx::query_as("SELECT m.content, u.nickname, m.sender_id FROM messages m JOIN users u ON m.sender_id = u.id WHERE m.id = ?")
            .bind(ref_msg_id)
            .fetch_optional(&state.db)
            .await
            .ok()
            .flatten()
    } else {
        None
    };
    
    let _ = state.broadcast.broadcast_to_group(&req.group_id, WsMessage { 
        event: "message".into(), 
        data: json!({
            "id": id, 
            "groupId": req.group_id, 
            "senderId": claims.sub, 
            "senderNickname": claims.nickname,
            "senderAvatar": avatar, 
            "content": req.content, 
            "msgType": msg_type,
            "fileName": req.file_name, 
            "fileSize": req.file_size, 
            "burnAfter": burn,
            "replyTo": reply_to,
            "replyInfo": reply_info.as_ref().map(|r| json!({
                "content": r.0,
                "senderNickname": r.1,
                "senderId": r.2
            })),
            "createdAt": now
        })
    });
    
    // 解析@提及
    let mentions = parse_mentions(&req.content);
    for mentioned_nick in mentions {
        // 查找被提及的用户
        let mentioned_user: Option<String> = sqlx::query_scalar(
            "SELECT id FROM users WHERE nickname = ?"
        )
        .bind(&mentioned_nick)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten();
        
        if let Some(mentioned_id) = mentioned_user {
            // 检查用户是否在频道中
            let is_member: Option<String> = sqlx::query_scalar(
                "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
            )
            .bind(&req.group_id)
            .bind(&mentioned_id)
            .fetch_optional(&state.db)
            .await
            .ok()
            .flatten();
            
            if is_member.is_some() {
                let mention_id = uuid::Uuid::new_v4().to_string();
                let _ = sqlx::query(
                    "INSERT OR IGNORE INTO mentions (id, message_id, user_id, mentioned_by, group_id) VALUES (?, ?, ?, ?, ?)"
                )
                .bind(&mention_id)
                .bind(&id)
                .bind(&mentioned_id)
                .bind(&claims.sub)
                .bind(&req.group_id)
                .execute(&state.db)
                .await;
                
                // 发送提及通知
                let _ = state.broadcast.broadcast_to_user(&mentioned_id, WsMessage {
                    event: "mention".into(),
                    data: json!({
                        "messageId": id,
                        "groupId": req.group_id,
                        "mentionedBy": claims.nickname,
                        "content": req.content.chars().take(50).collect::<String>()
                    })
                });
            }
        }
    }
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "senderId": claims.sub,
            "senderNickname": claims.nickname,
            "senderAvatar": avatar,
            "content": req.content,
            "msgType": msg_type,
            "fileName": req.file_name,
            "fileSize": req.file_size,
            "burnAfter": burn,
            "replyTo": reply_to,
            "createdAt": now
        }
    })))
}

use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub before: Option<String>,  // 加载此时间之前的消息
    pub limit: Option<i64>,       // 每页数量，默认50
}

pub async fn get_messages(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(group_id): Path<String>,
    Query(pagination): Query<PaginationParams>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    let limit = pagination.limit.unwrap_or(50).min(100);
    
    let messages: Vec<(String, String, String, String, String, Option<String>, i64, i64, Option<String>, i64, String, Option<String>)> = 
        if let Some(before) = &pagination.before {
            // 分页查询：加载指定时间之前的消息
            sqlx::query_as(r#"
                SELECT m.id, m.sender_id, u.nickname, m.content, m.type, m.file_name, m.file_size, m.burn_after, m.reply_to, m.pinned, m.created_at, u.avatar
                FROM messages m 
                JOIN users u ON m.sender_id = u.id 
                WHERE m.group_id = ? AND m.created_at < ?
                ORDER BY m.pinned DESC, m.created_at DESC
                LIMIT ?
            "#)
                .bind(&group_id).bind(before).bind(limit)
                .fetch_all(&state.db).await?
        } else {
            // 初始加载：获取最新的消息
            sqlx::query_as(r#"
                SELECT m.id, m.sender_id, u.nickname, m.content, m.type, m.file_name, m.file_size, m.burn_after, m.reply_to, m.pinned, m.created_at, u.avatar
                FROM messages m 
                JOIN users u ON m.sender_id = u.id 
                WHERE m.group_id = ? 
                ORDER BY m.pinned DESC, m.created_at DESC
                LIMIT ?
            "#)
                .bind(&group_id).bind(limit)
                .fetch_all(&state.db).await?
        };
    
    // 获取总数
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE group_id = ?")
        .bind(&group_id)
        .fetch_one(&state.db)
        .await?;
    
    // 反转顺序（从旧到新显示）
    let mut messages = messages;
    messages.reverse();
    
    let has_more = messages.len() as i64 == limit && (messages.len() as i64) < total;
    
    // 获取引用消息的信息
    let reply_ids: Vec<String> = messages.iter()
        .filter_map(|m| m.8.clone())
        .collect();
    
    let reply_infos: std::collections::HashMap<String, (String, String)> = if !reply_ids.is_empty() {
        let placeholders = reply_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT m.id, m.content, u.nickname FROM messages m JOIN users u ON m.sender_id = u.id WHERE m.id IN ({})",
            placeholders
        );
        let mut q = sqlx::query_as::<_, (String, String, String)>(&query);
        for id in &reply_ids {
            q = q.bind(id);
        }
        q.fetch_all(&state.db)
            .await
            .map(|rows| rows.into_iter().map(|r| (r.0, (r.1, r.2))).collect())
            .unwrap_or_default()
    } else {
        std::collections::HashMap::new()
    };
    
    Ok(Json(json!({
        "success": true,
        "data": messages.iter().map(|m| {
            let reply_info = m.8.as_ref().and_then(|id| reply_infos.get(id)).map(|(content, nick)| json!({
                "content": content,
                "senderNickname": nick
            }));
            json!({
                "id": m.0,
                "senderId": m.1,
                "senderNickname": m.2,
                "senderAvatar": m.11,
                "content": m.3,
                "msgType": m.4,
                "fileName": m.5,
                "fileSize": m.6,
                "burnAfter": m.7,
                "replyTo": m.8,
                "replyInfo": reply_info,
                "pinned": m.9 == 1,
                "createdAt": m.10
            })
        }).collect::<Vec<_>>(),
        "pagination": {
            "total": total,
            "hasMore": has_more,
            "oldestCreatedAt": messages.first().map(|m| m.10.clone()),
            "newestCreatedAt": messages.last().map(|m| m.10.clone())
        }
    })))
}

pub async fn clear_messages(State(state): State<AppState>, headers: HeaderMap, Path(group_id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&group_id).fetch_optional(&state.db).await?;
    let owner = owner.ok_or(crate::error::AppError::NotFound)?;
    
    if claims.role != "admin" && owner != claims.sub {
        check_permission(&claims, "message_delete")?;
    }
    
    sqlx::query("DELETE FROM messages WHERE group_id = ?").bind(&group_id).execute(&state.db).await?;
    Ok(Json(json!({"success": true})))
}

/// 撤回消息 - 用户只能撤回自己发送的消息，且在时间限制内
pub async fn recall_message(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取消息信息
    let msg: Option<(String, String, String)> = sqlx::query_as(
        "SELECT sender_id, group_id, created_at FROM messages WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;
    
    let msg = msg.ok_or_else(|| crate::error::AppError::NotFound)?;
    
    // 检查是否是消息发送者
    if msg.0 != claims.sub {
        // 如果不是发送者，检查是否有删除权限
        check_permission(&claims, "message_delete")?;
    } else {
        // 如果是发送者，检查时间限制（2分钟内可撤回）
        if let Ok(created_at) = chrono::DateTime::parse_from_rfc3339(&msg.2) {
            let elapsed = chrono::Utc::now() - created_at.with_timezone(&chrono::Utc);
            if elapsed.num_minutes() > 2 {
                return Err(crate::error::AppError::BadRequest("消息发送超过2分钟，无法撤回".to_string()));
            }
        }
    }
    
    // 删除消息
    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(&id).execute(&state.db).await?;
    
    // 广播撤回通知
    let _ = state.broadcast.broadcast_to_group(&msg.1, WsMessage {
        event: "message_recall".into(),
        data: json!({
            "id": id,
            "groupId": msg.1
        })
    });
    
    Ok(Json(json!({"success": true})))
}

pub async fn delete_message(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "message_delete")?;
    
    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(&id).execute(&state.db).await?;
    
    Ok(Json(json!({"success": true})))
}

pub async fn upload_file(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(group_id): Path<String>,
    mut multipart: Multipart
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    check_permission(&claims, "file_upload")?;
    
    if is_muted(&state.db, &claims.sub).await? {
        return Err(crate::error::AppError::BadRequest("你已被禁言".to_string()));
    }
    
    if !check_rate_limit(&state.db, &claims.sub, "file_upload", &state.config).await? {
        return Err(crate::error::AppError::BadRequest("上传太快，请稍后再试".to_string()));
    }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group_id).bind(&claims.sub).fetch_optional(&state.db).await?;
    if member.is_none() { return Err(crate::error::AppError::Forbidden); }
    
    while let Some(field) = multipart.next_field().await.map_err(|e| 
        crate::error::AppError::BadRequest(format!("上传失败: {}", e))
    )? {
        let content_type = field.content_type().map(|s| s.to_string()).unwrap_or_default();
        let file_name = field.file_name().map(|s| s.to_string()).unwrap_or_else(|| "file".to_string());
        
        let is_image = content_type.starts_with("image/");
        let is_text = content_type.starts_with("text/") || file_name.to_lowercase().ends_with(".txt");
        
        if !is_image && !is_text {
            return Err(crate::error::AppError::BadRequest("只支持图片和文本文件".to_string()));
        }
        
        let data = field.bytes().await.map_err(|e| 
            crate::error::AppError::BadRequest(format!("读取失败: {}", e))
        )?;
        
        if data.len() > state.config.max_file_size {
            return Err(crate::error::AppError::BadRequest("文件太大".to_string()));
        }
        
        // 使用文件存储而不是base64
        let file_url = if is_image {
            state.storage.save_image(&data, &content_type)
                .map_err(|e| crate::error::AppError::Internal(format!("保存失败: {}", e)))?
        } else {
            state.storage.save_file(&data, &file_name)
                .map_err(|e| crate::error::AppError::Internal(format!("保存失败: {}", e)))?
        };
        
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let msg_type = if is_image { "image" } else { "file" };
        
        sqlx::query("INSERT INTO messages (id, sender_id, group_id, content, type, file_name, file_size, burn_after, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 0, ?)")
            .bind(&id).bind(&claims.sub).bind(&group_id).bind(&file_url).bind(msg_type)
            .bind(&file_name).bind(data.len() as i64).bind(&now)
            .execute(&state.db).await?;
        
        let avatar: Option<String> = sqlx::query_scalar("SELECT avatar FROM users WHERE id = ?")
            .bind(&claims.sub).fetch_optional(&state.db).await?
            .flatten();
        
        let _ = state.broadcast.broadcast_to_group(&group_id, WsMessage { 
            event: "message".into(), 
            data: json!({
                "id": id, 
                "groupId": group_id, 
                "senderId": claims.sub, 
                "senderNickname": claims.nickname,
                "senderAvatar": avatar, 
                "content": file_url, 
                "msgType": msg_type,
                "fileName": file_name, 
                "fileSize": data.len(), 
                "burnAfter": 0, 
                "createdAt": now
            })
        });
        
        return Ok(Json(json!({
            "success": true,
            "data": {
                "id": id,
                "groupId": group_id,
                "senderId": claims.sub,
                "senderNickname": claims.nickname,
                "senderAvatar": avatar,
                "content": file_url,
                "msgType": msg_type,
                "fileName": file_name,
                "fileSize": data.len(),
                "createdAt": now
            }
        })));
    }
    
    Err(crate::error::AppError::BadRequest("未找到文件".to_string()))
}

/// 标记消息已读
pub async fn mark_read(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(msg_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查消息是否存在且用户有权限查看
    let msg: Option<(String,)> = sqlx::query_as(
        "SELECT group_id FROM messages WHERE id = ?"
    )
    .bind(&msg_id)
    .fetch_optional(&state.db)
    .await?;
    
    let msg = msg.ok_or_else(|| crate::error::AppError::NotFound)?;
    
    // 检查用户是否是频道成员
    let member: Option<String> = sqlx::query_scalar(
        "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
    )
    .bind(&msg.0).bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;
    
    if member.is_none() {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 标记已读
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT OR IGNORE INTO message_reads (id, message_id, user_id) VALUES (?, ?, ?)")
        .bind(&id).bind(&msg_id).bind(&claims.sub)
        .execute(&state.db)
        .await?;
    
    // 获取已读人数并广播
    let read_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM message_reads WHERE message_id = ?")
        .bind(&msg_id)
        .fetch_one(&state.db)
        .await?;
    
    let _ = state.broadcast.broadcast_to_group(&msg.0, WsMessage {
        event: "message_read".into(),
        data: json!({
            "id": msg_id,
            "groupId": msg.0,
            "userId": claims.sub,
            "readCount": read_count
        })
    });
    
    Ok(Json(json!({"success": true, "data": {"readCount": read_count}})))
}

/// 批量标记频道消息已读
pub async fn mark_group_read(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(group_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查用户是否是频道成员
    let member: Option<String> = sqlx::query_scalar(
        "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
    )
    .bind(&group_id).bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;
    
    if member.is_none() {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 获取频道中所有未读消息
    let unread_msgs: Vec<String> = sqlx::query_scalar(r#"
        SELECT m.id FROM messages m
        WHERE m.group_id = ? AND m.sender_id != ?
        AND NOT EXISTS (
            SELECT 1 FROM message_reads mr WHERE mr.message_id = m.id AND mr.user_id = ?
        )
    "#)
    .bind(&group_id).bind(&claims.sub).bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    // 批量标记已读
    for msg_id in &unread_msgs {
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT OR IGNORE INTO message_reads (id, message_id, user_id) VALUES (?, ?, ?)")
            .bind(&id).bind(msg_id).bind(&claims.sub)
            .execute(&state.db)
            .await.ok();
    }
    
    Ok(Json(json!({"success": true, "data": {"markedCount": unread_msgs.len()}})))
}

/// 搜索消息
#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    pub q: String,          // 搜索关键词
    pub limit: Option<i64>, // 结果数量限制
}

pub async fn search_messages(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(group_id): Path<String>,
    Query(params): Query<SearchParams>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查用户是否是频道成员
    let member: Option<String> = sqlx::query_scalar(
        "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
    )
    .bind(&group_id).bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;
    
    if member.is_none() {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 验证搜索关键词
    let keyword = params.q.trim();
    if keyword.is_empty() || keyword.len() < 2 {
        return Err(crate::error::AppError::BadRequest("搜索关键词至少2个字符".to_string()));
    }
    if keyword.len() > 50 {
        return Err(crate::error::AppError::BadRequest("搜索关键词最多50个字符".to_string()));
    }
    
    let limit = params.limit.unwrap_or(20).min(50);
    let search_pattern = format!("%{}%", keyword);
    
    // 搜索消息内容
    let messages: Vec<(String, String, String, String, String, Option<String>, i64, String, Option<String>)> = 
        sqlx::query_as(r#"
            SELECT m.id, m.sender_id, u.nickname, m.content, m.type, m.file_name, m.file_size, m.created_at, u.avatar
            FROM messages m 
            JOIN users u ON m.sender_id = u.id 
            WHERE m.group_id = ? AND m.content LIKE ?
            ORDER BY m.created_at DESC
            LIMIT ?
        "#)
        .bind(&group_id).bind(&search_pattern).bind(limit)
        .fetch_all(&state.db)
        .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": messages.iter().map(|m| json!({
            "id": m.0,
            "senderId": m.1,
            "senderNickname": m.2,
            "senderAvatar": m.8,
            "content": m.3,
            "msgType": m.4,
            "fileName": m.5,
            "fileSize": m.6,
            "createdAt": m.7
        })).collect::<Vec<_>>(),
        "keyword": keyword,
        "count": messages.len()
    })))
}

/// 置顶/取消置顶消息
pub async fn toggle_pin_message(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取消息信息
    let msg: Option<(String, i64)> = sqlx::query_as("SELECT group_id, pinned FROM messages WHERE id = ?")
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;
    
    let msg = msg.ok_or(crate::error::AppError::NotFound)?;
    
    // 检查权限：管理员或频道所有者
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&msg.0)
        .fetch_optional(&state.db)
        .await?;
    
    let owner = owner.ok_or(crate::error::AppError::NotFound)?;
    
    if claims.role != "admin" && owner != claims.sub {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 切换置顶状态
    let new_pinned = if msg.1 == 0 { 1 } else { 0 };
    sqlx::query("UPDATE messages SET pinned = ? WHERE id = ?")
        .bind(new_pinned).bind(&id)
        .execute(&state.db)
        .await?;
    
    // 广播置顶状态变更
    let _ = state.broadcast.broadcast_to_group(&msg.0, WsMessage {
        event: "message_pin".into(),
        data: json!({
            "id": id,
            "groupId": msg.0,
            "pinned": new_pinned == 1
        })
    });
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "pinned": new_pinned == 1
        }
    })))
}

/// 获取用户的提及列表
pub async fn get_mentions(
    State(state): State<AppState>, 
    headers: HeaderMap
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    let mentions: Vec<(String, String, String, String, String, i64)> = sqlx::query_as(r#"
        SELECT m.id, m.message_id, u.nickname, g.name, msg.content, m.read
        FROM mentions m
        JOIN users u ON m.mentioned_by = u.id
        JOIN groups g ON m.group_id = g.id
        JOIN messages msg ON m.message_id = msg.id
        WHERE m.user_id = ?
        ORDER BY m.created_at DESC
        LIMIT 50
    "#)
    .bind(&claims.sub)
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": mentions.iter().map(|m| json!({
            "id": m.0,
            "messageId": m.1,
            "mentionedBy": m.2,
            "groupName": m.3,
            "content": m.4.chars().take(100).collect::<String>(),
            "read": m.5 == 1
        })).collect::<Vec<_>>()
    })))
}

/// 标记提及为已读
pub async fn mark_mention_read(
    State(state): State<AppState>, 
    headers: HeaderMap,
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    sqlx::query("UPDATE mentions SET read = 1 WHERE id = ? AND user_id = ?")
        .bind(&id).bind(&claims.sub)
        .execute(&state.db)
        .await?;
    
    Ok(Json(json!({"success": true})))
}
