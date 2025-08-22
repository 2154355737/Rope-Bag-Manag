use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Cursor;
use anyhow::{Result, anyhow};
use actix_multipart::Multipart;
use actix_web::web::Bytes;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static TOKEN_CACHE: Lazy<Mutex<HashMap<(String, String), String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

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
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    pub raw_url: Option<String>, // 文件的真实下载地址
    pub provider: Option<String>, // 存储提供商
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListResponse {
    pub content: Option<Vec<FileInfo>>,
    pub total: i32,
    pub readme: Option<String>,
    pub write: bool,
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryInfo {
    pub name: Option<String>,
    pub size: Option<i64>,
    pub is_dir: bool,
    pub modified: Option<String>,
    pub created: Option<String>,
    pub sign: Option<String>,
    pub thumb: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    pub raw_url: Option<String>,
    pub readme: Option<String>,
    pub header: Option<String>,
    pub provider: Option<String>,
}

impl AListService {
    pub fn new() -> Self {
        // 默认从环境变量读取，若不存在则使用保底值
        log::info!("🔧 初始化AList服务客户端...");
        let base_url = std::env::var("ALIST_BASE_URL").unwrap_or_else(|_| "http://alist.tiecode.org.cn".to_string());
        let username = std::env::var("ALIST_USERNAME").unwrap_or_else(|_| "2154355737@qq.com".to_string());
        let password = std::env::var("ALIST_PASSWORD").unwrap_or_else(|_| "".to_string());

        log::info!("📡 AList服务器: {}", base_url);
        log::info!("👤 登录用户: {}", username);
        if !password.is_empty() { log::info!("🔐 已提供密码: ***"); } else { log::warn!("⚠️ 未提供ALIST_PASSWORD，可能无法登录"); }
        
        // 从全局缓存读取 token
        let cache_key = (base_url.clone(), username.clone());
        let token = TOKEN_CACHE.lock().ok().and_then(|m| m.get(&cache_key).cloned());
        
        Self {
            client: reqwest::Client::new(),
            base_url,
            username,
            password: password.to_string(),
            token,
        }
    }
    
    /// 获取当前 base_url
    pub fn base_url(&self) -> &str { &self.base_url }
    
    /// 通过参数创建（便于从数据库设置加载）
    pub fn new_with_params(base_url: String, username: String, password: String) -> Self {
        log::info!("🔧 初始化AList服务客户端(自定义参数)...");
        // 从全局缓存读取 token
        let cache_key = (base_url.clone(), username.clone());
        let token = TOKEN_CACHE.lock().ok().and_then(|m| m.get(&cache_key).cloned());
        Self {
            client: reqwest::Client::new(),
            base_url,
            username,
            password,
            token,
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
                // 写入全局缓存
                if let Ok(mut m) = TOKEN_CACHE.lock() {
                    m.insert((self.base_url.clone(), self.username.clone()), data.token.clone());
                }
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
    
    /// 确保已登录（如果没有token就先登录），优先使用缓存
    async fn ensure_logged_in(&mut self) -> Result<()> {
        if self.token.is_none() {
            // 尝试从缓存拿一次
            if let Ok(m) = TOKEN_CACHE.lock() {
                if let Some(tok) = m.get(&(self.base_url.clone(), self.username.clone())).cloned() {
                    self.token = Some(tok);
                }
            }
        }
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
                // 清空本地与缓存
                self.token = None; 
                if let Ok(mut m) = TOKEN_CACHE.lock() { m.remove(&(self.base_url.clone(), self.username.clone())); }
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

    /// 获取文件/目录信息（用于判断存在性与获取 raw_url）
    pub async fn get_info(&mut self, path: &str) -> Result<EntryInfo> {
        for attempt in 1..=2 {
            self.ensure_logged_in().await?;
            let url = format!("{}/api/fs/get", self.base_url);
            let body = json!({
                "path": path,
                "password": "",
                "page": 1,
                "per_page": 0,
                "refresh": false
            });
            let resp = self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", self.token.as_ref().unwrap())
                .json(&body)
                .send()
                .await?;
            let text = resp.text().await?;
            let parsed: AListResponse<EntryInfo> = serde_json::from_str(&text)?;
            if parsed.code == 200 {
                if let Some(data) = parsed.data { return Ok(data); }
                else { return Err(anyhow!("获取条目信息失败: data为空")); }
            }
            if parsed.message.to_lowercase().contains("token") && attempt == 1 {
                self.token = None; // 下次重试刷新 token
                if let Ok(mut m) = TOKEN_CACHE.lock() { m.remove(&(self.base_url.clone(), self.username.clone())); }
                continue;
            }
            return Err(anyhow!("获取条目信息失败: {}", parsed.message));
        }
        Err(anyhow!("获取条目信息失败: 超过最大重试次数"))
    }

    /// 判断目录是否存在（使用 /api/fs/get）
    pub async fn folder_exists(&mut self, path: &str) -> Result<bool> {
        match self.get_info(path).await {
            Ok(info) => Ok(info.is_dir),
            Err(e) => {
                let msg = e.to_string().to_lowercase();
                if msg.contains("not exist") || msg.contains("not found") || msg.contains("不存在") {
                    Ok(false)
                } else {
                    Err(anyhow!("检查目录存在性失败: {}", e))
                }
            }
        }
    }

    /// 若目录不存在则创建，存在则跳过
    pub async fn create_folder_if_missing(&mut self, path: &str) -> Result<bool> {
        match self.folder_exists(path).await {
            Ok(true) => {
                log::debug!("📁 目录已存在, 跳过创建: {}", path);
                Ok(false)
            }
            Ok(false) => {
                self.create_folder(path).await?;
                Ok(true)
            }
            Err(e) => Err(e),
        }
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
                if let Ok(mut m) = TOKEN_CACHE.lock() { m.remove(&(self.base_url.clone(), self.username.clone())); }
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
                if let Ok(mut m) = TOKEN_CACHE.lock() { m.remove(&(self.base_url.clone(), self.username.clone())); }
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
    
    /// 通过API获取下载直链（优先 raw_url，失败则退回 /d 路径）
    pub async fn get_download_link(&mut self, file_path: &str) -> Result<String> {
        match self.get_info(file_path).await {
            Ok(info) => {
                if let Some(url) = info.raw_url { return Ok(url); }
                // raw_url 为空时退回 /d 直链
                Ok(format!("{}/d{}", self.base_url, file_path))
            }
            Err(e) => {
                log::warn!("通过API获取 raw_url 失败，回退直链: {}", e);
                Ok(format!("{}/d{}", self.base_url, file_path))
            }
        }
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
        
        // token 失效清理缓存
        if response_json.message.contains("token") {
            self.token = None;
            if let Ok(mut m) = TOKEN_CACHE.lock() { m.remove(&(self.base_url.clone(), self.username.clone())); }
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

    /// 获取文件信息，包括下载直链
    pub async fn get_file_info(&mut self, file_path: &str) -> Result<FileInfo> {
        self.login().await?;

        let url = format!("{}/api/fs/get", self.base_url);
        
        let request_body = json!({
            "path": file_path,
            "password": ""
        });
        
        log::info!("🔍 获取文件信息: {}", file_path);
        
        let response = self.client
            .post(&url)
            .header("Authorization", self.token.as_ref().unwrap())
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        let result: AListResponse<FileInfo> = serde_json::from_str(&response_text)
            .map_err(|e| anyhow!("解析文件信息响应失败: {} - 响应: {}", e, response_text))?;
        
        if result.code == 200 {
            match result.data {
                Some(file_info) => {
                    log::info!("✅ 获取文件信息成功: {} - 大小: {} bytes, 下载地址: {:?}", 
                             file_info.name, file_info.size, file_info.raw_url);
                    Ok(file_info)
                },
                None => Err(anyhow!("文件信息为空")),
            }
        } else {
            Err(anyhow!("获取文件信息失败: {}", result.message))
        }
    }
} 