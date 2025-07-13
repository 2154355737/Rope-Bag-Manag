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
use crate::handlers::user::{bind_qq, get_user_permissions, check_user_permission, set_user_role};
use crate::data_manager::DataManager;
use crate::handlers::admin::get_users_db as admin_get_users_db;
use crate::handlers::admin::get_categories;

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
            .service(nicknames)
            .service(bind_qq)
            .service(get_user_permissions)
            .service(check_user_permission)
            .service(set_user_role)
            .service(delete_user)
            .service(edit_user)
            .service(batch_delete_users)
            .service(get_users)
            .service(add_user)
            
            // 绳包相关路由
            .service(get_data_db)
            .service(download_rope_package)
            .service(update_rope_package)
            .service(delete_rope_package)
            .service(add_rope_package)
            
            // 管理员路由
            .service(admin_user_info)
            .service(admin_set_star)
            .service(admin_ban_user)
            .service(admin_set_user)
            .service(admin_set_role)
            .service(set_admin)
            .service(admin_get_users_db)
            .service(get_categories)
            
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
            .service(handlers::comments::get_comments)
            .service(handlers::comments::create_comment)
            .service(handlers::comments::get_comment)
            .service(handlers::comments::delete_comment)
            .service(handlers::comments::update_comment_status)
            .service(handlers::comments::update_comment)
            .service(handlers::comments::reply_comment)
            // 用户行为记录
            .service(handlers::user_actions::get_user_actions)
            .service(handlers::user_actions::create_user_action)
            .service(handlers::user_actions::get_user_action_stats)
            .service(handlers::user_actions::delete_user_action)
            .service(handlers::user_actions::batch_delete_user_actions)
            .service(handlers::user_actions::export_user_actions)
            // 资源记录
            .service(handlers::resource_records::get_resource_records)
            .service(handlers::resource_records::create_resource_record)
            .service(handlers::resource_records::get_resource_record_stats)
            .service(handlers::resource_records::delete_resource_record)
            .service(handlers::resource_records::batch_delete_resource_records)
            .service(handlers::resource_records::export_resource_records)
            // 备份记录
            .service(handlers::backup_records::get_backup_records)
            .service(handlers::backup_records::create_backup_record)
            .service(handlers::backup_records::configure_auto_backup)
            .service(handlers::backup_records::delete_backup_record)
            .service(handlers::backup_records::batch_delete_backup_records)
            .service(
                web::scope("/api/backup-records")
                    .route("/{id}/download", web::get().to(handlers::backup_records::download_backup))
                    .route("/{id}/restore", web::post().to(handlers::backup_records::restore_backup))
                    .route("/{id}/status", web::put().to(handlers::backup_records::update_backup_status))
                    .route("/stats", web::get().to(handlers::backup_records::get_backup_stats))
                    .route("/manual", web::post().to(handlers::backup_records::perform_manual_backup))
            )
            // 公告管理
            .service(handlers::announcements::get_announcements)
            .service(handlers::announcements::create_announcement)
            .service(handlers::announcements::get_announcement)
            .service(handlers::announcements::update_announcement)
            .service(handlers::announcements::delete_announcement)
            .service(handlers::announcements::get_active_announcements)
            .service(handlers::announcements::get_announcement_stats)
            
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