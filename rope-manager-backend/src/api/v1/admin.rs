use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use crate::services::admin_service::AdminService;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(
                web::resource("/stats")
                    .route(web::get().to(get_stats))
            )
            .service(
                web::resource("/user-stats")
                    .route(web::get().to(get_user_stats))
            )
            .service(
                web::resource("/categories")
                    .route(web::get().to(get_categories))
            )
            .service(
                web::resource("/user-actions")
                    .route(web::get().to(get_user_actions))
            )
            .service(
                web::resource("/logs")
                    .route(web::get().to(get_logs))
            )
            .service(
                web::resource("/backup")
                    .route(web::post().to(create_backup))
            )
            .service(
                web::resource("/backups")
                    .route(web::get().to(get_backups))
            )
            .service(
                web::resource("/backup/stats")
                    .route(web::get().to(get_backup_stats))
            )
            .service(
                web::resource("/backup/batch-delete")
                    .route(web::post().to(batch_delete_backups))
            )
            .service(
                web::resource("/backup/{backup_id}")
                    .route(web::get().to(get_backup_details))
                    .route(web::delete().to(delete_backup))
            )
            .service(
                web::resource("/backup/{backup_id}/download")
                    .route(web::get().to(download_backup))
            )
            .service(
                web::resource("/backup/{backup_id}/restore")
                    .route(web::post().to(restore_backup))
            )
            .service(
                web::resource("/announcements")
                    .route(web::get().to(get_announcements))
                    .route(web::post().to(create_announcement))
            )
            .service(
                web::resource("/announcements/{id}")
                    .route(web::put().to(update_announcement))
                    .route(web::delete().to(delete_announcement))
            )
            .service(
                web::resource("/theme-settings")
                    .route(web::get().to(get_theme_settings))
                    .route(web::put().to(update_theme_settings))
            )
            .service(
                web::resource("/resource-records")
                    .route(web::get().to(get_resource_records))
            )
    );
}

async fn get_stats(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_stats().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_user_stats(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_user_stats().await {
        Ok((total_users, active_users, total_actions)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "total_users": total_users,
                "active_users": active_users,
                "total_actions": total_actions
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_categories(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_categories().await {
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

async fn get_user_actions(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_user_actions().await {
        Ok(actions) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": actions
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_logs(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_logs().await {
        Ok(logs) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": logs,
                "total": logs.len(),
                "page": 1,
                "pageSize": logs.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 更新创建备份接口
async fn create_backup(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
    req_info: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // 从请求中提取参数
    let backup_type = req.get("backup_type").and_then(|v| v.as_str()).unwrap_or("Manual");
    let description = req.get("description").and_then(|v| v.as_str());
    
    // 从JWT中提取用户ID
    let user_id = crate::middleware::auth::extract_user_id(&req_info);
    
    match admin_service.create_backup(backup_type, description, user_id).await {
        Ok(backup_info) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "备份创建成功",
            "data": backup_info
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("备份创建失败: {}", e)
        })))
    }
}

// 更新获取备份列表接口
async fn get_backups(
    query: web::Query<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 提取分页参数
    let page = query.get("page").and_then(|v| v.as_u64()).map(|v| v as u32);
    let page_size = query.get("page_size").and_then(|v| v.as_u64()).map(|v| v as u32);
    
    match admin_service.get_backups(page, page_size).await {
        Ok((backups, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": backups,
                "total": total,
                "page": page.unwrap_or(1),
                "page_size": page_size.unwrap_or(10)
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 获取备份详情接口
async fn get_backup_details(
    path: web::Path<String>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let backup_id = path.into_inner();
    
    match admin_service.get_backup_details(&backup_id).await {
        Ok(backup_info) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": backup_info
        }))),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": format!("备份不存在或无法访问: {}", e)
        })))
    }
}

// 更新下载备份接口
async fn download_backup(
    path: web::Path<String>,
    admin_service: web::Data<AdminService>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let backup_id = path.into_inner();
    match admin_service.get_backup_download_path(&backup_id).await {
        Ok(file_path) => {
            // 获取文件名
            let filename = std::path::Path::new(&file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("backup.db");
            
            // 使用actix_files提供文件下载
            Ok(actix_files::NamedFile::open(&file_path)?
                .set_content_disposition(actix_web::http::header::ContentDisposition {
                    disposition: actix_web::http::header::DispositionType::Attachment,
                    parameters: vec![actix_web::http::header::DispositionParam::Filename(filename.to_string())],
                })
                .into_response(&req))
        },
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": e.to_string()
        })))
    }
}

// 更新删除备份接口
async fn delete_backup(
    path: web::Path<String>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let backup_id = path.into_inner();
    match admin_service.delete_backup(&backup_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "备份删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("备份删除失败: {}", e)
        })))
    }
}

// 批量删除备份接口
async fn batch_delete_backups(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 提取备份ID列表
    let backup_ids = req.get("backup_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();
    
    if backup_ids.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "未提供有效的备份ID列表"
        })));
    }
    
    match admin_service.batch_delete_backups(&backup_ids).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": format!("成功删除 {} 个备份", count)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("批量删除备份失败: {}", e)
        })))
    }
}

// 添加恢复备份接口
async fn restore_backup(
    path: web::Path<String>,
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let backup_id = path.into_inner();
    
    // 检查确认参数
    let confirm = req.get("confirm").and_then(|v| v.as_bool()).unwrap_or(false);
    if !confirm {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "恢复操作需要确认参数"
        })));
    }
    
    match admin_service.restore_backup(&backup_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "备份恢复成功，请重新启动服务以完成恢复"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("备份恢复失败: {}", e)
        })))
    }
}

// 添加备份统计接口
async fn get_backup_stats(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_backup_stats().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取备份统计失败: {}", e)
        })))
    }
}

async fn get_announcements(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_announcements().await {
        Ok(announcements) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": announcements,
                "total": announcements.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn create_announcement(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.create_announcement(&req).await {
        Ok(announcement) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "公告创建成功",
            "data": announcement
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

async fn update_announcement(
    path: web::Path<i32>,
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match admin_service.update_announcement(id, &req).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "公告更新成功"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

async fn delete_announcement(
    path: web::Path<i32>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match admin_service.delete_announcement(id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "公告删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_theme_settings(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_theme_settings().await {
        Ok(settings) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": settings
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn update_theme_settings(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.update_theme_settings(&req).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "主题设置更新成功"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

async fn get_resource_records(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_resource_records().await {
        Ok(records) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": records,
                "total": records.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 