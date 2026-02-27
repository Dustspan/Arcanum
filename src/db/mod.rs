use sqlx::{query, query_as, SqlitePool};
use crate::config::Config;
use crate::utils;

pub async fn init_db(url: &str) -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(url).await?)
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
    
    // 添加 avatar 和 muted_until 列（如果不存在）
    query("ALTER TABLE users ADD COLUMN avatar TEXT DEFAULT ''").execute(pool).await.ok();
    query("ALTER TABLE users ADD COLUMN muted_until TEXT").execute(pool).await.ok();
    
    // 频道表
    query("CREATE TABLE IF NOT EXISTS groups (id TEXT PRIMARY KEY, name TEXT UNIQUE NOT NULL, cipher_hash TEXT NOT NULL, owner_id TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(pool).await?;
    
    // 添加频道描述和公告列
    query("ALTER TABLE groups ADD COLUMN description TEXT").execute(pool).await.ok();
    query("ALTER TABLE groups ADD COLUMN announcement TEXT").execute(pool).await.ok();
    
    // 频道成员表
    query("CREATE TABLE IF NOT EXISTS group_members (id TEXT PRIMARY KEY, group_id TEXT NOT NULL, user_id TEXT NOT NULL, joined_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE(group_id, user_id))").execute(pool).await?;
    
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
    
    // 添加文件相关列
    query("ALTER TABLE messages ADD COLUMN file_name TEXT").execute(pool).await.ok();
    query("ALTER TABLE messages ADD COLUMN file_size INTEGER DEFAULT 0").execute(pool).await.ok();
    
    // 添加消息引用列
    query("ALTER TABLE messages ADD COLUMN reply_to TEXT").execute(pool).await.ok();
    
    // 添加消息置顶列
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
    
    Ok(())
}

pub async fn init_admin(pool: &SqlitePool, config: &Config) -> anyhow::Result<()> {
    let exists: Option<(String,)> = query_as("SELECT id FROM users WHERE uid = ?").bind(&config.admin_uid).fetch_optional(pool).await?;
    if exists.is_some() { return Ok(()); }
    
    let id = uuid::Uuid::new_v4().to_string();
    let hash = utils::hash_password(&config.admin_password)?;
    
    query("INSERT INTO users (id, uid, nickname, password_hash, role, account_status, token_version, online) VALUES (?, ?, ?, ?, 'admin', 'active', 0, 0)")
        .bind(&id).bind(&config.admin_uid).bind("管理员").bind(&hash).execute(pool).await?;
    
    // 给管理员授予所有权限
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
