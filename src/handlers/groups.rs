use axum::{extract::{State, Path}, http::HeaderMap, Json};
use serde_json::json;
use serde::Deserialize;
use crate::{error::Result, models::{EnterGroupRequest, CreateGroupRequest}, handlers::auth::{get_claims, get_claims_full}, utils::hash_password, utils::check_permission, AppState};

#[derive(Debug, Deserialize)]
pub struct UpdateGroupRequest {
    pub description: Option<String>,
    pub announcement: Option<String>,
}

pub async fn enter_by_name(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<EnterGroupRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    if req.name.is_empty() { return Err(crate::error::AppError::BadRequest("请输入频道名".to_string())); }
    
    let group: Option<(String, String, Option<String>, Option<String>)> = sqlx::query_as("SELECT id, cipher_hash, description, announcement FROM groups WHERE name = ?")
        .bind(&req.name).fetch_optional(&state.db).await?;
    let group = group.ok_or(crate::error::AppError::NotFound)?;
    
    if !verify_cipher(&req.name, &group.1)? { return Err(crate::error::AppError::Auth("频道不存在".to_string())); }
    
    let member: Option<String> = sqlx::query_scalar("SELECT id FROM group_members WHERE group_id = ? AND user_id = ?")
        .bind(&group.0).bind(&claims.sub).fetch_optional(&state.db).await?;
    
    if member.is_none() {
        let member_id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO group_members (id, group_id, user_id) VALUES (?, ?, ?)")
            .bind(&member_id).bind(&group.0).bind(&claims.sub).execute(&state.db).await?;
    }
    
    Ok(Json(json!({"success":true,"data":{"id":group.0,"name":req.name,"description":group.2,"announcement":group.3}})))
}

fn verify_cipher(name: &str, hash: &str) -> Result<bool> {
    use argon2::{password_hash::PasswordHash, Argon2, password_hash::PasswordVerifier};
    let parsed = PasswordHash::new(hash).map_err(|e| crate::error::AppError::Internal(e.to_string()))?;
    Ok(Argon2::default().verify_password(name.as_bytes(), &parsed).is_ok())
}

pub async fn create_group(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<CreateGroupRequest>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "group_create")?;
    
    if req.name.is_empty() { return Err(crate::error::AppError::BadRequest("频道名不能为空".to_string())); }
    
    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM groups WHERE name = ?").bind(&req.name).fetch_optional(&state.db).await?;
    if exists.is_some() { return Err(crate::error::AppError::BadRequest("频道已存在".to_string())); }
    
    let hash = hash_password(&req.name)?;
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO groups (id, name, cipher_hash, owner_id) VALUES (?, ?, ?, ?)")
        .bind(&id).bind(&req.name).bind(&hash).bind(&claims.sub).execute(&state.db).await?;
    
    let member_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO group_members (id, group_id, user_id) VALUES (?, ?, ?)")
        .bind(&member_id).bind(&id).bind(&claims.sub).execute(&state.db).await?;
    
    Ok(Json(json!({"success":true,"data":{"id":id,"name":req.name}})))
}

pub async fn list_my_groups(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    let groups: Vec<(String, String)> = sqlx::query_as(
        "SELECT g.id, g.name FROM groups g JOIN group_members gm ON g.id = gm.group_id WHERE gm.user_id = ?"
    ).bind(&claims.sub).fetch_all(&state.db).await?;
    Ok(Json(json!({"success":true,"data":groups.iter().map(|g|json!({"id":g.0,"name":g.1})).collect::<Vec<_>>()})))
}

pub async fn list_all_groups(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "group_view")?;
    
    let groups: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT g.id, g.name, g.owner_id, g.created_at FROM groups g ORDER BY g.created_at DESC"
    ).fetch_all(&state.db).await?;
    
    let mut result = Vec::new();
    for g in groups {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM group_members WHERE group_id = ?")
            .bind(&g.0).fetch_one(&state.db).await?;
        result.push(json!({"id":g.0,"name":g.1,"ownerId":g.2,"memberCount":count,"createdAt":g.3}));
    }
    
    Ok(Json(json!({"success":true,"data":result})))
}

pub async fn delete_group(State(state): State<AppState>, headers: HeaderMap, Path(id): Path<String>) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    check_permission(&claims, "group_delete")?;
    
    sqlx::query("DELETE FROM messages WHERE group_id = ?").bind(&id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM group_members WHERE group_id = ?").bind(&id).execute(&state.db).await.ok();
    sqlx::query("DELETE FROM groups WHERE id = ?").bind(&id).execute(&state.db).await?;
    
    Ok(Json(json!({"success":true})))
}

/// 更新频道信息（描述、公告）
pub async fn update_group(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>,
    Json(req): Json<UpdateGroupRequest>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查是否是频道所有者或管理员
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&id).fetch_optional(&state.db).await?;
    
    let owner = owner.ok_or(crate::error::AppError::NotFound)?;
    
    if claims.role != "admin" && owner != claims.sub {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 更新频道信息
    if let Some(desc) = &req.description {
        if desc.len() > 200 {
            return Err(crate::error::AppError::BadRequest("描述不能超过200字".to_string()));
        }
        sqlx::query("UPDATE groups SET description = ? WHERE id = ?")
            .bind(desc).bind(&id).execute(&state.db).await?;
    }
    
    if let Some(ann) = &req.announcement {
        if ann.len() > 500 {
            return Err(crate::error::AppError::BadRequest("公告不能超过500字".to_string()));
        }
        sqlx::query("UPDATE groups SET announcement = ? WHERE id = ?")
            .bind(ann).bind(&id).execute(&state.db).await?;
    }
    
    // 获取更新后的信息
    let info: Option<(Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT description, announcement FROM groups WHERE id = ?"
    )
    .bind(&id).fetch_optional(&state.db).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "description": info.as_ref().and_then(|i| i.0.clone()),
            "announcement": info.as_ref().and_then(|i| i.1.clone())
        }
    })))
}

