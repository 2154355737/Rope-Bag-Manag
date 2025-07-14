use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use log::info;

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
    conn.execute_batch(include_str!("../sql/init.sql"))
        .expect("数据库初始化失败");
    
    // 创建服务实例
    let user_repo = repositories::UserRepository::new(config.database_url())
        .expect("创建用户仓库失败");
    let package_repo = repositories::PackageRepository::new(config.database_url())
        .expect("创建绳包仓库失败");
    let comment_repo = repositories::CommentRepository::new(config.database_url())
        .expect("创建评论仓库失败");
    let system_repo = repositories::SystemRepository::new(config.database_url())
        .expect("创建系统仓库失败");
    
    let auth_service = services::AuthService::new(user_repo.clone(), config.jwt_secret().to_string());
    let user_service = services::UserService::new(user_repo);
    let package_service = services::PackageService::new(package_repo, config.upload_path().to_string());
    let admin_service = services::AdminService::new(system_repo);
    let community_service = services::CommunityService::new(comment_repo);
    
    info!("服务器启动在 http://{}", config.server_address());
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(package_service.clone()))
            .app_data(web::Data::new(admin_service.clone()))
            .app_data(web::Data::new(community_service.clone()))
            .configure(api::configure_routes)
    })
    .workers(config.server.workers)
    .bind(config.server_address())?
    .run()
    .await
}
