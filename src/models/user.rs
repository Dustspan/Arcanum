use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest { pub uid: String, pub password: String }

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest { 
    pub uid: Option<String>, 
    pub nickname: String, 
    pub password: String 
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GrantPermissionRequest {
    pub permission_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MuteUserRequest {
    pub duration_minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims { 
    pub sub: String, 
    pub uid: String, 
    pub nickname: String, 
    pub role: String,
    pub permissions: Vec<String>,
    pub token_version: i64,
    pub exp: i64 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub uid: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub role: String,
    pub status: String,
    pub muted_until: Option<String>,
    pub online: bool,
    pub last_ip: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: String,
}
