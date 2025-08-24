use actix_web::{web, HttpRequest, HttpResponse};
use crate::infrastructure::database::repositories::NotificationRepository;
use crate::shared::utils::jwt as jwt_util;
use serde::Deserialize;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .route("/unread-count", web::get().to(unread_count))
            .route("", web::get().to(list))
            .route("/{id}/read", web::put().to(mark_read))
            .route("/{id}/read", web::post().to(mark_read))
            .route("/mark-all-read", web::post().to(mark_all_read))
            .route("/delete-read", web::delete().to(delete_read))
            .route("/{id}", web::delete().to(delete_one))
    );
}

#[derive(Deserialize)]
struct ListQuery { 
    page: Option<i64>, 
    #[serde(alias = "size")]
    pageSize: Option<i64>, 
    unreadOnly: Option<bool> 
}

async fn list(app: web::Data<crate::core::AppState>, req: HttpRequest, query: web::Query<ListQuery>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.pageSize.unwrap_or(20).clamp(1, 100);
    let unread_only = query.unreadOnly.unwrap_or(false);
    let repo = NotificationRepository::new(app.db.pool());
    let page_data = repo.list(uid, page, page_size, unread_only).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let list: Vec<serde_json::Value> = page_data.data.into_iter().map(|n| {
        serde_json::json!({
            "id": n.id,
            "user_id": n.user_id,
            "title": n.title,
            "content": n.content,
            "notif_type": n.r#type,
            "data": n.data,
            "is_read": n.read_at.is_some(),
            "created_at": n.created_at
        })
    }).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "data": {
            "list": list,
            "total": page_data.total,
            "page": page,
            "pageSize": page_size
        }
    })))
}

async fn mark_read(app: web::Data<crate::core::AppState>, req: HttpRequest, path: web::Path<i64>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let id = path.into_inner();
    let repo = NotificationRepository::new(app.db.pool());
    let _ = repo.mark_read(uid, id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已标记为已读"})))
}

async fn mark_all_read(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = NotificationRepository::new(app.db.pool());
    let affected = repo.mark_all_read(uid).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"全部标记为已读", "data": {"affected": affected}})))
}

async fn delete_read(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = NotificationRepository::new(app.db.pool());
    let deleted = repo.delete_read(uid).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"deleted_count": deleted}})))
}

async fn delete_one(app: web::Data<crate::core::AppState>, req: HttpRequest, path: web::Path<i64>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let id = path.into_inner();
    let repo = NotificationRepository::new(app.db.pool());
    let _ = repo.delete_by_id(uid, id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已删除"})))
}

async fn unread_count(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let mut user_id: Option<i64> = None;
    if let Some(hv) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = hv.to_str() {
            if let Some(t) = s.strip_prefix("Bearer ") {
                user_id = jwt_util::verify(t, &app.config.jwt);
            }
        }
    }
    let count = if let Some(uid) = user_id {
        let repo = NotificationRepository::new(app.db.pool());
        repo.unread_count(uid).await.unwrap_or(0)
    } else { 0 };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "data": {"count": count}
    })))
}

fn extract_uid(req: &HttpRequest, app: &crate::core::AppState) -> Result<i64, actix_web::Error> {
    if let Some(hv) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = hv.to_str() {
            if let Some(t) = s.strip_prefix("Bearer ") {
                if let Some(uid) = jwt_util::verify(t, &app.config.jwt) { return Ok(uid); }
            }
        }
    }
    Err(actix_web::error::ErrorUnauthorized("未登录"))
} 