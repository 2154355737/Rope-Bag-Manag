use actix_web::{web, HttpResponse, HttpRequest};
use crate::core::services::PostService;
use crate::core::domain::post::PostSearchParams;
use crate::shared::utils::jwt as jwt_util;
use serde::Deserialize;
use tracing::{info, warn, error};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(list_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}/like", web::post().to(like_post))
            .route("/{id}/like", web::delete().to(unlike_post))
            .route("/{id}/bookmark", web::post().to(bookmark_post))
            .route("/{id}/bookmark", web::delete().to(unbookmark_post))
    );
}

#[derive(Deserialize)]
struct CreatePostReq {
    title: String,
    content: String,
    #[serde(default)] tags: Option<Vec<String>>,
    #[serde(default)] images: Option<Vec<String>>,
    #[serde(default)] code_snippet: Option<String>,
}

async fn create_post(
    app: web::Data<crate::core::AppState>,
    req: HttpRequest,
    payload: web::Json<CreatePostReq>,
) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    
    info!("用户 {} 开始创建帖子: {}", uid, payload.title);

    // 验证必填字段
    if payload.title.trim().is_empty() || payload.content.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "code": 1,
            "message": "标题和内容不能为空"
        })));
    }

    // 开始事务处理
    let mut tx = app.db.pool().begin().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("开始事务失败: {}", e)))?;

    let req_model = crate::core::domain::post::CreatePostRequest {
        title: payload.title.clone(),
        content: payload.content.clone(),
        tags: payload.tags.clone(),
        images: None, // 图片稍后通过文件上传接口处理
        code_snippet: payload.code_snippet.clone(),
    };

    // 使用事务创建帖子
    let post_result = app.services.post_service
        .create_post_with_transaction(&mut tx, uid, req_model)
        .await;

    let post_detail = match post_result {
        Ok(detail) => detail,
        Err(e) => {
            // 回滚事务
            if let Err(rollback_err) = tx.rollback().await {
                error!("事务回滚失败: {}", rollback_err);
            }
            return Err(actix_web::error::ErrorBadRequest(e.to_string()));
        }
    };

    // 提交事务
    if let Err(e) = tx.commit().await {
        error!("提交事务失败: {}", e);
        return Err(actix_web::error::ErrorInternalServerError("创建帖子失败"));
    }

    info!("帖子创建成功，等待图片上传: {} (ID: {})", post_detail.post.title, post_detail.post.id);

    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {
        "id": post_detail.post.id,
        "title": post_detail.post.title,
        "status": format!("{:?}", post_detail.post.status).to_lowercase(),
        "created_at": post_detail.post.created_at,
    }})))
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

    let mut v = serde_json::to_value(&result).unwrap_or(serde_json::json!({}));
    // 标准化 images：JSON 字符串 -> [url]
    if let Some(img_str) = v.get("images").and_then(|x| x.as_str()).map(|s| s.to_string()) {
        let arr = serde_json::from_str::<Vec<String>>(&img_str).unwrap_or_default();
        let urls: Vec<String> = arr
            .into_iter()
            .map(|s| if s.starts_with("/storage/") || s.starts_with("http") { s } else { format!("/storage/file/{}", s.trim_start_matches('/')) })
            .collect();
        v["images"] = serde_json::json!(urls);
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"code": 0, "data": v})))
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