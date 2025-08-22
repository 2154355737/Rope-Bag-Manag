use crate::services::alist_service::{AListService, FileInfo, FileListResponse};
use crate::models::Package;
use crate::repositories::package_repo::PackageRepository;
use anyhow::{Result, anyhow};
use actix_web::web::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// 存储统计信息
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_files: usize,
    pub total_size: i64,
    pub file_count_by_type: HashMap<String, usize>,
    pub size_by_type: HashMap<String, i64>,
    pub file_count_by_category: HashMap<String, usize>,
    pub size_by_category: HashMap<String, i64>,
    pub orphaned_files: usize,
    pub orphaned_size: i64,
}

// 清理结果
#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupResult {
    pub deleted_files: usize,
    pub freed_space: i64,
    pub failed_files: usize,
    pub details: Vec<String>,
}

// 存储服务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageServiceStatus {
    Uninitialized,  // 未初始化
    Initializing,   // 正在初始化
    Ready,          // 已准备好
    Failed,         // 初始化失败
}

impl std::fmt::Display for StorageServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uninitialized => write!(f, "未初始化"),
            Self::Initializing => write!(f, "正在初始化"),
            Self::Ready => write!(f, "已就绪"),
            Self::Failed => write!(f, "初始化失败"),
        }
    }
}

// 全局存储服务实例缓存
static STORAGE_SERVICE_INSTANCE: Lazy<Arc<Mutex<Option<PackageStorageService>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(None)));

// 全局存储服务状态
static STORAGE_SERVICE_STATUS: Lazy<Arc<RwLock<StorageServiceStatus>>> = 
    Lazy::new(|| Arc::new(RwLock::new(StorageServiceStatus::Uninitialized)));

// 全局存储服务错误信息
static STORAGE_SERVICE_ERROR: Lazy<Arc<RwLock<Option<String>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(None)));

// 全局存储服务最后健康检查时间
static LAST_HEALTH_CHECK: Lazy<Arc<RwLock<Option<Instant>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(None)));

// 健康检查间隔（10分钟）
const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(10 * 60);

#[derive(Debug, Clone)]
pub struct PackageStorageService {
    package_repo: PackageRepository,
    alist_service: AListService,
    storage_base_path: String,
    db_path: String,
    status: StorageServiceStatus,
    last_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResult {
    pub file_path: String,
    pub download_url: String,
    pub file_size: i64,
}

impl PackageStorageService {
    /// 获取或创建全局存储服务实例
    pub async fn get_instance(db_path: &str) -> Result<Self> {
        // 尝试从全局缓存获取实例
        {
            let instance_lock = STORAGE_SERVICE_INSTANCE.lock().unwrap();
            if let Some(instance) = &*instance_lock {
                log::debug!("♻️ 使用缓存的存储服务实例");
                return Ok(instance.clone());
            }
        }
        
        // 如果没有缓存实例，创建新实例
        log::info!("🚀 初始化存储服务...");
        
        let package_repo = PackageRepository::new(db_path)
            .map_err(|e| anyhow!("创建包仓库失败: {}", e))?;
        log::info!("✅ 数据库连接已建立: {}", db_path);
        
        // 优先从系统设置读取AList参数，否则退回到环境变量/默认
        let (base_url, username, password) = {
            use rusqlite::Connection;
            let conn = Connection::open(db_path).ok();
            if let Some(conn) = conn {
                let get = |k: &str| -> Option<String> {
                    conn.query_row("SELECT value FROM system_settings WHERE key = ?", [k], |r| r.get::<_, String>(0)).ok()
                };
                let bu = get("alist_base_url");
                let un = get("alist_username");
                let pw = get("alist_password");
                (bu, un, pw)
            } else { (None, None, None) }
        };
        
        let alist_service = match (base_url, username, password) {
            (Some(bu), Some(un), Some(pw)) => {
                log::info!("📡 使用数据库配置初始化AList服务: {}", bu);
                AListService::new_with_params(bu, un, pw)
            },
            _ => {
                log::info!("📡 使用环境变量/默认值初始化AList服务");
                AListService::new()
            },
        };
        
        let storage_base_path = "/image/结绳社区".to_string();
        log::info!("📁 存储基础路径: {}", storage_base_path);
        
        let instance = Self {
            package_repo,
            alist_service,
            storage_base_path,
            db_path: db_path.to_string(),
            status: StorageServiceStatus::Uninitialized,
            last_error: None,
        };
        
        // 将实例保存到全局缓存
        {
            let mut instance_lock = STORAGE_SERVICE_INSTANCE.lock().unwrap();
            *instance_lock = Some(instance.clone());
        }
        
        Ok(instance)
    }
    
