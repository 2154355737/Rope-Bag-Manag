use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use serde::Deserialize;
use crate::services::package_service::PackageService;
use crate::models::{CreatePackageRequest, UpdatePackageRequest};
use crate::services::comment_service::CommentService;
use crate::repositories::system_repo::SystemRepository;
use crate::require_auth;
use crate::utils::auth_helper::AuthHelper;
use futures_util::StreamExt;
use crate::services::user_action_service::UserActionService;
use crate::models::user_action::CreateUserActionRequest;


#[derive(Debug, Deserialize, Clone)]
pub struct PackageQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category_id: Option<i32>,
    pub search: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TopQuery { limit: Option<i32> }

#[derive(Debug, Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub file_url: String,
    pub cover_url: Option<String>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/packages")
            .service(
                web::resource("")
                    .route(web::get().to(get_packages))
            )
            .service(
                web::resource("/top-downloads")
                    .route(web::get().to(get_top_downloads))
            )
            .service(
                web::resource("/top-likes")
                    .route(web::get().to(get_top_likes))
            )
            // 审核相关路由 - 必须在 /{id} 之前定义
            .service(
                web::resource("/pending")
                    .route(web::get().to(get_pending_resources))
            )
            .service(
                web::resource("/categories")
                    .route(web::get().to(get_package_categories))
            )
            // 用户提交资源接口（普通用户使用，自动设置作者和待审核状态）
            .service(
                web::resource("/user-submit")
                    .route(web::post().to(user_submit_resource))
            )
            // 管理员创建资源接口（管理员/元老使用，可设置任意作者和状态）
            .service(
                web::resource("/admin-create")
                    .route(web::post().to(admin_create_package))
            )
            // 参数化路由放在最后
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_package))
                    .route(web::put().to(update_package))
                    .route(web::delete().to(delete_package))
            )
            .service(
                web::resource("/{id}/download")
                    .route(web::get().to(download_package))
            )
            .service(
                web::resource("/{id}/upload")
                    .route(web::post().to(upload_package_file))
            )
            // 获取包评论 /packages/{id}/comments
            .service(
                web::resource("/{id}/comments")
                    .route(web::get().to(get_package_comments))
            )
            .service(
                web::resource("/{id}/report")
                    .route(web::post().to(report_package))
            )
            .service(
                web::resource("/{id}/review")
                    .route(web::post().to(review_resource))
            )
            .service(
                web::resource("/{id}/like")
                    .route(web::post().to(like_package))
                    .route(web::delete().to(unlike_package))
            )
            .service(
                web::resource("/{id}/like-status")
                    .route(web::get().to(check_like_status))
            )
            .service(
                web::resource("/{id}/bookmark")
                    .route(web::post().to(favorite_package_handler))
                    .route(web::delete().to(unfavorite_package_handler))
            )
            .service(
                web::resource("/{id}/bookmark-status")
                    .route(web::get().to(check_favorite_status_handler))
            )
    );

    // 新增：/resources 别名，映射到与 /packages 相同的处理器
    cfg.service(
        web::scope("/resources")
            .service(
                web::resource("")
                    .route(web::get().to(get_packages))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_package))
                    .route(web::put().to(update_package))
                    .route(web::delete().to(delete_package))
            )
            .service(
                web::resource("/{id}/download")
                    .route(web::get().to(download_package))
            )
            .service(
                web::resource("/{id}/comments")
                    .route(web::get().to(get_package_comments))
                    .route(web::post().to(create_resource_comment))
            )
            .service(
                web::resource("/{id}/report")
                    .route(web::post().to(report_package))
            )
            .service(
                web::resource("/{id}/like")
                    .route(web::post().to(like_package))
                    .route(web::delete().to(unlike_package))
            )
            .service(
                web::resource("/{id}/like-status")
                    .route(web::get().to(check_like_status))
            )
            .service(
                web::resource("/{id}/bookmark")
                    .route(web::post().to(favorite_package_handler))
                    .route(web::delete().to(unfavorite_package_handler))
            )
            .service(
                web::resource("/{id}/bookmark-status")
                    .route(web::get().to(check_favorite_status_handler))
            )
    );
}

// 审核资源请求结构
#[derive(Debug, Deserialize)]
pub struct ReviewResourceRequest {
    pub status: String,      // "approved" 或 "rejected"
    pub comment: Option<String>, // 审核备注
}

