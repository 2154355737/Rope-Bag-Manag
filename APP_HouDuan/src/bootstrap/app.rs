// 应用构建器和配置模块

use actix_web::{App, HttpServer, web, middleware};
use actix_cors::Cors;
use actix_files::Files;
use log::info;
use std::sync::Arc;

use crate::config::Config;
use crate::middleware::api_logger::ApiLogger;
use super::{ServiceContainer, BootstrapError};

/// 应用构建器
pub struct AppBuilder {
    config: Config,
    services: ServiceContainer,
}

impl AppBuilder {
    /// 创建新的应用构建器
    pub fn new(config: Config, services: ServiceContainer) -> Self {
        Self { config, services }
    }
    
    /// 构建并运行应用
    pub async fn build_and_run(self) -> Result<(), BootstrapError> {
        info!("🌐 构建并启动Web应用...");
        
        let server_address = self.config.server_address().to_string();
        let workers = self.config.server.workers;
        let uploads_dir = self.config.file.upload_path.clone();
        
        info!("✅ 所有服务初始化完成");
        info!("🌐 API服务启动在: http://{}", server_address);
        
        // 移动 services 到 move 闭包中
        let services = Arc::new(self.services);
        
        HttpServer::new(move || {
            let services_clone = services.clone();
            App::new()
                .wrap(ApiLogger)
                .wrap(Self::create_cors_config())
                .configure(move |cfg| Self::configure_services(cfg, &services_clone))
                .configure(crate::api::configure_routes)
                .service(
                    web::scope("/uploads")
                        .service(Files::new("/", &uploads_dir).show_files_listing())
                )
        })
        .workers(workers)
        .bind(&server_address)?
        .run()
        .await?;
        
        Ok(())
    }
    
    /// 创建CORS配置
    fn create_cors_config() -> Cors {
        Cors::default()
            // 开发环境
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            // Tauri 桌面应用
            .allowed_origin("http://tauri.localhost")
            .allowed_origin("https://tauri.localhost")
            // 生产环境 - 添加你的服务器地址
            .allowed_origin("http://39.105.113.219")
            .allowed_origin("https://39.105.113.219")
            // 安卓 WebView 资源源（Tauri Android）
            .allowed_origin("https://appassets.androidplatform.net")
            // Capacitor Android 应用
            .allowed_origin("http://localhost")
            .allowed_origin("https://localhost")
            .allowed_origin("capacitor://localhost")
            .allowed_origin("ionic://localhost")
            // Capacitor HTTP 插件
            .allowed_origin("http://10.0.2.2:15201")
            .allowed_origin("http://127.0.0.1:15201")
            // 移动应用的特殊来源
            .allowed_origin_fn(|origin, req_head| {
                let origin_str = origin.as_bytes();
                // 允许 file:// 协议（移动应用本地文件）
                if origin_str.starts_with(b"file://") {
                    return true;
                }
                // 允许 null 来源（某些移动应用会发送 null origin）
                if origin_str == b"null" {
                    return true;
                }
                // 允许来自移动应用的请求（通过 User-Agent 判断）
                if let Some(user_agent) = req_head.headers().get("user-agent") {
                    if let Ok(ua_str) = user_agent.to_str() {
                        return ua_str.contains("CapacitorApp") || 
                               ua_str.contains("Mobile") ||
                               ua_str.contains("Android");
                    }
                }
                false
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CACHE_CONTROL,
                actix_web::http::header::PRAGMA,
                actix_web::http::header::EXPIRES,
                actix_web::http::header::USER_AGENT,
                actix_web::http::header::ORIGIN,
            ])
            .supports_credentials() // 支持认证
            .max_age(3600)
    }
    
    /// 配置服务依赖注入
    fn configure_services(cfg: &mut web::ServiceConfig, services: &ServiceContainer) {
        cfg
            .app_data(web::Data::new(services.auth_service.clone()))
            .app_data(web::Data::new(services.jwt_utils.clone()))
            .app_data(web::Data::new(services.user_service.clone()))
            .app_data(web::Data::new(services.package_service.clone()))
            .app_data(web::Data::new(services.admin_service.clone()))
            .app_data(web::Data::new(services.comment_service.clone()))
            .app_data(web::Data::new(services.forbidden_word_service.clone()))
            .app_data(web::Data::new(services.community_service.clone()))
            .app_data(web::Data::new(services.system_repo.clone()))
            .app_data(web::Data::new(services.user_action_service.clone()))
            .app_data(web::Data::new(services.email_service.clone()))
            .app_data(web::Data::new(services.subscription_repo.clone()))
            .app_data(web::Data::new(Arc::new(services.package_repo.clone())))
            .app_data(web::Data::new(services.post_service.clone()))
            .app_data(web::Data::new(services.tag_service.clone()))
            .app_data(web::Data::new(services.notification_service.clone()))
            .app_data(web::Data::new(services.download_security_service.clone()))
            .app_data(web::Data::new(services.security_action_service.clone()))
            .app_data(web::Data::new(services.anti_fraud_service.clone()))
            .app_data(web::Data::new(Arc::new(services.follow_repo.clone())))
            .app_data(web::Data::new(Arc::new(services.user_repo.clone())))
            .app_data(web::Data::new(Arc::new(services.post_repo.clone())));
    }
}