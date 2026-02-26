use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires: i64,
    pub admin_uid: String,
    pub admin_password: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:arcanum.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "arcanum-secret-change-in-production".to_string()),
            jwt_expires: env::var("JWT_EXPIRES").ok().and_then(|s| s.parse().ok()).unwrap_or(604800),
            admin_uid: env::var("ADMIN_UID").unwrap_or_else(|_| "ARCANUM-ADMIN-0000".to_string()),
            admin_password: env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123456".to_string()),
        })
    }
}
