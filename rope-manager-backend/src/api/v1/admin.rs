use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::admin_service::AdminService;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stats")
            .route(web::get().to(get_stats))
    )
    .service(
        web::resource("/categories")
            .route(web::get().to(get_categories))
    )
    .service(
        web::resource("/user-actions")
            .route(web::get().to(get_user_actions))
    );
}

async fn get_stats(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_stats().await {
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

async fn get_categories(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_categories().await {
        Ok(categories) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": categories
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_user_actions(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_user_actions().await {
        Ok(actions) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": actions
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 