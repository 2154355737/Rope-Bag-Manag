use crate::services::alist_service::{AListService, FileInfo};
use crate::models::Package;
use crate::repositories::package_repo::PackageRepository;
use anyhow::{Result, anyhow};
use actix_web::web::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct PackageStorageService {
    package_repo: PackageRepository,
    alist_service: AListService,
    storage_base_path: String,
    db_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResult {
    pub file_path: String,
    pub download_url: String,
    pub file_size: i64,
}

impl PackageStorageService {
    pub fn new(db_path: &str) -> Result<Self> {
        log::info!("🚀 初始化存储服务...");
        
        let package_repo = PackageRepository::new(db_path)?;
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
            (Some(bu), Some(un), Some(pw)) => AListService::new_with_params(bu, un, pw),
            _ => AListService::new(),
        };
        log::info!("✅ AList服务已初始化");
        
        let storage_base_path = "/image/结绳社区".to_string();
        log::info!("📁 存储基础路径: {}", storage_base_path);
        
        Ok(Self {
            package_repo,
            alist_service,
            storage_base_path,
            db_path: db_path.to_string(),
        })
    }
    
    /// 初始化存储（创建必要的文件夹）
    pub async fn initialize_storage(&mut self) -> Result<()> {
        log::info!("🔧 开始初始化存储目录结构...");
        
        // 创建基础目录（存在则跳过）
        log::info!("📁 确认基础目录: {}", self.storage_base_path);
        match self.alist_service.create_folder_if_missing(&self.storage_base_path).await {
            Ok(created) => {
                if created { log::info!("✅ 基础目录创建成功"); } else { log::info!("✅ 基础目录已存在"); }
            }
            Err(e) => log::warn!("⚠️ 基础目录检查/创建失败: {}", e),
        }
        
        // 获取所有分类并创建对应目录
        log::info!("🗂️  正在获取分类列表...");
        match self.get_all_categories().await {
            Ok(categories) => {
                if categories.is_empty() {
                    log::warn!("⚠️  未找到任何分类，将创建默认分类目录");
                    self.create_category_directories(&["默认分类"]).await?;
                } else {
                    log::info!("📋 找到 {} 个分类", categories.len());
                    let category_names: Vec<&str> = categories.iter().map(|c| c.as_str()).collect();
                    self.create_category_directories(&category_names).await?;
                }
            },
            Err(e) => {
                log::error!("❌ 获取分类失败: {}", e);
                log::info!("🔄 使用默认分类目录");
                self.create_category_directories(&["默认分类"]).await?;
            }
        }
        
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
                Err(e) => log::warn!("⚠️ 分类目录检查/创建失败: {} ({})", category, e),
            }
            
            log::info!("📅 创建月份目录: {}", monthly_path);
            match self.alist_service.create_folder_if_missing(&monthly_path).await {
                Ok(created) => {
                    if created { log::info!("✅ 月份目录创建成功: {}/{}", category, year_month); } else { log::info!("📁 月份目录已存在: {}/{}", category, year_month); }
                }
                Err(e) => log::warn!("⚠️ 月份目录检查/创建失败: {}/{} ({})", category, year_month, e),
            }
        }
        
        Ok(())
    }
    
    /// 获取所有分类名称
    async fn get_all_categories(&self) -> Result<Vec<String>> {
        // 从数据库获取分类信息
        use rusqlite::Connection;
        
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT name FROM categories WHERE 1=1")?;
        let rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;
        
        let mut categories = Vec::new();
        for row in rows {
            categories.push(row?);
        }
        
        Ok(categories)
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
        self.initialize_storage().await?;
        
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
        
        // 生成唯一文件名防止冲突
        let file_extension = std::path::Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let unique_name = if !file_extension.is_empty() {
            format!("{}_{}.{}", 
                Uuid::new_v4().to_string().replace("-", "")[..12].to_string(),
                file_name.trim_end_matches(&format!(".{}", file_extension)),
                file_extension
            )
        } else {
            format!("{}_{}", 
                Uuid::new_v4().to_string().replace("-", "")[..12].to_string(),
                file_name
            )
        };
        
        log::info!("🔄 生成唯一文件名: {}", unique_name);
        
        // 按分类和年月存储: /image/分类名称/年月/文件
        let now = chrono::Utc::now();
        let year_month = now.format("%Y-%m").to_string();
        let storage_path = format!("{}/{}/{}", self.storage_base_path, category_name, year_month);
        
        log::info!("📁 目标存储路径: {}", storage_path);
        
        // 确保分类目录存在
        let category_path = format!("{}/{}", self.storage_base_path, category_name);
        self.alist_service.create_folder(&category_path).await.ok();
        self.alist_service.create_folder(&storage_path).await.ok();
        
        // 上传文件
        log::info!("⬆️  正在上传文件到AList...");
        let file_path = self.alist_service.upload_file(
            &storage_path, 
            &unique_name, 
            file_data.clone()
        ).await?;
        
        log::info!("✅ 文件上传成功: {}", file_path);
        
        // 不在上传时获取下载链接，而是在实际下载时获取
        // 避免权限问题："You are not an admin"
        let result = UploadResult {
            file_path: file_path.clone(),
            download_url: format!("alist:{}", file_path), // 标记为AList文件路径
            file_size: file_data.len() as i64,
        };
        
        log::info!("🎉 包文件上传完成: {} -> {} ({}字节)", file_name, file_path, file_data.len());
        
        Ok(result)
    }
    
    /// 根据包ID获取分类名称
    async fn get_package_category_name(&self, package_id: i32) -> Result<String> {
        use rusqlite::Connection;
        
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare(
            "SELECT c.name FROM packages p 
             LEFT JOIN categories c ON p.category_id = c.id 
             WHERE p.id = ?"
        )?;
        
        let category_name: String = stmt.query_row([package_id], |row| {
            let name: Option<String> = row.get(0)?;
            Ok(name.unwrap_or_else(|| "默认分类".to_string()))
        })?;
        
        Ok(category_name)
    }
    
    /// 获取包的下载链接
    pub async fn get_package_download_url(&mut self, file_path: &str) -> Result<String> {
        // 构造AList的直接访问URL，避免权限问题
        // AList的直接访问格式通常是: http://domain/d/file_path
        // 优先调用 AList API 获取 raw_url，失败则落回 /d 直链
        match self.alist_service.get_download_link(file_path).await {
            Ok(url) => {
                log::info!("🔗 生成AList下载链接: {}", url);
                Ok(url)
            }
            Err(e) => {
                log::warn!("获取 raw_url 失败，回退直链: {}", e);
                let direct_url = format!("{}/d{}", self.alist_service.base_url(), file_path);
                log::info!("🔗 生成AList直接访问链接: {}", direct_url);
                Ok(direct_url)
            }
        }
    }
    
    /// 删除包文件
    pub async fn delete_package_file(&mut self, file_path: &str) -> Result<()> {
        self.alist_service.delete_file(file_path).await
    }
    
    /// 重命名包文件
    pub async fn rename_package_file(&mut self, old_path: &str, new_name: &str) -> Result<String> {
        self.alist_service.rename_file(old_path, new_name).await?;
        
        // 返回新路径
        let path_parts: Vec<&str> = old_path.rsplitn(2, '/').collect();
        if path_parts.len() == 2 {
            Ok(format!("{}/{}", path_parts[1], new_name))
        } else {
            Ok(new_name.to_string())
        }
    }
    
    /// 列出存储中的文件
    pub async fn list_storage_files(&mut self, path: Option<&str>) -> Result<Vec<FileInfo>> {
        let list_path = path.unwrap_or(&self.storage_base_path);
        let file_list = self.alist_service.list_files(list_path).await?;
        Ok(file_list.content.unwrap_or_default())
    }
    
    /// 递归列出存储中的所有文件完整路径（分类/年月两级目录）
    pub async fn list_storage_file_paths(&mut self) -> Result<Vec<String>> {
        let mut paths: Vec<String> = Vec::new();
        let base = self.storage_base_path.clone();
        let top = self.alist_service.list_files(&base).await?;
        for item in top.content.unwrap_or_default() {
            if item.is_dir {
                let cat_path = format!("{}/{}", base, item.name);
                let months = self.alist_service.list_files(&cat_path).await?;
                for m in months.content.unwrap_or_default() {
                    if m.is_dir {
                        let month_path = format!("{}/{}", cat_path, m.name);
                        let files = self.alist_service.list_files(&month_path).await?;
                        for f in files.content.unwrap_or_default() {
                            if !f.is_dir {
                                paths.push(format!("{}/{}", month_path, f.name));
                            }
                        }
                    } else {
                        // 某些情况下文件直接在分类目录下
                        if !m.is_dir {
                            paths.push(format!("{}/{}", cat_path, m.name));
                        }
                    }
                }
            } else {
                // 顶层直接文件
                paths.push(format!("{}/{}", base, item.name));
            }
        }
        Ok(paths)
    }
    
    /// 获取存储统计信息
    pub async fn get_storage_stats(&mut self) -> Result<StorageStats> {
        let files = self.list_storage_files(None).await?;
        
        let mut total_files = 0;
        let mut total_size = 0;
        
        for file in files {
            if !file.is_dir {
                total_files += 1;
                total_size += file.size;
            }
        }
        
        Ok(StorageStats {
            total_files,
            total_size,
            storage_path: self.storage_base_path.clone(),
        })
    }
    
    /// 清理过期或无用的文件（可以定期调用）
    pub async fn cleanup_orphaned_files(&mut self) -> Result<CleanupResult> {
        let mut deleted_count = 0;
        let mut freed_space = 0;
        
        // 获取存储中的所有文件
        let storage_files = self.list_storage_files(None).await?;
        
        // 获取数据库中的所有包文件路径
        let packages = self.package_repo.get_all_packages().await?;
        let db_file_paths: std::collections::HashSet<String> = packages
            .iter()
            .map(|p| p.file_url.clone())
            .collect();
        
        // 查找孤立文件（存储中有但数据库中没有的）
        for file in storage_files {
            if !file.is_dir {
                let file_path = format!("{}/{}", self.storage_base_path, file.name);
                if !db_file_paths.contains(&file_path) {
                    match self.alist_service.delete_file(&file_path).await {
                        Ok(_) => {
                            deleted_count += 1;
                            freed_space += file.size;
                            log::info!("🗑️ 清理孤立文件: {}", file_path);
                        },
                        Err(e) => {
                            log::warn!("⚠️ 删除孤立文件失败 {}: {}", file_path, e);
                        }
                    }
                }
            }
        }
        
        Ok(CleanupResult {
            deleted_files: deleted_count,
            freed_space,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_files: i32,
    pub total_size: i64,
    pub storage_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupResult {
    pub deleted_files: i32,
    pub freed_space: i64,
} 