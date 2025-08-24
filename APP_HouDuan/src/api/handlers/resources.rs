use actix_web::{web, HttpResponse, HttpRequest};
use crate::core::domain::resource::ResourceSearchParams;
use crate::shared::utils::jwt as jwt_util;
use crate::infrastructure::database::repositories::CommentRepository;
use crate::core::domain::comment::{CreateCommentRequest, CommentResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .route("", web::get().to(list_resources))
            .route("/{id}", web::get().to(get_resource))
            .route("/{id}/like", web::post().to(like_resource))
            .route("/{id}/like", web::delete().to(unlike_resource))
            .route("/{id}/bookmark", web::post().to(bookmark_resource))
            .route("/{id}/bookmark", web::delete().to(unbookmark_resource))
            .route("/{id}/comments", web::get().to(list_resource_comments))
            .route("/{id}/comments", web::post().to(create_resource_comment))
    );
}

async fn list_resources(
    svc: web::Data<crate::core::AppState>,
    q: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let params = ResourceSearchParams {
        query: q.get("q").and_then(|v| v.as_str()).map(|s| s.to_string()),
        category_id: q.get("categoryId").and_then(|v| v.as_i64()),
        tag: None,
        status: None,
        author_id: None,
        is_featured: None,
        page: q.get("page").and_then(|v| v.as_i64()).unwrap_or(1),
        page_size: q.get("pageSize").and_then(|v| v.as_i64()).unwrap_or(20),
    };
    let result = svc.services.resource_service.search_resources(params).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_resource(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let user_id = extract_uid_optional(&req, &svc);
    let result = svc.services.resource_service.get_resource_detail(id, user_id).await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}

async fn like_resource(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    let cnt = svc.services.resource_service.like_resource(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"like_count": cnt})))
}

async fn unlike_resource(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    let cnt = svc.services.resource_service.unlike_resource(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"like_count": cnt})))
}

async fn bookmark_resource(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    svc.services.resource_service.bookmark_resource(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已收藏"})))
}

async fn unbookmark_resource(
    svc: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &svc)?;
    let id = path.into_inner();
    svc.services.resource_service.unbookmark_resource(user_id, id).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已取消收藏"})))
}

async fn list_resource_comments(
    app: web::Data<crate::core::AppState>,
    path: web::Path<i64>,
    q: web::Query<serde_json::Value>,
) -> Result<HttpResponse, actix_web::Error> {
    let resource_id = path.into_inner();
    let page = q.get("page").and_then(|v| v.as_i64()).unwrap_or(1);
    let page_size = q.get("pageSize").and_then(|v| v.as_i64()).unwrap_or(20);
    let repo = CommentRepository::new(app.db.pool());
    let page_data = repo.list_by_target("package", resource_id, page, page_size).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": page_data})))
}

async fn create_resource_comment(
    app: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
    payload: web::Json<CreateCommentRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = extract_uid(&req, &app)?;
    let resource_id = path.into_inner();
    if payload.content.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1, "message":"内容不能为空"})))
    }
    let repo = CommentRepository::new(app.db.pool());
    let item = repo.create(user_id, "package", resource_id, &payload.content, payload.parent_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": item})))
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