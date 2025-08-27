// 启动引导模块
// 负责应用程序的初始化和配置

pub mod database;
pub mod services;
pub mod app;

pub use database::DatabaseManager;
pub use services::ServiceContainer;
pub use app::AppBuilder;

use anyhow::Result;
use log::info;

/// 启动错误类型
#[derive(Debug, thiserror::Error)]
pub enum BootstrapError {
    #[error("数据库初始化失败: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("服务初始化失败: {0}")]
    Service(String),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
}

/// 应用启动协调器
pub struct Bootstrap;

impl Bootstrap {
    /// 执行完整的应用启动流程
    pub async fn run() -> Result<(), BootstrapError> {
        info!("🚀 开始启动绳包管理器后端服务");
        
        // 1. 初始化日志系统
        Self::init_logging()?;
        
        // 2. 加载配置
        let config = Self::load_config()?;
        
        // 3. 初始化数据库
        let db_manager = DatabaseManager::new(&config)?;
        db_manager.initialize().await?;
        
        // 4. 初始化服务容器
        let services = ServiceContainer::new(&config).await?;
        
        // 5. 构建并启动应用
        let app_builder = AppBuilder::new(config, services);
        app_builder.build_and_run().await?;
        
        Ok(())
    }
    
    /// 初始化日志系统
    fn init_logging() -> Result<(), BootstrapError> {
        crate::utils::logger::init_logger();
        crate::utils::logger::log_system_start("1.0.0", "15201");
        Ok(())
    }
    
    /// 加载和验证配置
    fn load_config() -> Result<crate::config::Config, BootstrapError> {
        let config = crate::config::Config::load()
            .map_err(|e| BootstrapError::Config(format!("加载配置失败: {}", e)))?;
        
        // 验证配置
        Self::validate_config(&config)?;
        
        info!("✅ 配置加载和验证完成");
        Ok(config)
    }
    
    /// 验证配置
    fn validate_config(config: &crate::config::Config) -> Result<(), BootstrapError> {
        // 验证必要的配置项
        if config.server.port == 0 {
            return Err(BootstrapError::Config("服务器端口不能为0".to_string()));
        }
        
        if config.database.url.is_empty() {
            return Err(BootstrapError::Config("数据库URL不能为空".to_string()));
        }
        
        if config.auth.jwt_secret.len() < 32 {
            return Err(BootstrapError::Config("JWT密钥长度至少需要32个字符".to_string()));
        }
        
        Ok(())
    }
}