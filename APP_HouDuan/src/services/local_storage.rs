use anyhow::{Result, anyhow};
use actix_web::web::Bytes;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LocalStorageService {
    uploads_dir: String,       // 物理存储根目录（如 uploads）
    url_prefix: String,        // 对外访问前缀（固定为 /uploads）
    storage_subdir: String,    // 业务子目录（默认 image/结绳社区）
    last_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub size: i64,
    pub is_dir: bool,
    pub modified: String,
    pub sign: Option<String>,
    pub thumb: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    pub raw_url: Option<String>,
    pub provider: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileListResponse {
    pub content: Option<Vec<FileInfo>>,
    pub total: i32,
    pub readme: Option<String>,
    pub write: bool,
    pub provider: String,
}

impl LocalStorageService {
    pub fn new() -> Self {
        let config = crate::config::Config::load().unwrap_or_default();
        let uploads_dir = config.upload_path().to_string();
        Self {
            uploads_dir,
            url_prefix: "/uploads".to_string(),
            storage_subdir: "结绳社区".to_string(),
            last_error: None,
        }
    }

    pub fn new_with_params(uploads_dir: String, url_prefix: String) -> Self {
        Self {
            uploads_dir,
            url_prefix,
            storage_subdir: "结绳社区".to_string(),
            last_error: None,
        }
    }

    pub fn base_url(&self) -> &str { &self.url_prefix }

    fn to_fs_path(&self, relative_path: &str) -> PathBuf {
        // relative_path 允许以 "/image/..." 或者 "image/..." 形式
        let rel = relative_path.trim_start_matches('/');
        Path::new(&self.uploads_dir).join(rel)
    }

    fn ensure_parent_dir(&self, fs_path: &Path) -> Result<()> {
        if let Some(parent) = fs_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    fn now_rfc3339() -> String {
        chrono::Utc::now().to_rfc3339()
    }

    pub async fn health_check(&mut self) -> bool {
        let base = self.to_fs_path(&format!("/{}", self.storage_subdir));
        if let Err(e) = fs::create_dir_all(&base) {
            self.last_error = Some(format!("创建本地存储根目录失败: {}", e));
            return false;
        }
        true
    }

    pub fn last_error(&self) -> Option<&str> { self.last_error.as_deref() }

    pub async fn create_folder_if_missing(&mut self, path: &str) -> Result<bool> {
        let fs_path = self.to_fs_path(path);
        if fs_path.exists() {
            return Ok(false);
        }
        fs::create_dir_all(&fs_path)?;
        Ok(true)
    }

    pub async fn create_folder(&mut self, path: &str) -> Result<()> {
        let fs_path = self.to_fs_path(path);
        fs::create_dir_all(&fs_path)?;
        Ok(())
    }

    pub async fn upload_file(&mut self, file_path: &str, file_name: &str, file_data: Bytes) -> Result<String> {
        let full_rel_dir = format!("{}/{}", file_path.trim_end_matches('/'), file_name);
        let fs_path = self.to_fs_path(&full_rel_dir);
        self.ensure_parent_dir(&fs_path)?;
        fs::write(&fs_path, &file_data)?;
        Ok(format!("{}", full_rel_dir))
    }

    pub async fn get_download_link(&mut self, file_path: &str) -> Result<String> {
        // 接受 "/image/..." 或 "/uploads/..."，统一返回 "/uploads/..."
        let rel = if file_path.starts_with("/uploads/") {
            file_path.trim_start_matches("/uploads").to_string()
        } else {
            file_path.to_string()
        };
        Ok(format!("{}{}", self.url_prefix, rel))
    }

    pub async fn delete_file(&mut self, file_path: &str) -> Result<()> {
        let rel = if file_path.starts_with("/uploads/") {
            file_path.trim_start_matches("/uploads").to_string()
        } else {
            file_path.to_string()
        };
        let fs_path = self.to_fs_path(&rel);
        if fs_path.exists() {
            fs::remove_file(&fs_path)?;
        }
        Ok(())
    }

    pub async fn verify_file_exists(&mut self, file_path: &str) -> Result<bool> {
        let rel = if file_path.starts_with("/uploads/") {
            file_path.trim_start_matches("/uploads").to_string()
        } else {
            file_path.to_string()
        };
        let fs_path = self.to_fs_path(&rel);
        Ok(fs_path.exists())
    }

    pub async fn get_file_info(&mut self, file_path: &str) -> Result<FileInfo> {
        let rel = if file_path.starts_with("/uploads/") {
            file_path.trim_start_matches("/uploads").to_string()
        } else {
            file_path.to_string()
        };
        let fs_path = self.to_fs_path(&rel);
        let meta = fs::metadata(&fs_path)
            .map_err(|e| anyhow!("读取文件信息失败: {}", e))?;
        let modified = meta.modified().unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        Ok(FileInfo {
            name: Path::new(&rel).file_name().and_then(|s| s.to_str()).unwrap_or("").to_string(),
            size: meta.len() as i64,
            is_dir: meta.is_dir(),
            modified: chrono::NaiveDateTime::from_timestamp_opt(modified as i64, 0)
                .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
                .format("%Y-%m-%d %H:%M:%S").to_string(),
            sign: None,
            thumb: None,
            type_: None,
            raw_url: Some(format!("{}{}", self.url_prefix, rel)),
            provider: Some("local".to_string()),
        })
    }

    pub async fn list_files(&mut self, path: &str) -> Result<FileListResponse> {
        let fs_path = self.to_fs_path(path);
        let mut list = Vec::new();
        if fs_path.exists() {
            let it = fs::read_dir(&fs_path)?;
            for entry in it {
                let entry = entry?;
                let meta = entry.metadata()?;
                let name = entry.file_name().to_string_lossy().to_string();
                let rel = Path::new(path).join(&name).to_string_lossy().to_string();
                let modified = meta.modified().unwrap_or(SystemTime::now())
                    .duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
                list.push(FileInfo {
                    name,
                    size: if meta.is_file() { meta.len() as i64 } else { 0 },
                    is_dir: meta.is_dir(),
                    modified: chrono::NaiveDateTime::from_timestamp_opt(modified as i64, 0)
                        .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
                        .format("%Y-%m-%d %H:%M:%S").to_string(),
                    sign: None,
                    thumb: None,
                    type_: None,
                    raw_url: if meta.is_file() { Some(format!("{}{}", self.url_prefix, rel)) } else { None },
                    provider: Some("local".to_string()),
                });
            }
        }
        let total_count = list.len() as i32;
        Ok(FileListResponse {
            content: Some(list),
            total: total_count,
            readme: None,
            write: true,
            provider: "local".to_string(),
        })
    }
} 