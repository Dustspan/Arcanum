use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::config::Config;
use crate::error::{AppError, Result};
use crate::models::Claims;

pub fn generate_token(id: &str, uid: &str, nickname: &str, role: &str, token_version: i64, config: &Config) -> Result<String> {
    let claims = Claims {
        sub: id.to_string(), 
        uid: uid.to_string(), 
        nickname: nickname.to_string(),
        role: role.to_string(),
        token_version,
        exp: chrono::Utc::now().timestamp() + config.jwt_expires,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))
        .map_err(|e| AppError::Internal(format!("Token error: {}", e)))
}

pub fn verify_token(token: &str, config: &Config) -> Result<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(config.jwt_secret.as_bytes()), &Validation::default())
        .map(|d| d.claims).map_err(|_| AppError::Unauthorized)
}
