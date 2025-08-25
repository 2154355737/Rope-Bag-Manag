use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use tracing::{info, warn};
use std::time::Duration;

mod core;
mod infrastructure;
mod api;
mod shared;
mod config;

use crate::config::AppConfig;
use crate::infrastructure::database::DatabaseManager;
use crate::shared::errors::AppResult;
use crate::shared::utils::logging::{init_logging, log_system_info, log_system_shutdown};
use crate::api::middleware::RequestTracing;

#[actix_web::main]
async fn main() -> AppResult<()> {
    // 加载配置
    let config = AppConfig::load()?;
    
    // 初始化日志系统 - 必须在其他操作之前
    init_logging(&config.logging)
        .map_err(|e| shared::errors::AppError::Internal)?;
    
    // 记录系统启动信息
    log_system_info("2.0.0", config.server.port);
    info!("📋 配置加载完成: {}:{}", config.server.host, config.server.port);
    
    // 创建必要的目录
    create_directories(&config).await?;
    
    // 初始化数据库
    let db_manager = DatabaseManager::new(&config.database).await?;
    info!("💾 数据库连接池初始化完成");
    
    // 运行数据库迁移
    db_manager.run_migrations().await?;
    info!("📦 数据库迁移完成");
    
    // 创建应用状态
    let app_state = create_app_state(config.clone(), db_manager).await?;
    info!("⚙️ 应用状态初始化完成");
    
    // 启动HTTP服务器
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    info!("🌐 服务器准备启动: http://{}", bind_address);
    
    // 设置优雅关闭处理
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            // 设置文件上传大小限制 (100MB)
            .app_data(web::PayloadConfig::new(100 * 1024 * 1024))
            // 设置JSON负载大小限制 (10MB)
            .app_data(web::JsonConfig::default().limit(10 * 1024 * 1024))
            .wrap(setup_cors())
            .wrap(RequestTracing::new())
            .configure(api::configure_routes)
    })
    .workers(config.server.workers)
    // 设置连接超时和保持连接超时
    .client_timeout(Duration::from_secs(300)) // 5分钟客户端超时
    .client_disconnect_timeout(Duration::from_secs(30)) // 30秒断开超时
    .bind(&bind_address)
    .map_err(|e| shared::errors::AppError::Io(e))?;
    
    info!("✅ 服务器启动成功，监听地址: http://{}", bind_address);
    
    // 启动服务器
    let result = server.run().await;
    
    // 记录系统关闭
    log_system_shutdown();
    
    result.map_err(|e| shared::errors::AppError::Io(e))?;
    Ok(())
}

/// 创建必要的目录结构
async fn create_directories(config: &AppConfig) -> AppResult<()> {
    let db_dir = config.database.url.strip_prefix("sqlite:")
        .and_then(|p| std::path::Path::new(p).parent())
        .and_then(|p| p.to_str())
        .unwrap_or("./data");
    
    let directories = vec![
        config.storage.upload_path.as_str(),
        config.storage.temp_path.as_str(),
        db_dir,
    ];
    
    if let Some(log_path) = &config.logging.file_path {
        tokio::fs::create_dir_all(log_path).await?;
    }
    
    for dir in directories {
        if let Err(e) = tokio::fs::create_dir_all(dir).await {
            warn!("创建目录失败 {}: {}", dir, e);
        } else {
            info!("📁 目录确认存在: {}", dir);
        }
    }
    
    Ok(())
}

/// 设置CORS
fn setup_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_origin("http://127.0.0.1:5173") 
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_origin("tauri://localhost")
        .allowed_origin("https://tauri.localhost")
        .allowed_origin("capacitor://localhost")
        .allowed_origin("ionic://localhost")
        .allowed_origin_fn(|origin, _req_head| {
            let o = origin.as_bytes();
            o.starts_with(b"tauri://") || o == b"null" || o == b"file://"
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::ACCEPT,
            actix_web::http::header::COOKIE,
        ])
        .supports_credentials()
        .max_age(3600)
}

/// 创建应用状态
async fn create_app_state(
    config: AppConfig,
    db_manager: DatabaseManager,
) -> AppResult<core::AppState> {
    info!("🔧 正在初始化服务容器...");
    core::AppState::new(config, db_manager).await
}