use actix_web::{web, HttpResponse, Result, HttpRequest};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use crate::models::ApiResponse;
use crate::services::package_storage_service::{PackageStorageService, StorageStats, CleanupResult};
use crate::services::package_service::PackageService;
use crate::middleware::auth::AuthenticatedUser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::models::PackageFile;
use crate::services::post_service::PostService;
use crate::models::UpdatePostRequest;
use rusqlite::OptionalExtension;

#[derive(Deserialize)]
pub struct PresignRequest {
    pub filename: String,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub scope: Option<String>,
}

#[derive(Serialize)]
pub struct PresignResponseBody {
    #[serde(rename = "uploadUrl")]
    pub upload_url: String,
    #[serde(rename = "publicUrl")]
    pub public_url: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
}

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
    // 上传预签名（对齐前端约定，复用现有 /storage/upload）
    cfg.service(presign_upload);
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

// 统一的“预签名”接口：前端拿到 uploadUrl 后，直接以 multipart/form-data 向该地址上传
// 返回的 publicUrl 需在上传完成后由 /storage/upload 响应中的 download_url 获取
#[actix_web::post("/uploads/presign")]
async fn presign_upload(
    _req: web::Json<PresignRequest>,
    _auth_user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let body = PresignResponseBody {
        upload_url: "/api/v1/storage/upload".to_string(),
        public_url: None,
        headers: std::collections::HashMap::new(),
    };
    Ok(HttpResponse::Ok().json(ApiResponse::success(body)))
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
    let mut post_id: Option<i32> = None;

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
            "post_id" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    if let Ok(id_str) = std::str::from_utf8(&data) {
                        post_id = id_str.parse().ok();
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

    // 判断是否为截图上传
    let is_image = file_name.to_lowercase().ends_with(".jpg") ||
                  file_name.to_lowercase().ends_with(".jpeg") ||
                  file_name.to_lowercase().ends_with(".png") ||
                  file_name.to_lowercase().ends_with(".gif") ||
                  file_name.to_lowercase().ends_with(".webp");

    // 根据文件类型和package_id选择上传方法
    let upload_result = if is_image && package_id.is_some() {
        // 截图上传，使用专门的截图上传方法
        storage_service.upload_package_screenshot(
            &file_name,
            actix_web::web::Bytes::from(file_data),
            package_id.unwrap()
        ).await
    } else {
        // 普通文件上传
        storage_service.upload_package_file(
            &file_name,
            actix_web::web::Bytes::from(file_data),
            package_id
        ).await
    };

    match upload_result {
        Ok(result) => {
            // 根据上传类型更新Package/帖子信息
            if let Some(pkg_id) = package_id {
                let package_repo = crate::repositories::PackageRepository::new(db_path).unwrap();
                let package_service = PackageService::new(package_repo, "uploads".to_string());
                
                if let Ok(Some(package)) = package_service.get_package_by_id(pkg_id).await {
                    if is_image {
                        // 截图上传：更新screenshots字段
                        let mut screenshots = package.screenshots.unwrap_or_else(Vec::new);
                        screenshots.push(result.download_url.clone());
                        
                        let update_req = crate::models::UpdatePackageRequest {
                            name: None,
                            version: None,
                            description: None,
                            category_id: None,
                            status: None,
                            file_url: None,
                            file_size: None,
                            is_pinned: None,
                            is_featured: None,
                            reviewer_id: None,
                            reviewed_at: None,
                            review_comment: None,
                            tags: None,
                            screenshots: Some(screenshots),
                            cover_image: None,
                            requirements: None,
                            included_files: None,
                        };
                        
                        match package_service.update_package(pkg_id, &update_req).await {
                            Ok(_) => {
                                log::info!("📷 已将截图添加到资源 {} 的screenshots字段: {}", pkg_id, result.download_url);
                            },
                            Err(e) => {
                                log::error!("❌ 更新资源 {} 的screenshots字段失败: {}", pkg_id, e);
                            }
                        }
                    } else {
                        // 文件上传：更新file_url和file_size字段，并记录原始文件名到 included_files
                        // 推断文件类型
                        let ext = std::path::Path::new(&file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        let file_type = if ["zip","rar","7z","tar","gz"].contains(&ext.as_str()) {
                            "压缩包".to_string()
                        } else if ["exe","msi","dmg","pkg"].contains(&ext.as_str()) {
                            "安装程序".to_string()
                        } else if ["apk","ipa"].contains(&ext.as_str()) {
                            "移动应用".to_string()
                        } else if ["pdf","doc","docx","txt","md"].contains(&ext.as_str()) {
                            "文档".to_string()
                        } else {
                            "文件".to_string()
                        };

                        // 合并/追加到 included_files（按名称去重）
                        let mut included_files = package.included_files.unwrap_or_else(Vec::new);
                        if let Some(f) = included_files.iter_mut().find(|f| f.name == file_name) {
                            f.size = result.file_size;
                            f.file_type = file_type.clone();
                        } else {
                            included_files.push(PackageFile { name: file_name.clone(), size: result.file_size, file_type: file_type.clone() });
                        }

                        let update_req = crate::models::UpdatePackageRequest {
                            name: None,
                            version: None,
                            description: None,
                            category_id: None,
                            status: None,
                            file_url: Some(result.download_url.clone()),
                            file_size: Some(result.file_size),
                            is_pinned: None,
                            is_featured: None,
                            reviewer_id: None,
                            reviewed_at: None,
                            review_comment: None,
                            tags: None,
                            screenshots: None,
                            cover_image: None,
                            requirements: None,
                            included_files: Some(included_files),
                        };
                        
                        let _ = package_service.update_package(pkg_id, &update_req).await;
                        log::info!("📁 已将文件信息更新到资源 {} - 文件: {}, 下载地址: {}, 大小: {} bytes", 
                                 pkg_id, file_name, result.download_url, result.file_size);
                    }
                }
            }
            // 如果是帖子图片上传：将图片URL追加到 posts.images
            if post_id.is_some() && is_image {
                let pid = post_id.unwrap();
                let post_service = PostService::new(db_path.to_string());
                // 读取现有帖子
                if let Ok(Some(post)) = post_service.get_post(pid).await {
                    let mut imgs = post.images.unwrap_or_else(Vec::new);
                    imgs.push(result.download_url.clone());
                    let update_req = UpdatePostRequest {
                        title: None,
                        content: None,
                        category_id: None,
                        tags: None,
                        status: None,
                        is_pinned: None,
                        is_featured: None,
                        images: Some(imgs),
                        code_snippet: None,
                    };
                    match post_service.update_post(pid, update_req).await {
                        Ok(_) => log::info!("🖼 已将图片添加到帖子 {} 的 images: {}", pid, result.download_url),
                        Err(e) => log::error!("❌ 更新帖子 {} 的 images 失败: {}", pid, e),
                    }
                }
            }

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
            // 兼容按原始文件名请求：尝试在基础路径下查找最近上传的同名文件
            log::warn!("直接获取下载链接失败，尝试按文件名回退: {}", e);
            // 递归列出并匹配最近同名文件（包含年月目录）
            match storage_service.list_storage_file_paths().await {
                Ok(paths) => {
                    let name = std::path::Path::new(&req.file_path).file_name().and_then(|s| s.to_str()).unwrap_or("");
                    if name.is_empty() { 
                        log::error!("文件名为空，无法回退");
                    } else {
                        // 优先精确匹配文件名结尾
                        if let Some(path) = paths.into_iter().filter(|p| p.ends_with(name)).last() {
                            match storage_service.get_package_download_url(&path).await {
                                Ok(url) => {
                                    return Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                                        "download_url": url
                                    }))));
                                }
                                Err(e2) => log::error!("按文件名回退仍失败: {}", e2),
                            }
                        }
                    }
                }
                Err(e2) => log::error!("列出存储文件失败: {}", e2),
            }
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
    query: web::Query<std::collections::HashMap<String, String>>,
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

    // 可选路径
    let path_opt = query.get("path").map(|s| s.to_string());
    let path_ref = path_opt.as_deref();

    match storage_service.list_storage_files(path_ref).await {
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