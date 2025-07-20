use actix_web::{web, HttpResponse};
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
                web::resource("/backup/{backup_id}/download")
                    .route(web::get().to(download_backup))
            )
            .service(
                web::resource("/backup/{backup_id}")
                    .route(web::delete().to(delete_backup))
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

async fn create_backup(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.create_backup().await {
        Ok(backup_info) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "备份创建成功",
            "data": backup_info
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_backups(
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    match admin_service.get_backups().await {
        Ok(backups) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": backups
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn download_backup(
    path: web::Path<String>,
    admin_service: web::Data<AdminService>,
) -> Result<HttpResponse, actix_web::Error> {
    let backup_id = path.into_inner();
    match admin_service.download_backup(&backup_id).await {
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
            "message": e.to_string()
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