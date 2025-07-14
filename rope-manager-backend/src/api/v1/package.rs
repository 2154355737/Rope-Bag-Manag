use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::package_service::PackageService;
use crate::models::{CreatePackageRequest, UpdatePackageRequest};

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
    );
}

async fn get_packages(
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    match package_service.get_packages().await {
        Ok(packages) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": packages,
                "total": packages.len(),
                "page": 1,
                "size": packages.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_package(
    path: web::Path<i32>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match package_service.get_package(package_id).await {
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
            "message": "创建成功",
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
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "更新成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
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
            "message": "删除成功"
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
        Ok(file_path) => {
            // 这里应该返回文件下载响应
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "下载成功",
                "data": {
                    "file_path": file_path
                }
            })))
        },
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": e.to_string()
        })))
    }
} 