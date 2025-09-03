// 服务容器和依赖注入模块

use anyhow::Result;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::services::{
    auth_service::AuthService,
    user_service::UserService,
    package_service::PackageService,
    admin_service::AdminService,
    comment_service::CommentService,
    forbidden_word_service::ForbiddenWordService,
    community_service::CommunityService,
    user_action_service::UserActionService,
    post_service::PostService,
    tag_service::TagService,
    notification_service::NotificationService,
    download_security_service::DownloadSecurityService,
    security_action_service::SecurityActionService,
    email_service::EmailService,
    package_storage_service::PackageStorageService,
    anti_fraud_service::AntiFraudService,
};
use crate::repositories::{
    UserRepository,
    PackageRepository,
    CommentRepository,
    SystemRepository,
    SubscriptionRepository,
    EmailVerificationRepository,
    forbidden_word_repo::ForbiddenWordRepository,
    user_action_repo::UserActionRepository,
    notification_repo::NotificationRepository,
    mail_repo::MailRepository,
    follow_repo::FollowRepository,
    post_repo::PostRepository,
};
use crate::models::download_security::{DownloadSecurityConfig, SecurityConfig};
use super::BootstrapError;

/// 服务容器，管理所有服务实例
pub struct ServiceContainer {
    pub auth_service: AuthService,
    pub user_service: UserService,
    pub package_service: PackageService,
    pub admin_service: AdminService,
    pub comment_service: CommentService,
    pub forbidden_word_service: ForbiddenWordService,
    pub community_service: CommunityService,
    pub user_action_service: UserActionService,
    pub post_service: PostService,
    pub tag_service: TagService,
    pub notification_service: NotificationService,
    pub download_security_service: DownloadSecurityService,
    pub security_action_service: SecurityActionService,
    pub email_service: Arc<RwLock<EmailService>>,
    pub anti_fraud_service: AntiFraudService,
    
    // 仓库实例
    pub user_repo: UserRepository,
    pub package_repo: PackageRepository,
    pub system_repo: SystemRepository,
    pub subscription_repo: SubscriptionRepository,
    pub follow_repo: FollowRepository,
    pub post_repo: PostRepository,
    
    // JWT工具
    pub jwt_utils: crate::utils::jwt::JwtUtils,
}

impl ServiceContainer {
    /// 创建并初始化服务容器
    pub async fn new(config: &Config) -> Result<Self, BootstrapError> {
        info!("🔧 开始初始化服务容器...");
        
        let db_url = config.database_url().to_string();
        let upload_path = config.upload_path().to_string();
        let jwt_secret = config.jwt_secret().to_string();
        
        // 创建仓库实例
        let repositories = Self::create_repositories(&db_url).await?;
        
        // 创建邮件服务
        let email_service = Self::create_email_service(&db_url).await?;
        
        // 创建安全服务
        let (download_security_service, security_action_service) = 
            Self::create_security_services(&db_url).await?;
        
        // 创建反欺诈服务
        let anti_fraud_service = Self::create_anti_fraud_service(&db_url).await?;
        
        // 创建通知服务
        let notification_service = Self::create_notification_service(&db_url).await?;
        
        // 创建业务服务
        let services = Self::create_business_services(
            &repositories,
            &db_url,
            &upload_path,
            &jwt_secret,
            email_service.clone(),
            &download_security_service,
            &notification_service,
        ).await?;
        
        // 启动后台任务
        Self::start_background_tasks(&db_url).await;
        
        info!("✅ 服务容器初始化完成");
        
        Ok(ServiceContainer {
            auth_service: services.auth_service,
            user_service: services.user_service,
            package_service: services.package_service,
            admin_service: services.admin_service,
            comment_service: services.comment_service,
            forbidden_word_service: services.forbidden_word_service,
            community_service: services.community_service,
            user_action_service: services.user_action_service,
            post_service: services.post_service,
            tag_service: services.tag_service,
            notification_service,
            download_security_service,
            security_action_service,
            email_service,
            anti_fraud_service,
            user_repo: repositories.user_repo,
            package_repo: repositories.package_repo,
            system_repo: repositories.system_repo,
            subscription_repo: repositories.subscription_repo,
            follow_repo: repositories.follow_repo,
            post_repo: repositories.post_repo,
            jwt_utils: crate::utils::jwt::JwtUtils::new(jwt_secret),
        })
    }
    
