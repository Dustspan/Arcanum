use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest { pub uid: String, pub password: String }

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest { pub uid: Option<String>, pub nickname: String, pub password: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims { 
    pub sub: String, 
    pub uid: String, 
    pub nickname: String, 
    pub role: String, 
    pub token_version: i64,
    pub exp: i64 
}
