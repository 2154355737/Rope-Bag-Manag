use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::notification_service::NotificationService;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::notification::NotificationQuery;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(list_notifications))
            .route("/unread-count", web::get().to(unread_count))
            .route("/{id}/read", web::post().to(mark_read))
            .route("/mark-all-read", web::post().to(mark_all_read))
    );
}

async fn list_notifications(q: web::Query<NotificationQuery>, svc: web::Data<NotificationService>, user: AuthenticatedUser) -> HttpResponse {
    match svc.list(user.id, q.into_inner()).await {
        Ok(list) => HttpResponse::Ok().json(json!({"code":0, "data": {"list": list}})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
}

async fn unread_count(svc: web::Data<NotificationService>, user: AuthenticatedUser) -> HttpResponse {
    match svc.unread_count(user.id).await {
        Ok(count) => HttpResponse::Ok().json(json!({"code":0, "data": {"count": count}})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
}

async fn mark_read(path: web::Path<i32>, svc: web::Data<NotificationService>, user: AuthenticatedUser) -> HttpResponse {
    let id = path.into_inner();
    match svc.mark_read(user.id, id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"code":0, "message":"ok"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
}

async fn mark_all_read(svc: web::Data<NotificationService>, user: AuthenticatedUser) -> HttpResponse {
    match svc.mark_all_read(user.id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"code":0, "message":"ok"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
} 