pub mod domain;
pub mod services;
pub mod ports;

use std::sync::Arc;
use tracing::{info, instrument};

use crate::config::AppConfig;
use crate::infrastructure::database::{DatabaseManager, repositories};
use crate::shared::errors::{AppError, AppResult};
use crate::infrastructure::storage::{OpenDalStorage, Storage};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseManager,
    pub services: ServiceContainer,
    pub storage: Arc<dyn Storage>,
}

/// 服务容器
#[derive(Clone)]
pub struct ServiceContainer {
    pub user_service: Arc<services::UserService>,
    pub package_service: Arc<services::PackageService>,
    pub auth_service: Arc<services::AuthService>,
    pub category_service: Arc<services::CategoryService>,
    pub comment_service: Arc<services::CommentService>,
    pub notification_service: Arc<services::NotificationService>,
    // 新增：帖子与资源服务
    pub post_service: Arc<services::PostService>,
    pub resource_service: Arc<services::ResourceService>,
}

impl AppState {
    #[instrument(skip(config, db_manager))]
    pub async fn new(config: AppConfig, db_manager: DatabaseManager) -> AppResult<Self> {
        let services = ServiceContainer::new(&config, &db_manager).await?;
        // 初始化存储（OpenDAL FS）
        let storage = Arc::new(
            OpenDalStorage::new_fs(&config.storage.upload_path)
                .map_err(|e| AppError::File(e.to_string()))?
        ) as Arc<dyn Storage>;
        
        info!("✅ 应用状态初始化完成");
        
        Ok(Self {
            config,
            db: db_manager,
            services,
            storage,
        })
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> AppResult<()> {
        // 检查数据库连接
        self.db.health_check().await?;
        
        // 检查各个服务
        if !self.services.user_service.health_check().await {
            return Err(AppError::Internal);
        }
        
        if !self.services.auth_service.health_check().await {
            return Err(AppError::Internal);
        }
        
        // 可以添加更多健康检查...
        
        Ok(())
    }
}

impl ServiceContainer {
    #[instrument(skip(config, db))]
    pub async fn new(config: &AppConfig, db: &DatabaseManager) -> AppResult<Self> {
        info!("🔧 正在初始化仓储层...");
        
        // 创建仓储实例
        let user_repo = Arc::new(repositories::UserRepository::new(db.pool()));
        let package_repo = Arc::new(repositories::PackageRepository::new(db.pool()));
        let category_repo = Arc::new(repositories::CategoryRepository::new(db.pool()));
        let comment_repo = Arc::new(repositories::CommentRepository::new(db.pool()));
        let notification_repo = Arc::new(repositories::NotificationRepository::new(db.pool()));

        // 新增仓储：post/resource + like/bookmark
        let post_repo = Arc::new(repositories::SqlxPostRepository::new(db.pool()));
        let resource_repo = Arc::new(repositories::SqlxResourceRepository::new(db.pool()));
        let like_repo = Arc::new(repositories::SqlxLikeRepository::new(db.pool()));
        let bookmark_repo = Arc::new(repositories::SqlxBookmarkRepository::new(db.pool()));
        
        info!("📦 仓储层初始化完成");
        info!("🛠️ 正在初始化服务层...");
        
        // 创建服务实例
        let user_service = Arc::new(services::UserService::new(user_repo.clone()));
        let package_service = Arc::new(services::PackageService::new(
            package_repo,
            user_repo.clone(),
            category_repo.clone(),
        ));
        let auth_service = Arc::new(services::AuthService::new(
            user_repo.clone(),
            &config.jwt,
        ));
        let category_service = Arc::new(services::CategoryService::new(category_repo));
        let comment_service = Arc::new(services::CommentService::new(
            comment_repo,
            user_repo.clone(),
        ));
        let notification_service = Arc::new(services::NotificationService::new(notification_repo));
        
        // 新增：帖子与资源服务
        let post_service = Arc::new(services::PostService::new(
            post_repo.clone(),
            like_repo.clone(),
            bookmark_repo.clone(),
        ));
        let resource_service = Arc::new(services::ResourceService::new(
            resource_repo.clone(),
            like_repo.clone(),
            bookmark_repo.clone(),
        ));
        
        info!("⚡ 服务层初始化完成");
        
        Ok(Self {
            user_service,
            package_service,
            auth_service,
            category_service,
            comment_service,
            notification_service,
            post_service,
            resource_service,
        })
    }
} 