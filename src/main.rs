mod auth;
mod cmd;
mod config;
mod data_manager;
mod handlers;
mod logger;
mod middleware;
mod models;
mod storage;
mod utils;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use std::sync::Arc;
use crate::config::load_config;
use crate::models::{AppState, StatsData};
use crate::handlers::*;
use crate::handlers::user::{bind_qq, get_user_permissions, check_user_permission, set_user_role, star_package};
use crate::data_manager::DataManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();
    
    // 加载配置
    let config = load_config();
    
    // 创建数据管理器
    let data_manager = Arc::new(DataManager::new());
    
    // 创建应用状态
    let app_state = web::Data::new(AppState {
        config: config.clone(),
        data_manager: data_manager.clone(),
        limiter: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        global: Arc::new(std::sync::Mutex::new((0, 0))),
        stats: Arc::new(std::sync::Mutex::new(StatsData::default())),
    });

    println!("🚀 绳包管理器服务器启动中...");
    println!("📍 服务器地址: http://{}:{}", config.server.host, config.server.port);
    println!("🔧 工作线程数: {}", config.server.workers);
    println!("📊 限流状态: {}", if config.rate_limit.enabled { "启用" } else { "禁用" });

    // 启动HTTP服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_state.clone())
            
            // 认证相关路由
            .service(login)
            .service(register)
            
            // 用户相关路由
            .service(user_info)
            .service(sign_in)
            .service(change_password)
            .service(change_nickname)
            .service(nicknames)
            .service(bind_qq)
            .service(get_user_permissions)
            .service(check_user_permission)
            .service(set_user_role)
            .service(star_package)
            
            // 绳包相关路由
            .service(get_data_db)
            .service(download_rope_package)
            .service(update_rope_package)
            .service(delete_rope_package)
            .service(add_rope_package)
            
            // 管理员路由
            .service(admin_user_info)
            .service(admin_set_user)
            .service(admin_set_star)
            .service(admin_ban_user)
            .service(admin_add_rope_package)
            .service(admin_update_rope_package)
            .service(admin_delete_rope_package)
            .service(set_admin)
            .service(get_users_db)
            
            // 分类管理路由
            .service(get_categories)
            .service(add_category)
            .service(update_category)
            .service(delete_category)
            
            // 统计相关路由
            .service(get_dashboard_data)
            .service(stats_api_counts)
            .service(stats_downloads)
            .service(get_api_call_stats)
            .service(get_api_performance)
            .service(get_recent_calls)
            
            // 日志相关路由
            .service(get_log_stats)
            .service(get_log_entries)
            .service(clear_logs)
            
            // 设置相关路由
            .service(get_settings)
            .service(update_settings)
            .service(get_system_status)
            .service(check_feature)
            
            // 评论管理
            .service(
                web::scope("/comments")
                    .route("", web::get().to(handlers::comments::get_comments))
                    .route("", web::post().to(handlers::comments::create_comment))
                    .route("/{id}", web::get().to(handlers::comments::get_comment))
                    .route("/{id}", web::put().to(handlers::comments::update_comment))
                    .route("/{id}", web::delete().to(handlers::comments::delete_comment))
                    .route("/{id}/status", web::put().to(handlers::comments::update_comment_status))
                    .route("/{id}/reply", web::post().to(handlers::comments::reply_comment))
            )
            // 用户行为记录
            .service(
                web::scope("/user-actions")
                    .route("", web::get().to(handlers::user_actions::get_user_actions))
                    .route("", web::post().to(handlers::user_actions::create_user_action))
                    .route("/stats", web::get().to(handlers::user_actions::get_user_action_stats))
                    .route("/{id}", web::delete().to(handlers::user_actions::delete_user_action))
                    .route("/batch", web::delete().to(handlers::user_actions::batch_delete_user_actions))
                    .route("/export", web::get().to(handlers::user_actions::export_user_actions))
            )
            // 资源记录
            .service(
                web::scope("/resource-records")
                    .route("", web::get().to(handlers::resource_records::get_resource_records))
                    .route("", web::post().to(handlers::resource_records::create_resource_record))
                    .route("/stats", web::get().to(handlers::resource_records::get_resource_record_stats))
                    .route("/{id}", web::delete().to(handlers::resource_records::delete_resource_record))
                    .route("/batch", web::delete().to(handlers::resource_records::batch_delete_resource_records))
                    .route("/export", web::get().to(handlers::resource_records::export_resource_records))
            )
            // 备份记录
            .service(
                web::scope("/backup-records")
                    .route("", web::get().to(handlers::backup_records::get_backup_records))
                    .route("", web::post().to(handlers::backup_records::create_backup_record))
                    .route("/{id}/download", web::get().to(handlers::backup_records::download_backup))
                    .route("/{id}", web::delete().to(handlers::backup_records::delete_backup_record))
                    .route("/{id}/status", web::put().to(handlers::backup_records::update_backup_status))
                    .route("/stats", web::get().to(handlers::backup_records::get_backup_stats))
                    .route("/manual", web::post().to(handlers::backup_records::perform_manual_backup))
            )
            // 公告管理
            .service(
                web::scope("/announcements")
                    .route("", web::get().to(handlers::announcements::get_announcements))
                    .route("", web::post().to(handlers::announcements::create_announcement))
                    .route("/{id}", web::get().to(handlers::announcements::get_announcement))
                    .route("/{id}", web::put().to(handlers::announcements::update_announcement))
                    .route("/{id}", web::delete().to(handlers::announcements::delete_announcement))
                    .route("/active", web::get().to(handlers::announcements::get_active_announcements))
                    .route("/stats", web::get().to(handlers::announcements::get_announcement_stats))
            )
            
            // 健康检查
            .service(health_check)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .workers(config.server.workers)
    .run()
    .await
}

// 健康检查接口
#[actix_web::get("/health")]
async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}