use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::{get, post, delete, put}, Router, response::Html};
use sqlx::SqlitePool;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer, services::ServeDir};
use handlers::health::SystemStats;

mod config; mod db; mod error; mod handlers; mod models; mod utils; mod ws; mod static_files; mod storage; mod broadcast; mod cache;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: SqlitePool,
    pub broadcast: broadcast::BroadcastManager,
    pub config: config::Config,
    pub storage: storage::FileStorage,
    pub cache: cache::PermissionCache,
    pub stats: SystemStats,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    
    let config = config::Config::from_env()?;
    let db = db::init_db(&config.database_url).await?;
    db::run_migrations(&db).await?;
    db::init_admin(&db, &config).await.ok();
    
    let storage = storage::FileStorage::new(&config.data_dir)?;
    let broadcast_manager = broadcast::BroadcastManager::new();
    let permission_cache = cache::PermissionCache::new();
    let system_stats = SystemStats::new();
    
    let state: AppState = Arc::new(AppStateInner { 
        db, 
        broadcast: broadcast_manager, 
        config, 
        storage,
        cache: permission_cache,
        stats: system_stats,
    });
    
    // 静态文件服务
    let files_service = ServeDir::new(state.storage.base_path())
        .append_index_html_on_directories(false);
    
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
        .route("/api/users/:id", get(handlers::users::get_user_info))
        .route("/api/users/profile", put(handlers::users::update_profile))
        .route("/api/users/password", put(handlers::users::change_password))
        // IP管理路由
        .route("/api/admin/ips", get(handlers::admin::list_banned_ips))
        .route("/api/admin/ips/:ip", delete(handlers::admin::unban_ip))
        .route("/api/admin/ips/:ip", post(handlers::admin::ban_ip))
        // 频道路由
        .route("/api/groups/enter", post(handlers::groups::enter_by_name))
        .route("/api/groups", post(handlers::groups::create_group))
        .route("/api/groups", get(handlers::groups::list_my_groups))
        .route("/api/groups/:id", get(handlers::groups::get_group_info))
        .route("/api/groups/:id", put(handlers::groups::update_group))
        .route("/api/groups/:id/members", get(handlers::groups::get_group_members))
        .route("/api/admin/groups", get(handlers::groups::list_all_groups))
        .route("/api/admin/groups/:id", delete(handlers::groups::delete_group))
        // 消息路由
        .route("/api/messages", post(handlers::messages::send_message))
        .route("/api/messages/group/:id", get(handlers::messages::get_messages))
        .route("/api/messages/group/:id/search", get(handlers::messages::search_messages))
        .route("/api/messages/group/:id", delete(handlers::messages::clear_messages))
        .route("/api/messages/:id", delete(handlers::messages::delete_message))
        .route("/api/messages/:id/recall", post(handlers::messages::recall_message))
        .route("/api/messages/:id/read", post(handlers::messages::mark_read))
        .route("/api/messages/:id/pin", post(handlers::messages::toggle_pin_message))
        .route("/api/messages/group/:id/read", post(handlers::messages::mark_group_read))
        .route("/api/messages/file/:id", post(handlers::messages::upload_file))
        .route("/api/mentions", get(handlers::messages::get_mentions))
        .route("/api/mentions/:id/read", post(handlers::messages::mark_mention_read))
        // WebSocket
        .route("/ws", get(ws::ws_handler))
        .route("/health", get(handlers::health::health_check))
        .route("/health/simple", get(handlers::health::health_simple))
        // 静态文件服务
        .nest_service("/files", files_service)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    let addr: SocketAddr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or_else(|_| "3000".to_string())).parse()?;
    tracing::info!("ARCANUM: http://{}", addr);
    tracing::info!("Data directory: {}", std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()));
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
