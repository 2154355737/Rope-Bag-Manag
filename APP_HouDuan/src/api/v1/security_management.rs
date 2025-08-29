use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use crate::services::security_action_service::SecurityActionService;
use crate::models::download_security::SecurityConfig;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/security-management")
            .service(
                web::resource("/ip-bans")
                .route(web::get().to(get_ip_bans))
                .route(web::post().to(ban_ip))
                .route(web::delete().to(unban_ip))
            )
            .service(
                web::resource("/ip-whitelist")
                .route(web::get().to(get_ip_whitelist))
                .route(web::post().to(add_ip_to_whitelist))
                .route(web::delete().to(remove_ip_from_whitelist))
            )
            .service(
                web::resource("/ban-stats")
                .route(web::get().to(get_ban_stats))
            )
            .service(
                web::resource("/security-config")
                .route(web::get().to(get_security_config))
                .route(web::put().to(update_security_config))
            )
            .service(
                web::resource("/view-stats")
                .route(web::get().to(get_view_security_stats))
            )
    );
}

// 获取IP封禁列表
async fn get_ip_bans(
    req: HttpRequest,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看IP封禁列表"
        })));
    }

    match security_service.get_ip_bans().await {
        Ok(bans) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "bans": bans
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取IP封禁列表失败: {}", e)
        })))
    }
}

// 封禁IP
async fn ban_ip(
    req: HttpRequest,
    data: web::Json<serde_json::Value>,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以封禁IP"
        })));
    }

    let ip_address = data["ip_address"].as_str().unwrap_or("");
    let reason = data["reason"].as_str().unwrap_or("管理员手动封禁");
    let duration_hours = data["duration_hours"].as_i64().unwrap_or(24) as i32;

    if ip_address.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "IP地址不能为空"
        })));
    }

    match security_service.ban_ip(ip_address, reason, "high", Some(duration_hours)).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "IP封禁成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("封禁失败: {}", e)
        })))
    }
}

// 解除IP封禁
async fn unban_ip(
    req: HttpRequest,
    data: web::Json<serde_json::Value>,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以解除IP封禁"
        })));
    }

    let ip_address = data["ip_address"].as_str().unwrap_or("");
    let admin_name = AuthHelper::get_username(&req).unwrap_or_else(|| "admin".to_string());

    if ip_address.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "IP地址不能为空"
        })));
    }

    match security_service.unban_ip(ip_address, &admin_name).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "IP封禁解除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("解除封禁失败: {}", e)
        })))
    }
}

// 获取IP白名单
async fn get_ip_whitelist(
    req: HttpRequest,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看IP白名单"
        })));
    }

    match security_service.get_ip_whitelist().await {
        Ok(whitelist) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "whitelist": whitelist
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取IP白名单失败: {}", e)
        })))
    }
}

// 添加IP到白名单
async fn add_ip_to_whitelist(
    req: HttpRequest,
    data: web::Json<serde_json::Value>,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以管理IP白名单"
        })));
    }

    let ip_address = data["ip_address"].as_str().unwrap_or("");
    let description = data["description"].as_str().unwrap_or("管理员添加");
    let admin_name = AuthHelper::get_username(&req).unwrap_or_else(|| "admin".to_string());

    if ip_address.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "IP地址不能为空"
        })));
    }

    match security_service.add_ip_to_whitelist(ip_address, description, &admin_name).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "IP已添加到白名单"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("添加白名单失败: {}", e)
        })))
    }
}

// 从白名单移除IP
async fn remove_ip_from_whitelist(
    req: HttpRequest,
    data: web::Json<serde_json::Value>,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以管理IP白名单"
        })));
    }

    let ip_address = data["ip_address"].as_str().unwrap_or("");
    let admin_name = AuthHelper::get_username(&req).unwrap_or_else(|| "admin".to_string());

    if ip_address.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "IP地址不能为空"
        })));
    }

    match security_service.remove_ip_from_whitelist(ip_address, &admin_name).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "IP已从白名单移除"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("移除白名单失败: {}", e)
        })))
    }
}

// 获取封禁统计
async fn get_ban_stats(
    req: HttpRequest,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看封禁统计"
        })));
    }

    match security_service.get_ban_stats().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取统计失败: {}", e)
        })))
    }
}

// 获取安全配置
async fn get_security_config(
    req: HttpRequest,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看安全配置"
        })));
    }

    // 改为返回数据库优先的有效配置
    match security_service.load_effective_config().await {
        Ok(cfg) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": cfg
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 更新安全配置
async fn update_security_config(
    req: HttpRequest,
    config: web::Json<SecurityConfig>,
    security_service: web::Data<SecurityActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以更新安全配置"
        })));
    }

    // 持久化安全配置
    let cfg = config.into_inner();
    if let Err(e) = security_service.persist_config(cfg.clone()).await {
        return Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("配置保存失败: {}", e)
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "配置更新成功",
        "data": cfg
    })))
}

// 新增：获取视图安全统计（转发下载安全服务统计中的view部分）
async fn get_view_security_stats(
    req: HttpRequest,
    download_security: web::Data<crate::services::download_security_service::DownloadSecurityService>,
) -> Result<HttpResponse, actix_web::Error> {
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看"
        })));
    }

    match download_security.get_combined_stats(24).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats.get("view").cloned().unwrap_or(json!({}))
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 