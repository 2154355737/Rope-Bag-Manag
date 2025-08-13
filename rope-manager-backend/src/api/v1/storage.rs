use actix_web::{web, HttpResponse, Result, HttpRequest};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use crate::models::ApiResponse;
use crate::services::package_storage_service::{PackageStorageService, StorageStats, CleanupResult};
use crate::middleware::auth::AuthenticatedUser;
use serde::{Deserialize, Serialize};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/storage")
            .service(upload_file)
            .service(get_download_link)
            .service(delete_file)
            .service(list_files)
            .service(get_storage_stats)
            .service(cleanup_storage)
    );
}

#[derive(Deserialize)]
pub struct UploadRequest {
    pub package_id: Option<i32>,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub file_path: String,
    pub download_url: String,
    pub file_size: i64,
    pub message: String,
}

#[derive(Deserialize)]
pub struct FilePathRequest {
    pub file_path: String,
}

// 上传文件到AList存储 - 所有文件将被存储在 /image/结绳社区/ 目录下
#[actix_web::post("/upload")]
async fn upload_file(
    mut payload: Multipart,
    _auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let db_path = "data.db"; // 从配置中获取
    let mut storage_service = match PackageStorageService::new(db_path) {
        Ok(service) => service,
        Err(e) => {
            log::error!("创建存储服务失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, "存储服务初始化失败"
            )));
        }
    };

    let mut file_name = String::new();
    let mut file_data = Vec::new();
    let mut package_id: Option<i32> = None;

    // 处理multipart数据
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                file_name = field.content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("unknown")
                    .to_string();

                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    file_data.extend_from_slice(&data);
                }
            }
            "package_id" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    if let Ok(id_str) = std::str::from_utf8(&data) {
                        package_id = id_str.parse().ok();
                    }
                }
            }
            _ => {
                // 忽略其他字段
                while let Some(_chunk) = field.next().await {}
            }
        }
    }

    if file_data.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            400, "没有接收到文件数据"
        )));
    }

    // 上传文件到AList
    match storage_service.upload_package_file(
        &file_name,
        actix_web::web::Bytes::from(file_data),
        package_id
    ).await {
        Ok(result) => {
            let response = UploadResponse {
                file_path: result.file_path,
                download_url: result.download_url,
                file_size: result.file_size,
                message: "文件已成功上传到结绳社区目录".to_string(),
            };
            
            log::info!("📤 文件上传到结绳社区目录成功: {} ({} bytes)", file_name, result.file_size);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => {
            log::error!("文件上传失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("文件上传失败: {}", e)
            )))
        }
    }
}

// 获取文件下载链接
#[actix_web::post("/download-link")]
async fn get_download_link(
    req: web::Json<FilePathRequest>,
    _auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let db_path = "data.db";
    let mut storage_service = match PackageStorageService::new(db_path) {
        Ok(service) => service,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("存储服务初始化失败: {}", e)
            )));
        }
    };

    match storage_service.get_package_download_url(&req.file_path).await {
        Ok(download_url) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                "download_url": download_url
            }))))
        },
        Err(e) => {
            log::error!("获取下载链接失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取下载链接失败: {}", e)
            )))
        }
    }
}

// 删除文件
#[actix_web::post("/delete")]
async fn delete_file(
    req: web::Json<FilePathRequest>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    // 只有管理员可以删除文件
    if !auth_user.is_admin() {
        return Ok(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            403, "只有管理员可以删除文件"
        )));
    }

    let db_path = "data.db";
    let mut storage_service = match PackageStorageService::new(db_path) {
        Ok(service) => service,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("存储服务初始化失败: {}", e)
            )));
        }
    };

    match storage_service.delete_package_file(&req.file_path).await {
        Ok(_) => {
            log::info!("🗑️ 文件删除成功: {}", req.file_path);
            Ok(HttpResponse::Ok().json(ApiResponse::<()>::success_msg("文件删除成功")))
        },
        Err(e) => {
            log::error!("文件删除失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("文件删除失败: {}", e)
            )))
        }
    }
}

// 列出存储文件
#[actix_web::get("/files")]
async fn list_files(
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    // 只有管理员可以查看存储文件列表
    if !auth_user.is_admin() {
        return Ok(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            403, "只有管理员可以查看存储文件"
        )));
    }

    let db_path = "data.db";
    let mut storage_service = match PackageStorageService::new(db_path) {
        Ok(service) => service,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("存储服务初始化失败: {}", e)
            )));
        }
    };

    match storage_service.list_storage_files(None).await {
        Ok(files) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(files)))
        },
        Err(e) => {
            log::error!("获取文件列表失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取文件列表失败: {}", e)
            )))
        }
    }
}

// 获取存储统计信息
#[actix_web::get("/stats")]
async fn get_storage_stats(
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    // 只有管理员可以查看存储统计
    if !auth_user.is_admin() {
        return Ok(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            403, "只有管理员可以查看存储统计"
        )));
    }

    let db_path = "data.db";
    let mut storage_service = match PackageStorageService::new(db_path) {
        Ok(service) => service,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("存储服务初始化失败: {}", e)
            )));
        }
    };

    match storage_service.get_storage_stats().await {
        Ok(stats) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(stats)))
        },
        Err(e) => {
            log::error!("获取存储统计失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取存储统计失败: {}", e)
            )))
        }
    }
}

// 清理孤立文件
#[actix_web::post("/cleanup")]
async fn cleanup_storage(
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    // 只有管理员可以执行清理
    if !auth_user.is_admin() {
        return Ok(HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            403, "只有管理员可以执行存储清理"
        )));
    }

    let db_path = "data.db";
    let mut storage_service = match PackageStorageService::new(db_path) {
        Ok(service) => service,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("存储服务初始化失败: {}", e)
            )));
        }
    };

    match storage_service.cleanup_orphaned_files().await {
        Ok(result) => {
            log::info!("🧹 存储清理完成: 删除{}个文件，释放{}字节空间", 
                result.deleted_files, result.freed_space);
            Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
        },
        Err(e) => {
            log::error!("存储清理失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("存储清理失败: {}", e)
            )))
        }
    }
} 