mod models;
mod utils;
mod auth;
mod handlers;
mod logger;
mod config;
mod cmd;

use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use models::{AppState, Users, RopePackages, StatsData, GlobalLimiter, GlobalCount, RawDataJson, RopePackage};

use utils::load_json;
use logger::{init_logger, RequestLogger};
use config::load_config;
use cmd::start_command_listener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    let app_config = load_config();
    
    // 初始化日志系统
    if let Err(e) = init_logger(
        app_config.logging.console_output,
        app_config.logging.file_output,
        &app_config.logging.log_level
    ) {
        eprintln!("日志系统初始化失败: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
    }

    log::info!("绳包管理器服务启动中...");
    log::info!("配置加载完成 - 日志级别: {}", app_config.logging.log_level);

    // 加载用户数据
    let users: Users = Arc::new(Mutex::new(load_json("data/users.json")));

    // 加载绳包数据并转换为 HashMap<u32, RopePackage>
    let raw_data: RawDataJson = load_json("data/data.json");
    let mut ropes_map: HashMap<u32, RopePackage> = HashMap::new();

    for item in raw_data.绳包列表 {
        ropes_map.insert(item.id, RopePackage {
            id: item.id,
            name: item.绳包名称,
            author: item.作者,
            version: item.版本,
            desc: item.简介,
            url: item.项目直链,
            downloads: 0,
        });
    }

    let ropes: RopePackages = Arc::new(Mutex::new(ropes_map));

    // 启动后台命令监听线程
    start_command_listener(users.clone(), ropes.clone(), app_config.clone());

    // 加载其他数据
    let stats: StatsData = Arc::new(Mutex::new(load_json("data/stats.json")));
    let app_config = load_config(); // 使用新的配置系统
    let limiter: GlobalLimiter = Arc::new(Mutex::new(HashMap::new()));
    let global: GlobalCount = Arc::new(Mutex::new((0, 0)));

    // 克隆配置用于服务器启动
    let server_config = app_config.clone();

    log::info!("数据加载完成，启动HTTP服务...");
    log::info!("服务地址: {}:{}", app_config.server.host, app_config.server.port);

    // 启动服务
    HttpServer::new(move || {
        App::new()
            .wrap(RequestLogger)
            .app_data(web::Data::new(AppState {
                users: users.clone(),
                ropes: ropes.clone(),
                stats: stats.clone(),
                config: app_config.clone(),
                limiter: limiter.clone(),
                global: global.clone(),
            }))
            // 注册所有 handlers
            .service(handlers::register)
            .service(handlers::login)
            .service(handlers::user_info)
            .service(handlers::sign_in)
            .service(handlers::change_password)
            .service(handlers::change_nickname)
            .service(handlers::nicknames)
            .service(handlers::add_rope_package)
            .service(handlers::download_rope_package)
            .service(handlers::get_data_db)
            .service(handlers::delete_rope_package)
            .service(handlers::update_rope_package)
            .service(handlers::admin_user_info)
            .service(handlers::admin_set_user)
            .service(handlers::admin_set_star)
            .service(handlers::admin_ban_user)
            .service(handlers::admin_add_rope_package)
            .service(handlers::admin_delete_rope_package)
            .service(handlers::set_admin)
            .service(handlers::stats_downloads)
            .service(handlers::stats_api_counts)
            .service(handlers::get_users_db)
            .service(handlers::get_log_stats)
            .service(handlers::get_log_entries)
            .service(handlers::clear_logs)
    })
    .bind((server_config.server.host.as_str(), server_config.server.port))?
    .workers(server_config.server.workers)
    .run()
    .await?;

    log::info!("服务已停止");
    Ok(())
}