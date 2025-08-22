use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use actix_files::Files;
use log::info;
use crate::utils::logger;
use crate::middleware::api_logger::ApiLogger;
use std::sync::Arc;
use tokio::sync::RwLock;
// use serde_json::Value;

mod config;
mod middleware; // 我们自己的middleware模块
mod utils;
mod models;
mod repositories;
mod services;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化优化的日志系统
    logger::init_logger();
    
    // 显示系统启动信息
    logger::log_system_start("1.0.0", "15201");
    
    // 读取配置
    let config = config::Config::load().expect("加载配置失败");
    
    // 创建必要的目录
    std::fs::create_dir_all(&config.file.upload_path).ok();
    std::fs::create_dir_all(&config.file.temp_path).ok();
    if let Some(log_path) = &config.logging.file_path {
        if let Some(log_dir) = std::path::Path::new(log_path).parent() {
            std::fs::create_dir_all(log_dir).ok();
        }
    }
    
    // 初始化数据库
    let conn = repositories::get_connection().expect("数据库连接失败");
    
    // 执行数据库初始化SQL
    match conn.execute_batch(include_str!("../sql/init.sql")) {
        Ok(_) => info!("数据库初始化成功"),
        Err(e) => {
            // 分析错误信息，尝试确定是否是已经存在的表/索引问题
            if e.to_string().contains("already exists") {
                info!("数据库表已存在，跳过初始化");
            } else {
                eprintln!("数据库初始化失败: {}", e);
                // 不要panic，让服务继续启动
            }
        }
    }

    // 执行系统表初始化SQL
    match conn.execute_batch(include_str!("../sql/init_system_tables.sql")) {
        Ok(_) => info!("系统表初始化成功"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                info!("系统表已存在，跳过初始化");
            } else {
                eprintln!("系统表初始化失败: {}", e);
            }
        }
    }
    
    // 执行邮件系统迁移SQL
    conn.execute_batch(include_str!("../sql/migrate_email.sql")).ok();
    
    // 执行邮件配置表创建SQL
    match conn.execute_batch(include_str!("../sql/create_mail_settings.sql")) {
        Ok(_) => info!("邮件配置表初始化成功"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                info!("邮件配置表已存在，跳过初始化");
            } else {
                eprintln!("邮件配置表初始化失败: {}", e);
            }
        }
    }

    // 执行帖子和标签表创建SQL
    match conn.execute_batch(include_str!("../sql/migrate_posts_and_tags.sql")) {
        Ok(_) => info!("帖子和标签表初始化成功"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                info!("帖子和标签表已存在，跳过初始化");
            } else {
                eprintln!("帖子和标签表初始化失败: {}", e);
            }
        }
    }

    // 迁移帖子审核字段
    match conn.execute_batch(include_str!("../sql/migrate_post_review.sql")) {
        Ok(_) => info!("帖子审核字段迁移成功"),
        Err(e) => {
            // 可能因为列已存在而报错，忽略
            info!("帖子审核字段迁移可能已应用: {}", e);
        }
    }

    // 创建点赞表
    match conn.execute_batch(include_str!("../sql/migrate_likes.sql")) {
        Ok(_) => info!("likes 表初始化成功"),
        Err(e) => info!("likes 表迁移可能已应用: {}", e),
    }

    // 修复packages表结构，添加缺失的字段
    match conn.execute_batch(include_str!("../fix_packages_table.sql")) {
        Ok(_) => info!("packages表结构修复成功"),
        Err(e) => {
            if e.to_string().contains("duplicate column name") || e.to_string().contains("already exists") {
                info!("packages表字段已存在，跳过修复");
            } else {
                eprintln!("packages表结构修复失败: {}", e);
            }
        }
    }

    // 创建缺失的表（tags和package_tags）
    match conn.execute_batch(include_str!("../create_missing_tables.sql")) {
        Ok(_) => info!("缺失表创建成功"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                info!("所需表已存在，跳过创建");
            } else {
                eprintln!("缺失表创建失败: {}", e);
            }
        }
    }

    // 修复user_actions表结构，支持可选user_id
    match conn.execute_batch(include_str!("../fix_user_actions_table.sql")) {
        Ok(_) => info!("user_actions表结构修复成功"),
        Err(e) => {
            eprintln!("user_actions表结构修复失败: {}", e);
        }
    }

    // 统一公告表结构（迁移至带 enabled/start_time/end_time 的单一版本）
    match conn.execute_batch(include_str!("../sql/migrate_announcements.sql")) {
        Ok(_) => info!("公告表结构初始化/迁移成功"),
        Err(e) => {
            info!("公告表迁移可能已应用: {}", e);
        }
    }

    // 检查并启用邮件服务（如果配置了有效的SMTP信息）
    match conn.prepare("SELECT username, password, enabled FROM mail_settings WHERE id = 1") {
        Ok(mut stmt) => {
            if let Ok(row) = stmt.query_row([], |row| {
                Ok((
                    row.get::<_, String>(0)?, // username
                    row.get::<_, String>(1)?, // password
                    row.get::<_, i32>(2)?     // enabled
                ))
            }) {
                let (username, password, enabled) = row;
                if !username.is_empty() && !password.is_empty() && enabled == 0 {
                    match conn.execute("UPDATE mail_settings SET enabled = 1 WHERE id = 1", []) {
                        Ok(_) => info!("检测到有效的邮件配置，已自动启用邮件服务"),
                        Err(e) => eprintln!("自动启用邮件服务失败: {}", e),
                    }
                }
            }
        },
        Err(_) => {} // 表可能不存在，忽略
    }

    // 数据库URL和配置
    let db_url = config.database_url().to_string();
    let upload_path = config.upload_path().to_string();
    let jwt_secret = config.jwt_secret().to_string();

    // 创建JWT工具实例，供中间件解码Token
    let jwt_utils = utils::jwt::JwtUtils::new(jwt_secret.clone());
    let server_address = config.server_address().to_string();
    let workers = config.server.workers;
    
    // 初始化邮件服务（改进的稳定版本）
    let mail_repo = repositories::mail_repo::MailRepository::new(&config.database_url());
    let email_service = Arc::new(RwLock::new(
        match services::email_service::EmailService::new(mail_repo).await {
            Ok(service) => {
                info!("邮件服务初始化成功");
                service
            },
            Err(e) => {
                eprintln!("邮件服务初始化失败，但服务将继续运行: {}", e);
                // 重新尝试创建，这次不会panic
                let fallback_repo = repositories::mail_repo::MailRepository::new(&config.database_url());
                services::email_service::EmailService::new(fallback_repo).await.expect("邮件服务完全失败")
            }
        }
    ));
    
    // 初始化存储服务（改进版本）
    let storage_db_url = db_url.clone();
    tokio::spawn(async move {
        log::info!("🚀 正在初始化存储服务...");
        match services::package_storage_service::PackageStorageService::new(&storage_db_url) {
            Ok(mut storage_service) => {
                log::info!("📡 开始连接远程存储系统...");
                match storage_service.initialize_storage().await {
                    Ok(_) => {
                        log::info!("✅ 存储服务初始化成功");
                        
                        // 启动定期健康检查
                        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30 * 60)); // 30分钟检查一次
                        loop {
                            interval.tick().await;
                            log::debug!("🔄 执行存储服务定期健康检查");
                            if !storage_service.health_check().await {
                                log::warn!("⚠️ 存储服务健康检查失败，尝试重新初始化");
                                if let Err(e) = storage_service.initialize_storage().await {
                                    log::error!("❌ 存储服务重新初始化失败: {}", e);
                                } else {
                                    log::info!("✅ 存储服务重新初始化成功");
                                }
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("❌ 存储服务初始化失败: {}", e);
                        log::warn!("⚠️ 系统将继续运行，但文件上传功能可能不可用");
                    }
                }
            },
            Err(e) => {
                log::error!("❌ 创建存储服务实例失败: {}", e);
                log::warn!("⚠️ 系统将继续运行，但文件上传功能可能不可用");
            }
        }
    });
    
    // 启动服务器
    info!("✅ 所有服务初始化完成");
    info!("🌐 API服务启动在: http://{}", server_address);
    
    HttpServer::new(move || {
        // 为每个工作线程创建仓库实例
        let user_repo = repositories::UserRepository::new(&db_url)
            .expect("创建用户仓库失败");
        let package_repo = repositories::PackageRepository::new(&db_url)
            .expect("创建绳包仓库失败");
        let comment_repo = repositories::CommentRepository::new(&db_url)
            .expect("创建评论仓库失败");
        let forbidden_word_repo = repositories::forbidden_word_repo::ForbiddenWordRepository::new(&db_url)
            .expect("创建违禁词仓库失败");
        let system_repo = repositories::SystemRepository::new(&db_url)
            .expect("创建系统仓库失败");
        let user_action_repo = repositories::user_action_repo::UserActionRepository::new(
            Arc::new(tokio::sync::Mutex::new(
                rusqlite::Connection::open(&db_url).expect("打开数据库连接失败")
            ))
        );
        let email_verification_repo = repositories::EmailVerificationRepository::new(&db_url).expect("创建email repo失败");
        let subscription_repo = repositories::SubscriptionRepository::new(&db_url).expect("创建订阅repo失败");

        // 创建服务实例
        let auth_service = services::auth_service::AuthService::new(
            user_repo.clone(), jwt_secret.clone(), email_verification_repo.clone(), email_service.clone()
        );
        let user_service = services::user_service::UserService::new(
            user_repo.clone()
        );
        // 初始化防刷量服务
        let download_security_config = models::download_security::DownloadSecurityConfig::default();
        let security_config = models::download_security::SecurityConfig::default();
        
        let security_action_service = services::security_action_service::SecurityActionService::new(
            &db_url,
            security_config
        ).expect("创建安全操作服务失败");
        
        let download_security_service = services::download_security_service::DownloadSecurityService::new(
            &db_url, 
            download_security_config
        ).expect("创建防刷量服务失败")
        .with_security_action_service(security_action_service.clone());

        let notification_repo = repositories::notification_repo::NotificationRepository::new(&db_url).expect("创建通知仓库失败");
        let notification_service = services::notification_service::NotificationService::new(notification_repo.clone());

        let package_service = services::package_service::PackageService::new(
            package_repo.clone(), upload_path.clone()
        ).with_system_repo(system_repo.clone())
        .with_notifier(subscription_repo.clone(), email_service.clone())
        .with_user_repo(user_repo.clone())
        .with_download_security_service(&download_security_service)
        .with_notification_service(notification_service.clone());
        

        let admin_service = services::admin_service::AdminService::new(&db_url);
        let forbidden_word_service = services::forbidden_word_service::ForbiddenWordService::new(
            forbidden_word_repo.clone()
        );
        let comment_service = services::comment_service::CommentService::new(
            comment_repo.clone(), user_repo.clone()
        ).with_package_repo(package_repo.clone())
        .with_user_action_repo(user_action_repo.clone())
        .with_notification_service(notification_service.clone())
        .with_forbidden_service(forbidden_word_service.clone());
        let community_service = services::community_service::CommunityService::new(
            comment_repo.clone()
        );
        let user_action_service = services::user_action_service::UserActionService::new(
            user_action_repo.clone()
        );
        let post_service = services::post_service::PostService::new(db_url.clone())
            .with_notifier(notification_service.clone());
        let tag_service = services::tag_service::TagService::new(db_url.clone());

        let uploads_dir = &config.file.upload_path;

        App::new()
            .wrap(ApiLogger)
            .wrap(
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
                    // 允许 Tauri WebView（tauri:// 协议）
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
                        actix_web::http::header::SET_COOKIE,
                    ])
                    .supports_credentials() // 支持Cookie
                    .max_age(3600)
            )
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(jwt_utils.clone()))
            .app_data(web::Data::new(user_service))
            .app_data(web::Data::new(package_service))
            .app_data(web::Data::new(admin_service))
            .app_data(web::Data::new(comment_service))
            .app_data(web::Data::new(forbidden_word_service))
            .app_data(web::Data::new(community_service))
            .app_data(web::Data::new(system_repo))
            .app_data(web::Data::new(user_action_service))
            .app_data(web::Data::new(email_service.clone()))
            .app_data(web::Data::new(subscription_repo))
            .app_data(web::Data::new(package_repo))
            .app_data(web::Data::new(post_service))
            .app_data(web::Data::new(tag_service))
            .app_data(web::Data::new(notification_service))
            .app_data(web::Data::new(download_security_service))
            .app_data(web::Data::new(security_action_service))
            .configure(api::configure_routes)  // 配置根级别API路由（包含health接口）
            .service(
                web::scope("/uploads")
                    .service(Files::new("/", uploads_dir).show_files_listing())
            )
    })
    .workers(workers)
    .bind(server_address)?
    .run()
    .await
}
