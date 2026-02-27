use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires: i64,
    pub admin_uid: String,
    pub admin_password: String,
    pub max_file_size: usize,
    pub rate_limit_messages: usize,
    pub rate_limit_window_secs: i64,
    pub data_dir: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:arcanum.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "arcanum-secret-change-in-production".to_string()),
            jwt_expires: env::var("JWT_EXPIRES").ok().and_then(|s| s.parse().ok()).unwrap_or(604800),
            admin_uid: env::var("ADMIN_UID").unwrap_or_else(|_| "ARCANUM-ADMIN-0000".to_string()),
            admin_password: env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123456".to_string()),
            max_file_size: env::var("MAX_FILE_SIZE").ok().and_then(|s| s.parse().ok()).unwrap_or(5 * 1024 * 1024),
            rate_limit_messages: env::var("RATE_LIMIT_MESSAGES").ok().and_then(|s| s.parse().ok()).unwrap_or(10),
            rate_limit_window_secs: env::var("RATE_LIMIT_WINDOW").ok().and_then(|s| s.parse().ok()).unwrap_or(60),
            data_dir: env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()),
        })
    }
}
