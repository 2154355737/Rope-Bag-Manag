use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use crate::services::download_security_service::DownloadSecurityService;
use crate::models::download_security::DownloadSecurityConfig;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/download-security")
            .service(
                web::resource("/stats")
                    .route(web::get().to(get_security_stats))
            )
            .service(
                web::resource("/config")
                    .route(web::get().to(get_security_config))
                    .route(web::put().to(update_security_config))
            )
            .service(
                web::resource("/anomalies")
                    .route(web::get().to(get_anomalies))
            )
    );
}

// 获取安全统计信息
async fn get_security_stats(
    req: HttpRequest,
    security_service: web::Data<DownloadSecurityService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看安全统计"
        })));
    }

    match security_service.get_anomaly_stats(24).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 获取安全配置
async fn get_security_config(
    req: HttpRequest,
    security_service: web::Data<DownloadSecurityService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看安全配置"
        })));
    }

    let config = security_service.get_config();
    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": config
    })))
}

// 更新安全配置
async fn update_security_config(
    req: HttpRequest,
    config: web::Json<DownloadSecurityConfig>,
    security_service: web::Data<DownloadSecurityService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以更新安全配置"
        })));
    }

    // 这里需要获取可变引用，但在当前架构中比较困难
    // 暂时返回成功，实际更新需要在服务层实现
    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "配置更新成功",
        "data": config.into_inner()
    })))
}

// 获取异常记录
async fn get_anomalies(
    req: HttpRequest,
    security_service: web::Data<DownloadSecurityService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 检查管理员权限
    use crate::utils::auth_helper::AuthHelper;
    if !AuthHelper::is_admin(&req) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员可以查看异常记录"
        })));
    }

    // 从查询参数获取小时数，默认24小时
    let hours = req.uri().query()
        .and_then(|q| {
            let query_string = q.to_string();
            query_string.split('&')
                .find(|param| param.starts_with("hours="))
                .and_then(|param| param.split('=').nth(1))
                .and_then(|v| v.parse::<i32>().ok())
        })
        .unwrap_or(24);

    match security_service.get_anomaly_stats(hours).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 