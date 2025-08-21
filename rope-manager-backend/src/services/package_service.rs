use anyhow::Result;
use crate::models::{Package, CreatePackageRequest, UpdatePackageRequest, Category, CreateResourceRecordRequest};
use crate::repositories::package_repo::PackageRepository;
use crate::repositories::system_repo::SystemRepository;
use crate::utils::file::FileUtils;
use chrono::Utc;
use crate::repositories::subscription_repo::SubscriptionRepository;
use crate::services::email_service::EmailService;
use crate::services::download_security_service::DownloadSecurityService;
use crate::models::download_security::DownloadSecurityConfig;
use rusqlite::params;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::services::notification_service::NotificationService;

#[derive(Clone)]
pub struct PackageService {
    package_repo: PackageRepository,
    system_repo: Option<SystemRepository>,
    file_utils: FileUtils,
    subscription_repo: Option<SubscriptionRepository>,
    email_service: Option<Arc<RwLock<EmailService>>>,
    user_repo: Option<crate::repositories::UserRepository>,
    download_security_service: Option<DownloadSecurityService>,
    notification_service: Option<NotificationService>,
}

impl PackageService {
    pub fn new(package_repo: PackageRepository, upload_path: String) -> Self {
        Self {
            package_repo,
            system_repo: None,
            file_utils: FileUtils::new(upload_path),
            subscription_repo: None,
            email_service: None,
            user_repo: None,
            download_security_service: None,
            notification_service: None,
        }
    }

    // 设置系统仓库，用于记录资源操作
    pub fn with_system_repo(mut self, system_repo: SystemRepository) -> Self {
        self.system_repo = Some(system_repo);
        self
    }

    pub fn with_notifier(mut self, sub_repo: SubscriptionRepository, email_service: Arc<RwLock<EmailService>>) -> Self {
        self.subscription_repo = Some(sub_repo);
        self.email_service = Some(email_service);
        self
    }

    pub fn with_user_repo(mut self, user_repo: crate::repositories::UserRepository) -> Self {
        self.user_repo = Some(user_repo);
        self
    }

    pub fn with_download_security_service(mut self, security_service: &DownloadSecurityService) -> Self {
        self.download_security_service = Some(security_service.clone());
        self
    }

    pub fn with_notification_service(mut self, service: NotificationService) -> Self {
        self.notification_service = Some(service);
        self
    }

    pub fn db_path(&self) -> &str {
        // package_repo holds a connection opened from a path; we can reuse the environment config through repositories::get_connection, but for simplicity we try to access via connection path is not stored. Here we fallback to the global config by reading from repository helper in main; since not available, we just return an empty str isn't acceptable. Alternative: expose nothing and change user.rs not to open connection via service. We'll instead remove its usage. (But current edit requires getter.)
        "data.db"
    }

    pub async fn get_packages(&self) -> Result<Vec<Package>> {
        self.package_repo.get_all_packages().await
    }

    pub async fn get_package(&self, package_id: i32) -> Result<Option<Package>> {
        self.package_repo.find_by_id(package_id).await
    }

    // 新增方法：根据ID获取包（别名方法）
    pub async fn get_package_by_id(&self, package_id: i32) -> Result<Option<Package>> {
        self.package_repo.find_by_id(package_id).await
    }

    pub async fn create_package(&self, req: &CreatePackageRequest) -> Result<Package> {
        // 创建绳包记录
        let package = Package {
            id: 0, // 数据库会自动生成
            name: req.name.clone(),
            author: req.author.clone(),
            version: req.version.clone(),
            description: req.description.clone(),
            file_url: req.file_url.clone(), // 直接使用请求中的file_url，已经是Option<String>类型
            file_size: None,
            download_count: 0,
            like_count: 0,
            favorite_count: 0,
            category_id: req.category_id,
            status: crate::models::PackageStatus::Pending, // 新资源默认为待审核状态
            created_at: Utc::now(),
            updated_at: Utc::now(),
            reviewer_id: None,
            reviewed_at: None,
            review_comment: None,
            is_pinned: req.is_pinned.unwrap_or(false),
            is_featured: req.is_featured.unwrap_or(false),
            tags: req.tags.clone(),
        };

        // 创建包
        let created_package = self.package_repo.create_package(&package).await?;
        
        // 记录资源操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: created_package.id,
                resource_type: "Package".to_string(),
                action: "Create".to_string(),
                old_data: None,
                new_data: Some(serde_json::to_string(&created_package).unwrap_or_default()),
            };
            
