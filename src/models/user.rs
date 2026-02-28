use serde::{Deserialize, Serialize};

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest { 
    pub uid: String, 
    pub password: String 
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest { 
    pub uid: Option<String>, 
    pub nickname: String, 
    pub password: String 
}

/// 授权权限请求
#[derive(Debug, Deserialize)]
pub struct GrantPermissionRequest {
    pub permission_name: String,
}

/// 禁言用户请求
#[derive(Debug, Deserialize)]
pub struct MuteUserRequest {
    pub duration_minutes: i64,
}

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims { 
    pub sub: String,        // 用户ID
    pub uid: String,        // 用户UID
    pub nickname: String,   // 昵称
    pub role: String,       // 角色: admin/member
    pub permissions: Vec<String>, // 权限列表
    pub token_version: i64, // Token版本（用于踢出）
    pub exp: i64            // 过期时间
}

/// 用户信息（用于API返回）
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub uid: String,
    pub nickname: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar: Option<String>,
    pub role: String,
    pub account_status: String,
    pub muted_until: Option<String>,
    pub token_version: i64,
    pub online: i64,
    pub last_ip: Option<String>,
    pub created_at: String,
}
