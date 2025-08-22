use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};
use actix_web::web::Bytes;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

// 改进的Token缓存，包含过期时间
struct TokenCacheEntry {
    token: String,
    expires_at: Instant,
}

// 全局Token缓存，带过期时间
static TOKEN_CACHE: Lazy<Mutex<HashMap<(String, String), TokenCacheEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Token有效期（2小时）
const TOKEN_VALIDITY_DURATION: Duration = Duration::from_secs(2 * 60 * 60);

// 最大重试次数
const MAX_RETRY_ATTEMPTS: u8 = 3;

// 重试延迟（毫秒）
const RETRY_DELAY_MS: u64 = 500;

#[derive(Debug, Clone)]
pub struct AListService {
    client: reqwest::Client,
    base_url: String,
    username: String,
    password: String,
    token: Option<String>,
    last_error: Option<String>,
    is_available: bool,
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
        let token = Self::get_cached_token(&base_url, &username);
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        Self {
            client,
            base_url,
            username,
            password: password.to_string(),
            token,
            last_error: None,
            is_available: true,
        }
    }
    
    /// 获取当前 base_url
    pub fn base_url(&self) -> &str { &self.base_url }
    
    /// 通过参数创建（便于从数据库设置加载）
    pub fn new_with_params(base_url: String, username: String, password: String) -> Self {
        log::info!("🔧 初始化AList服务客户端(自定义参数)...");
        // 从全局缓存读取 token
        let token = Self::get_cached_token(&base_url, &username);
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        
        Self {
            client,
            base_url,
            username,
            password,
            token,
            last_error: None,
            is_available: true,
        }
    }
    
    /// 从缓存获取token
    fn get_cached_token(base_url: &str, username: &str) -> Option<String> {
        if let Ok(cache) = TOKEN_CACHE.lock() {
            let key = (base_url.to_string(), username.to_string());
            if let Some(entry) = cache.get(&key) {
                if Instant::now() < entry.expires_at {
                    return Some(entry.token.clone());
                }
            }
        }
        None
    }
    
    /// 缓存token
    fn cache_token(&self, token: &str) {
        if let Ok(mut cache) = TOKEN_CACHE.lock() {
            let key = (self.base_url.clone(), self.username.clone());
            let entry = TokenCacheEntry {
                token: token.to_string(),
                expires_at: Instant::now() + TOKEN_VALIDITY_DURATION,
            };
            cache.insert(key, entry);
        }
    }
    
    /// 清除缓存的token
    fn clear_cached_token(&self) {
        if let Ok(mut cache) = TOKEN_CACHE.lock() {
            let key = (self.base_url.clone(), self.username.clone());
            cache.remove(&key);
        }
    }
    
    /// 获取上次错误
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
    
    /// 检查服务是否可用
    pub fn is_available(&self) -> bool {
        self.is_available
    }
    
    /// 设置服务状态
    fn set_availability(&mut self, available: bool) {
        self.is_available = available;
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
        
        // 使用带重试的请求
        let response = self.send_request_with_retry(
            |client| {
                client
                    .post(&login_url)
                    .header("Content-Type", "application/json")
                    .json(&login_data)
            }
        ).await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<LoginResponse> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析登录响应失败: {}", e));
                return Err(anyhow!("解析登录响应失败: {}", e));
            }
        };
        
        if response_json.code == 200 {
            if let Some(data) = response_json.data {
                self.token = Some(data.token.clone());
                // 写入全局缓存
                self.cache_token(&data.token);
                log::info!("✅ AList 登录成功");
                log::debug!("🎫 获取到Token: {}...", &data.token[..20]);
                self.set_availability(true);
                self.last_error = None;
                return Ok(());
            } else {
                self.last_error = Some("登录响应中没有token数据".to_string());
                log::error!("❌ 登录响应中没有token数据");
            }
        } else {
            self.last_error = Some(format!("登录失败，错误码: {}, 消息: {}", response_json.code, response_json.message));
            log::error!("❌ 登录失败，错误码: {}, 消息: {}", response_json.code, response_json.message);
        }
        
        self.set_availability(false);
        Err(anyhow!("AList 登录失败: {}", response_json.message))
    }
    
    /// 确保已登录（如果没有token就先登录），优先使用缓存
    async fn ensure_logged_in(&mut self) -> Result<()> {
        if self.token.is_none() {
            // 尝试从缓存拿一次
            self.token = Self::get_cached_token(&self.base_url, &self.username);
        }
        if self.token.is_none() {
            self.login().await?;
        }
        Ok(())
    }
    
    /// 带重试的请求发送
    async fn send_request_with_retry<F>(&self, request_builder: F) -> Result<reqwest::Response> 
    where
        F: Fn(&reqwest::Client) -> reqwest::RequestBuilder,
    {
        let mut last_error = None;
        
        for attempt in 1..=MAX_RETRY_ATTEMPTS {
            match request_builder(&self.client).send().await {
                Ok(response) => {
                    if response.status().is_success() || response.status().as_u16() == 400 {
                        return Ok(response);
                    }
                    
                    // 服务器错误，可以重试
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_else(|_| "无法读取响应内容".to_string());
                    last_error = Some(anyhow!("服务器错误 (HTTP {}): {}", status, error_text));
                },
                Err(e) => {
                    last_error = Some(anyhow!("请求失败: {}", e));
                }
            }
            
            log::warn!("⚠️ 请求失败，尝试 {}/{}，错误: {:?}", attempt, MAX_RETRY_ATTEMPTS, last_error);
            
            if attempt < MAX_RETRY_ATTEMPTS {
                // 指数退避重试
                let delay = RETRY_DELAY_MS * 2u64.pow(attempt as u32 - 1);
                tokio::time::sleep(Duration::from_millis(delay)).await;
            }
        }
        
        Err(last_error.unwrap_or_else(|| anyhow!("请求失败，超过最大重试次数")))
    }
    
    /// 健康检查
    pub async fn health_check(&mut self) -> bool {
        match self.login().await {
            Ok(_) => {
                self.set_availability(true);
                true
            },
            Err(e) => {
                log::error!("❌ AList健康检查失败: {}", e);
                self.set_availability(false);
                self.last_error = Some(format!("健康检查失败: {}", e));
                false
            }
        }
    }
    
    /// 获取文件列表
    pub async fn list_files(&mut self, path: &str) -> Result<FileListResponse> {
        self.ensure_logged_in().await?;
        
        let list_url = format!("{}/api/fs/list", self.base_url);
        
        let list_data = json!({
            "path": path,
            "password": "",
            "page": 1,
            "per_page": 100,
            "refresh": false
        });
        
        log::debug!("🔄 获取文件列表: {}", path);
        
        let response = self.send_request_with_retry(
            |client| {
                client
                    .post(&list_url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", self.token.as_ref().unwrap())
                    .json(&list_data)
            }
        ).await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<FileListResponse> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析文件列表响应失败: {}", e));
                return Err(anyhow!("解析文件列表响应失败: {}", e));
            }
        };
        
        if response_json.code == 200 {
            if let Some(data) = response_json.data {
                log::debug!("✅ 获取文件列表成功: {}", path);
                return Ok(data);
            }
        }
        
        // 检查是否是token失效错误
        if response_json.message.contains("token is invalidated") || 
           response_json.message.contains("invalid token") {
            log::warn!("⚠️ Token失效，清除token并重新登录");
            self.token = None;
            self.clear_cached_token();
            self.login().await?;
            
            // 重新尝试请求 - 使用Box::pin避免无限大小的Future
            return Box::pin(self.list_files(path)).await;
        }
        
        self.last_error = Some(format!("获取文件列表失败: {}", response_json.message));
        Err(anyhow!("获取文件列表失败: {}", response_json.message))
    }

    /// 获取文件/目录信息（用于判断存在性与获取 raw_url）
    pub async fn get_info(&mut self, path: &str) -> Result<EntryInfo> {
        self.ensure_logged_in().await?;
        
        let url = format!("{}/api/fs/get", self.base_url);
        let body = json!({
            "path": path,
            "password": "",
            "page": 1,
            "per_page": 0,
            "refresh": false
        });
        
        let resp = self.send_request_with_retry(
            |client| {
                client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", self.token.as_ref().unwrap())
                    .json(&body)
            }
        ).await?;
        
        let text = resp.text().await?;
        let parsed: AListResponse<EntryInfo> = match serde_json::from_str(&text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析条目信息响应失败: {}", e));
                return Err(anyhow!("解析条目信息响应失败: {}", e));
            }
        };
        
        if parsed.code == 200 {
            if let Some(data) = parsed.data { 
                return Ok(data); 
            } else { 
                self.last_error = Some("获取条目信息失败: data为空".to_string());
                return Err(anyhow!("获取条目信息失败: data为空")); 
            }
        }
        
        // 检查是否是token失效错误
        if parsed.message.contains("token is invalidated") || 
           parsed.message.contains("invalid token") {
            log::warn!("⚠️ Token失效，清除token并重新登录");
            self.token = None;
            self.clear_cached_token();
            self.login().await?;
            
            // 重新尝试请求 - 使用Box::pin避免无限大小的Future
            return Box::pin(self.get_info(path)).await;
        }
        
        self.last_error = Some(format!("获取条目信息失败: {}", parsed.message));
        Err(anyhow!("获取条目信息失败: {}", parsed.message))
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
                    self.last_error = Some(format!("检查目录存在性失败: {}", e));
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
        self.ensure_logged_in().await?;
        
        let mkdir_url = format!("{}/api/fs/mkdir", self.base_url);
        
        let mkdir_data = json!({
            "path": path
        });
        
        log::debug!("🔄 创建文件夹: {}", path);
        
        let response = self.send_request_with_retry(
            |client| {
                client
                    .post(&mkdir_url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", self.token.as_ref().unwrap())
                    .json(&mkdir_data)
            }
        ).await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<Value> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析创建文件夹响应失败: {}", e));
                return Err(anyhow!("解析创建文件夹响应失败: {}", e));
            }
        };
        
        if response_json.code == 200 {
            log::info!("✅ 创建文件夹成功: {}", path);
            return Ok(());
        }
        
        // 检查是否是token失效错误
        if response_json.message.contains("token is invalidated") || 
           response_json.message.contains("invalid token") {
            log::warn!("⚠️ Token失效，清除token并重新登录");
            self.token = None;
            self.clear_cached_token();
            self.login().await?;
            
            // 重新尝试请求 - 使用Box::pin避免无限大小的Future
            return Box::pin(self.create_folder(path)).await;
        }
        
        self.last_error = Some(format!("创建文件夹失败: {}", response_json.message));
        Err(anyhow!("创建文件夹失败: {}", response_json.message))
    }
    
    /// 上传文件
    pub async fn upload_file(
        &mut self, 
        file_path: &str, 
        file_name: &str, 
        file_data: Bytes
    ) -> Result<String> {
        self.ensure_logged_in().await?;
        
        let upload_url = format!("{}/api/fs/put", self.base_url);
        let full_path = format!("{}/{}", file_path.trim_end_matches('/'), file_name);
        
        log::debug!("🔄 上传文件: {} (大小: {} 字节)", full_path, file_data.len());
        
        let response = self.send_request_with_retry(
            |client| {
                client
                    .put(&upload_url)
                    .header("Authorization", self.token.as_ref().unwrap())
                    .header("File-Path", &full_path)
                    .header("Content-Type", "application/octet-stream")
                    .body(file_data.clone())
            }
        ).await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<Value> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析上传文件响应失败: {}", e));
                return Err(anyhow!("解析上传文件响应失败: {}", e));
            }
        };
        
        if response_json.code == 200 {
            log::info!("✅ 文件上传成功: {}", full_path);
            return Ok(full_path);
        }
        
        // 检查是否是token失效错误
        if response_json.message.contains("token is invalidated") || 
           response_json.message.contains("invalid token") {
            log::warn!("⚠️ Token失效，清除token并重新登录");
            self.token = None;
            self.clear_cached_token();
            self.login().await?;
            
            // 重新尝试请求 - 使用Box::pin避免无限大小的Future
            return Box::pin(self.upload_file(file_path, file_name, file_data)).await;
        }
        
        self.last_error = Some(format!("文件上传失败: {}", response_json.message));
        Err(anyhow!("文件上传失败: {}", response_json.message))
    }
    
    /// 通过API获取下载直链（优先 raw_url，失败则退回 /d 路径）
    pub async fn get_download_link(&mut self, file_path: &str) -> Result<String> {
        match self.get_info(file_path).await {
            Ok(info) => {
                if let Some(url) = info.raw_url { 
                    return Ok(url); 
                }
                // raw_url 为空时退回 /d 直链
                Ok(format!("{}/d{}", self.base_url, file_path))
            }
            Err(e) => {
                log::warn!("通过API获取 raw_url 失败，回退直链: {}", e);
                self.last_error = Some(format!("获取下载链接失败: {}", e));
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
        
        log::debug!("🔄 删除文件: {}", file_path);
        
        let response = self.send_request_with_retry(
            |client| {
                client
                    .post(&remove_url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", self.token.as_ref().unwrap())
                    .json(&remove_data)
            }
        ).await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<Value> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析删除文件响应失败: {}", e));
                return Err(anyhow!("解析删除文件响应失败: {}", e));
            }
        };
        
        if response_json.code == 200 {
            log::info!("🗑️ 文件删除成功: {}", file_path);
            return Ok(());
        }
        
        // 检查是否是token失效错误
        if response_json.message.contains("token is invalidated") || 
           response_json.message.contains("invalid token") {
            log::warn!("⚠️ Token失效，清除token并重新登录");
            self.token = None;
            self.clear_cached_token();
            self.login().await?;
            
            // 重新尝试请求 - 使用Box::pin避免无限大小的Future
            return Box::pin(self.delete_file(file_path)).await;
        }
        
        self.last_error = Some(format!("文件删除失败: {}", response_json.message));
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
        
        log::debug!("🔄 重命名文件: {} -> {}", old_path, new_name);
        
        let response = self.send_request_with_retry(
            |client| {
                client
                    .post(&rename_url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", self.token.as_ref().unwrap())
                    .json(&rename_data)
            }
        ).await?;
        
        let response_text = response.text().await?;
        let response_json: AListResponse<Value> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                self.last_error = Some(format!("解析重命名文件响应失败: {}", e));
                return Err(anyhow!("解析重命名文件响应失败: {}", e));
            }
        };
        
        if response_json.code == 200 {
            log::info!("✏️ 文件重命名成功: {} -> {}", old_path, new_name);
            return Ok(());
        }
        
        // 检查是否是token失效错误
        if response_json.message.contains("token is invalidated") || 
           response_json.message.contains("invalid token") {
            log::warn!("⚠️ Token失效，清除token并重新登录");
            self.token = None;
            self.clear_cached_token();
            self.login().await?;
            
            // 重新尝试请求 - 使用Box::pin避免无限大小的Future
            return Box::pin(self.rename_file(old_path, new_name)).await;
        }
        
        self.last_error = Some(format!("文件重命名失败: {}", response_json.message));
        Err(anyhow!("文件重命名失败: {}", response_json.message))
    }

    /// 获取文件信息，包括下载直链
    pub async fn get_file_info(&mut self, file_path: &str) -> Result<FileInfo> {
        self.ensure_logged_in().await?;

        let url = format!("{}/api/fs/get", self.base_url);
        
        let request_body = json!({
            "path": file_path,
            "password": ""
        });
        
        log::debug!("🔍 获取文件信息: {}", file_path);
        
        let response = self.send_request_with_retry(
            |client| {
                client
                    .post(&url)
                    .header("Authorization", self.token.as_ref().unwrap())
                    .header("Content-Type", "application/json")
                    .json(&request_body)
            }
        ).await?;
            
        let response_text = response.text().await?;
        let result: AListResponse<FileInfo> = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                let error_msg = format!("解析文件信息响应失败: {} - 响应: {}", e, response_text);
                self.last_error = Some(error_msg.clone());
                return Err(anyhow!(error_msg));
            }
        };
        
        if result.code == 200 {
            match result.data {
                Some(file_info) => {
                    log::debug!("✅ 获取文件信息成功: {} - 大小: {} bytes, 下载地址: {:?}", 
                             file_info.name, file_info.size, file_info.raw_url);
                    Ok(file_info)
                },
                None => {
                    self.last_error = Some("文件信息为空".to_string());
                    Err(anyhow!("文件信息为空"))
                },
            }
        } else {
            // 检查是否是token失效错误
            if result.message.contains("token is invalidated") || 
               result.message.contains("invalid token") {
                log::warn!("⚠️ Token失效，清除token并重新登录");
                self.token = None;
                self.clear_cached_token();
                self.login().await?;
                
                // 重新尝试请求 - 使用Box::pin避免无限大小的Future
                return Box::pin(self.get_file_info(file_path)).await;
            }
            
            self.last_error = Some(format!("获取文件信息失败: {}", result.message));
            Err(anyhow!("获取文件信息失败: {}", result.message))
        }
    }
    
    /// 验证文件是否存在并可访问
    pub async fn verify_file_exists(&mut self, file_path: &str) -> Result<bool> {
        match self.get_file_info(file_path).await {
            Ok(_) => Ok(true),
            Err(e) => {
                let msg = e.to_string().to_lowercase();
                if msg.contains("not exist") || msg.contains("not found") || msg.contains("不存在") {
                    Ok(false)
                } else {
                    self.last_error = Some(format!("验证文件存在性失败: {}", e));
                    Err(anyhow!("验证文件存在性失败: {}", e))
                }
            }
        }
    }
}
