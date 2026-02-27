use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

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
    
    /// 保存图片文件，返回相对路径
    pub fn save_image(&self, data: &[u8], content_type: &str) -> Result<String> {
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

impl Clone for FileStorage {
    fn clone(&self) -> Self {
        Self {
            base_path: self.base_path.clone(),
        }
    }
}
