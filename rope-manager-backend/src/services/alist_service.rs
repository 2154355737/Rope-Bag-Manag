use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Cursor;
use anyhow::{Result, anyhow};
use actix_multipart::Multipart;
use actix_web::web::Bytes;

#[derive(Debug, Clone)]
pub struct AListService {
    client: reqwest::Client,
    base_url: String,
    username: String,
    password: String,
    token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AListResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: i64,
    pub is_dir: bool,
    pub modified: String,
    pub sign: Option<String>,
    pub thumb: Option<String>,
    pub type_: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListResponse {
    pub content: Vec<FileInfo>,
    pub total: i32,
    pub readme: Option<String>,
    pub write: bool,
    pub provider: String,
}

impl AListService {
    pub fn new() -> Self {
        log::info!("🔧 初始化AList服务客户端...");
        let base_url = "http://alist.tiecode.org.cn".to_string();
        let username = "2154355737@qq.com".to_string();
        let password = "ahk12378dx";
        
        log::info!("📡 AList服务器: {}", base_url);
        log::info!("👤 登录用户: {}", username);
        log::info!("🔐 密码: {}***", &password[..3]);
        
        Self {
            client: reqwest::Client::new(),
            base_url,
            username,
            password: password.to_string(),
            token: None,
        }
    }
    
    /// 登录获取token
    pub async fn login(&mut self) -> Result<()> {
        log::info!("🔑 正在登录AList服务...");
        let login_url = format!("{}/api/auth/login", self.base_url);
        log::debug!("🌐 登录URL: {}", login_url);
        
        let login_data = json!({
            "username": self.username,
            "password": self.password
        });
        
        log::info!("📤 发送登录请求...");
        let response = self.client
            .post(&login_url)
            .header("Content-Type", "application/json")
            .json(&login_data)
            .send()
            .await?;
        
        log::info!("📥 收到登录响应，状态码: {}", response.status());
        let response_text = response.text().await?;
        let response_json: AListResponse<LoginResponse> = serde_json::from_str(&response_text)?;
        
        if response_json.code == 200 {
            if let Some(data) = response_json.data {
                self.token = Some(data.token.clone());
                log::info!("✅ AList 登录成功");
                log::debug!("🎫 获取到Token: {}...", &data.token[..20]);
                return Ok(());
            } else {
                log::error!("❌ 登录响应中没有token数据");
            }
        } else {
            log::error!("❌ 登录失败，错误码: {}", response_json.code);
        }
        
        Err(anyhow!("AList 登录失败: {}", response_json.message))
    }
    
    /// 确保已登录（如果没有token就先登录）
    async fn ensure_logged_in(&mut self) -> Result<()> {
        if self.token.is_none() {
            self.login().await?;
        }
        Ok(())
    }
    
    /// 获取文件列表
    pub async fn list_files(&mut self, path: &str) -> Result<FileListResponse> {
        // 最多重试2次
        for attempt in 1..=2 {
            self.ensure_logged_in().await?;
            
            let list_url = format!("{}/api/fs/list", self.base_url);
            
            let list_data = json!({
                "path": path,
                "password": "",
                "page": 1,
                "per_page": 100,
                "refresh": false
            });
            
            log::debug!("🔄 获取文件列表尝试 {}: {}", attempt, path);
            
            let response = self.client
                .post(&list_url)
                .header("Content-Type", "application/json")
                .header("Authorization", self.token.as_ref().unwrap())
                .json(&list_data)
                .send()
                .await?;
            
            let response_text = response.text().await?;
            let response_json: AListResponse<FileListResponse> = serde_json::from_str(&response_text)?;
            
            if response_json.code == 200 {
                if let Some(data) = response_json.data {
                    log::debug!("✅ 获取文件列表成功: {}", path);
                    return Ok(data);
                }
            }
            
            // 检查是否是token失效错误
            if response_json.message.contains("token is invalidated") || response_json.message.contains("invalid token") {
                log::warn!("⚠️  Token失效，清除token并重试 (尝试 {})", attempt);
                self.token = None; // 清除失效的token
                if attempt == 2 {
                    return Err(anyhow!("获取文件列表失败，token重试后仍然无效: {}", response_json.message));
                }
                continue; // 重试
            }
            
            // 其他错误直接返回
            return Err(anyhow!("获取文件列表失败: {}", response_json.message));
        }
        
        Err(anyhow!("获取文件列表失败: 超过最大重试次数"))
    }
    
    /// 创建文件夹
    pub async fn create_folder(&mut self, path: &str) -> Result<()> {
        // 最多重试2次
        for attempt in 1..=2 {
            self.ensure_logged_in().await?;
            
            let mkdir_url = format!("{}/api/fs/mkdir", self.base_url);
            
            let mkdir_data = json!({
                "path": path
            });
            
            log::debug!("🔄 创建文件夹尝试 {}: {}", attempt, path);
            
            let response = self.client
                .post(&mkdir_url)
                .header("Content-Type", "application/json")
                .header("Authorization", self.token.as_ref().unwrap())
                .json(&mkdir_data)
                .send()
                .await?;
            
            let response_text = response.text().await?;
            let response_json: AListResponse<Value> = serde_json::from_str(&response_text)?;
            
            if response_json.code == 200 {
                log::info!("✅ 创建文件夹成功: {}", path);
                return Ok(());
            }
            
            // 检查是否是token失效错误
            if response_json.message.contains("token is invalidated") || response_json.message.contains("invalid token") {
                log::warn!("⚠️  Token失效，清除token并重试 (尝试 {})", attempt);
                self.token = None; // 清除失效的token
                if attempt == 2 {
                    return Err(anyhow!("创建文件夹失败，token重试后仍然无效: {}", response_json.message));
                }
                continue; // 重试
            }
            
            // 其他错误直接返回
            return Err(anyhow!("创建文件夹失败: {}", response_json.message));
        }
        
        Err(anyhow!("创建文件夹失败: 超过最大重试次数"))
    }
    
