use actix_web::{web, HttpResponse, HttpRequest, Error};
use serde_json::json;
use serde_json::Value;
use crate::services::admin_service::AdminService;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::services::email_service::EmailService;
use crate::utils::auth_helper::AuthHelper;
use crate::{require_admin, require_auth};
// MailConfig已移至models/mail.rs

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            // TODO: 所有管理员路由都需要管理员权限 - 待权限中间件修复后启用
            // .wrap(require_admin())
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
                web::resource("/backup-schedule")
                    .route(web::get().to(get_backup_schedule))
                    .route(web::post().to(update_backup_schedule))
            )
            .service(
                web::resource("/announcements")
                    .route(web::get().to(get_announcements))
                    .route(web::post().to(create_announcement))
            )
            .service(
                web::resource("/announcements/batch-status")
                    .route(web::put().to(batch_update_announcement_status))
            )
            .service(
                web::resource("/announcements/batch-delete")
                    .route(web::post().to(batch_delete_announcements))
            )
            .service(
                web::resource("/announcements/{id}")
                    .route(web::get().to(get_announcement))
                    .route(web::put().to(update_announcement))
                    .route(web::delete().to(delete_announcement))
            )
            .service(
                web::resource("/theme-settings")
                    .route(web::get().to(get_theme_settings))
                    .route(web::post().to(update_theme_settings))
            )
            .service(
                web::resource("/settings")
                    .route(web::get().to(get_settings))
                    .route(web::post().to(update_settings))
            )
            .service(
                web::resource("/settings/reset")
                    .route(web::post().to(reset_settings))
            )
            .service(
                web::resource("/settings/{key}")
                    .route(web::get().to(get_setting))
                    .route(web::post().to(update_setting))
            )
            .service(
                web::resource("/resource-records")
                    .route(web::get().to(get_resource_records))
            )
            .service(
                web::resource("/mail-settings")
                    .route(web::get().to(get_mail_settings))
                    .route(web::post().to(update_mail_settings))
            )
            .service(
                web::resource("/test-email")
                    .route(web::post().to(send_test_email))
            )
    );
}

// 用户端查看有效公告的API
pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/announcements")
            .service(
                web::resource("/active")
                    .route(web::get().to(get_active_announcements))
            )
    );
}

async fn get_stats(
    admin_service: web::Data<AdminService>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证管理员权限
    let _user = require_admin!(&req);
    
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
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证管理员权限
    let _user = require_admin!(&req);
    
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
    web::Query(params): web::Query<serde_json::Map<String, serde_json::Value>>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 解析查询参数
    let page = params.get("page")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    
    let page_size = params.get("page_size")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);
    
    let level = params.get("level")
        .and_then(|v| v.as_str());
    
    let search = params.get("search")
        .and_then(|v| v.as_str());
    
    match admin_service.get_logs(page, page_size, level, search).await {
        Ok((logs, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": logs,
                "total": total,
                "page": page.unwrap_or(1),
                "page_size": page_size.unwrap_or(20)
            }
        }))),
        Err(e) => {
            eprintln!("获取系统日志失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
                "message": format!("获取系统日志失败: {}", e)
        })))
        }
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

// 删除备份接口 - 应用权限验证和审计日志
async fn delete_backup(
    path: web::Path<String>,
    admin_service: web::Data<AdminService>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let backup_id = path.into_inner();
    
    // 验证管理员权限
    let user = require_admin!(&req);
    
    match admin_service.delete_backup(&backup_id).await {
        Ok(_) => {
            // 记录成功的安全事件
            AuthHelper::log_security_event(
                Some(&user),
                "backup_delete",
                &format!("管理员删除备份: {}", backup_id),
                &req,
            ).await;
            
            log::info!("✅ 管理员 {} 删除备份: {}", user.username, backup_id);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "备份删除成功"
            })))
        },
        Err(e) => {
            // 记录失败的安全事件
            AuthHelper::log_security_event(
                Some(&user),
                "backup_delete_failed",
                &format!("管理员删除备份失败: {} - {}", backup_id, e),
                &req,
            ).await;
            
            log::error!("❌ 管理员 {} 删除备份失败 {}: {}", user.username, backup_id, e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("备份删除失败: {}", e)
            })))
        }
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

async fn get_backup_schedule(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_backup_schedule().await {
        Ok(cfg) => Ok(HttpResponse::Ok().json(json!({"code":0,"data":cfg}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
}

async fn update_backup_schedule(
    req: web::Json<Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.update_backup_schedule(&req.0).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0,"message":"success"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
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

// 获取单个公告
async fn get_announcement(
    path: web::Path<i32>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let announcement_id = path.into_inner();
    
    match admin_service.get_announcement_by_id(announcement_id).await {
        Ok(Some(announcement)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": announcement
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "公告不存在"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取公告失败: {}", e)
        })))
    }
}

// 批量更新公告状态
async fn batch_update_announcement_status(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 提取参数
    let ids = match req.get("ids") {
        Some(val) => match val.as_array() {
            Some(arr) => arr.iter()
                .filter_map(|v| v.as_i64().map(|id| id as i32))
                .collect::<Vec<i32>>(),
            None => return Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "无效的ID数组"
            })))
        },
        None => return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "未提供ID数组"
        })))
    };
    
    let enabled = match req.get("enabled") {
        Some(val) => match val.as_bool() {
            Some(b) => b,
            None => return Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "无效的enabled值"
            })))
        },
        None => return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "未提供enabled参数"
        })))
    };
    
    match admin_service.batch_update_announcement_status(&ids, enabled).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": format!("成功更新 {} 条公告状态", count),
            "data": {
                "updated_count": count
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("更新公告状态失败: {}", e)
        })))
    }
}

