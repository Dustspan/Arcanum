use sqlx::{query, query_as, SqlitePool};
use crate::config::Config;
use crate::utils;

pub async fn init_db(url: &str) -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(url).await?)
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            uid TEXT UNIQUE NOT NULL,
            nickname TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT DEFAULT 'member',
            account_status TEXT DEFAULT 'active',
            token_version INTEGER DEFAULT 0,
            online INTEGER DEFAULT 0,
            last_ip TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#).execute(pool).await?;
    
    query("CREATE TABLE IF NOT EXISTS groups (id TEXT PRIMARY KEY, name TEXT UNIQUE NOT NULL, cipher_hash TEXT NOT NULL, owner_id TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(pool).await?;
    query("CREATE TABLE IF NOT EXISTS group_members (id TEXT PRIMARY KEY, group_id TEXT NOT NULL, user_id TEXT NOT NULL, joined_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE(group_id, user_id))").execute(pool).await?;
    query("CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, sender_id TEXT NOT NULL, group_id TEXT NOT NULL, content TEXT NOT NULL, type TEXT DEFAULT 'text', burn_after INTEGER DEFAULT 0, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(pool).await?;
    query("CREATE TABLE IF NOT EXISTS ip_bans (id TEXT PRIMARY KEY, ip TEXT NOT NULL UNIQUE, reason TEXT, banned_by TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)").execute(pool).await?;
    
    Ok(())
}

pub async fn init_admin(pool: &SqlitePool, config: &Config) -> anyhow::Result<()> {
    let exists: Option<(String,)> = query_as("SELECT id FROM users WHERE uid = ?").bind(&config.admin_uid).fetch_optional(pool).await?;
    if exists.is_some() { return Ok(()); }
    let id = uuid::Uuid::new_v4().to_string();
    let hash = utils::hash_password(&config.admin_password)?;
    query("INSERT INTO users (id, uid, nickname, password_hash, role, account_status, token_version, online) VALUES (?, ?, ?, ?, 'admin', 'active', 0, 0)")
        .bind(&id).bind(&config.admin_uid).bind("管理员").bind(&hash).execute(pool).await?;
    println!("\n════════════════════════════════════\n管理员: {} / {}\n════════════════════════════════════\n", config.admin_uid, config.admin_password);
    Ok(())
}
