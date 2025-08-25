use actix_web::{web, HttpResponse, HttpRequest};
use crate::core::domain::resource::ResourceSearchParams;
use crate::shared::utils::jwt as jwt_util;
use crate::infrastructure::database::repositories::CommentRepository;
use crate::core::domain::comment::{CreateCommentRequest, CommentResponse};
use serde::Deserialize;
use tracing::{info, warn, error};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .route("", web::get().to(list_resources))
            .route("", web::post().to(create_resource))
            .route("/{id}", web::get().to(get_resource))
            .route("/{id}/download", web::get().to(download_resource))
            .route("/{id}/like", web::post().to(like_resource))
            .route("/{id}/like", web::delete().to(unlike_resource))
            .route("/{id}/bookmark", web::post().to(bookmark_resource))
            .route("/{id}/bookmark", web::delete().to(unbookmark_resource))
            .route("/{id}/comments", web::get().to(list_resource_comments))
            .route("/{id}/comments", web::post().to(create_resource_comment))
    );
}

#[derive(Deserialize)]
struct PublishFileInfo { name: String, size: i64, #[serde(default)] r#type: Option<String> }

#[derive(Deserialize)]
struct CreateResourceReq {
    title: String,
    content: String,
    #[serde(default)] version: Option<String>,
    #[serde(default)] category: Option<String>,
    #[serde(default)] tags: Option<Vec<String>>,
    #[serde(default)] requirements: Option<Vec<String>>,
    #[serde(default)] files: Option<Vec<PublishFileInfo>>,
    #[serde(default)] screenshots: Option<Vec<PublishFileInfo>>,
}

fn slugify(input: &str) -> String {
    let mut s = input.to_lowercase();
    s = s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let mut out = String::with_capacity(s.len());
    let mut prev_dash = false;
    for ch in s.chars() {
        if ch == '-' { if !prev_dash { out.push('-'); prev_dash = true; } } else { out.push(ch); prev_dash = false; }
    }
    out.trim_matches('-').to_string()
}

async fn create_resource(
    app: web::Data<crate::core::AppState>,
    req: HttpRequest,
    payload: web::Json<CreateResourceReq>,
) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;

    let title = payload.title.trim();
    let name = slugify(title);
    let description = payload.content.trim().to_string();
    let version = payload.version.clone().unwrap_or_else(|| "1.0.0".to_string());
    let category_id = payload.category.as_deref().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let file_size = payload.files.as_ref().and_then(|v| v.first()).map(|f| f.size).unwrap_or(0);

    info!("用户 {} 开始创建资源: {}", uid, title);

    // 验证必填字段
    if title.is_empty() || description.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "code": 1,
            "message": "标题和描述不能为空"
        })));
    }

    // 开始事务处理
    let mut tx = app.db.pool().begin().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("开始事务失败: {}", e)))?;

    // 创建资源记录，但设置为草稿状态，等待文件验证
    let req_model = crate::core::domain::resource::CreateResourceRequest {
        title: title.to_string(),
        name,
        description,
        version,
        category_id,
        file_size,
        requirements: payload.requirements.clone(),
        tags: payload.tags.clone(),
        screenshots: None,
    };

    // 使用事务创建资源
    let resource_result = app.services.resource_service
        .create_resource_with_transaction(&mut tx, uid, req_model, String::new())
        .await;

    let resource_detail = match resource_result {
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
        return Err(actix_web::error::ErrorInternalServerError("创建资源失败"));
    }

    info!("资源创建成功，等待文件上传: {} (ID: {})", resource_detail.resource.title, resource_detail.resource.id);

    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {
        "id": resource_detail.resource.id,
        "title": resource_detail.resource.title,
        "status": format!("{:?}", resource_detail.resource.status).to_lowercase(),
        "created_at": resource_detail.resource.created_at,
    }})))
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

    // 序列化为可编辑的 JSON 值
    let mut v = serde_json::to_value(&result).unwrap_or(serde_json::json!({}));

    // 标准化 tags 与 requirements（JSON 字符串 -> 数组）
    if let Some(tags_str) = v.get("tags").and_then(|x| x.as_str()).map(|s| s.to_string()) {
        if let Ok(arr) = serde_json::from_str::<serde_json::Value>(&tags_str) {
            v["tags"] = arr;
        } else {
            v["tags"] = serde_json::json!([]);
        }
    }
    if let Some(req_str) = v.get("requirements").and_then(|x| x.as_str()).map(|s| s.to_string()) {
        if let Ok(arr) = serde_json::from_str::<serde_json::Value>(&req_str) {
            v["requirements"] = arr;
        } else {
            v["requirements"] = serde_json::json!([]);
        }
    }

    // 标准化 screenshots：JSON 字符串 -> [url]
    if let Some(sc_str) = v.get("screenshots").and_then(|x| x.as_str()).map(|s| s.to_string()) {
        let arr = serde_json::from_str::<Vec<String>>(&sc_str).unwrap_or_default();
        let urls: Vec<String> = arr
            .into_iter()
            .map(|s| if s.starts_with("/storage/") || s.starts_with("http") { s } else { format!("/storage/file/{}", s.trim_start_matches('/')) })
            .collect();
        v["screenshots"] = serde_json::json!(urls);
    }

    // 生成 files 数组（主包文件）
    let file_path_opt = v.get("file_path").and_then(|x| x.as_str()).map(|s| s.to_string());
    let file_size = v.get("file_size").and_then(|x| x.as_i64()).unwrap_or(0);
    if let Some(fp) = file_path_opt {
        if !fp.is_empty() {
            let url = format!("/storage/file/{}", fp.trim_start_matches('/'));
            // 推断文件名
            let title = v.get("title").and_then(|x| x.as_str()).unwrap_or("");
            let version = v.get("version").and_then(|x| x.as_str()).unwrap_or("");
            let ext = std::path::Path::new(&fp).extension().and_then(|e| e.to_str()).unwrap_or("zip");
            let fname = if !title.is_empty() && !version.is_empty() { format!("{}-{}.{}", title, version, ext) } else if !title.is_empty() { format!("{}.{}", title, ext) } else { format!("download.{}", ext) };
            v["files"] = serde_json::json!([
                {"name": fname, "type": ext, "size": file_size, "url": url, "file_path": fp}
            ]);
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({"code": 0, "data": v})))
}

async fn download_resource(
    app: web::Data<crate::core::AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let id = path.into_inner();
    let url = app.services.resource_service.prepare_download(id, uid).await
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": url})))
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