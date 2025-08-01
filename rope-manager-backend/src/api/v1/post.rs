use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use crate::services::post_service::PostService;
use crate::models::{CreatePostRequest, UpdatePostRequest, PostQueryParams};
use crate::utils::auth_helper::AuthHelper;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(
                web::resource("")
                    .route(web::get().to(get_posts))
                    .route(web::post().to(create_post))
            )
            .service(
                web::resource("/featured")
                    .route(web::get().to(get_featured_posts))
            )
            .service(
                web::resource("/popular")
                    .route(web::get().to(get_popular_posts))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_post))
                    .route(web::put().to(update_post))
                    .route(web::delete().to(delete_post))
            )
            .service(
                web::resource("/{id}/tags")
                    .route(web::get().to(get_post_tags))
            )
            .service(
                web::resource("/{id}/view")
                    .route(web::post().to(increment_view_count))
            )
    );
}

// 获取帖子列表
async fn get_posts(
    _http_req: HttpRequest,
    query: web::Query<PostQueryParams>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    match post_service.get_posts(query.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": response
        }))),
        Err(e) => {
            log::error!("获取帖子列表失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取帖子列表失败",
                "msg": "获取帖子列表失败"
            })))
        }
    }
}

// 创建帖子
async fn create_post(
    http_req: HttpRequest,
    req: web::Json<CreatePostRequest>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户权限
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "未授权访问"
            })));
        }
    };

    match post_service.create_post(req.into_inner(), user.id).await {
        Ok(post_id) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "帖子创建成功",
            "data": {
                "post_id": post_id
            }
        }))),
        Err(e) => {
            log::error!("创建帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "创建帖子失败"
            })))
        }
    }
}

// 获取单个帖子
async fn get_post(
    _http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    
    match post_service.get_post(post_id).await {
        Ok(Some(post)) => {
            // 增加浏览量
            let _ = post_service.increment_view_count(post_id).await;
            
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "msg": "success",
                "data": post
            })))
        },
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "帖子不存在"
        }))),
        Err(e) => {
            log::error!("获取帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取帖子失败"
            })))
        }
    }
}

// 更新帖子
async fn update_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    req: web::Json<UpdatePostRequest>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    
    // 验证用户权限
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "未授权访问"
            })));
        }
    };

    // 检查帖子是否存在
    match post_service.get_post(post_id).await {
        Ok(Some(post)) => {
            // 检查是否是作者或管理员
            if post.author_id != user.id && user.role != crate::models::UserRole::Admin && user.role != crate::models::UserRole::Elder {
                return Ok(HttpResponse::Forbidden().json(json!({
                    "code": 403,
                    "message": "无权限修改此帖子"
                })));
            }
        },
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "code": 404,
                "message": "帖子不存在"
            })));
        },
        Err(e) => {
            log::error!("检查帖子失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "检查帖子失败"
            })));
        }
    }

    match post_service.update_post(post_id, req.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "帖子更新成功"
        }))),
        Err(e) => {
            log::error!("更新帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "更新帖子失败"
            })))
        }
    }
}

// 删除帖子
async fn delete_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    
    // 验证用户权限
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "未授权访问"
            })));
        }
    };

    // 检查帖子是否存在
    match post_service.get_post(post_id).await {
        Ok(Some(post)) => {
            // 检查是否是作者或管理员
            if post.author_id != user.id && user.role != crate::models::UserRole::Admin && user.role != crate::models::UserRole::Elder {
                return Ok(HttpResponse::Forbidden().json(json!({
                    "code": 403,
                    "message": "无权限删除此帖子"
                })));
            }
        },
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(json!({
                "code": 404,
                "message": "帖子不存在"
            })));
        },
        Err(e) => {
            log::error!("检查帖子失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "检查帖子失败"
            })));
        }
    }

    match post_service.delete_post(post_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "帖子删除成功"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "帖子不存在"
        }))),
        Err(e) => {
            log::error!("删除帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "删除帖子失败"
            })))
        }
    }
}

// 获取帖子标签
async fn get_post_tags(
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    
    match post_service.get_post_tags(post_id).await {
        Ok(tags) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": tags
        }))),
        Err(e) => {
            log::error!("获取帖子标签失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取帖子标签失败"
            })))
        }
    }
}

// 增加浏览量
async fn increment_view_count(
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    
    match post_service.increment_view_count(post_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "浏览量增加成功"
        }))),
        Err(e) => {
            log::error!("增加浏览量失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "增加浏览量失败"
            })))
        }
    }
}

// 获取精选帖子
async fn get_featured_posts(
    query: web::Query<PostQueryParams>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut query_params = query.into_inner();
    query_params.is_featured = Some(true);
    
    match post_service.get_posts(query_params).await {
        Ok(response) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": response
        }))),
        Err(e) => {
            log::error!("获取精选帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取精选帖子失败"
            })))
        }
    }
}

// 获取热门帖子
async fn get_popular_posts(
    query: web::Query<PostQueryParams>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut query_params = query.into_inner();
    // 这里可以根据需要调整排序逻辑
    query_params.page_size = Some(10);
    
    match post_service.get_posts(query_params).await {
        Ok(response) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "msg": "success",
            "data": response
        }))),
        Err(e) => {
            log::error!("获取热门帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取热门帖子失败"
            })))
        }
    }
} 