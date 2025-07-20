use actix_web::{web, App, HttpServer, middleware::Logger, http};
use actix_cors::Cors;
use log::info;
use std::sync::Arc;

mod config;
mod models;
mod api;
mod services;
mod repositories;
mod middleware;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("启动绳包管理器后端服务...");
    
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
    

    // 数据库URL和配置
    let db_url = config.database_url().to_string();
    let upload_path = config.upload_path().to_string();
    let jwt_secret = config.jwt_secret().to_string();
    let server_address = config.server_address().to_string();
    let workers = config.server.workers;
    
    // 启动服务器
    info!("服务器启动在 http://{}", server_address);
    
    HttpServer::new(move || {
        // 为每个工作线程创建仓库实例
        let user_repo = repositories::UserRepository::new(&db_url)
            .expect("创建用户仓库失败");
        let package_repo = repositories::PackageRepository::new(&db_url)
            .expect("创建绳包仓库失败");
        let comment_repo = repositories::CommentRepository::new(&db_url)
            .expect("创建评论仓库失败");
        let system_repo = repositories::SystemRepository::new(&db_url)
            .expect("创建系统仓库失败");
        let user_action_repo = repositories::user_action_repo::UserActionRepository::new(
            Arc::new(tokio::sync::Mutex::new(
                rusqlite::Connection::open(&db_url).expect("打开数据库连接失败")
            ))
        );
        
        // 创建服务实例
        let auth_service = services::auth_service::AuthService::new(
            user_repo.clone(), jwt_secret.clone()
        );
        let user_service = services::user_service::UserService::new(
            user_repo.clone()
        );
        let package_service = services::package_service::PackageService::new(
            package_repo.clone(), upload_path.clone()
        ).with_system_repo(system_repo.clone());
        let admin_service = services::admin_service::AdminService::new(
            system_repo.clone(), user_service.clone()
        );
        let comment_service = services::comment_service::CommentService::new(
            comment_repo.clone(), user_repo.clone()
        );
        let community_service = services::community_service::CommunityService::new(
            comment_repo.clone()
        );
        let user_action_service = services::user_action_service::UserActionService::new(
            user_action_repo.clone()
        );

        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_origin("http://127.0.0.1:5173")
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                        http::header::ACCEPT,
                    ])
                    .max_age(3600)
            )
            .app_data(web::Data::new(auth_service))
            .app_data(web::Data::new(user_service))
            .app_data(web::Data::new(package_service))
            .app_data(web::Data::new(admin_service))
            .app_data(web::Data::new(comment_service))
            .app_data(web::Data::new(community_service))
            .app_data(web::Data::new(system_repo))
            .app_data(web::Data::new(user_action_service))
            .configure(api::configure_routes)
    })
    .workers(workers)
    .bind(server_address)?
    .run()
    .await
}