    /// 创建新实例（不使用缓存，主要用于测试）
    pub fn new(db_path: &str) -> Result<Self> {
        log::info!("🚀 创建新的存储服务实例（不使用缓存）...");
        
        let package_repo = PackageRepository::new(db_path)
            .map_err(|e| anyhow!("创建包仓库失败: {}", e))?;
        
        // 优先从系统设置读取AList参数，否则退回到环境变量/默认
        let (base_url, username, password) = {
            use rusqlite::Connection;
            let conn = Connection::open(db_path).ok();
            if let Some(conn) = conn {
                let get = |k: &str| -> Option<String> {
                    conn.query_row("SELECT value FROM system_settings WHERE key = ?", [k], |r| r.get::<_, String>(0)).ok()
                };
                let bu = get("alist_base_url");
                let un = get("alist_username");
                let pw = get("alist_password");
                (bu, un, pw)
            } else { (None, None, None) }
        };
        
        let alist_service = match (base_url, username, password) {
            (Some(bu), Some(un), Some(pw)) => {
                AListService::new_with_params(bu, un, pw)
            },
            _ => {
                AListService::new()
            },
        };
        
        let storage_base_path = "/image/结绳社区".to_string();
        
        Ok(Self {
            package_repo,
            alist_service,
            storage_base_path,
            db_path: db_path.to_string(),
            status: StorageServiceStatus::Uninitialized,
            last_error: None,
        })
    }
    
    /// 获取存储服务状态
    pub async fn get_status() -> StorageServiceStatus {
        *STORAGE_SERVICE_STATUS.read().await
    }
    
    /// 获取存储服务错误信息
    pub async fn get_error() -> Option<String> {
        STORAGE_SERVICE_ERROR.read().await.clone()
    }
    
    /// 设置存储服务状态
    async fn set_status(status: StorageServiceStatus) {
        let mut status_lock = STORAGE_SERVICE_STATUS.write().await;
        *status_lock = status;
    }
    
    /// 设置存储服务错误信息
    async fn set_error(error: Option<String>) {
        let mut error_lock = STORAGE_SERVICE_ERROR.write().await;
        *error_lock = error;
    }
    
    /// 初始化存储（创建必要的文件夹）
    pub async fn initialize_storage(&mut self) -> Result<()> {
        // 更新全局状态为初始化中
        Self::set_status(StorageServiceStatus::Initializing).await;
        self.status = StorageServiceStatus::Initializing;
        
        log::info!("🔧 开始初始化存储目录结构...");
        
        // 先进行健康检查，确保AList服务可用
        if !self.alist_service.health_check().await {
            let error = format!("AList服务健康检查失败: {}", 
                self.alist_service.last_error().unwrap_or("未知错误"));
            log::error!("❌ {}", error);
            
            // 更新全局状态为失败
            Self::set_status(StorageServiceStatus::Failed).await;
            Self::set_error(Some(error.clone())).await;
            
            self.status = StorageServiceStatus::Failed;
            self.last_error = Some(error.clone());
            
            return Err(anyhow!(error));
        }
        
        // 创建基础目录（存在则跳过）
        log::info!("📁 确认基础目录: {}", self.storage_base_path);
        match self.alist_service.create_folder_if_missing(&self.storage_base_path).await {
            Ok(created) => {
                if created { log::info!("✅ 基础目录创建成功"); } else { log::info!("✅ 基础目录已存在"); }
            }
            Err(e) => {
                let error = format!("基础目录检查/创建失败: {}", e);
                log::error!("❌ {}", error);
                
                // 更新全局状态为失败
                Self::set_status(StorageServiceStatus::Failed).await;
                Self::set_error(Some(error.clone())).await;
                
                self.status = StorageServiceStatus::Failed;
                self.last_error = Some(error.clone());
                
                return Err(anyhow!(error));
            }
        }
        
        // 获取所有分类并创建对应目录
        log::info!("🗂️  正在获取分类列表...");
        match self.get_all_categories().await {
            Ok(categories) => {
                if categories.is_empty() {
                    log::warn!("⚠️  未找到任何分类，将创建默认分类目录");
                    if let Err(e) = self.create_category_directories(&["默认分类"]).await {
                        let error = format!("创建默认分类目录失败: {}", e);
                        log::error!("❌ {}", error);
                        
                        // 更新全局状态为失败
                        Self::set_status(StorageServiceStatus::Failed).await;
                        Self::set_error(Some(error.clone())).await;
                        
                        self.status = StorageServiceStatus::Failed;
                        self.last_error = Some(error.clone());
                        
                        return Err(anyhow!(error));
                    }
                } else {
                    log::info!("📋 找到 {} 个分类", categories.len());
                    let category_names: Vec<&str> = categories.iter().map(|c| c.as_str()).collect();
                    if let Err(e) = self.create_category_directories(&category_names).await {
                        let error = format!("创建分类目录失败: {}", e);
                        log::error!("❌ {}", error);
                        
                        // 更新全局状态为失败
                        Self::set_status(StorageServiceStatus::Failed).await;
                        Self::set_error(Some(error.clone())).await;
                        
                        self.status = StorageServiceStatus::Failed;
                        self.last_error = Some(error.clone());
                        
                        return Err(anyhow!(error));
                    }
                }
            },
            Err(e) => {
                log::error!("❌ 获取分类失败: {}", e);
                log::info!("🔄 使用默认分类目录");
                if let Err(e) = self.create_category_directories(&["默认分类"]).await {
                    let error = format!("创建默认分类目录失败: {}", e);
                    log::error!("❌ {}", error);
                    
                    // 更新全局状态为失败
                    Self::set_status(StorageServiceStatus::Failed).await;
                    Self::set_error(Some(error.clone())).await;
                    
                    self.status = StorageServiceStatus::Failed;
                    self.last_error = Some(error.clone());
                    
                    return Err(anyhow!(error));
                }
            }
        }
        
        // 更新全局状态为就绪
        Self::set_status(StorageServiceStatus::Ready).await;
        Self::set_error(None).await;
        
        // 更新最后健康检查时间
        let mut last_check = LAST_HEALTH_CHECK.write().await;
        *last_check = Some(Instant::now());
        
        self.status = StorageServiceStatus::Ready;
        self.last_error = None;
        
        log::info!("🎉 存储目录初始化完成！");
        Ok(())
    }
    