// 新增：创建资源评论请求体
#[derive(Debug, Deserialize)]
struct CreateCommentBody { content: String, parent_id: Option<i32> }

// 新增：举报请求体
#[derive(Debug, Deserialize)]
struct ReportBody { reason: Option<String> }

// 新增：为资源创建评论（/resources/{id}/comments POST）
async fn create_resource_comment(
    http_req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<CreateCommentBody>,
    comment_service: web::Data<CommentService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let resource_id = path.into_inner();
    match comment_service.create_comment(
        user.id,
        "Package".to_string(),
        resource_id,
        body.content.clone(),
        body.parent_id,
    ).await {
        Ok(comment) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": comment}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

// 新增：举报资源（/packages/{id}/report、/resources/{id}/report）
async fn report_package(
    http_req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<ReportBody>,
    ua_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let id = path.into_inner();
    let ip = http_req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let ua = http_req.headers().get("User-Agent").and_then(|h| h.to_str().ok()).map(|s| s.to_string());
    let req = CreateUserActionRequest {
        user_id: Some(user.id),
        action_type: "Report".to_string(),
        target_type: Some("Package".to_string()),
        target_id: Some(id),
        details: body.reason.clone(),
        ip_address: ip,
        user_agent: ua,
    };
    match ua_service.log_user_action(&req).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
}

// 新增：收藏/取消收藏/状态
async fn favorite_package_handler(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let id = path.into_inner();
    match package_service.favorite_package(user.id, id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"favorite_count": count}}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

async fn unfavorite_package_handler(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let id = path.into_inner();
    match package_service.unfavorite_package(user.id, id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"favorite_count": count}}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

async fn check_favorite_status_handler(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let id = path.into_inner();
    match package_service.check_favorite_status(user.id, id).await {
        Ok(is_fav) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"favorited": is_fav}}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
}

async fn get_top_downloads(query: web::Query<TopQuery>, package_service: web::Data<PackageService>) -> Result<HttpResponse, actix_web::Error> {
    let limit = query.limit.unwrap_or(10).max(1).min(50);
    match package_service.top_by_downloads(limit).await {
        Ok(list) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"list": list}}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
}

async fn get_top_likes(query: web::Query<TopQuery>, package_service: web::Data<PackageService>) -> Result<HttpResponse, actix_web::Error> {
    let limit = query.limit.unwrap_or(10).max(1).min(50);
    match package_service.top_by_likes(limit).await {
        Ok(list) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"list": list}}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
}

// 审核资源（管理员和元老可用）
async fn review_resource(
    http_req: HttpRequest,
    path: web::Path<i32>,
    req: web::Json<ReviewResourceRequest>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    
    // 验证用户权限：管理员或元老
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    // 检查用户是否有审核权限（管理员或元老）
    if !matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员和元老可以审核资源"
        })));
    }
    
    let resource_id = path.into_inner();
    
    // 验证状态参数
    let new_status = match req.status.as_str() {
        "approved" => crate::models::PackageStatus::Active,
        "rejected" => crate::models::PackageStatus::Rejected,
        _ => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "无效的审核状态，只能是 approved 或 rejected"
            })));
        }
    };
    
    // 获取资源信息
    let package = match package_service.get_package_by_id(resource_id).await {
        Ok(Some(pkg)) => pkg,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "code": 404,
                "message": "资源不存在"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取资源失败: {}", e)
            })));
        }
    };
    
    // 检查资源是否处于待审核状态
    if !matches!(package.status, crate::models::PackageStatus::Pending) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "该资源不是待审核状态"
        })));
    }
    
    // 更新资源状态
    let update_req = crate::models::UpdatePackageRequest {
        name: None,
        version: None,
        description: None,
        category_id: None,
        status: Some(new_status),
        file_url: None,
        file_size: None,
        is_pinned: None,
        is_featured: None,
        reviewer_id: Some(user.id),
        reviewed_at: Some(chrono::Utc::now()),
        review_comment: req.comment.clone(),
        tags: None,
        // 新增字段
        screenshots: None,
        cover_image: None,
        requirements: None,
        included_files: None,
    };
    
    match package_service.update_package(resource_id, &update_req).await {
        Ok(updated_package) => {
            let status_text = if req.status == "approved" { "通过" } else { "拒绝" };
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": format!("资源审核{}", status_text),
                "data": updated_package
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("审核失败: {}", e)
        })))
    }
}