    /// 创建仓库实例
    async fn create_repositories(db_url: &str) -> Result<RepositoryContainer, BootstrapError> {
        info!("📦 创建仓库实例...");
        
        let user_repo = UserRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建用户仓库失败: {}", e)))?;
        
        let package_repo = PackageRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建绳包仓库失败: {}", e)))?;
        
        let comment_repo = CommentRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建评论仓库失败: {}", e)))?;
        
        let forbidden_word_repo = ForbiddenWordRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建违禁词仓库失败: {}", e)))?;
        
        let system_repo = SystemRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建系统仓库失败: {}", e)))?;
        
        let user_action_repo = UserActionRepository::new(
            Arc::new(tokio::sync::Mutex::new(
                rusqlite::Connection::open(db_url)
                    .map_err(|e| BootstrapError::Service(format!("打开数据库连接失败: {}", e)))?
            ))
        );
        
        let email_verification_repo = EmailVerificationRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建邮件验证仓库失败: {}", e)))?;
        
        let subscription_repo = SubscriptionRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建订阅仓库失败: {}", e)))?;
        
        let notification_repo = NotificationRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建通知仓库失败: {}", e)))?;
        
        let follow_repo = FollowRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建关注仓库失败: {}", e)))?;
        
        let post_repo = PostRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建帖子仓库失败: {}", e)))?;
        
        Ok(RepositoryContainer {
            user_repo,
            package_repo,
            comment_repo,
            forbidden_word_repo,
            system_repo,
            user_action_repo,
            email_verification_repo,
            subscription_repo,
            notification_repo,
            follow_repo,
            post_repo,
        })
    }
    
    /// 创建邮件服务
    async fn create_email_service(db_url: &str) -> Result<Arc<RwLock<EmailService>>, BootstrapError> {
        info!("📧 初始化邮件服务...");
        
        let mail_repo = MailRepository::new(db_url);
        let email_service = Arc::new(RwLock::new(
            match EmailService::new(mail_repo).await {
                Ok(service) => {
                    info!("邮件服务初始化成功");
                    service
                },
                Err(e) => {
                    warn!("邮件服务初始化失败，但服务将继续运行: {}", e);
                    let fallback_repo = MailRepository::new(db_url);
                    EmailService::new(fallback_repo).await
                        .map_err(|e| BootstrapError::Service(format!("邮件服务完全失败: {}", e)))?
                }
            }
        ));
        
        Ok(email_service)
    }
    
    /// 创建安全服务
    async fn create_security_services(db_url: &str) -> Result<(
        DownloadSecurityService,
        SecurityActionService
    ), BootstrapError> {
        info!("🛡️ 初始化安全服务...");
        
        let download_security_config = DownloadSecurityConfig::default();
        let security_config = SecurityConfig::default();
        
        let security_action_service = SecurityActionService::new(
            db_url,
            security_config
        ).map_err(|e| BootstrapError::Service(format!("创建安全操作服务失败: {}", e)))?;
        
        let download_security_service = DownloadSecurityService::new(
            db_url, 
            download_security_config
        ).map_err(|e| BootstrapError::Service(format!("创建防刷量服务失败: {}", e)))?
        .with_security_action_service(security_action_service.clone());
        
        Ok((download_security_service, security_action_service))
    }
    
    /// 创建反欺诈服务
    async fn create_anti_fraud_service(db_url: &str) -> Result<AntiFraudService, BootstrapError> {
        info!("🔍 初始化反欺诈服务...");
        
        let conn = Arc::new(tokio::sync::Mutex::new(
            rusqlite::Connection::open(db_url)
                .map_err(|e| BootstrapError::Service(format!("创建反欺诈服务数据库连接失败: {}", e)))?
        ));
        
        Ok(AntiFraudService::new(conn))
    }
    
    /// 创建通知服务
    async fn create_notification_service(db_url: &str) -> Result<NotificationService, BootstrapError> {
        info!("🔔 初始化通知服务...");
        
        let notification_repo = NotificationRepository::new(db_url)
            .map_err(|e| BootstrapError::Service(format!("创建通知仓库失败: {}", e)))?;
        
        Ok(NotificationService::new(notification_repo))
    }
    
