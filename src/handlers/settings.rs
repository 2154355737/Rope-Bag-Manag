use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{ApiResponse, SystemSettings};
use crate::utils::{parse_query_params, now_ts, record_api_call};
use crate::auth::admin_auth;
use crate::models::AppState;

// 获取系统设置
#[get("/api/settings")]
pub async fn get_settings(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = &data.data_manager;
    let start_time = now_ts();
    
    // 记录API调用
    record_api_call(
        req.query_string(),
        &data.stats,
        "get_settings",
        start_time,
        200,
        true,
        None,
    );
    
    // 检查管理员权限
    if !admin_auth(&req, &data.config, data_manager) {
        return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 1,
            msg: "需要管理员权限".to_string(),
            data: None
        });
    }
    
    // 创建默认系统设置
    let settings = SystemSettings {
        theme: crate::models::ThemeSettings {
            community_theme: "light".to_string(),
            admin_theme: "light".to_string(),
        },
        system_mode: crate::models::SystemMode::Normal,
        feature_flags: crate::models::FeatureFlags {
            enable_registration: true,
            enable_community: true,
            enable_upload: true,
            enable_comments: true,
            enable_qq_binding: true,
        },
        backend_config: crate::models::BackendConfig {
            proxy_address: "".to_string(),
            api_timeout: 30,
            max_upload_size: 100,
        },
        backup_settings: crate::models::BackupSettings {
            enable_auto_backup: false,
            backup_interval_hours: 24,
            backup_location: "./backups".to_string(),
            max_backup_files: 10,
        },
        global_announcement: crate::models::GlobalAnnouncement {
            id: 1,
            enabled: false,
            title: "".to_string(),
            content: "".to_string(),
            type_: crate::models::AnnouncementType::Info,
            start_time: "".to_string(),
            end_time: None,
            priority: 5,
            create_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        },
    };
    
    HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "获取系统设置成功".to_string(),
        data: Some(settings)
    })
}

// 更新系统设置
#[post("/api/settings")]
pub async fn update_settings(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
    settings: web::Json<SystemSettings>,
) -> impl Responder {
    let data_manager = &data.data_manager;
    let start_time = now_ts();
    
    // 记录API调用
    record_api_call(
        req.query_string(),
        &data.stats,
        "update_settings",
        start_time,
        200,
        true,
        None,
    );
    
    // 检查管理员权限
    if !admin_auth(&req, &data.config, data_manager) {
        return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 1,
            msg: "需要管理员权限".to_string(),
            data: None
        });
    }
    
    // 保存系统设置到文件
    match crate::utils::save_json("data/system_settings.json", &settings.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            code: 0,
            msg: "更新系统设置成功".to_string(),
            data: None
        }),
        Err(e) => {
            eprintln!("更新系统设置失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "更新系统设置失败".to_string(),
                data: None
            })
        }
    }
}

// 获取系统状态
#[get("/api/system-status")]
pub async fn get_system_status(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = &data.data_manager;
    let start_time = now_ts();
    
    // 记录API调用
    record_api_call(
        req.query_string(),
        &data.stats,
        "get_system_status",
        start_time,
        200,
        true,
        None,
    );
    
    // 检查管理员权限
    if !admin_auth(&req, &data.config, data_manager) {
        return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 1,
            msg: "需要管理员权限".to_string(),
            data: None
        });
    }
    
    // 获取系统状态信息
    let status = serde_json::json!({
        "system_mode": "Normal",
        "uptime": chrono::Utc::now().timestamp(),
        "version": "1.0.0",
        "features": {
            "registration": true,
            "community": true,
            "upload": true,
            "comments": true,
            "qq_binding": true
        }
    });
    
    HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "获取系统状态成功".to_string(),
        data: Some(status)
    })
}

// 检查功能开关
#[get("/api/check-feature")]
pub async fn check_feature(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let feature = match params.get("feature") {
        Some(f) => f,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少功能参数".to_string(),
            data: None
        }),
    };
    
    let start_time = now_ts();
    
    // 记录API调用
    record_api_call(
        req.query_string(),
        &data.stats,
        "check_feature",
        start_time,
        200,
        true,
        None,
    );
    
    // 根据功能名称返回开关状态
    let enabled = match feature.as_str() {
        "registration" => true,
        "community" => true,
        "upload" => true,
        "comments" => true,
        "qq_binding" => true,
        _ => false
    };
    
    HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "检查功能开关成功".to_string(),
        data: Some(enabled)
    })
} 