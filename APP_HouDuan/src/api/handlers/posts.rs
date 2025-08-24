use actix_web::{web, HttpResponse, HttpRequest};
use crate::core::services::PostService;
use crate::core::domain::post::PostSearchParams;
use crate::shared::utils::jwt as jwt_util;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(list_posts))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}/like", web::post().to(like_post))
            .route("/{id}/like", web::delete().to(unlike_post))
            .route("/{id}/bookmark", web::post().to(bookmark_post))
            .route("/{id}/bookmark", web::delete().to(unbookmark_post))
    );
}

async fn list_posts(
    svc: web::Data<crate::core::AppState>,
    q: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let params = PostSearchParams {
        query: q.get("q").and_then(|v| v.as_str()).map(|s| s.to_string()),
        tag: q.get("tag").and_then(|v| v.as_str()).map(|s| s.to_string()),
        status: None,
        author_id: None,
        is_featured: None,
        page: q.get("page").and_then(|v| v.as_i64()).unwrap_or(1),
        page_size: q.get("pageSize").and_then(|v| v.as_i64()).unwrap_or(20),
    };
    let result = svc.services.post_service.search_posts(params).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_post(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let user_id = extract_uid_optional(&req, &svc);
    let result = svc.services.post_service.get_post_detail(id, user_id).await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}

async fn like_post(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    let cnt = svc.services.post_service.like_post(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"like_count": cnt})))
}

async fn unlike_post(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    let cnt = svc.services.post_service.unlike_post(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"like_count": cnt})))
}

async fn bookmark_post(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    svc.services.post_service.bookmark_post(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已收藏"})))
}

async fn unbookmark_post(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    svc.services.post_service.unbookmark_post(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已取消收藏"})))
}

fn extract_uid_optional(req: &HttpRequest, app: &crate::core::AppState) -> Option<i64> {
    if let Some(h) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = h.to_str() {
            if let Some(t) = s.strip_prefix("Bearer ") {
                return jwt_util::verify(t, &app.config.jwt);
            }
        }
    }
    None
}

fn extract_uid(req: &HttpRequest, app: &crate::core::AppState) -> Result<i64, actix_web::Error> {
    extract_uid_optional(req, app).ok_or_else(|| actix_web::error::ErrorUnauthorized("未登录"))
} 