    /// 创建分类目录
    async fn create_category_directories(&mut self, categories: &[&str]) -> Result<()> {
        let now = chrono::Utc::now();
        let year_month = now.format("%Y-%m").to_string();
        
        for category in categories {
            let category_path = format!("{}/{}", self.storage_base_path, category);
            let monthly_path = format!("{}/{}", category_path, year_month);
            
            log::info!("📂 创建分类目录: {}", category_path);
            match self.alist_service.create_folder_if_missing(&category_path).await {
                Ok(created) => {
                    if created { log::info!("✅ 分类目录创建成功: {}", category); } else { log::info!("📁 分类目录已存在: {}", category); }
                }
                Err(e) => {
                    log::warn!("⚠️ 分类目录检查/创建失败: {} ({})", category, e);
                    // 继续尝试其他分类，不要因为一个分类失败就中断整个流程
                    continue;
                }
            }
            
            log::info!("📅 创建月份目录: {}", monthly_path);
            match self.alist_service.create_folder_if_missing(&monthly_path).await {
                Ok(created) => {
                    if created { log::info!("✅ 月份目录创建成功: {}/{}", category, year_month); } else { log::info!("📁 月份目录已存在: {}/{}", category, year_month); }
                }
                Err(e) => {
                    log::warn!("⚠️ 月份目录检查/创建失败: {}/{} ({})", category, year_month, e);
                    // 继续尝试其他分类，不要因为一个分类失败就中断整个流程
                }
            }
        }
        
        Ok(())
    }
    
    /// 获取所有分类名称
    async fn get_all_categories(&self) -> Result<Vec<String>> {
        // 从数据库获取分类信息
        use rusqlite::Connection;
        
        let conn = Connection::open(&self.db_path)
            .map_err(|e| anyhow!("打开数据库连接失败: {}", e))?;
        let mut stmt = conn.prepare("SELECT name FROM categories WHERE 1=1")
            .map_err(|e| anyhow!("准备SQL语句失败: {}", e))?;
        let rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        }).map_err(|e| anyhow!("执行查询失败: {}", e))?;
        
        let mut categories = Vec::new();
        for row in rows {
            categories.push(row.map_err(|e| anyhow!("读取分类名称失败: {}", e))?);
        }
        
