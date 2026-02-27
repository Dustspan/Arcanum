use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::{get, post, delete, put}, Router, response::Html};
use sqlx::SqlitePool;
use tokio::sync::broadcast;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};

mod config; mod db; mod error; mod handlers; mod models; mod utils; mod ws; mod static_files;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: SqlitePool,
    pub tx: broadcast::Sender<ws::WsMessage>,
    pub config: config::Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    
    let config = config::Config::from_env()?;
    let db = db::init_db(&config.database_url).await?;
    db::run_migrations(&db).await?;
    db::init_admin(&db, &config).await.ok();
    
    let (tx, _) = broadcast::channel(1000);
    let state: AppState = Arc::new(AppStateInner { db, tx, config });
    
    let app = Router::new()
        .route("/", get(|| async { Html(static_files::INDEX_HTML) }))
        // 认证路由
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout))
        .route("/api/auth/me", get(handlers::auth::me))
        // 用户管理路由
        .route("/api/admin/users", post(handlers::users::create_user))
        .route("/api/admin/users", get(handlers::users::list_users))
        .route("/api/admin/users/:uid", delete(handlers::users::delete_user))
        .route("/api/admin/users/:uid/ban", put(handlers::users::ban_user))
        .route("/api/admin/users/:uid/unban", put(handlers::users::unban_user))
        .route("/api/admin/users/:uid/kick", put(handlers::users::kick_user))
        .route("/api/admin/users/:uid/mute", put(handlers::users::mute_user))
        .route("/api/admin/users/:uid/unmute", put(handlers::users::unmute_user))
        .route("/api/admin/users/:uid/permissions", post(handlers::users::grant_user_permission))
        .route("/api/admin/users/:uid/permissions", delete(handlers::users::revoke_user_permission))
        .route("/api/admin/permissions", get(handlers::users::list_permissions))
        .route("/api/users/avatar", post(handlers::users::upload_avatar))
        // IP管理路由
        .route("/api/admin/ips", get(handlers::admin::list_banned_ips))
        .route("/api/admin/ips/:ip", delete(handlers::admin::unban_ip))
        .route("/api/admin/ips/:ip", post(handlers::admin::ban_ip))
        // 频道路由
        .route("/api/groups/enter", post(handlers::groups::enter_by_name))
        .route("/api/groups", post(handlers::groups::create_group))
        .route("/api/groups", get(handlers::groups::list_my_groups))
        .route("/api/admin/groups", get(handlers::groups::list_all_groups))
        .route("/api/admin/groups/:id", delete(handlers::groups::delete_group))
        // 消息路由
        .route("/api/messages", post(handlers::messages::send_message))
        .route("/api/messages/group/:id", get(handlers::messages::get_messages))
        .route("/api/messages/group/:id", delete(handlers::messages::clear_messages))
        .route("/api/messages/:id", delete(handlers::messages::delete_message))
        .route("/api/messages/file/:id", post(handlers::messages::upload_file))
        // WebSocket
        .route("/ws", get(ws::ws_handler))
        .route("/health", get(|| async { "OK" }))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    let addr: SocketAddr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or_else(|_| "3000".to_string())).parse()?;
    tracing::info!("ARCANUM: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
