use sqlx::{query, query_as, SqlitePool, sqlite::SqlitePoolOptions};
use crate::config::Config;
use crate::utils;

// 消息保留策略配置
pub const MESSAGE_RETENTION_DAYS: i64 = 30;        // 普通消息保留30天
pub const PINNED_MESSAGE_RETENTION_DAYS: i64 = 90; // 置顶消息保留90天
pub const MAX_MESSAGES_PER_GROUP: i64 = 5000;      // 每个频道最多5000条消息

/// 初始化数据库连接池（优化配置）
pub async fn init_db(url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(Some(std::time::Duration::from_secs(300)))
        .max_lifetime(Some(std::time::Duration::from_secs(1800)))
        .connect(url)
        .await?;
    
    // 启用SQLite优化
    sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await.ok();
    sqlx::query("PRAGMA synchronous = NORMAL").execute(&pool).await.ok();
    sqlx::query("PRAGMA cache_size = -64000").execute(&pool).await.ok();
    sqlx::query("PRAGMA temp_store = MEMORY").execute(&pool).await.ok();
    
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    // 用户表
    query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            uid TEXT UNIQUE NOT NULL,
            nickname TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            avatar TEXT DEFAULT '',
            role TEXT DEFAULT 'member',
            account_status TEXT DEFAULT 'active',
            muted_until TEXT,
            token_version INTEGER DEFAULT 0,
            online INTEGER DEFAULT 0,
            last_ip TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    
    query("CREATE INDEX IF NOT EXISTS idx_users_uid ON users(uid)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_users_online ON users(online)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_users_status ON users(account_status)").execute(pool).await.ok();
    
    query("ALTER TABLE users ADD COLUMN avatar TEXT DEFAULT ''").execute(pool).await.ok();
    query("ALTER TABLE users ADD COLUMN muted_until TEXT").execute(pool).await.ok();
    
    // 频道表
    query("CREATE TABLE IF NOT EXISTS groups (id TEXT PRIMARY KEY, name TEXT UNIQUE NOT NULL, cipher_hash TEXT NOT NULL, owner_id TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_groups_owner ON groups(owner_id)").execute(pool).await.ok();
    
    query("ALTER TABLE groups ADD COLUMN description TEXT").execute(pool).await.ok();
    query("ALTER TABLE groups ADD COLUMN announcement TEXT").execute(pool).await.ok();
    
    // 频道成员表
    query("CREATE TABLE IF NOT EXISTS group_members (id TEXT PRIMARY KEY, group_id TEXT NOT NULL, user_id TEXT NOT NULL, joined_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE(group_id, user_id))").execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_group_members_group ON group_members(group_id)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_group_members_user ON group_members(user_id)").execute(pool).await.ok();
    
    // 消息表
    query(r#"
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY, 
            sender_id TEXT NOT NULL, 
            group_id TEXT NOT NULL, 
            content TEXT NOT NULL, 
            type TEXT DEFAULT 'text', 
            file_name TEXT,
            file_size INTEGER DEFAULT 0,
            burn_after INTEGER DEFAULT 0, 
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    
    query("CREATE INDEX IF NOT EXISTS idx_messages_group ON messages(group_id, created_at DESC)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages(sender_id)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_messages_created ON messages(created_at)").execute(pool).await.ok();
    
    query("ALTER TABLE messages ADD COLUMN file_name TEXT").execute(pool).await.ok();
    query("ALTER TABLE messages ADD COLUMN file_size INTEGER DEFAULT 0").execute(pool).await.ok();
    query("ALTER TABLE messages ADD COLUMN reply_to TEXT").execute(pool).await.ok();
    query("ALTER TABLE messages ADD COLUMN pinned INTEGER DEFAULT 0").execute(pool).await.ok();
    
    // 消息提及表
    query(r#"
        CREATE TABLE IF NOT EXISTS mentions (
            id TEXT PRIMARY KEY,
            message_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            mentioned_by TEXT NOT NULL,
            group_id TEXT NOT NULL,
            read INTEGER DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(message_id, user_id)
        )
    "#).execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_mentions_user ON mentions(user_id, read)").execute(pool).await.ok();
    
    // 消息已读状态表
    query(r#"
        CREATE TABLE IF NOT EXISTS message_reads (
            id TEXT PRIMARY KEY,
            message_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            read_at TEXT DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(message_id, user_id)
        )
    "#).execute(pool).await?;
    
    // IP封禁表
    query("CREATE TABLE IF NOT EXISTS ip_bans (id TEXT PRIMARY KEY, ip TEXT NOT NULL UNIQUE, reason TEXT, banned_by TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(pool).await?;
    
    // 权限表
    query(r#"
        CREATE TABLE IF NOT EXISTS permissions (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT
        )
    "#).execute(pool).await?;
    
    // 用户权限关联表
    query(r#"
        CREATE TABLE IF NOT EXISTS user_permissions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            permission_id TEXT NOT NULL,
            granted_by TEXT,
            granted_at TEXT DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id, permission_id)
        )
    "#).execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_user_permissions_user ON user_permissions(user_id)").execute(pool).await.ok();
    
    // 私聊消息表
    query(r#"
        CREATE TABLE IF NOT EXISTS direct_messages (
            id TEXT PRIMARY KEY,
            sender_id TEXT NOT NULL,
            receiver_id TEXT NOT NULL,
            content TEXT NOT NULL,
            type TEXT DEFAULT 'text',
            file_name TEXT,
            file_size INTEGER DEFAULT 0,
            read INTEGER DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_direct_receiver ON direct_messages(receiver_id, read, created_at)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_direct_sender ON direct_messages(sender_id)").execute(pool).await.ok();
    
    // 好友关系表
    query(r#"
        CREATE TABLE IF NOT EXISTS friendships (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            friend_id TEXT NOT NULL,
            status TEXT DEFAULT 'pending',
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id, friend_id)
        )
    "#).execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_friendships_user ON friendships(user_id, status)").execute(pool).await.ok();
    query("CREATE INDEX IF NOT EXISTS idx_friendships_friend ON friendships(friend_id, status)").execute(pool).await.ok();
    
    // 敏感词表
    query(r#"
        CREATE TABLE IF NOT EXISTS sensitive_words (
            id TEXT PRIMARY KEY,
            word TEXT UNIQUE NOT NULL,
            replacement TEXT DEFAULT '***',
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    
    // 操作日志表
    query(r#"
        CREATE TABLE IF NOT EXISTS audit_logs (
            id TEXT PRIMARY KEY,
            user_id TEXT,
            action TEXT NOT NULL,
            target_type TEXT,
            target_id TEXT,
            details TEXT,
            ip_address TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at)").execute(pool).await.ok();
    
    // 邀请链接表
    query(r#"
        CREATE TABLE IF NOT EXISTS invite_links (
            id TEXT PRIMARY KEY,
            code TEXT UNIQUE NOT NULL,
            group_id TEXT NOT NULL,
            created_by TEXT NOT NULL,
            max_uses INTEGER DEFAULT 0,
            uses INTEGER DEFAULT 0,
            expires_at TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    
    // 初始化权限列表
    let permissions = vec![
        ("user_create", "创建用户"),
        ("user_view", "查看用户列表"),
        ("user_ban", "封禁/解封用户"),
        ("user_kick", "踢出用户"),
        ("user_mute", "禁言用户"),
        ("group_create", "创建频道"),
        ("group_view", "查看所有频道"),
        ("group_delete", "删除频道"),
        ("message_delete", "删除消息"),
        ("ip_ban", "封禁IP"),
        ("permission_grant", "授予权限"),
        ("file_upload", "上传文件"),
    ];
    
    for (name, desc) in permissions {
        let id = uuid::Uuid::new_v4().to_string();
        query("INSERT OR IGNORE INTO permissions (id, name, description) VALUES (?, ?, ?)")
            .bind(&id).bind(name).bind(desc)
            .execute(pool).await.ok();
    }
    
    // 速率限制表
    query(r#"
        CREATE TABLE IF NOT EXISTS rate_limits (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            action_type TEXT NOT NULL,
            count INTEGER DEFAULT 1,
            window_start TEXT NOT NULL,
            UNIQUE(user_id, action_type)
        )
    "#).execute(pool).await?;
    query("CREATE INDEX IF NOT EXISTS idx_rate_limits_window ON rate_limits(window_start)").execute(pool).await.ok();
    
    Ok(())
}

pub async fn init_admin(pool: &SqlitePool, config: &Config) -> anyhow::Result<()> {
    let exists: Option<(String,)> = query_as("SELECT id FROM users WHERE uid = ?").bind(&config.admin_uid).fetch_optional(pool).await?;
    if exists.is_some() { return Ok(()); }
    
    let id = uuid::Uuid::new_v4().to_string();
    let hash = utils::hash_password(&config.admin_password)?;
    
    query("INSERT INTO users (id, uid, nickname, password_hash, role, account_status, token_version, online) VALUES (?, ?, ?, ?, 'admin', 'active', 0, 0)")
        .bind(&id).bind(&config.admin_uid).bind("管理员").bind(&hash).execute(pool).await?;
    
    let perms: Vec<(String, String)> = query_as("SELECT id, name FROM permissions").fetch_all(pool).await?;
    for (perm_id, _) in perms {
        let up_id = uuid::Uuid::new_v4().to_string();
        query("INSERT OR IGNORE INTO user_permissions (id, user_id, permission_id, granted_by) VALUES (?, ?, ?, ?)")
            .bind(&up_id).bind(&id).bind(&perm_id).bind(&id)
            .execute(pool).await.ok();
    }
    
    println!("\n════════════════════════════════════\n管理员: {} / {}\n════════════════════════════════════\n", config.admin_uid, config.admin_password);
    Ok(())
}

/// 清理过期数据（定期调用）
pub async fn cleanup_expired_data(pool: &SqlitePool) -> anyhow::Result<CleanupStats> {
    let now = chrono::Utc::now();
    let mut stats = CleanupStats::default();
    
    // 1. 清理过期的普通消息（保留置顶消息）
    let msg_expire = (now - chrono::Duration::days(MESSAGE_RETENTION_DAYS)).to_rfc3339();
    let result = sqlx::query(
        "DELETE FROM messages WHERE created_at < ? AND pinned = 0"
    )
    .bind(&msg_expire)
    .execute(pool)
    .await?;
    stats.messages_deleted = result.rows_affected();
    
    // 2. 清理过期的置顶消息
    let pinned_expire = (now - chrono::Duration::days(PINNED_MESSAGE_RETENTION_DAYS)).to_rfc3339();
    let result = sqlx::query(
        "DELETE FROM messages WHERE created_at < ? AND pinned = 1"
    )
    .bind(&pinned_expire)
    .execute(pool)
    .await?;
    stats.pinned_deleted = result.rows_affected();
    
    // 3. 清理每个频道超出限制的消息（保留最新的）
    let groups: Vec<String> = sqlx::query_scalar("SELECT id FROM groups")
        .fetch_all(pool)
        .await?;
    
    for group_id in groups {
        // 获取该频道的消息数量
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE group_id = ?")
            .bind(&group_id)
            .fetch_one(pool)
            .await?;
        
        if count > MAX_MESSAGES_PER_GROUP {
            // 删除最旧的非置顶消息
            let to_delete = count - MAX_MESSAGES_PER_GROUP;
            sqlx::query(r#"
                DELETE FROM messages 
                WHERE id IN (
                    SELECT id FROM messages 
                    WHERE group_id = ? AND pinned = 0 
                    ORDER BY created_at ASC 
                    LIMIT ?
                )
            "#)
            .bind(&group_id)
            .bind(to_delete as i64)
            .execute(pool)
            .await?;
            
            stats.overflow_deleted += to_delete as u64;
        }
    }
    
    // 4. 清理私聊消息（已读的7天后删除）
    let dm_expire = (now - chrono::Duration::days(7)).to_rfc3339();
    let result = sqlx::query("DELETE FROM direct_messages WHERE created_at < ? AND read = 1")
        .bind(&dm_expire)
        .execute(pool)
        .await?;
    stats.direct_deleted = result.rows_affected();
    
    // 5. 清理未读私聊消息（30天后强制删除）
    let dm_force_expire = (now - chrono::Duration::days(30)).to_rfc3339();
    let result = sqlx::query("DELETE FROM direct_messages WHERE created_at < ?")
        .bind(&dm_force_expire)
        .execute(pool)
        .await?;
    stats.direct_deleted += result.rows_affected();
    
    // 6. 清理操作日志（7天后删除）
    let log_expire = (now - chrono::Duration::days(7)).to_rfc3339();
    let result = sqlx::query("DELETE FROM audit_logs WHERE created_at < ?")
        .bind(&log_expire)
        .execute(pool)
        .await?;
    stats.logs_deleted = result.rows_affected();
    
    // 7. 清理过期的速率限制记录
    let rate_expire = (now - chrono::Duration::hours(1)).to_rfc3339();
    sqlx::query("DELETE FROM rate_limits WHERE window_start < ?")
        .bind(&rate_expire)
        .execute(pool)
        .await?;
    
    // 8. 清理过期的邀请链接
    sqlx::query("DELETE FROM invite_links WHERE expires_at IS NOT NULL AND expires_at < ?")
        .bind(&now.to_rfc3339())
        .execute(pool)
        .await?;
    
    // 9. 清理孤立的消息引用
    sqlx::query("DELETE FROM mentions WHERE message_id NOT IN (SELECT id FROM messages)")
        .execute(pool)
        .await?;
    
    sqlx::query("DELETE FROM message_reads WHERE message_id NOT IN (SELECT id FROM messages)")
        .execute(pool)
        .await?;
    
    // 10. 执行VACUUM优化数据库（每周一次，这里简化处理）
    // 注意：VACUUM会锁定数据库，谨慎使用
    // sqlx::query("VACUUM").execute(pool).await?;
    
    tracing::info!(
        "清理完成: 消息{}条, 置顶{}条, 溢出{}条, 私聊{}条, 日志{}条",
        stats.messages_deleted,
        stats.pinned_deleted,
        stats.overflow_deleted,
        stats.direct_deleted,
        stats.logs_deleted
    );
    
    Ok(stats)
}

/// 获取数据库大小估算
pub async fn get_db_stats(pool: &SqlitePool) -> anyhow::Result<DbStats> {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool).await?;
    
    let groups: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM groups")
        .fetch_one(pool).await?;
    
    let messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
        .fetch_one(pool).await?;
    
    let direct_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM direct_messages")
        .fetch_one(pool).await?;
    
    let files: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE type IN ('image', 'file')")
        .fetch_one(pool).await?;
    
    let total_file_size: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(file_size), 0) FROM messages WHERE type IN ('image', 'file')")
        .fetch_one(pool).await?;
    
    Ok(DbStats {
        users,
        groups,
        messages,
        direct_messages,
        files,
        total_file_size,
    })
}

#[derive(Debug, Default)]
pub struct CleanupStats {
    pub messages_deleted: u64,
    pub pinned_deleted: u64,
    pub overflow_deleted: u64,
    pub direct_deleted: u64,
    pub logs_deleted: u64,
}

#[derive(Debug)]
pub struct DbStats {
    pub users: i64,
    pub groups: i64,
    pub messages: i64,
    pub direct_messages: i64,
    pub files: i64,
    pub total_file_size: i64,
}