// 获取待审核资源列表（管理员和元老可用）
async fn get_pending_resources(
    http_req: HttpRequest,
    query: web::Query<PackageQueryParams>, // Changed from GetPackagesQuery to PackageQueryParams
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    use crate::utils::auth_helper::AuthHelper;
    
    // 验证用户权限：管理员或元老
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    // 检查用户是否有审核权限（管理员或元老）
    if !matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员和元老可以查看待审核资源"
        })));
    }
    
    // 强制设置状态为 pending（小写），与数据库中的值保持一致
    let mut modified_query = query.clone();
    modified_query.status = Some("pending".to_string());
    
    match package_service.get_packages_advanced(
        modified_query.page.unwrap_or(1),
        modified_query.page_size.unwrap_or(20),
        modified_query.category_id,
        modified_query.search.clone(),
        modified_query.status.clone(),
    ).await {
        Ok((packages, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": packages,
                "total": total,
                "page": modified_query.page.unwrap_or(1),
                "page_size": modified_query.page_size.unwrap_or(20)
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_packages(
    http_req: HttpRequest,
    package_service: web::Data<PackageService>,
    query: web::Query<PackageQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    
    log::debug!("🔍 get_packages called with query: {:?}", query);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    
    // 统一状态过滤：
    // 1. 如果查询参数中未显式指定 status，则默认为 "active"，无论用户角色如何；
    // 2. 如果指定了 status，则统一转换为小写后再传递到仓库层，避免大小写不一致导致无法匹配。
    let mut status_filter = query
        .status
        .clone()
        .map(|s| s.to_lowercase());

    if status_filter.is_none() {
        status_filter = Some("active".to_string());
    }
    
    // 使用高级搜索功能
    match package_service.get_packages_advanced(
        page,
        page_size,
        query.category_id,
        query.search.clone(),
        status_filter
    ).await {
        Ok((packages, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": packages,
                "total": total,
                "page": page,
                "pageSize": page_size,
                "totalPages": (total as f64 / page_size as f64).ceil() as u32
            }
        }))),
        Err(e) => {
            log::error!("❌ get_packages error: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
        }
    }
}

async fn get_package(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();

    // 获取资源
    let package_opt = match package_service.get_package_by_id(package_id).await {
        Ok(pkg) => pkg,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": e.to_string()
            })));
        }
    };

    let package = match package_opt {
        Some(p) => p,
        None => {
            return Ok(HttpResponse::NotFound().json(json!({
                "code": 404,
                "message": "绳包不存在"
            })));
        }
    };

    // 权限检查：普通用户/游客只能访问 Active 资源；管理员和元老可以访问任何资源
    let mut is_admin_or_elder = false;
    if let Ok(user) = AuthHelper::verify_user(&http_req) {
        if matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder) {
            is_admin_or_elder = true;
        }
    }

    if !is_admin_or_elder && !matches!(package.status, crate::models::PackageStatus::Active) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "资源未审核通过"
        })));
    }

    // 记录访问量（只有已审核的资源才记录）
    if matches!(package.status, crate::models::PackageStatus::Active) {
        let user_id = AuthHelper::verify_user(&http_req).ok().map(|u| u.id);
        let ip_address = http_req.connection_info().realip_remote_addr().map(|s| s.to_string());
        let user_agent = http_req.headers().get("User-Agent").and_then(|h| h.to_str().ok()).map(|s| s.to_string());
        
        // 异步记录访问，不影响响应速度
        let _ = package_service.record_view(package_id, user_id, ip_address, user_agent).await;
    }

    // 将package序列化为JSON，并添加 files 字段作为 included_files 的别名，便于前端兼容
    let mut pkg_value = serde_json::to_value(&package).unwrap_or_else(|_| json!({}));
    if let serde_json::Value::Object(ref mut map) = pkg_value {
        if !map.contains_key("files") {
            if let Some(ref included) = package.included_files {
                map.insert("files".to_string(), serde_json::to_value(included).unwrap_or_else(|_| json!([])));
            } else {
                map.insert("files".to_string(), json!([]));
            }
        }

        // 补充统计字段：view_count 和 comment_count
        if let Ok((views, comments)) = package_service.get_view_and_comment_counts(package_id).await {
            map.entry("view_count").or_insert(json!(views));
            map.entry("comment_count").or_insert(json!(comments));
        }
    }

    Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": pkg_value
    })))
}