/// 获取频道信息
pub async fn get_group_info(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    
    // 检查是否是频道成员
    let member: Option<String> = sqlx::query_scalar(
        "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
    )
    .bind(&id).bind(&claims.sub).fetch_optional(&state.db).await?;
    
    if member.is_none() {
        return Err(crate::error::AppError::Forbidden);
    }
    
    let info: Option<(String, Option<String>, Option<String>, String, i64)> = sqlx::query_as(
        "SELECT name, description, announcement, owner_id, (SELECT COUNT(*) FROM group_members WHERE group_id = ?) FROM groups WHERE id = ?"
    )
    .bind(&id).bind(&id).fetch_optional(&state.db).await?;
    
    let info = info.ok_or(crate::error::AppError::NotFound)?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "id": id,
            "name": info.0,
            "description": info.1,
            "announcement": info.2,
            "ownerId": info.3,
            "memberCount": info.4
        }
    })))
}

/// 获取频道成员列表
pub async fn get_group_members(
    State(state): State<AppState>, 
    headers: HeaderMap, 
    Path(id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims(&headers, &state.config)?;
    
    // 检查是否是频道成员
    let member: Option<String> = sqlx::query_scalar(
        "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
    )
    .bind(&id).bind(&claims.sub).fetch_optional(&state.db).await?;
    
    if member.is_none() {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 获取成员列表
    let members: Vec<(String, String, Option<String>, String)> = sqlx::query_as(r#"
        SELECT u.id, u.nickname, u.avatar, u.role
        FROM group_members gm
        JOIN users u ON gm.user_id = u.id
        WHERE gm.group_id = ?
        ORDER BY u.role = 'admin' DESC, gm.joined_at ASC
    "#)
    .bind(&id)
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": members.iter().map(|m| json!({
            "id": m.0,
            "nickname": m.1,
            "avatar": m.2,
            "role": m.3,
            "isOnline": true  // 简化处理，实际可以通过WebSocket连接状态判断
        })).collect::<Vec<_>>()
    })))
}

// 邀请链接功能
use rand::Rng;

/// 创建邀请链接
pub async fn create_invite_link(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(group_id): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 检查是否是频道所有者或管理员
    let owner: Option<String> = sqlx::query_scalar("SELECT owner_id FROM groups WHERE id = ?")
        .bind(&group_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(crate::error::AppError::NotFound)?;
    
    if owner.as_deref() != Some(&claims.sub) && claims.role != "admin" {
        return Err(crate::error::AppError::Forbidden);
    }
    
    // 生成随机邀请码
    let code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO invite_links (id, code, group_id, created_by) VALUES (?, ?, ?, ?)")
        .bind(&id).bind(&code).bind(&group_id).bind(&claims.sub)
        .execute(&state.db)
        .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "code": code,
            "link": format!("/invite/{}", code)
        }
    })))
}

/// 通过邀请链接加入频道
pub async fn join_by_invite(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(code): Path<String>
) -> Result<Json<serde_json::Value>> {
    let claims = get_claims_full(&headers, &state).await?;
    
    // 获取邀请链接信息
    let invite: Option<(String, String, i64, i64, Option<String>)> = sqlx::query_as(
        "SELECT id, group_id, max_uses, uses, expires_at FROM invite_links WHERE code = ?"
    )
    .bind(&code)
    .fetch_optional(&state.db)
    .await?;
    
    let invite = invite.ok_or(crate::error::AppError::BadRequest("邀请链接无效".to_string()))?;
    
    // 检查是否过期
    if let Some(expires) = &invite.4 {
        if chrono::DateTime::parse_from_rfc3339(expires)
            .map(|d| d.timestamp() < chrono::Utc::now().timestamp())
            .unwrap_or(false)
        {
            return Err(crate::error::AppError::BadRequest("邀请链接已过期".to_string()));
        }
    }
    
    // 检查使用次数
    if invite.2 > 0 && invite.3 >= invite.2 {
        return Err(crate::error::AppError::BadRequest("邀请链接已用完".to_string()));
    }
    
    // 检查是否已是成员
    let member: Option<String> = sqlx::query_scalar(
        "SELECT id FROM group_members WHERE group_id = ? AND user_id = ?"
    )
    .bind(&invite.1).bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;
    
    if member.is_some() {
        return Err(crate::error::AppError::BadRequest("你已经是频道成员".to_string()));
    }
    
    // 加入频道
    let member_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO group_members (id, group_id, user_id) VALUES (?, ?, ?)")
        .bind(&member_id).bind(&invite.1).bind(&claims.sub)
        .execute(&state.db)
        .await?;
    
    // 更新使用次数
    sqlx::query("UPDATE invite_links SET uses = uses + 1 WHERE id = ?")
        .bind(&invite.0)
        .execute(&state.db)
        .await?;
    
    // 获取频道信息
    let group_name: String = sqlx::query_scalar("SELECT name FROM groups WHERE id = ?")
        .bind(&invite.1)
        .fetch_one(&state.db)
        .await?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "groupId": invite.1,
            "groupName": group_name
        }
    })))
}
