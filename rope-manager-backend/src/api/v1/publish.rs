use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use crate::models::{
    PublishResourceRequest, PublishPostRequest, CreatePackageRequest, CreatePostRequest,
    PackageStatus, PostStatus,
};
use crate::services::package_service::PackageService;
use crate::services::post_service::PostService;
use crate::repositories::system_repo::SystemRepository;
use crate::require_auth;
use crate::utils::auth_helper::AuthHelper;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/publish")
            .service(
                web::resource("/resource")
                    .route(web::post().to(publish_resource))
            )
            .service(
                web::resource("/post")
                    .route(web::post().to(publish_post))
            )
    );
}

// 发布资源接口 - 对应前端发布页面的资源类型
async fn publish_resource(
    http_req: HttpRequest,
    req: web::Json<PublishResourceRequest>,
    package_service: web::Data<PackageService>,
    system_repo: web::Data<SystemRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证
    let user = require_auth!(&http_req);
    
    log::info!("📦 用户 {} 发布资源: {}", user.username, req.title);
    
    // 验证标题和内容
    if req.title.trim().is_empty() || req.content.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "标题和内容不能为空"
        })));
    }
    
    // 处理分类ID或分类名称
    let category_id = if let Some(category_str) = &req.category {
        // 首先尝试将其解析为ID（数字）
        if let Ok(id) = category_str.parse::<i32>() {
            Some(id)
        } else {
            // 如果不是数字，则按名称查找
            match system_repo.get_categories().await {
                Ok(categories) => {
                    categories.iter()
                        .find(|cat| cat.name == *category_str)
                        .map(|cat| cat.id)
                },
                Err(e) => {
                    log::error!("获取分类失败: {}", e);
                    None
                }
            }
        }
    } else {
        None
    };
    
    // 创建Package记录
    let create_req = CreatePackageRequest {
        name: req.title.clone(),
        author: user.username.clone(),
        version: req.version.clone(),
        description: Some(req.content.clone()),
        category_id,
        file_url: None, // 发布时文件为空，后续通过上传接口填充
        tags: req.tags.clone(),
        is_pinned: Some(false),
        is_featured: Some(false),
        // 新增字段
        screenshots: None, // 发布时截图为空，后续通过上传接口填充
        cover_image: None,
        requirements: req.requirements.clone(),
        included_files: None,
    };
    
    match package_service.create_package(&create_req).await {
        Ok(package) => {
            log::info!("✅ 资源发布成功: package_id={}, title={}", package.id, package.name);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "资源发布成功，等待审核",
                "data": {
                    "id": package.id,
                    "title": package.name,
                    "status": "pending",
                    "created_at": package.created_at
                }
            })))
        },
        Err(e) => {
            log::error!("❌ 资源发布失败: {}", e);
            Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": format!("发布失败: {}", e)
            })))
        }
    }
}

// 发布帖子接口 - 对应前端发布页面的帖子类型
async fn publish_post(
    http_req: HttpRequest,
    req: web::Json<PublishPostRequest>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证
    let user = require_auth!(&http_req);
    
    log::info!("📝 用户 {} 发布帖子: {}", user.username, req.title);
    
    // 验证标题和内容
    if req.title.trim().is_empty() || req.content.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "标题和内容不能为空"
        })));
    }
    
    // 创建Post记录：默认草稿，审核中
    let create_req = CreatePostRequest {
        title: req.title.clone(),
        content: req.content.clone(),
        category_id: None,
        tags: req.tags.clone(),
        status: Some(PostStatus::Draft),
        images: None, // 发布时不写入图片，占位使用 []
        code_snippet: req.code_snippet.clone(),
    };
    
    match post_service.create_post(create_req, user.id).await {
        Ok(post_id) => {
            // 读取完整帖子对象返回（包含 images/tags -> [] 而非 null）
            match post_service.get_post(post_id).await {
                Ok(Some(post)) => Ok(HttpResponse::Ok().json(json!({
                    "code": 0,
                    "message": "帖子已提交，等待审核",
                    "data": post
                }))),
                Ok(None) => Ok(HttpResponse::Ok().json(json!({
                    "code": 0,
                    "message": "帖子已提交，等待审核",
                    "data": {"id": post_id, "title": req.title, "status": "pending"}
                }))),
                Err(e) => {
                    log::warn!("读取帖子失败: {}", e);
                    Ok(HttpResponse::Ok().json(json!({
                        "code": 0,
                        "message": "帖子已提交，等待审核",
                        "data": {"id": post_id, "title": req.title, "status": "pending"}
                    })))
                }
            }
        },
        Err(e) => {
            log::error!("❌ 帖子发布失败: {}", e);
            Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": format!("发布失败: {}", e)
            })))
        }
    }
} 