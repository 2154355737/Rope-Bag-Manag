use actix_web::{web, HttpResponse};
use serde_json::json;
use serde::Deserialize;
use crate::services::package_service::PackageService;
use crate::models::{CreatePackageRequest, UpdatePackageRequest};

#[derive(Debug, Deserialize)]
pub struct PackageQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category_id: Option<i32>,
    pub search: Option<String>,
    pub status: Option<String>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_packages))
            .route(web::post().to(create_package))
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
        web::resource("/{id}/upload")
            .route(web::post().to(upload_package_file))
    )
    .service(
        web::resource("/categories")
            .route(web::get().to(get_package_categories))
    );
}

async fn get_packages(
    package_service: web::Data<PackageService>,
    query: web::Query<PackageQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("[DEBUG] get_packages called with query: {:?}", query);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    
    // 使用高级搜索功能
    match package_service.get_packages_advanced(
        page,
        page_size,
        query.category_id,
        query.search.clone(),
        query.status.clone()
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
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match package_service.get_package_by_id(package_id).await {
        Ok(Some(package)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": package
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "绳包不存在"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn create_package(
    req: web::Json<CreatePackageRequest>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    match package_service.create_package(&req).await {
        Ok(package) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "绳包创建成功",
            "data": package
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

async fn update_package(
    path: web::Path<i32>,
    req: web::Json<UpdatePackageRequest>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match package_service.update_package(package_id, &req).await {
        Ok(package) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "绳包更新成功",
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