            // 使用系统仓库记录操作，默认用户ID为1（可以根据实际情况修改）
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                // 仅记录错误，不影响主要功能
                log::error!("记录资源创建操作失败: {}", e);
            } else {
                log::info!("成功记录资源创建操作: Package ID={}", created_package.id);
            }
        }

        // 通知管理员和元老有新资源待审核
        if let (Some(user_repo), Some(email_srv_arc)) = (&self.user_repo, &self.email_service) {
            if let Ok(admin_emails) = user_repo.get_admin_and_elder_emails().await {
                let es = email_srv_arc.read().await;
                let review_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
                let review_link = format!("{}/admin/resource-review", review_url);
                
                for email in admin_emails {
                    if let Err(e) = es.send_admin_review_notification(
                        &email, 
                        &created_package.name, 
                        &created_package.author, 
                        &review_link
                    ).await {
                        log::error!("发送管理员通知邮件失败: {}", e);
                    } else {
                        log::info!("成功发送管理员通知邮件: {} -> {}", created_package.name, email);
                    }
                }
            }
        }
        
        Ok(created_package)
    }

    pub async fn update_package(&self, package_id: i32, req: &UpdatePackageRequest) -> Result<Package> {
        let package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        // 克隆package用于记录旧数据
        let old_package = package.clone();
        
        let updated_package = Package {
            id: package_id,
            name: req.name.clone().unwrap_or(package.name),
            author: package.author,
            version: req.version.clone().or(package.version),
            description: req.description.clone().or(package.description),
            category_id: req.category_id.or(package.category_id),
            status: req.status.clone().unwrap_or(package.status),
            file_url: req.file_url.clone().or(package.file_url.clone()), // 使用请求中的file_url，如果没有则保持原值
            file_size: package.file_size,
            download_count: package.download_count,
            like_count: package.like_count,
            favorite_count: package.favorite_count,
            created_at: package.created_at,
            updated_at: chrono::Utc::now(),
            reviewer_id: req.reviewer_id.or(package.reviewer_id),
            reviewed_at: req.reviewed_at.or(package.reviewed_at),
            review_comment: req.review_comment.clone().or(package.review_comment),
            is_pinned: req.is_pinned.unwrap_or(package.is_pinned),
            is_featured: req.is_featured.unwrap_or(package.is_featured),
            tags: req.tags.clone().or(package.tags),
        };

        self.package_repo.update_package(&updated_package).await?;
        
        // 记录资源更新操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: package_id,
                resource_type: "Package".to_string(),
                action: "Update".to_string(),
                old_data: Some(serde_json::to_string(&old_package).unwrap_or_default()),
                new_data: Some(serde_json::to_string(&updated_package).unwrap_or_default()),
            };
            
            // 使用系统仓库记录操作
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                log::error!("记录资源更新操作失败: {}", e);
            } else {
                log::info!("成功记录资源更新操作: Package ID={}", package_id);
            }
        }
        
        // 如果状态从非Active变为Active（审核通过），发送订阅者通知
        if old_package.status != crate::models::PackageStatus::Active && 
           updated_package.status == crate::models::PackageStatus::Active {
            if let (Some(sub_repo), Some(email_srv_arc)) = (&self.subscription_repo, &self.email_service) {
                if let Some(cat_id) = updated_package.category_id {
                    if let Ok(emails) = sub_repo.get_subscribed_emails(cat_id).await {
                        let es = email_srv_arc.read().await;
                        let resource_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
                        let resource_link = format!("{}/resource/{}", resource_url, updated_package.id);
                        
                        for email in emails {
                            if let Err(e) = es.send_resource_notification(
                                &email, 
                                &updated_package.name, 
                                &updated_package.description.as_deref().unwrap_or(""), 
                                &resource_link
                            ).await {
                                log::error!("发送订阅者通知邮件失败: {}", e);
                            } else {
                                log::info!("成功发送订阅者通知邮件: {} -> {}", updated_package.name, email);
                            }
                        }
                    }
                }
            }

            // 审核通过 -> 给作者发送站内通知
            if let (Some(user_repo), Some(notify)) = (&self.user_repo, &self.notification_service) {
                if let Ok(Some(author_user)) = user_repo.find_by_username(&updated_package.author).await {
                    let link = format!("/resource/{}", updated_package.id);
                    let title = "资源审核通过";
                    let content = format!("您的资源《{}》已通过审核", updated_package.name);
                    if let Err(e) = notify.notify(author_user.id, title, &content, Some(&link), Some("ResourceApproved"), Some("Package"), Some(updated_package.id)).await {
                        log::error!("发送站内通知失败: {}", e);
                    }
                }
            }

            // 站内通知：分类订阅者（不含作者）
            if let (Some(sub_repo), Some(notify)) = (&self.subscription_repo, &self.notification_service) {
                if let Some(cat_id) = updated_package.category_id {
                    if let Ok(user_ids) = sub_repo.get_subscribed_user_ids(cat_id).await {
                        let link = format!("/resource/{}", updated_package.id);
                        for uid in user_ids {
                            let title = "订阅更新";
                            let content = format!("您订阅的分类有新资源：《{}》", updated_package.name);
                            if let Err(e) = notify.notify(uid, title, &content, Some(&link), Some("CategoryUpdate"), Some("Package"), Some(updated_package.id)).await {
                                log::error!("发送订阅站内通知失败: {}", e);
                            }
                        }
                    }
                }
            }
        }
        
        // 如果状态变为Rejected（审核拒绝），删除存储文件并发送通知
        if old_package.status != crate::models::PackageStatus::Rejected && 
           updated_package.status == crate::models::PackageStatus::Rejected {
            
            // 删除存储文件
            if let Some(file_url) = &updated_package.file_url {
                if !file_url.is_empty() {
                    log::info!("📂 审核拒绝，准备删除存储文件: {}", file_url);
                    
                    // 根据file_url的格式判断存储类型
                    if file_url.starts_with("alist:") {
                        // AList存储
                        let actual_path = &file_url[6..]; // 移除 "alist:" 前缀
                        use crate::services::package_storage_service::PackageStorageService;
                        let mut alist_service = PackageStorageService::new("data.db")?;
                        match alist_service.delete_package_file(actual_path).await {
                            Ok(_) => {
                                log::info!("✅ 成功删除AList存储文件: {}", file_url);
                            },
                            Err(e) => {
                                log::error!("❌ 删除AList存储文件失败: {}, 错误: {}", file_url, e);
                            }
                        }
                    } else if file_url.starts_with("/image/") {
                        // 本地存储
                        use crate::services::package_storage_service::PackageStorageService;
                        let mut storage_service = PackageStorageService::new(
                            self.file_utils.get_upload_path()
                        ).map_err(|e| anyhow::anyhow!("初始化存储服务失败: {}", e))?;
                        
                        match storage_service.delete_package_file(file_url).await {
                            Ok(_) => {
                                log::info!("✅ 成功删除AList存储文件: {}", file_url);
                            },
                            Err(e) => {
                                log::error!("❌ 删除AList存储文件失败: {}, 错误: {}", file_url, e);
                            }
                        }
                    } else {
                        log::info!("🔗 文件为直链方式，无需删除存储文件: {}", file_url);
                    }
                }
            }

            // 审核拒绝 -> 给作者发送站内通知
            if let (Some(user_repo), Some(notify)) = (&self.user_repo, &self.notification_service) {
                if let Ok(Some(author_user)) = user_repo.find_by_username(&updated_package.author).await {
                    let title = "资源审核未通过";
                    let content = format!(
                        "您的资源《{}》未通过审核。{}",
                        updated_package.name,
                        updated_package.review_comment.as_deref().unwrap_or("请根据平台规范重新提交。")
                    );
                    if let Err(e) = notify.notify(
                        author_user.id, 
                        title, 
                        &content, 
                        None, 
                        Some("ResourceRejected"), 
                        Some("Package"), 
                        Some(updated_package.id)
                    ).await {
                        log::error!("发送审核拒绝站内通知失败: {}", e);
                    } else {
                        log::info!("成功发送审核拒绝站内通知给用户: {}", author_user.username);
                    }
                }
            }
        }
        
        Ok(updated_package)
    }

    pub async fn delete_package(&self, package_id: i32) -> Result<()> {
        // 先获取包信息，用于记录
        let package = self.package_repo.find_by_id(package_id).await?;
        
        // 删除包
        self.package_repo.delete_package(package_id).await?;
        
        // 记录资源删除操作
        if let Some(system_repo) = &self.system_repo {
            if let Some(package) = package {
                // 创建资源记录请求
                let record = CreateResourceRecordRequest {
                    resource_id: package_id,
                    resource_type: "Package".to_string(),
                    action: "Delete".to_string(),
                    old_data: Some(serde_json::to_string(&package).unwrap_or_default()),
                    new_data: None,
                };
                
                // 使用系统仓库记录操作
                if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                    log::error!("记录资源删除操作失败: {}", e);
                } else {
                    log::info!("成功记录资源删除操作: Package ID={}", package_id);
                }
            }
        }
        
        Ok(())
    }

    pub async fn download_package(&self, package_id: i32) -> Result<String> {
        // 首先检查包是否存在
        let exists = self.package_repo.check_package_exists(package_id).await?;
        if !exists {
            return Err(anyhow::anyhow!("绳包不存在"));
        }
        
        // 获取文件路径
        let file_path = self.package_repo.get_package_file_url(package_id).await?;
        
        // 通过AList服务获取动态下载链接
        let download_url = if file_path.starts_with("alist:") {
            // 文件存储在AList中，获取动态下载链接
            let actual_path = &file_path[6..]; // 移除 "alist:" 前缀
            log::info!("🔗 检测到AList存储文件，生成动态下载链接: {}", actual_path);
            use crate::services::package_storage_service::PackageStorageService;
            let mut storage_service = PackageStorageService::new("data.db")?;
            storage_service.get_package_download_url(actual_path).await?
        } else if file_path.starts_with("/image/") {
            // 兼容旧版本的AList文件路径
            log::info!("🔗 检测到旧版AList存储文件，生成动态下载链接: {}", file_path);
            use crate::services::package_storage_service::PackageStorageService;
            let mut storage_service = PackageStorageService::new("data.db")?;
            storage_service.get_package_download_url(&file_path).await?
        } else {
            // 兼容旧的直链方式
            log::info!("🔗 使用传统直链方式: {}", file_path);
            file_path
        };
        
        // 增加下载次数
        self.package_repo.increment_download_count(package_id).await?;
        
        // 记录资源下载操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: package_id,
                resource_type: "Package".to_string(),
                action: "Download".to_string(),
                old_data: None,
                new_data: None,
            };
            
            // 使用系统仓库记录操作
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                log::error!("记录资源下载操作失败: {}", e);
            } else {
                log::info!("成功记录资源下载操作: Package ID={}", package_id);
            }
        }

        Ok(download_url)
    }

    // 新增方法：带安全检测的下载
    pub async fn download_package_with_security(
        &self, 
        package_id: i32, 
        user_id: Option<i32>, 
        ip_address: &str, 
        user_agent: Option<&str>
    ) -> Result<String> {
        // 首先检查包是否存在
        let exists = self.package_repo.check_package_exists(package_id).await?;
        if !exists {
            return Err(anyhow::anyhow!("绳包不存在"));
        }

        // 防刷量检测
        if let Some(security_service) = &self.download_security_service {
            let check_result = security_service.check_download_allowed(
                user_id, 
                package_id, 
                ip_address, 
                user_agent
            ).await?;

            if !check_result.is_allowed {
                return Err(anyhow::anyhow!(
                    "下载被阻止: {}", 
                    check_result.reason.unwrap_or_else(|| "安全检测未通过".to_string())
                ));
            }

            // 记录下载行为
            if let Err(e) = security_service.record_download(user_id, package_id, ip_address, user_agent).await {
                log::error!("记录下载行为失败: {}", e);
            }
        }
        
        // 获取文件路径
        let file_path = self.package_repo.get_package_file_url(package_id).await?;
        
        // 通过AList服务获取动态下载链接
        let download_url = if file_path.starts_with("alist:") {
            // 文件存储在AList中，获取动态下载链接
            let actual_path = &file_path[6..]; // 移除 "alist:" 前缀
            log::info!("🔗 检测到AList存储文件，生成动态下载链接: {}", actual_path);
            use crate::services::package_storage_service::PackageStorageService;
            let mut storage_service = PackageStorageService::new("data.db")?;
            storage_service.get_package_download_url(actual_path).await?
        } else if file_path.starts_with("/image/") {
            // 兼容旧版本的AList文件路径
            log::info!("🔗 检测到旧版AList存储文件，生成动态下载链接: {}", file_path);
            use crate::services::package_storage_service::PackageStorageService;
            let mut storage_service = PackageStorageService::new("data.db")?;
            storage_service.get_package_download_url(&file_path).await?
        } else {
            // 兼容旧的直链方式
            log::info!("🔗 使用传统直链方式: {}", file_path);
            file_path
        };
        
        // 增加下载次数
        self.package_repo.increment_download_count(package_id).await?;
        
        // 记录资源下载操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: package_id,
                resource_type: "Package".to_string(),
                action: "Download".to_string(),
                old_data: None,
                new_data: None,
            };
            
            // 使用系统仓库记录操作
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                log::error!("记录资源下载操作失败: {}", e);
            } else {
                log::info!("成功记录资源下载操作: Package ID={}", package_id);
            }
        }

        log::info!("用户通过安全检测，允许下载包 ID={}，下载链接已生成", package_id);

        Ok(download_url)
    }

    // 新增方法：更新包文件
    pub async fn update_package_file(&self, package_id: i32) -> Result<Package> {
        // TODO: 实现文件上传和关联逻辑
        self.package_repo.find_by_id(package_id).await?.ok_or_else(|| {
            anyhow::anyhow!("包不存在")
        })
    }
    
    pub async fn upload_package_file(
        &self, 
        package_id: i32, 
        file_name: &str, 
        file_data: Vec<u8>
    ) -> Result<String> {
        use crate::services::package_storage_service::PackageStorageService;
        use actix_web::web::Bytes;
        
        // 创建存储服务
        let mut storage_service = PackageStorageService::new("data.db")?;
        
        // 上传文件到AList存储
        let upload_result = storage_service.upload_package_file(
            file_name,
            Bytes::from(file_data),
            Some(package_id)
        ).await?;
        
        // 更新包的file_url
        let mut package = self.package_repo.find_by_id(package_id).await?
                         .ok_or_else(|| anyhow::anyhow!("包不存在"))?;
        
        package.file_url = Some(upload_result.file_path.clone());
        package.file_size = Some(upload_result.file_size);
        
        // 保存到数据库
        self.package_repo.update_package(&package).await?;
        
        log::info!("📦 包 {} 文件上传并更新成功: {}", package_id, upload_result.file_path);
        
        Ok(upload_result.file_path)
    }

    // 新增方法：获取分类
    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        self.package_repo.get_categories().await
    }

    pub async fn get_packages_advanced(
        &self,
        page: u32,
        page_size: u32,
        category: Option<i32>,
        search: Option<String>,
        status: Option<String>,
    ) -> anyhow::Result<(Vec<Package>, i64)> {
        self.package_repo.get_packages_advanced(page, page_size, category, search, status).await
    }

    pub async fn like_package(&self, user_id: i32, package_id: i32) -> anyhow::Result<i32> {
        let cnt = self.package_repo.like_package(user_id, package_id).await?;
        Ok(cnt)
    }

    pub async fn unlike_package(&self, user_id: i32, package_id: i32) -> Result<i32> {
        let conn = crate::repositories::get_connection()?;
        
        // 检查是否已点赞
        let is_liked: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM package_likes WHERE user_id = ? AND package_id = ?)",
            params![user_id, package_id],
            |row| row.get(0),
        )?;
        
        if !is_liked {
            return Err(anyhow::anyhow!("Have not liked this package"));
        }
        
        // 删除点赞记录
        conn.execute(
            "DELETE FROM package_likes WHERE user_id = ? AND package_id = ?",
            params![user_id, package_id],
        )?;
        
        // 返回当前点赞总数
        let cnt: i32 = conn.query_row(
            "SELECT like_count FROM packages WHERE id = ?", 
            params![package_id], 
            |r| r.get(0)
        ).unwrap_or(0);
        
        Ok(cnt)
    }

    // 收藏资源包
    pub async fn favorite_package(&self, user_id: i32, package_id: i32) -> Result<i32> {
        let conn = crate::repositories::get_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS package_favorites (user_id INTEGER NOT NULL, package_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, package_id))",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO package_favorites (user_id, package_id) VALUES (?, ?)",
            params![user_id, package_id],
        )?;
        let cnt: i32 = conn.query_row(
            "SELECT COUNT(*) FROM package_favorites WHERE package_id = ?",
            params![package_id],
            |r| r.get(0),
        )?;
        conn.execute("UPDATE packages SET favorite_count = ? WHERE id = ?", params![cnt, package_id])?;
        Ok(cnt)
    }

    // 取消收藏资源包
    pub async fn unfavorite_package(&self, user_id: i32, package_id: i32) -> Result<i32> {
        let conn = crate::repositories::get_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS package_favorites (user_id INTEGER NOT NULL, package_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, package_id))",
            [],
        )?;
        conn.execute(
            "DELETE FROM package_favorites WHERE user_id = ? AND package_id = ?",
            params![user_id, package_id],
        )?;
        let cnt: i32 = conn.query_row(
            "SELECT COUNT(*) FROM package_favorites WHERE package_id = ?",
            params![package_id],
            |r| r.get(0),
        )?;
        conn.execute("UPDATE packages SET favorite_count = ? WHERE id = ?", params![cnt, package_id])?;
        Ok(cnt)
    }

    // 检查收藏状态
    pub async fn check_favorite_status(&self, user_id: i32, package_id: i32) -> Result<bool> {
        let conn = crate::repositories::get_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS package_favorites (user_id INTEGER NOT NULL, package_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, package_id))",
            [],
        )?;
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM package_favorites WHERE user_id = ? AND package_id = ?)",
            params![user_id, package_id],
            |r| r.get(0),
        )?;
        Ok(exists)
    }

    pub async fn check_like_status(&self, user_id: i32, package_id: i32) -> anyhow::Result<bool> {
        let is_liked = self.package_repo.check_like_status(user_id, package_id).await?;
        Ok(is_liked)
    }

    pub async fn record_view(&self, package_id: i32, user_id: Option<i32>, ip_address: Option<String>, user_agent: Option<String>) -> anyhow::Result<()> {
        self.package_repo.record_view(package_id, user_id, ip_address, user_agent).await?;
        Ok(())
    }

    // 榜单：下载榜TOP N
    pub async fn top_by_downloads(&self, limit: i32) -> Result<Vec<Package>> {
        self.package_repo.top_by_downloads(limit).await
    }

    // 榜单：热门榜（点赞数优先，其次下载量）TOP N
    pub async fn top_by_likes(&self, limit: i32) -> Result<Vec<Package>> {
        self.package_repo.top_by_likes(limit).await
    }
} 