    /// 上传文件
    pub async fn upload_file(
        &mut self, 
        file_path: &str, 
        file_name: &str, 
        file_data: Bytes
    ) -> Result<String> {
        // 最多重试2次
        for attempt in 1..=2 {
            self.ensure_logged_in().await?;
            
            let upload_url = format!("{}/api/fs/put", self.base_url);
            let full_path = format!("{}/{}", file_path.trim_end_matches('/'), file_name);
            
            log::debug!("🔄 上传文件尝试 {}: {}", attempt, full_path);
            
            let response = self.client
                .put(&upload_url)
                .header("Authorization", self.token.as_ref().unwrap())
                .header("File-Path", &full_path)
                .header("Content-Type", "application/octet-stream")
                .body(file_data.clone())
                .send()
                .await?;
            
            let response_text = response.text().await?;
            let response_json: AListResponse<Value> = serde_json::from_str(&response_text)?;
            
            if response_json.code == 200 {
                log::info!("✅ 文件上传成功: {}", full_path);
                return Ok(full_path);
            }
            
            // 检查是否是token失效错误
            if response_json.message.contains("token is invalidated") || response_json.message.contains("invalid token") {
                log::warn!("⚠️  Token失效，清除token并重试 (尝试 {})", attempt);
                self.token = None; // 清除失效的token
                if attempt == 2 {
                    return Err(anyhow!("文件上传失败，token重试后仍然无效: {}", response_json.message));
                }
                continue; // 重试
            }
            
            // 其他错误直接返回
            return Err(anyhow!("文件上传失败: {}", response_json.message));
        }
        
        Err(anyhow!("文件上传失败: 超过最大重试次数"))
    }
    
    /// 获取文件下载链接
    pub async fn get_download_link(&mut self, file_path: &str) -> Result<String> {
        // 最多重试2次
        for attempt in 1..=2 {
            self.ensure_logged_in().await?;
            
            let link_url = format!("{}/api/fs/link", self.base_url);
            
            let link_data = json!({
                "path": file_path,
                "password": ""
            });
            
            log::debug!("🔄 获取下载链接尝试 {}: {}", attempt, file_path);
            
            let response = self.client
                .post(&link_url)
                .header("Content-Type", "application/json")
                .header("Authorization", self.token.as_ref().unwrap())
                .json(&link_data)
                .send()
                .await?;
            
            let response_text = response.text().await?;
            let response_json: AListResponse<Value> = serde_json::from_str(&response_text)?;
            
            if response_json.code == 200 {
                if let Some(data) = response_json.data {
                    if let Some(raw_url) = data.get("raw_url").and_then(|u| u.as_str()) {
                        log::debug!("✅ 获取下载链接成功: {}", file_path);
                        return Ok(raw_url.to_string());
                    }
                }
            }
            
            // 检查是否是token失效错误
            if response_json.message.contains("token is invalidated") || response_json.message.contains("invalid token") {
                log::warn!("⚠️  Token失效，清除token并重试 (尝试 {})", attempt);
                self.token = None; // 清除失效的token
                if attempt == 2 {
                    return Err(anyhow!("获取下载链接失败，token重试后仍然无效: {}", response_json.message));
                }
                continue; // 重试
            }
            
            // 其他错误直接返回
            return Err(anyhow!("获取下载链接失败: {}", response_json.message));
        }
        
        Err(anyhow!("获取下载链接失败: 超过最大重试次数"))
    }
    
    /// 删除文件
    pub async fn delete_file(&mut self, file_path: &str) -> Result<()> {
        self.ensure_logged_in().await?;
        
        let remove_url = format!("{}/api/fs/remove", self.base_url);
        
        let remove_data = json!({
            "names": [file_path],
            "dir": "/"
        });
        
        let response = self.client
            .post(&remove_url)
            .header("Content-Type", "application/json")
            .header("Authorization", self.token.as_ref().unwrap())
            .json(&remove_data)
            .send()
            .await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<Value> = serde_json::from_str(&response_text)?;
        
        if response_json.code == 200 {
            log::info!("🗑️ 文件删除成功: {}", file_path);
            return Ok(());
        }
        
        Err(anyhow!("文件删除失败: {}", response_json.message))
    }
    
    /// 重命名文件
    pub async fn rename_file(&mut self, old_path: &str, new_name: &str) -> Result<()> {
        self.ensure_logged_in().await?;
        
        let rename_url = format!("{}/api/fs/rename", self.base_url);
        
        let rename_data = json!({
            "path": old_path,
            "name": new_name
        });
        
        let response = self.client
            .post(&rename_url)
            .header("Content-Type", "application/json")
            .header("Authorization", self.token.as_ref().unwrap())
            .json(&rename_data)
            .send()
            .await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<Value> = serde_json::from_str(&response_text)?;
        
        if response_json.code == 200 {
            log::info!("✏️ 文件重命名成功: {} -> {}", old_path, new_name);
            return Ok(());
        }
        
        Err(anyhow!("文件重命名失败: {}", response_json.message))
    }
} 