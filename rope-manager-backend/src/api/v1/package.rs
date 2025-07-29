use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use serde::Deserialize;
use crate::services::package_service::PackageService;
use crate::models::{CreatePackageRequest, UpdatePackageRequest};
use crate::services::comment_service::CommentService;
use crate::repositories::system_repo::SystemRepository;
use crate::require_auth;
use crate::utils::auth_helper::AuthHelper;

#[derive(Debug, Deserialize, Clone)]
pub struct PackageQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category_id: Option<i32>,
    pub search: Option<String>,
    pub status: Option<String>,
}

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
                web::resource("/{id}/review")
                    .route(web::post().to(review_resource))
            )
    );
}

// 审核资源请求结构
#[derive(Debug, Deserialize)]
pub struct ReviewResourceRequest {
    pub status: String,      // "approved" 或 "rejected"
    pub comment: Option<String>, // 审核备注
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
        reviewer_id: Some(user.id),
        reviewed_at: Some(chrono::Utc::now()),
        review_comment: req.comment.clone(),
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
    use crate::utils::auth_helper::AuthHelper;
    
    println!("[DEBUG] get_packages called with query: {:?}", query);
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
            println!("[ERROR] get_packages error: {}", e);
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

    Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": package
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
    
    // 验证URL格式
    if !req.file_url.starts_with("http://") && !req.file_url.starts_with("https://") {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "资源文件链接必须是有效的HTTP或HTTPS地址"
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
        file_url: Some(req.file_url.clone()),
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
    
    // 验证管理员权限
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    // 检查是否为管理员或元老
    if !matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "只有管理员和元老可以直接创建资源"
        })));
    }
    
    // 验证URL格式（如果提供了）
    if let Some(file_url) = &req.file_url {
        if !file_url.starts_with("http://") && !file_url.starts_with("https://") {
            return Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "资源文件链接必须是有效的HTTP或HTTPS地址"
            })));
        }
    }
    
    match package_service.create_package(&req).await {
        Ok(package) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "资源创建成功",
            "data": package
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": format!("创建失败: {}", e)
        })))
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
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match package_service.download_package(package_id).await {
        Ok(file_path) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": file_path
        }))),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": e.to_string()
        })))
    }
}

async fn upload_package_file(
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    // TODO: 实现文件上传逻辑
    match package_service.update_package_file(package_id).await {
        Ok(package) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "文件上传成功",
            "data": package
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
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