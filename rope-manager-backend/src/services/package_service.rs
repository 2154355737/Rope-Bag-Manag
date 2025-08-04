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
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct PackageService {
    package_repo: PackageRepository,
    system_repo: Option<SystemRepository>,
    file_utils: FileUtils,
    subscription_repo: Option<SubscriptionRepository>,
    email_service: Option<Arc<RwLock<EmailService>>>,
    user_repo: Option<crate::repositories::UserRepository>,
    download_security_service: Option<DownloadSecurityService>,
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
            file_url: req.file_url.clone().unwrap_or_else(String::new), // 使用请求中的file_url
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
            file_url: req.file_url.clone().unwrap_or(package.file_url), // 使用请求中的file_url，如果没有则保持原值
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
        
        // 获取文件URL
        let file_url = self.package_repo.get_package_file_url(package_id).await?;
        
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

        Ok(file_url)
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
        
        // 获取文件URL
        let file_url = self.package_repo.get_package_file_url(package_id).await?;
        
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

        Ok(file_url)
    }

    // 新增方法：更新包文件
    pub async fn update_package_file(&self, package_id: i32) -> Result<Package> {
        let package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        // TODO: 实现文件上传逻辑
        // 这里应该处理文件上传并更新包信息

        Ok(package)
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
} 