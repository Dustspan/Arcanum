use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")] Database(#[from] sqlx::Error),
    #[error("认证失败: {0}")] Auth(String),
    #[error("未授权")] Unauthorized,
    #[error("无权限")] Forbidden,
    #[error("频道不存在")] NotFound,
    #[error("参数错误: {0}")] BadRequest(String),
    #[error("账户已被封禁")] Banned,
    #[error("IP已被封禁")] IpBanned,
    #[error("账户已被踢出")] Kicked,
    #[error("内部错误: {0}")] Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            AppError::Database(e) => { tracing::error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".to_string()) }
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未登录".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "无权限".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "频道不存在".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Banned => (StatusCode::FORBIDDEN, "账户已被封禁".to_string()),
            AppError::IpBanned => (StatusCode::FORBIDDEN, "IP已被封禁".to_string()),
            AppError::Kicked => (StatusCode::UNAUTHORIZED, "账户已被踢出，请重新登录".to_string()),
            AppError::Internal(msg) => { tracing::error!("Internal: {}", msg); (StatusCode::INTERNAL_SERVER_ERROR, "服务器错误".to_string()) }
        };
        (status, Json(json!({"success": false, "error": message}))).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
