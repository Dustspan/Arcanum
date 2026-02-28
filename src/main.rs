use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::{get, post, delete, put}, Router, response::Html};
use sqlx::SqlitePool;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer, services::ServeDir, timeout::TimeoutLayer};
use handlers::health::SystemStats;
use tokio::signal;

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
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
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
    
    // 启动后台清理任务
    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 每5分钟
        loop {
            interval.tick().await;
            
            // 清理广播通道
            state_clone.broadcast.cleanup();
            
            // 清理过期数据（每小时执行一次）
            let hour_interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
            tokio::select! {
                _ = hour_interval.tick() => {
                    if let Err(e) = db::cleanup_expired_data(&state_clone.db).await {
                        tracing::warn!("清理过期数据失败: {}", e);
                    } else {
                        tracing::info!("已清理过期数据");
                    }
                }
            }
        }
    });
    
    // 静态文件服务
    let files_service = ServeDir::new(state.storage.base_path())
        .append_index_html_on_directories(false);
    
    let app = Router::new()
        .route("/", get(|| async { Html(static_files::INDEX_HTML) }))
        // PWA
        .route("/manifest.json", get(|| async { 
            axum::response::IntoResponse::into_response(
                ([(axum::http::header::CONTENT_TYPE, "application/json")], static_files::MANIFEST_JSON)
            )
        }))
        .route("/sw.js", get(|| async {
            axum::response::IntoResponse::into_response(
                ([(axum::http::header::CONTENT_TYPE, "application/javascript")], static_files::SERVICE_WORKER_JS)
            )
        }))
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
        // 敏感词管理
        .route("/api/admin/sensitive-words", get(handlers::admin::list_sensitive_words))
        .route("/api/admin/sensitive-words", post(handlers::admin::add_sensitive_word))
        .route("/api/admin/sensitive-words/:id", delete(handlers::admin::delete_sensitive_word))
        // 操作日志
        .route("/api/admin/audit-logs", get(handlers::admin::list_audit_logs))
        // 频道路由
        .route("/api/groups/enter", post(handlers::groups::enter_by_name))
        .route("/api/groups", post(handlers::groups::create_group))
        .route("/api/groups", get(handlers::groups::list_my_groups))
        .route("/api/groups/:id", get(handlers::groups::get_group_info))
        .route("/api/groups/:id", put(handlers::groups::update_group))
        .route("/api/groups/:id/members", get(handlers::groups::get_group_members))
        .route("/api/groups/:id/invite", post(handlers::groups::create_invite_link))
        .route("/api/invite/:code", post(handlers::groups::join_by_invite))
        .route("/api/admin/groups", get(handlers::groups::list_all_groups))
        .route("/api/admin/groups/:id", delete(handlers::groups::delete_group))
        .route("/api/admin/statistics", get(handlers::health::get_statistics))
        // 消息路由
        .route("/api/messages", post(handlers::messages::send_message))
        .route("/api/messages/group/:id", get(handlers::messages::get_messages))
        .route("/api/messages/group/:id/search", get(handlers::messages::search_messages))
        .route("/api/messages/group/:id", delete(handlers::messages::clear_messages))
        .route("/api/messages/:id", delete(handlers::messages::delete_message))
        .route("/api/messages/:id/recall", post(handlers::messages::recall_message))
        .route("/api/messages/:id/read", post(handlers::messages::mark_read))
        .route("/api/messages/:id/pin", post(handlers::messages::toggle_pin_message))
        .route("/api/messages/:id/forward", post(handlers::messages::forward_message))
        .route("/api/messages/group/:id/read", post(handlers::messages::mark_group_read))
        .route("/api/messages/file/:id", post(handlers::messages::upload_file))
        .route("/api/mentions", get(handlers::messages::get_mentions))
        .route("/api/mentions/:id/read", post(handlers::messages::mark_mention_read))
        // 私聊路由
        .route("/api/direct/:id", post(handlers::direct::send_direct_message))
        .route("/api/direct/:id", get(handlers::direct::get_direct_messages))
        .route("/api/conversations", get(handlers::direct::get_conversations))
        // 好友路由
        .route("/api/friends", get(handlers::direct::get_friends))
        .route("/api/friends/requests", get(handlers::direct::get_friend_requests))
        .route("/api/friends/:id", post(handlers::direct::add_friend))
        .route("/api/friends/:id/accept", post(handlers::direct::accept_friend))
        // WebSocket
        .route("/ws", get(ws::ws_handler))
        .route("/health", get(handlers::health::health_check))
        .route("/health/simple", get(handlers::health::health_simple))
        // 静态文件服务
        .nest_service("/files", files_service)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30))) // 请求超时30秒
        .with_state(state);
    
    let addr: SocketAddr = format!("0.0.0.0:{}", std::env::var("PORT").unwrap_or_else(|_| "3000".to_string())).parse()?;
    tracing::info!("ARCANUM: http://{}", addr);
    tracing::info!("Data directory: {}", std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()));
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // 优雅关闭
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    tracing::info!("服务器已关闭");
    Ok(())
}

/// 优雅关闭信号处理
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("无法安装Ctrl+C处理器");
    };
    
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("无法安装信号处理器")
            .recv()
            .await;
    };
    
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    tracing::info!("收到关闭信号，正在优雅关闭...");
}
