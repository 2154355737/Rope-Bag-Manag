use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use crate::services::tag_service::TagService;
use crate::models::{CreateTagRequest, UpdateTagRequest, TagQueryParams};
use crate::utils::auth_helper::AuthHelper;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tags")
            .service(
                web::resource("")
                    .route(web::get().to(get_tags))
                    .route(web::post().to(create_tag))
            )
            .service(
                web::resource("/popular")
                    .route(web::get().to(get_popular_tags))
            )
            .service(
                web::resource("/all")
                    .route(web::get().to(get_all_tags))
            )
            .service(
                web::resource("/update-counts")
                    .route(web::post().to(update_tag_counts))
            )
            .service(
                web::resource("/stats")
                    .route(web::get().to(get_tag_stats))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_tag))
                    .route(web::put().to(update_tag))
                    .route(web::delete().to(delete_tag))
            )
    );
}

// 获取标签列表
async fn get_tags(
    query: web::Query<TagQueryParams>,
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    match tag_service.get_tags(query.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": response
        }))),
        Err(e) => {
            log::error!("获取标签列表失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取标签列表失败",
                "msg": "获取标签列表失败"
            })))
        }
    }
}

// 创建标签
async fn create_tag(
    http_req: HttpRequest,
    req: web::Json<CreateTagRequest>,
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户权限（只有管理员和元老可以创建标签）
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "未授权访问"
            })));
        }
    };

    if user.role != crate::models::UserRole::Admin && user.role != crate::models::UserRole::Elder {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "无权限创建标签"
        })));
    }

    match tag_service.create_tag(req.into_inner()).await {
        Ok(tag_id) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "标签创建成功",
            "msg": "标签创建成功",
            "data": {
                "tag_id": tag_id
            }
        }))),
        Err(e) => {
            log::error!("创建标签失败: {}", e);
            
            // 检查是否是唯一约束错误
            let error_message = if let rusqlite::Error::SqliteFailure(_, Some(msg)) = &e {
                if msg.contains("已存在") {
                    msg.clone()
                } else {
                    "创建标签失败".to_string()
                }
            } else if e.to_string().contains("UNIQUE constraint failed") {
                "标签名称已存在，请使用其他名称".to_string()
            } else {
                "创建标签失败".to_string()
            };
            
            Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": error_message,
                "msg": error_message
            })))
        }
    }
}

// 获取单个标签
async fn get_tag(
    path: web::Path<i32>,
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    let tag_id = path.into_inner();
    
    match tag_service.get_tag(tag_id).await {
        Ok(Some(tag)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": tag
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "标签不存在"
        }))),
        Err(e) => {
            log::error!("获取标签失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取标签失败"
            })))
        }
    }
}

// 更新标签
async fn update_tag(
    http_req: HttpRequest,
    path: web::Path<i32>,
    req: web::Json<UpdateTagRequest>,
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    let tag_id = path.into_inner();
    
    // 验证用户权限（只有管理员和元老可以更新标签）
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "未授权访问"
            })));
        }
    };

    if user.role != crate::models::UserRole::Admin && user.role != crate::models::UserRole::Elder {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "无权限更新标签"
        })));
    }

    match tag_service.update_tag(tag_id, req.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "标签更新成功",
            "msg": "标签更新成功"
        }))),
        Err(e) => {
            log::error!("更新标签失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "更新标签失败",
                "msg": "更新标签失败"
            })))
        }
    }
}

// 删除标签
async fn delete_tag(
    http_req: HttpRequest,
    path: web::Path<i32>,
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    let tag_id = path.into_inner();
    
    // 验证用户权限（只有管理员可以删除标签）
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "未授权访问"
            })));
        }
    };

    if user.role != crate::models::UserRole::Admin {
        return Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "无权限删除标签"
        })));
    }

    match tag_service.delete_tag(tag_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "标签删除成功",
            "msg": "标签删除成功"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "标签不存在",
            "msg": "标签不存在"
        }))),
        Err(e) => {
            log::error!("删除标签失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "删除标签失败",
                "msg": "删除标签失败"
            })))
        }
    }
}

// 获取热门标签
async fn get_popular_tags(
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    match tag_service.get_popular_tags(Some(20)).await {
        Ok(tags) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": tags
        }))),
        Err(e) => {
            log::error!("获取热门标签失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取热门标签失败"
            })))
        }
    }
}

// 获取所有标签（用于下拉选择）
async fn get_all_tags(
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    match tag_service.get_all_tags().await {
        Ok(tags) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": tags
        }))),
        Err(e) => {
            log::error!("获取所有标签失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取所有标签失败"
            })))
        }
    }
}

// 更新标签使用次数
async fn update_tag_counts(
    http_req: HttpRequest,
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证管理员权限
    let _user = match AuthHelper::require_admin(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };

    match tag_service.update_all_tag_counts().await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "标签使用次数更新成功",
            "msg": "标签使用次数更新成功"
        }))),
        Err(e) => {
            log::error!("更新标签使用次数失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "更新标签使用次数失败"
            })))
        }
    }
}

// 获取标签使用统计
async fn get_tag_stats(
    tag_service: web::Data<TagService>,
) -> Result<HttpResponse, actix_web::Error> {
    match tag_service.get_tag_usage_stats().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": stats
        }))),
        Err(e) => {
            log::error!("获取标签统计失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取标签统计失败"
            })))
        }
    }
} 