// 普通用户提交资源接口
async fn user_submit_resource(
    http_req: HttpRequest,
    req: web::Json<CreateResourceRequest>,
    package_service: web::Data<PackageService>,
    system_repo: web::Data<SystemRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证
    let user = require_auth!(&http_req);
    
    // file_url现在是可选的，如果没有提供，表示将后续上传文件
    if !req.file_url.is_empty() && !req.file_url.starts_with("http://") && !req.file_url.starts_with("https://") {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "如果提供资源文件链接，必须是有效的HTTP或HTTPS地址"
        })));
    }
    
    // 根据分类名称查找分类ID
    let category_id = if let Some(category_name) = &req.category {
        match system_repo.get_categories().await {
            Ok(categories) => {
                categories.iter()
                    .find(|cat| cat.name == *category_name)
                    .map(|cat| cat.id)
            },
            Err(e) => {
                log::error!("获取分类失败: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // 创建资源记录请求，自动设置作者为当前用户
    let create_req = CreatePackageRequest {
        name: req.title.clone(),
        author: user.username.clone(), // 自动使用当前用户名
        version: None,
        description: req.description.clone(),
        category_id,
        file_url: if req.file_url.is_empty() { None } else { Some(req.file_url.clone()) },
        tags: req.tags.clone(),
        is_pinned: None,
        is_featured: None,
        // 新增字段
        screenshots: None,
        cover_image: None,
        requirements: None,
        included_files: None,
    };
    
    match package_service.create_package(&create_req).await {
        Ok(package) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "资源提交成功，等待审核",
            "data": package
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": format!("提交失败: {}", e)
        })))
    }
}

// 管理员创建资源接口
async fn admin_create_package(
    http_req: HttpRequest,
    req: web::Json<CreatePackageRequest>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    use crate::utils::auth_helper::AuthHelper;
    
    log::debug!("🔍 admin_create_package called with data: {:?}", req);
    
    // 验证管理员权限
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => {
            log::debug!("🔍 User verified: {:?}", user.username);
            user
        },
        Err(e) => {
            log::error!("❌ User verification failed: {:?}", e);
            return Ok(e.to_response());
        }
    };
    
    // 检查是否为管理员或元老
    if !matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder) {
        log::error!("❌ User role not allowed: {:?}", user.role);
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员和元老可以直接创建资源"
        })));
    }
    
    log::debug!("🔍 User role check passed");
    
    // 验证URL格式（如果提供了且不为空）
    if let Some(file_url) = &req.file_url {
        log::debug!("🔍 Checking file_url: '{}'", file_url);
        // 放宽URL验证：只要不为空就接受，可以是任意文本
        // 管理员可以输入任意形式的资源标识符
        if file_url.is_empty() {
            log::debug!("🔍 Empty URL, will be stored as empty string");
        } else {
            log::debug!("🔍 URL accepted: '{}'", file_url);
        }
    }
    
    log::debug!("🔍 URL validation passed, calling package_service.create_package");
    
    match package_service.create_package(&req).await {
        Ok(package) => {
            log::debug!("🔍 Package created successfully: {:?}", package.id);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "资源创建成功",
                "data": package
            })))
        },
        Err(e) => {
            log::error!("❌ Package creation failed: {}", e);
            log::error!("❌ Error details: {:?}", e);
            Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": format!("创建失败: {}", e)
            })))
        }
    }
}