        Ok(categories)
    }
    
    /// 健康检查
    pub async fn health_check(&mut self) -> bool {
        // 检查是否需要进行健康检查
        let should_check = {
            let last_check = LAST_HEALTH_CHECK.read().await;
            match *last_check {
                Some(time) => Instant::now().duration_since(time) > HEALTH_CHECK_INTERVAL,
                None => true,
            }
        };
        
        if !should_check {
            return self.status == StorageServiceStatus::Ready;
        }
        
        // 进行健康检查
        let result = self.alist_service.health_check().await;
        
        // 更新最后健康检查时间
        let mut last_check = LAST_HEALTH_CHECK.write().await;
        *last_check = Some(Instant::now());
        
        if result {
            // 如果之前状态是失败，尝试重新初始化
            if self.status == StorageServiceStatus::Failed {
                log::info!("🔄 存储服务之前失败，尝试重新初始化");
                match self.initialize_storage().await {
                    Ok(_) => {
                        log::info!("✅ 存储服务重新初始化成功");
                        return true;
                    },
                    Err(e) => {
                        log::error!("❌ 存储服务重新初始化失败: {}", e);
                        return false;
                    }
                }
            }
            
            // 如果状态是未初始化，尝试初始化
            if self.status == StorageServiceStatus::Uninitialized {
                log::info!("🔄 存储服务未初始化，尝试初始化");
                match self.initialize_storage().await {
                    Ok(_) => {
                        log::info!("✅ 存储服务初始化成功");
                        return true;
                    },
                    Err(e) => {
                        log::error!("❌ 存储服务初始化失败: {}", e);
                        return false;
                    }
                }
            }
            
            return true;
        } else {
            // 更新状态为失败
            self.status = StorageServiceStatus::Failed;
            self.last_error = self.alist_service.last_error().map(|s| s.to_string());
            
            // 更新全局状态
            Self::set_status(StorageServiceStatus::Failed).await;
            Self::set_error(self.last_error.clone()).await;
            
            log::error!("❌ 存储服务健康检查失败: {}", 
                self.last_error.as_deref().unwrap_or("未知错误"));
            
            return false;
        }
    }
    
    /// 确保存储服务已初始化并可用
    async fn ensure_storage_ready(&mut self) -> Result<()> {
        match self.status {
            StorageServiceStatus::Ready => Ok(()),
            StorageServiceStatus::Initializing => {
                Err(anyhow!("存储服务正在初始化中，请稍后再试"))
            },
            StorageServiceStatus::Failed => {
                // 尝试重新初始化
                log::info!("🔄 存储服务之前失败，尝试重新初始化");
                self.initialize_storage().await
            },
            StorageServiceStatus::Uninitialized => {
                // 初始化
                log::info!("🔄 存储服务未初始化，尝试初始化");
                self.initialize_storage().await
            }
        }
    }
    
    /// 上传包截图文件
    pub async fn upload_package_screenshot(
        &mut self,
        file_name: &str,
        file_data: Bytes,
        package_id: i32
    ) -> Result<UploadResult> {
        log::info!("📷 开始上传截图: {} (资源ID: {})", file_name, package_id);
        
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 获取包的分类名称和资源名称
        let (category_name, package_name) = match self.get_package_info(package_id).await {
            Ok((cat, name)) => {
                log::info!("📂 资源分类: {}, 资源名称: {}", cat, name);
                (cat, name)
            },
            Err(e) => {
                log::warn!("⚠️  获取资源信息失败: {}，使用默认值", e);
                ("默认分类".to_string(), format!("resource_{}", package_id))
            }
        };
        
        // 生成文件名: 资源id_文件名.扩展名
        let file_extension = std::path::Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("png");
        
        // 清理文件名中的特殊字符
        let clean_package_name = package_name
            .replace("/", "_")
            .replace("\\", "_")
            .replace(":", "_")
            .replace("*", "_")
            .replace("?", "_")
            .replace("\"", "_")
            .replace("<", "_")
            .replace(">", "_")
            .replace("|", "_");
        
        let unique_name = format!("{}_{}.{}", package_id, file_name.trim_end_matches(&format!(".{}", file_extension)), file_extension);
        log::info!("🔄 生成截图文件名: {}", unique_name);
        
        // 按分类和年月存储: 结绳社区/分类名称/年月/资源id-资源名称.扩展名
        let now = chrono::Utc::now();
        let year_month = now.format("%Y-%m").to_string();
        let storage_path = format!("{}/{}/{}", self.storage_base_path, category_name, year_month);
        log::info!("📁 目标存储路径: {}", storage_path);
        
        // 确保目录存在
        let category_path = format!("{}/{}", self.storage_base_path, category_name);
        if let Err(e) = self.alist_service.create_folder_if_missing(&category_path).await {
            log::warn!("⚠️ 创建分类目录失败: {}, 尝试使用默认分类", e);
            // 尝试使用默认分类
            let default_category_path = format!("{}/默认分类", self.storage_base_path);
            self.alist_service.create_folder_if_missing(&default_category_path).await
                .map_err(|e| anyhow!("创建默认分类目录失败: {}", e))?;
            
            // 更新存储路径
            let default_storage_path = format!("{}/默认分类/{}", self.storage_base_path, year_month);
            self.alist_service.create_folder_if_missing(&default_storage_path).await
                .map_err(|e| anyhow!("创建默认月份目录失败: {}", e))?;
            
            // 使用默认路径
            let storage_path = default_storage_path;
        } else {
            // 确保月份目录存在
            if let Err(e) = self.alist_service.create_folder_if_missing(&storage_path).await {
                log::warn!("⚠️ 创建月份目录失败: {}, 尝试使用分类根目录", e);
                // 使用分类根目录
                let storage_path = category_path;
            }
        }
        
        // 上传文件
        log::info!("⬆️  正在上传截图到AList...");
        let file_path = self.alist_service.upload_file(
            &storage_path, 
            &unique_name, 
            file_data.clone()
        ).await.map_err(|e| anyhow!("上传截图文件失败: {}", e))?;
        
        log::info!("✅ 截图上传成功: {}", file_path);
        
        // 验证文件是否成功上传
        log::info!("🔍 验证截图文件是否成功上传...");
        if !self.alist_service.verify_file_exists(&file_path).await
            .map_err(|e| anyhow!("验证截图文件存在性失败: {}", e))? {
            return Err(anyhow!("截图文件上传后无法访问，请检查存储服务配置"));
        }
        
        // 获取文件信息和大小
        log::info!("🔍 获取截图文件信息...");
        let file_info = self.alist_service.get_file_info(&file_path).await
            .map_err(|e| anyhow!("获取截图文件信息失败: {}", e))?;
        
        // 构建AList的标准下载URL: {base_url}/d{file_path}
        let download_url = format!("{}/d{}", 
            self.alist_service.base_url(),
            file_path
        );
        
        log::info!("🔗 截图AList下载地址: {}", download_url);
        
        Ok(UploadResult {
            file_path,
            download_url,
            file_size: file_info.size,
        })
    }

    /// 上传包文件
    pub async fn upload_package_file(
        &mut self,
        file_name: &str,
        file_data: Bytes,
        package_id: Option<i32>
    ) -> Result<UploadResult> {
        log::info!("📤 开始上传文件: {}", file_name);
        
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 获取包的分类名称
        let category_name = if let Some(pkg_id) = package_id {
            match self.get_package_category_name(pkg_id).await {
                Ok(name) => {
                    log::info!("📂 资源分类: {}", name);
                    name
                },
                Err(e) => {
                    log::warn!("⚠️  获取资源分类失败: {}，使用默认分类", e);
                    "默认分类".to_string()
                }
            }
        } else {
            log::info!("📂 未指定包ID，使用默认分类");
            "默认分类".to_string()
        };
        
        // 生成文件名: 资源id_文件名.扩展名
        let file_extension = std::path::Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let unique_name = if let Some(pkg_id) = package_id {
            if !file_extension.is_empty() {
                format!("{}_{}.{}", 
                    pkg_id,
                    file_name.trim_end_matches(&format!(".{}", file_extension)),
                    file_extension
                )
            } else {
                format!("{}_{}", pkg_id, file_name)
            }
        } else {
            // 如果没有包ID，使用UUID作为前缀
            if !file_extension.is_empty() {
                format!("{}_{}.{}", 
                    Uuid::new_v4().to_string().replace("-", "")[..8].to_string(),
                    file_name.trim_end_matches(&format!(".{}", file_extension)),
                    file_extension
                )
            } else {
                format!("{}_{}", 
                    Uuid::new_v4().to_string().replace("-", "")[..8].to_string(),
                    file_name
                )
            }
        };
        
        log::info!("🔄 生成唯一文件名: {}", unique_name);
        
        // 按分类和年月存储: /image/分类名称/年月/文件
        let now = chrono::Utc::now();
        let year_month = now.format("%Y-%m").to_string();
        let storage_path = format!("{}/{}/{}", self.storage_base_path, category_name, year_month);
        
        log::info!("📁 目标存储路径: {}", storage_path);
        
        // 确保分类目录存在
        let category_path = format!("{}/{}", self.storage_base_path, category_name);
        if let Err(e) = self.alist_service.create_folder_if_missing(&category_path).await {
            log::warn!("⚠️ 创建分类目录失败: {}, 尝试使用默认分类", e);
            // 尝试使用默认分类
            let default_category_path = format!("{}/默认分类", self.storage_base_path);
            self.alist_service.create_folder_if_missing(&default_category_path).await
                .map_err(|e| anyhow!("创建默认分类目录失败: {}", e))?;
            
            // 更新存储路径
            let default_storage_path = format!("{}/默认分类/{}", self.storage_base_path, year_month);
            self.alist_service.create_folder_if_missing(&default_storage_path).await
                .map_err(|e| anyhow!("创建默认月份目录失败: {}", e))?;
            
            // 使用默认路径
            let storage_path = default_storage_path;
        }
        
        // 确保月份目录存在
        if let Err(e) = self.alist_service.create_folder_if_missing(&storage_path).await {
            log::warn!("⚠️ 创建月份目录失败: {}, 尝试使用分类根目录", e);
            // 使用分类根目录
            let storage_path = category_path;
        }
        
        // 上传文件
        log::info!("⬆️  正在上传文件到AList...");
        let file_path = self.alist_service.upload_file(
            &storage_path, 
            &unique_name, 
            file_data.clone()
        ).await.map_err(|e| anyhow!("上传文件失败: {}", e))?;
        
        log::info!("✅ 文件上传成功: {}", file_path);
        
        // 验证文件是否成功上传
        log::info!("🔍 验证文件是否成功上传...");
        if !self.alist_service.verify_file_exists(&file_path).await
            .map_err(|e| anyhow!("验证文件存在性失败: {}", e))? {
            return Err(anyhow!("文件上传后无法访问，请检查存储服务配置"));
        }
        
        // 获取文件信息和大小
        log::info!("🔍 获取文件信息...");
        let file_info = self.alist_service.get_file_info(&file_path).await
            .map_err(|e| anyhow!("获取文件信息失败: {}", e))?;
        
        // 构建AList的标准下载URL: {base_url}/d{file_path}
        let download_url = format!("{}/d{}", 
            self.alist_service.base_url(),
            file_path
        );
        
        log::info!("🔗 文件AList下载地址: {}", download_url);
        
        Ok(UploadResult {
            file_path,
            download_url,
            file_size: file_info.size,
        })
    }
    
    /// 获取包的分类名称和资源名称
    async fn get_package_info(&self, package_id: i32) -> Result<(String, String)> {
        // 从数据库获取包信息
        let package = self.package_repo.find_by_id(package_id).await
            .map_err(|e| anyhow!("获取资源信息失败: {}", e))?
            .ok_or_else(|| anyhow!("资源不存在: ID={}", package_id))?;
        
        // 获取分类名称
        let category_name = match package.category_id {
            Some(cat_id) => {
                match self.get_category_name(cat_id).await {
                    Ok(name) => name,
                    Err(_) => "默认分类".to_string()
                }
            },
            None => "默认分类".to_string()
        };
        
        Ok((category_name, package.name))
    }
    
    /// 获取包的分类名称
    async fn get_package_category_name(&self, package_id: i32) -> Result<String> {
        // 从数据库获取包信息
        use rusqlite::Connection;
        
        let conn = Connection::open(&self.db_path)
            .map_err(|e| anyhow!("打开数据库连接失败: {}", e))?;
        
        let category_name = conn.query_row(
            "SELECT c.name FROM packages p JOIN categories c ON p.category_id = c.id WHERE p.id = ?", 
            [package_id], 
            |row| row.get::<_, String>(0)
        ).unwrap_or_else(|_| "默认分类".to_string());
        
        Ok(category_name)
    }
    
    /// 获取分类名称
    async fn get_category_name(&self, category_id: i32) -> Result<String> {
        // 从数据库获取分类信息
        use rusqlite::Connection;
        
        let conn = Connection::open(&self.db_path)
            .map_err(|e| anyhow!("打开数据库连接失败: {}", e))?;
        let name = conn.query_row(
            "SELECT name FROM categories WHERE id = ?", 
            [category_id], 
            |row| row.get::<_, String>(0)
        ).map_err(|e| anyhow!("获取分类名称失败: {}", e))?;
        
        Ok(name)
    }
    
    /// 获取文件下载链接
    pub async fn get_package_download_url(&mut self, file_path: &str) -> Result<String> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 使用AList服务获取下载链接
        self.alist_service.get_download_link(file_path).await
            .map_err(|e| anyhow!("获取文件下载链接失败: {}", e))
    }
    
    /// 删除文件
    pub async fn delete_package_file(&mut self, file_path: &str) -> Result<()> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 使用AList服务删除文件
        self.alist_service.delete_file(file_path).await
            .map_err(|e| anyhow!("删除文件失败: {}", e))
    }
    
    /// 验证文件是否存在
    pub async fn verify_file_exists(&mut self, file_path: &str) -> Result<bool> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 使用AList服务验证文件是否存在
        self.alist_service.verify_file_exists(file_path).await
            .map_err(|e| anyhow!("验证文件存在性失败: {}", e))
    }
    
    /// 获取文件信息
    pub async fn get_file_info(&mut self, file_path: &str) -> Result<i64> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 使用AList服务获取文件信息
        let file_info = self.alist_service.get_file_info(file_path).await
            .map_err(|e| anyhow!("获取文件信息失败: {}", e))?;
        
        Ok(file_info.size)
    }
    
    /// 列出存储文件路径
    pub async fn list_storage_file_paths(&mut self) -> Result<Vec<String>> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 递归列出所有文件路径
        let path = self.storage_base_path.clone();
        self.list_files_recursive(&path).await
    }
    
    /// 递归列出目录下所有文件路径
    async fn list_files_recursive(&mut self, path: &str) -> Result<Vec<String>> {
        let mut result = Vec::new();
        
        // 获取当前目录下的文件和子目录
        let list_result = self.alist_service.list_files(path).await
            .map_err(|e| anyhow!("列出文件失败: {}", e))?;
        
        if let Some(files) = list_result.content {
            for file in files {
                let file_path = format!("{}/{}", path, file.name);
                
                if file.is_dir {
                    // 递归处理子目录
                    // 使用Box::pin来处理异步递归
                    let sub_files = Box::pin(self.list_files_recursive(&file_path)).await?;
                    result.extend(sub_files);
                } else {
                    // 添加文件路径
                    result.push(file_path);
                }
            }
        }
        
        Ok(result)
    }
    
    /// 列出存储文件
    pub async fn list_storage_files(&mut self, path_opt: Option<&str>) -> Result<Vec<FileInfo>> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 确定要列出的路径
        let path = path_opt.unwrap_or(&self.storage_base_path);
        
        // 获取文件列表
        let list_result = self.alist_service.list_files(path).await
            .map_err(|e| anyhow!("列出文件失败: {}", e))?;
        
        if let Some(files) = list_result.content {
            Ok(files)
        } else {
            Ok(Vec::new())
        }
    }
    
    /// 获取存储统计信息
    pub async fn get_storage_stats(&mut self) -> Result<StorageStats> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 获取所有文件路径
        let all_files = self.list_storage_file_paths().await?;
        
        let mut total_files = 0;
        let mut total_size: i64 = 0;
        let mut file_count_by_type = HashMap::new();
        let mut size_by_type = HashMap::new();
        let mut file_count_by_category = HashMap::new();
        let mut size_by_category = HashMap::new();
        let mut orphaned_files = 0;
        let mut orphaned_size: i64 = 0;
        
        // 获取数据库中记录的所有文件URL
        use rusqlite::Connection;
        let conn = Connection::open(&self.db_path)
            .map_err(|e| anyhow!("打开数据库连接失败: {}", e))?;
        
        // 收集包文件URL
        let mut db_files = Vec::new();
        let mut stmt = conn.prepare("SELECT file_url FROM packages WHERE file_url IS NOT NULL")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        for row in rows {
            if let Ok(url) = row {
                // 从URL提取文件路径
                if url.starts_with(&format!("{}/d", self.alist_service.base_url())) {
                    let path = url.replace(&format!("{}/d", self.alist_service.base_url()), "");
                    db_files.push(path);
                }
            }
        }
        
        // 收集截图URL
        let mut stmt = conn.prepare("SELECT screenshots FROM packages WHERE screenshots IS NOT NULL")?;
        let rows = stmt.query_map([], |row| {
            let screenshots_json: String = row.get(0)?;
            Ok(screenshots_json)
        })?;
        
        for row in rows {
            if let Ok(screenshots_json) = row {
                if let Ok(screenshots) = serde_json::from_str::<Vec<String>>(&screenshots_json) {
                    for url in screenshots {
                        if url.starts_with(&format!("{}/d", self.alist_service.base_url())) {
                            let path = url.replace(&format!("{}/d", self.alist_service.base_url()), "");
                            db_files.push(path);
                        }
                    }
                }
            }
        }
        
        // 收集帖子图片URL
        let mut stmt = conn.prepare("SELECT images FROM posts WHERE images IS NOT NULL")?;
        let rows = stmt.query_map([], |row| {
            let images_json: String = row.get(0)?;
            Ok(images_json)
        })?;
        
        for row in rows {
            if let Ok(images_json) = row {
                if let Ok(images) = serde_json::from_str::<Vec<String>>(&images_json) {
                    for url in images {
                        if url.starts_with(&format!("{}/d", self.alist_service.base_url())) {
                            let path = url.replace(&format!("{}/d", self.alist_service.base_url()), "");
                            db_files.push(path);
                        }
                    }
                }
            }
        }
        
        // 处理每个文件
        for file_path in &all_files {
            total_files += 1;
            
            // 获取文件信息
            if let Ok(file_info) = self.alist_service.get_file_info(file_path).await {
                total_size += file_info.size;
                
                // 按文件类型统计
                let ext = std::path::Path::new(file_path)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_lowercase();
                
                *file_count_by_type.entry(ext.clone()).or_insert(0) += 1;
                *size_by_type.entry(ext).or_insert(0) += file_info.size;
                
                // 按分类统计
                let category = if file_path.contains("/") {
                    let parts: Vec<&str> = file_path.split('/').collect();
                    if parts.len() > 2 {
                        parts[2].to_string()
                    } else {
                        "未知".to_string()
                    }
                } else {
                    "未知".to_string()
                };
                
                *file_count_by_category.entry(category.clone()).or_insert(0) += 1;
                *size_by_category.entry(category).or_insert(0) += file_info.size;
                
                // 检查是否为孤立文件
                if !db_files.iter().any(|db_path| file_path.ends_with(db_path)) {
                    orphaned_files += 1;
                    orphaned_size += file_info.size;
                }
            }
        }
        
        Ok(StorageStats {
            total_files,
            total_size,
            file_count_by_type,
            size_by_type,
            file_count_by_category,
            size_by_category,
            orphaned_files,
            orphaned_size,
        })
    }
    
    /// 清理孤立文件
    pub async fn cleanup_orphaned_files(&mut self) -> Result<CleanupResult> {
        // 确保存储已初始化
        self.ensure_storage_ready().await?;
        
        // 获取所有文件路径
        let all_files = self.list_storage_file_paths().await?;
        
        // 获取数据库中记录的所有文件URL
        use rusqlite::Connection;
        let conn = Connection::open(&self.db_path)
            .map_err(|e| anyhow!("打开数据库连接失败: {}", e))?;
        
        // 收集包文件URL
        let mut db_files = Vec::new();
        let mut stmt = conn.prepare("SELECT file_url FROM packages WHERE file_url IS NOT NULL")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        for row in rows {
            if let Ok(url) = row {
                // 从URL提取文件路径
                if url.starts_with(&format!("{}/d", self.alist_service.base_url())) {
                    let path = url.replace(&format!("{}/d", self.alist_service.base_url()), "");
                    db_files.push(path);
                }
            }
        }
        
        // 收集截图URL
        let mut stmt = conn.prepare("SELECT screenshots FROM packages WHERE screenshots IS NOT NULL")?;
        let rows = stmt.query_map([], |row| {
            let screenshots_json: String = row.get(0)?;
            Ok(screenshots_json)
        })?;
        
        for row in rows {
            if let Ok(screenshots_json) = row {
                if let Ok(screenshots) = serde_json::from_str::<Vec<String>>(&screenshots_json) {
                    for url in screenshots {
                        if url.starts_with(&format!("{}/d", self.alist_service.base_url())) {
                            let path = url.replace(&format!("{}/d", self.alist_service.base_url()), "");
                            db_files.push(path);
                        }
                    }
                }
            }
        }
        
        // 收集帖子图片URL
        let mut stmt = conn.prepare("SELECT images FROM posts WHERE images IS NOT NULL")?;
        let rows = stmt.query_map([], |row| {
            let images_json: String = row.get(0)?;
            Ok(images_json)
        })?;
        
        for row in rows {
            if let Ok(images_json) = row {
                if let Ok(images) = serde_json::from_str::<Vec<String>>(&images_json) {
                    for url in images {
                        if url.starts_with(&format!("{}/d", self.alist_service.base_url())) {
                            let path = url.replace(&format!("{}/d", self.alist_service.base_url()), "");
                            db_files.push(path);
                        }
                    }
                }
            }
        }
        
        // 处理孤立文件
        let mut deleted_files = 0;
        let mut freed_space: i64 = 0;
        let mut failed_files = 0;
        let mut details = Vec::new();
        
        for file_path in all_files {
            // 检查是否为孤立文件
            if !db_files.iter().any(|db_path| file_path.ends_with(db_path)) {
                // 获取文件大小
                let file_size = match self.alist_service.get_file_info(&file_path).await {
                    Ok(info) => info.size,
                    Err(_) => 0,
                };
                
                // 删除文件
                match self.alist_service.delete_file(&file_path).await {
                    Ok(_) => {
                        deleted_files += 1;
                        freed_space += file_size;
                        details.push(format!("已删除: {} ({}字节)", file_path, file_size));
                    },
                    Err(e) => {
                        failed_files += 1;
                        details.push(format!("删除失败: {} - {}", file_path, e));
                    }
                }
            }
        }
        
        Ok(CleanupResult {
            deleted_files,
            freed_space,
            failed_files,
            details,
        })
    }
}