    /// 创建业务服务
    async fn create_business_services(
        repos: &RepositoryContainer,
        db_url: &str,
        upload_path: &str,
        jwt_secret: &str,
        email_service: Arc<RwLock<EmailService>>,
        download_security_service: &DownloadSecurityService,
        notification_service: &NotificationService,
    ) -> Result<BusinessServices, BootstrapError> {
        info!("💼 创建业务服务...");
        
        let auth_service = AuthService::new(
            repos.user_repo.clone(),
            jwt_secret.to_string(),
            repos.email_verification_repo.clone(),
            email_service.clone()
        );
        
        let user_service = UserService::new(repos.user_repo.clone());
        
        let forbidden_word_service = ForbiddenWordService::new(
            repos.forbidden_word_repo.clone()
        );
        
        let package_service = PackageService::new(
            repos.package_repo.clone(),
            upload_path.to_string()
        )
        .with_system_repo(repos.system_repo.clone())
        .with_notifier(repos.subscription_repo.clone(), email_service.clone())
        .with_user_repo(repos.user_repo.clone())
        .with_download_security_service(download_security_service)
        .with_notification_service(notification_service.clone());
        
        let comment_service = CommentService::new(
            repos.comment_repo.clone(),
            repos.user_repo.clone()
        )
        .with_package_repo(repos.package_repo.clone())
        .with_user_action_repo(repos.user_action_repo.clone())
        .with_notification_service(notification_service.clone())
        .with_forbidden_service(forbidden_word_service.clone());
        
        let community_service = CommunityService::new(
            repos.comment_repo.clone()
        );
        
        let user_action_service = UserActionService::new(
            repos.user_action_repo.clone()
        );
        
        let post_service = PostService::new(db_url.to_string())
            .with_notifier(notification_service.clone());
        
        let tag_service = TagService::new(db_url.to_string());
        
        let admin_service = AdminService::new(db_url);
        
        Ok(BusinessServices {
            auth_service,
            user_service,
            package_service,
            admin_service,
            comment_service,
            forbidden_word_service,
            community_service,
            user_action_service,
            post_service,
            tag_service,
        })
    }
    
    /// 启动后台任务
    async fn start_background_tasks(db_url: &str) {
        let storage_db_url = db_url.to_string();
        tokio::spawn(async move {
            info!("🚀 正在初始化存储服务...");
            match PackageStorageService::new(&storage_db_url) {
                Ok(mut storage_service) => {
                    info!("📡 开始连接远程存储系统...");
                    match storage_service.initialize_storage().await {
                        Ok(_) => {
                            info!("✅ 存储服务初始化成功");
                            
                            // 启动定期健康检查
                            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30 * 60));
                            loop {
                                interval.tick().await;
                                if !storage_service.health_check().await {
                                    warn!("⚠️ 存储服务健康检查失败，尝试重新初始化");
                                    if let Err(e) = storage_service.initialize_storage().await {
                                        error!("❌ 存储服务重新初始化失败: {}", e);
                                    } else {
                                        info!("✅ 存储服务重新初始化成功");
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            error!("❌ 存储服务初始化失败: {}", e);
                            warn!("⚠️ 系统将继续运行，但文件上传功能可能不可用");
                        }
                    }
                },
                Err(e) => {
                    error!("❌ 创建存储服务实例失败: {}", e);
                    warn!("⚠️ 系统将继续运行，但文件上传功能可能不可用");
                }
            }
        });
    }
}

/// 仓库容器
struct RepositoryContainer {
    user_repo: UserRepository,
    package_repo: PackageRepository,
    comment_repo: CommentRepository,
    forbidden_word_repo: ForbiddenWordRepository,
    system_repo: SystemRepository,
    user_action_repo: UserActionRepository,
    email_verification_repo: EmailVerificationRepository,
    subscription_repo: SubscriptionRepository,
    notification_repo: NotificationRepository,
    follow_repo: FollowRepository,
    post_repo: PostRepository,
}

/// 业务服务容器
struct BusinessServices {
    auth_service: AuthService,
    user_service: UserService,
    package_service: PackageService,
    admin_service: AdminService,
    comment_service: CommentService,
    forbidden_word_service: ForbiddenWordService,
    community_service: CommunityService,
    user_action_service: UserActionService,
    post_service: PostService,
    tag_service: TagService,
}