async fn update_package(
    http_req: HttpRequest,
    path: web::Path<i32>,
    req: web::Json<UpdatePackageRequest>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();

    // 获取调用用户
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(u) => u,
        Err(e) => return Ok(e.to_response()),
    };

    // 获取资源，检查作者
    let package_opt = package_service.get_package_by_id(package_id).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(e.to_string())
    })?;

    let package = match package_opt {
        Some(p) => p,
        None => return Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "绳包不存在"
        }))),
    };

    let is_admin = matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder);
    let is_owner = package.author == user.username;

    // 权限校验
    if !is_admin && !is_owner {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "无权限更新该资源"
        })));
    }

    // 如果非管理员/元老更新资源，则强制重新进入待审核状态
    let mut override_req = req.into_inner();
    if !is_admin {
        override_req.status = Some(crate::models::PackageStatus::Pending);
    }

    match package_service.update_package(package_id, &override_req).await {
        Ok(package) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "绳包更新成功，等待审核",
            "data": package
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

async fn delete_package(
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match package_service.delete_package(package_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "绳包删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn download_package(
    req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    
    // 获取用户信息
    let user_id = crate::utils::auth_helper::AuthHelper::extract_user_id(&req);
    
    // 获取IP地址
    let connection_info = req.connection_info();
    let ip_address = connection_info.realip_remote_addr().unwrap_or("unknown");
    
    // 获取User-Agent
    let user_agent = req.headers().get("User-Agent").and_then(|v| v.to_str().ok());
    
    match package_service.download_package_with_security(
        package_id, 
        user_id, 
        ip_address, 
        user_agent
    ).await {
        Ok(file_path) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": file_path
        }))),
        Err(e) => {
            // 如果是安全检测阻止的下载，返回403状态码
            if e.to_string().contains("下载被阻止") {
                Ok(HttpResponse::Forbidden().json(json!({
                    "code": 403,
                    "message": e.to_string()
                })))
            } else {
                Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": e.to_string()
        })))
            }
        }
    }
}

async fn upload_package_file(
    http_req: HttpRequest,
    path: web::Path<i32>,
    mut payload: actix_multipart::Multipart,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证
    let user = require_auth!(&http_req);
    let package_id = path.into_inner();
    
    log::info!("📤 用户 {} 为包 {} 上传文件", user.username, package_id);
    
    // 检查包是否存在且用户有权限
    match package_service.get_package_by_id(package_id).await {
        Ok(Some(package)) => {
            // 检查权限：只有作者或管理员可以上传文件
            if package.author != user.username && !matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder) {
                return Ok(HttpResponse::Forbidden().json(json!({
                    "code": 403,
                    "message": "只有资源作者或管理员可以上传文件"
                })));
            }
        },
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "code": 404,
                "message": "资源不存在"
            })));
        },
        Err(e) => {
            log::error!("获取包信息失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取资源信息失败"
            })));
        }
    }
    
    let mut file_name = String::new();
    let mut file_data = Vec::new();
    
    // 处理multipart数据
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            log::error!("处理multipart字段失败: {}", e);
            actix_web::error::ErrorBadRequest("无效的文件数据")
        })?;
        
        let field_name = field.name().unwrap_or("").to_string();
        
        if field_name == "file" {
            file_name = field.content_disposition()
                .and_then(|cd| cd.get_filename())
                .unwrap_or("unknown")
                .to_string();
            
            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| {
                    log::error!("读取文件数据失败: {}", e);
                    actix_web::error::ErrorBadRequest("读取文件数据失败")
                })?;
                file_data.extend_from_slice(&data);
            }
        }
    }
    
    if file_data.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "没有接收到文件数据"
        })));
    }
    
    // 上传文件到存储服务
    match package_service.upload_package_file(package_id, &file_name, file_data).await {
        Ok(file_path) => {
            log::info!("📦 包 {} 文件上传成功: {}", package_id, file_path);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "文件上传成功",
                "data": {
                    "file_path": file_path,
                    "file_name": file_name
                }
            })))
        },
        Err(e) => {
            log::error!("文件上传失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("文件上传失败: {}", e)
            })))
        }
    }
}

async fn get_package_categories(
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    match package_service.get_categories().await {
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

// 获取包评论
async fn get_package_comments(
    path: web::Path<i32>,
    query: web::Query<crate::api::v1::comment::CommentQueryParams>,
    comment_service: web::Data<CommentService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);
    match comment_service.get_package_comments(package_id, page, size).await {
        Ok((comments, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
              "list": comments,
              "total": total,
              "page": page,
              "size": size
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 

async fn like_package(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let package_id = path.into_inner();
    match package_service.like_package(user.id, package_id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"like_count": count}}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

async fn unlike_package(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let package_id = path.into_inner();
    match package_service.unlike_package(user.id, package_id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"like_count": count}}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

async fn check_like_status(
    http_req: HttpRequest,
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let package_id = path.into_inner();
    match package_service.check_like_status(user.id, package_id).await {
        Ok(is_liked) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"liked": is_liked}}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
} 