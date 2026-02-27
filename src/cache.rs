use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use sqlx::SqlitePool;
use crate::error::Result;
use crate::error::AppError;

/// 用户权限缓存
#[derive(Clone, Default)]
pub struct UserPermissions {
    pub permissions: Vec<String>,
    pub role: String,
}

/// 权限缓存管理器
pub struct PermissionCache {
    /// user_id -> UserPermissions
    cache: Arc<RwLock<HashMap<String, UserPermissions>>>,
}

impl PermissionCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 获取用户权限（优先从缓存读取）
    pub async fn get_permissions(&self, pool: &SqlitePool, user_id: &str) -> Result<UserPermissions> {
        // 先检查缓存
        {
            let cache = self.cache.read().await;
            if let Some(perms) = cache.get(user_id) {
                return Ok(perms.clone());
            }
        }
        
        // 缓存未命中，从数据库加载
        self.load_from_db(pool, user_id).await
    }
    
    /// 从数据库加载权限并缓存
    pub async fn load_from_db(&self, pool: &SqlitePool, user_id: &str) -> Result<UserPermissions> {
        // 获取用户角色
        let role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .flatten();
        
        let role = role.unwrap_or_else(|| "member".to_string());
        
        // 管理员拥有所有权限
        let permissions = if role == "admin" {
            sqlx::query_scalar("SELECT name FROM permissions")
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?
        } else {
            sqlx::query_scalar(r#"
                SELECT p.name FROM user_permissions up
                JOIN permissions p ON up.permission_id = p.id
                WHERE up.user_id = ?
            "#)
            .bind(user_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        };
        
        let perms = UserPermissions { permissions, role };
        
        // 更新缓存
        {
            let mut cache = self.cache.write().await;
            cache.insert(user_id.to_string(), perms.clone());
        }
        
        Ok(perms)
    }
    
    /// 检查用户是否拥有指定权限
    pub async fn has_permission(&self, pool: &SqlitePool, user_id: &str, permission_name: &str) -> Result<bool> {
        let perms = self.get_permissions(pool, user_id).await?;
        
        if perms.role == "admin" {
            return Ok(true);
        }
        
        Ok(perms.permissions.contains(&permission_name.to_string()))
    }
    
    /// 使缓存失效（权限变更时调用）
    pub async fn invalidate(&self, user_id: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(user_id);
    }
    
    /// 清空所有缓存
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    /// 获取缓存大小
    pub async fn cache_size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }
}

impl Default for PermissionCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PermissionCache {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
        }
    }
}
