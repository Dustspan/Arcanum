use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

// 存储限制配置
pub const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;      // 单文件最大5MB
pub const MAX_TOTAL_STORAGE: u64 = 200 * 1024 * 1024;   // 总存储最大200MB
pub const MAX_FILES_COUNT: usize = 1000;                // 最大文件数量

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: &str) -> Result<Self> {
        let base = PathBuf::from(base_path);
        
        // 创建必要的目录
        fs::create_dir_all(&base)?;
        fs::create_dir_all(base.join("images"))?;
        fs::create_dir_all(base.join("files"))?;
        fs::create_dir_all(base.join("avatars"))?;
        
        Ok(Self { base_path: base })
    }
    
    /// 检查存储空间是否足够
    pub fn check_storage_available(&self, additional_size: u64) -> Result<bool> {
        let usage = self.get_storage_usage()?;
        Ok(usage.total_size + additional_size <= MAX_TOTAL_STORAGE)
    }
    
    /// 获取存储使用情况
    pub fn get_storage_usage(&self) -> Result<StorageUsage> {
        let mut total_size: u64 = 0;
        let mut files_count: usize = 0;
        let mut images_size: u64 = 0;
        let mut files_size: u64 = 0;
        let mut avatars_size: u64 = 0;
        
        // 统计各目录
        for subdir in &["images", "files", "avatars"] {
            let dir = self.base_path.join(subdir);
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            let size = metadata.len();
                            total_size += size;
                            files_count += 1;
                            
                            match *subdir {
                                "images" => images_size += size,
                                "files" => files_size += size,
                                "avatars" => avatars_size += size,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        
        Ok(StorageUsage {
            total_size,
            files_count,
            images_size,
            files_size,
            avatars_size,
            max_size: MAX_TOTAL_STORAGE,
            available: MAX_TOTAL_STORAGE.saturating_sub(total_size),
        })
    }
    
    /// 保存图片文件，返回相对路径
    pub fn save_image(&self, data: &[u8], content_type: &str) -> Result<String> {
        // 检查文件大小
        if data.len() > MAX_FILE_SIZE {
            return Err(anyhow::anyhow!("文件太大，最大允许{}MB", MAX_FILE_SIZE / 1024 / 1024));
        }
        
        // 检查存储空间
        if !self.check_storage_available(data.len() as u64)? {
            // 尝试清理旧文件
            self.cleanup_old_files(50)?;
            
            // 再次检查
            if !self.check_storage_available(data.len() as u64)? {
                return Err(anyhow::anyhow!("存储空间不足"));
            }
        }
        
        let ext = Self::get_extension(content_type);
        let filename = format!("{}_{}.{}", 
            Utc::now().format("%Y%m%d%H%M%S"),
            Uuid::new_v4().to_string().split('-').next().unwrap(),
            ext
        );
        
        let path = self.base_path.join("images").join(&filename);
        fs::write(&path, data)?;
        
        Ok(format!("/files/images/{}", filename))
    }
    
    /// 保存普通文件
    pub fn save_file(&self, data: &[u8], original_name: &str) -> Result<String> {
        // 检查文件大小
        if data.len() > MAX_FILE_SIZE {
            return Err(anyhow::anyhow!("文件太大，最大允许{}MB", MAX_FILE_SIZE / 1024 / 1024));
        }
        
        // 检查存储空间
        if !self.check_storage_available(data.len() as u64)? {
            self.cleanup_old_files(50)?;
            if !self.check_storage_available(data.len() as u64)? {
                return Err(anyhow::anyhow!("存储空间不足"));
            }
        }
        
        let ext = Path::new(original_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("bin");
        
        let filename = format!("{}_{}.{}", 
            Utc::now().format("%Y%m%d%H%M%S"),
            Uuid::new_v4().to_string().split('-').next().unwrap(),
            ext
        );
        
        let path = self.base_path.join("files").join(&filename);
        fs::write(&path, data)?;
        
        Ok(format!("/files/files/{}", filename))
    }
    
    /// 保存头像
    pub fn save_avatar(&self, data: &[u8], content_type: &str) -> Result<String> {
        // 头像限制更小
        const MAX_AVATAR_SIZE: usize = 1 * 1024 * 1024; // 1MB
        if data.len() > MAX_AVATAR_SIZE {
            return Err(anyhow::anyhow!("头像太大，最大允许1MB"));
        }
        
        let ext = Self::get_extension(content_type);
        let filename = format!("{}.{}", Uuid::new_v4(), ext);
        
        let path = self.base_path.join("avatars").join(&filename);
        fs::write(&path, data)?;
        
        Ok(format!("/files/avatars/{}", filename))
    }
    
    /// 获取文件内容
    pub fn read_file(&self, relative_path: &str) -> Result<Vec<u8>> {
        let path = self.base_path.join(relative_path.trim_start_matches("/files/"));
        Ok(fs::read(path)?)
    }
    
    /// 删除文件
    pub fn delete_file(&self, relative_path: &str) -> Result<()> {
        let path = self.base_path.join(relative_path.trim_start_matches("/files/"));
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
    
    /// 清理旧文件（按修改时间）
    pub fn cleanup_old_files(&self, keep_count: usize) -> Result<usize> {
        let mut deleted = 0;
        
        for subdir in &["images", "files"] {
            let dir = self.base_path.join(subdir);
            if !dir.exists() { continue; }
            
            // 收集所有文件及其修改时间
            let mut files: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            if let Ok(modified) = metadata.modified() {
                                files.push((entry.path(), modified));
                            }
                        }
                    }
                }
            }
            
            // 按修改时间排序（新的在前）
            files.sort_by(|a, b| b.1.cmp(&a.1));
            
            // 删除超出保留数量的文件
            for (path, _) in files.into_iter().skip(keep_count) {
                if fs::remove_file(&path).is_ok() {
                    deleted += 1;
                }
            }
        }
        
        Ok(deleted)
    }
    
    /// 清理孤立文件（数据库中不存在的文件）
    pub fn cleanup_orphaned_files(&self, db_files: &[String]) -> Result<usize> {
        let mut deleted = 0;
        
        for subdir in &["images", "files"] {
            let dir = self.base_path.join(subdir);
            if !dir.exists() { continue; }
            
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            if let Some(filename) = entry.file_name().to_str() {
                                let url = format!("/files/{}/{}", subdir, filename);
                                // 如果文件不在数据库中，删除
                                if !db_files.contains(&url) {
                                    if fs::remove_file(entry.path()).is_ok() {
                                        deleted += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(deleted)
    }
    
    /// 获取基础路径
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }
    
    fn get_extension(content_type: &str) -> &str {
        match content_type {
            "image/jpeg" | "image/jpg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            "image/svg+xml" => "svg",
            _ => "bin",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageUsage {
    pub total_size: u64,
    pub files_count: usize,
    pub images_size: u64,
    pub files_size: u64,
    pub avatars_size: u64,
    pub max_size: u64,
    pub available: u64,
}

impl StorageUsage {
    pub fn usage_percent(&self) -> f64 {
        if self.max_size == 0 { return 0.0; }
        (self.total_size as f64 / self.max_size as f64) * 100.0
    }
    
    pub fn format_size(&self, bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

impl Clone for FileStorage {
    fn clone(&self) -> Self {
        Self {
            base_path: self.base_path.clone(),
        }
    }
}