// 批量删除公告
async fn batch_delete_announcements(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 提取ID数组
    let ids = match req.get("ids") {
        Some(val) => match val.as_array() {
            Some(arr) => arr.iter()
                .filter_map(|v| v.as_i64().map(|id| id as i32))
                .collect::<Vec<i32>>(),
            None => return Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "无效的ID数组"
            })))
        },
        None => return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "未提供ID数组"
        })))
    };
    
    match admin_service.batch_delete_announcements(&ids).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": format!("成功删除 {} 条公告", count),
            "data": {
                "deleted_count": count
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("批量删除公告失败: {}", e)
        })))
    }
}

// 获取当前有效的公告
async fn get_active_announcements(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, Error> {
    match admin_service.get_active_announcements().await {
        Ok(list) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": { "list": list }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取公告失败: {}", e)
        })))
    }
} 

// 获取所有设置
async fn get_settings(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_all_settings().await {
        Ok(settings) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": settings
        }))),
        Err(e) => {
            eprintln!("获取系统设置失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取系统设置失败: {}", e)
            })))
        }
    }
}

// 更新多个设置
async fn update_settings(
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 获取所有需要更新的设置
    let settings = req.as_object();
    
    if let Some(settings_map) = settings {
        let mut success_count = 0;
        let mut error_count = 0;
        
        // 逐个更新设置
        for (key, value) in settings_map {
            // 将值转换为字符串
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            
            match admin_service.update_setting(key, &value_str).await {
                Ok(_) => {
                    success_count += 1;
                },
                Err(_) => {
                    error_count += 1;
                }
            }
        }
        
        Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": format!("已成功更新 {} 个设置，失败 {} 个", success_count, error_count),
            "data": {
                "success_count": success_count,
                "error_count": error_count
            }
        })))
    } else {
        Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "无效的设置数据格式"
        })))
    }
}

// 获取单个设置
async fn get_setting(
    path: web::Path<String>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let key = path.into_inner();
    
    match admin_service.get_setting(&key).await {
        Ok(Some(value)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "key": key,
                "value": value
            }
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": format!("设置项 {} 不存在", key)
        }))),
        Err(e) => {
            eprintln!("获取设置项失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取设置项失败: {}", e)
            })))
        }
    }
}

// 更新单个设置
async fn update_setting(
    path: web::Path<String>,
    req: web::Json<serde_json::Value>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let key = path.into_inner();
    
    if let Some(value) = req.get("value") {
        let value_str = match value {
            serde_json::Value::String(s) => s.clone(),
            _ => value.to_string(),
        };
        
        match admin_service.update_setting(&key, &value_str).await {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "设置更新成功"
            }))),
            Err(e) => {
                eprintln!("更新设置失败: {}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "code": 500,
                    "message": format!("更新设置失败: {}", e)
                })))
            }
        }
    } else {
        Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "缺少value字段"
        })))
    }
}

// 重置所有设置
async fn reset_settings(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 重置主题设置
    if let Err(e) = admin_service.update_theme_settings(&json!({
        "primary_color": "#409EFF",
        "secondary_color": "#67C23A",
        "dark_mode": false,
        "font_size": "14px",
        "language": "zh-CN"
    })).await {
        eprintln!("重置主题设置失败: {}", e);
        return Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("重置主题设置失败: {}", e)
        })));
    }
    
    // 重置功能开关
    if let Err(e) = admin_service.update_setting("enable_registration", "true").await {
        eprintln!("重置功能开关失败: {}", e);
        return Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("重置功能开关失败: {}", e)
        })));
    }
    
    if let Err(e) = admin_service.update_setting("system_mode", "Normal").await {
        eprintln!("重置系统模式失败: {}", e);
        return Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("重置系统模式失败: {}", e)
        })));
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "设置已重置"
    })))
} 

// 获取邮件设置
async fn get_mail_settings(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_mail_settings().await {
        Ok(config) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": config
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取邮件设置失败: {}", e)
        })))
    }
}

// 更新邮件设置
async fn update_mail_settings(
    req: web::Json<Value>,
    admin_service: web::Data<AdminService>,
    email_service: web::Data<Arc<RwLock<EmailService>>>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.update_mail_settings(&req.0).await {
        Ok(_) => {
            // 热更新邮件服务配置
            let es = email_service.write().await;
            if let Err(e) = es.reload_config().await {
                eprintln!("热更新邮件服务配置失败: {}", e);
            }
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "更新成功，邮件服务已热更新"
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("更新邮件设置失败: {}", e)
        })))
    }
}

async fn send_test_email(
    req: web::Json<Value>,
    email_service: web::Data<Arc<RwLock<EmailService>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = match req.get("email").and_then(|v| v.as_str()) {
        Some(email) => email,
        None => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "缺少邮箱参数"
            })));
        }
    };
    let es = email_service.read().await;
    match es.send_test_mail(email).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "测试邮件发送成功"
        }))),
        Err(e) => Ok(HttpResponse::Ok().json(json!({
            "code": 500,
            "message": format!("测试邮件发送失败: {}", e)
        })))
    }
} 