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
                web::resource("/pending")
                    .route(web::get().to(get_pending_posts))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_post))
                    .route(web::put().to(update_post))
                    .route(web::delete().to(delete_post))
            )
            .service(
                web::resource("/{id}/review")
                    .route(web::post().to(review_post))
            )
            .service(
                web::resource("/{id}/tags")
                    .route(web::get().to(get_post_tags))
            )
            .service(
                web::resource("/{id}/view")
                    .route(web::post().to(increment_view_count))
            )
            .service(
                web::resource("/{id}/like")
                    .route(web::post().to(like_post))
                    .route(web::delete().to(unlike_post))
            )
            .service(
                web::resource("/{id}/like-status")
                    .route(web::get().to(check_like_status))
            )
            .service(
                web::resource("/{id}/comments")
                    .route(web::get().to(get_post_comments))
            )
            .service(
                web::resource("/{id}/bookmark")
                    .route(web::post().to(bookmark_post))
                    .route(web::delete().to(unbookmark_post))
            )
            .service(
                web::resource("/{id}/bookmark-status")
                    .route(web::get().to(check_bookmark_status))
            )
            .service(
                web::resource("/{id}/report")
                    .route(web::post().to(report_post))
            )
    );
}

// 获取帖子列表
async fn get_posts(
    _http_req: HttpRequest,
    query: web::Query<PostQueryParams>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut q = query.into_inner();
    // 默认仅显示已发布 + 审核通过的帖子
    if q.status.is_none() { q.status = Some("Published".to_string()); }
    match post_service.get_posts(q).await {
        Ok(response) => {
            // 在API层统一补充作者头像/author_detail
            let mut enriched: Vec<serde_json::Value> = Vec::with_capacity(response.list.len());
            if let Ok(user_repo) = crate::repositories::user_repo::UserRepository::new("data.db") {
                for p in &response.list {
                    let mut v = serde_json::to_value(p).unwrap_or_else(|_| json!({}));
                    if let serde_json::Value::Object(ref mut map) = v {
                        if let Ok(Some(u)) = user_repo.find_by_id(p.author_id).await {
                            let name = u.nickname.clone().unwrap_or(u.username.clone());
                            let avatar = u.avatar_url.clone().unwrap_or_default();
                            map.insert("author_name".to_string(), json!(name));
                            map.insert("author_avatar".to_string(), json!(avatar));
                            map.insert("author_detail".to_string(), json!({"name": name, "avatar": avatar}));
                        }
                    }
                    enriched.push(v);
                }
            } else {
                enriched = response.list.into_iter().map(|p| serde_json::to_value(p).unwrap_or_else(|_| json!({}))).collect();
            }

            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "msg": "success",
                "data": {
                    "list": enriched,
                    "total": response.total,
                    "page": response.page,
                    "size": response.size
                }
            })))
        },
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
    http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    
    match post_service.get_post(post_id).await {
        Ok(Some(mut post)) => {
            // 非管理员访问未审核通过的帖子：返回403
            let is_admin = AuthHelper::is_admin(&http_req);
            if !is_admin {
                if let Some(rs) = &post.review_status { if rs != "approved" { 
                    return Ok(HttpResponse::Forbidden().json(json!({ "code": 403, "message": "帖子未审核通过" })));
                }}
            }
            // 增加浏览量
            let _ = post_service.increment_view_count(post_id).await;
            // 绝对化作者头像与 images
            let cfg = crate::config::Config::load().unwrap_or_default();
            let mut base_prefix = cfg.public_base_url().map(|s| s.trim_end_matches('/').to_string());
            if base_prefix.is_none() {
                let ci = http_req.connection_info();
                let scheme = ci.scheme();
                let host = ci.host();
                if !host.is_empty() { base_prefix = Some(format!("{}://{}", scheme, host).trim_end_matches('/').to_string()); }
            }
            if let Some(bp) = base_prefix.as_ref() {
                // 目前 Post 结构不含作者头像字段，这里仅处理 images[]
                if let Some(ref mut images) = post.images {
                    for url in images.iter_mut() {
                        if !url.starts_with("http://") && !url.starts_with("https://") && url.starts_with("/uploads/") {
                            *url = format!("{}/{}", bp, url.trim_start_matches('/'));
                        }
                    }
                }
            }
            
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

#[derive(serde::Deserialize)]
struct ReviewRequest { status: String, comment: Option<String> }

// 审核帖子（管理员/元老）
async fn review_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    req: web::Json<ReviewRequest>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 仅管理员/元老
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    if user.role != crate::models::UserRole::Admin && user.role != crate::models::UserRole::Elder {
        return Ok(HttpResponse::Forbidden().json(json!({ "code": 403, "message": "权限不足" })));
    }
    let post_id = path.into_inner();
    let status = req.status.to_lowercase();
    let allowed = status == "approved" || status == "rejected";
    if !allowed { return Ok(HttpResponse::BadRequest().json(json!({"code":400,"message":"无效状态"})));
    }

    // 同步业务状态：通过=>Published，拒绝=>Draft（保持为草稿）
    use rusqlite::{Connection, params};
    let conn = Connection::open(post_service.db_path()).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let new_business_status = if status == "approved" { "Published" } else { "Draft" };
    conn.execute(
        "UPDATE posts SET review_status = ?, review_comment = ?, reviewer_id = ?, reviewed_at = CURRENT_TIMESTAMP, status = ? WHERE id = ?",
        params![status, req.comment.clone().unwrap_or_default(), user.id, new_business_status, post_id]
    ).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(json!({"code":0, "message":"审核成功"})))
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

async fn like_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let post_id = path.into_inner();
    match post_service.like_post(user.id, post_id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"like_count": count}}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

async fn unlike_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let post_id = path.into_inner();
    match post_service.unlike_post(user.id, post_id).await {
        Ok(count) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"like_count": count}}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400, "message": e.to_string()})))
    }
}

// 获取待审核帖子（管理员/元老）
async fn get_pending_posts(
    http_req: HttpRequest,
    query: web::Query<PostQueryParams>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证权限：仅管理员/元老可以查看待审核帖子
    let user = match AuthHelper::verify_user(&http_req) { 
        Ok(u) => u, 
        Err(e) => return Ok(e.to_response()) 
    };
    
    if user.role != crate::models::UserRole::Admin && user.role != crate::models::UserRole::Elder {
        return Ok(HttpResponse::Forbidden().json(json!({ 
            "code": 403, 
            "message": "权限不足：只有管理员和元老可以查看待审核帖子" 
        })));
    }

    // 修改查询参数，只获取待审核状态的帖子
    let mut params = query.into_inner();
    params.status = Some("pending".to_string()); // 设置为待审核状态
    
    match post_service.get_posts(params).await {
        Ok(posts_response) => {
            log::info!("✅ 管理员 {} 查看待审核帖子列表，共 {} 条", user.username, posts_response.total);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "data": posts_response,
                "message": "获取待审核帖子成功"
            })))
        },
        Err(e) => {
            log::error!("❌ 获取待审核帖子失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取待审核帖子失败: {}", e)
            })))
        }
    }
} 

// 检查帖子点赞状态
async fn check_like_status(
    http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    match post_service.is_post_liked_by_user(user.id, post_id).await {
        Ok(liked) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"liked": liked}}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
} 

// 书签实现（简单表驱动）
async fn bookmark_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    _post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let post_id = path.into_inner();
    use rusqlite::{Connection, params};
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    conn.execute("CREATE TABLE IF NOT EXISTS post_favorites (user_id INTEGER NOT NULL, post_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, post_id))", [])
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    conn.execute("INSERT OR IGNORE INTO post_favorites (user_id, post_id) VALUES (?, ?)", params![user.id, post_id])
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let cnt: i32 = conn.query_row("SELECT COUNT(*) FROM post_favorites WHERE post_id = ?", params![post_id], |r| r.get(0))
        .unwrap_or(0);
    conn.execute("UPDATE posts SET comment_count = comment_count WHERE id = ?", params![post_id]).ok();
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"success", "data": {"favorite_count": cnt}})))
}

async fn unbookmark_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    _post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let post_id = path.into_inner();
    use rusqlite::{Connection, params};
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    conn.execute("CREATE TABLE IF NOT EXISTS post_favorites (user_id INTEGER NOT NULL, post_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, post_id))", [])
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    conn.execute("DELETE FROM post_favorites WHERE user_id = ? AND post_id = ?", params![user.id, post_id])
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    let cnt: i32 = conn.query_row("SELECT COUNT(*) FROM post_favorites WHERE post_id = ?", params![post_id], |r| r.get(0))
        .unwrap_or(0);
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"success", "data": {"favorite_count": cnt}})))
}

async fn check_bookmark_status(
    http_req: HttpRequest,
    path: web::Path<i32>,
    _post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let post_id = path.into_inner();
    use rusqlite::{Connection, params};
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    conn.execute("CREATE TABLE IF NOT EXISTS post_favorites (user_id INTEGER NOT NULL, post_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, post_id))", [])
        .ok();
    let liked: bool = conn.query_row("SELECT EXISTS(SELECT 1 FROM post_favorites WHERE user_id = ? AND post_id = ?)", params![user.id, post_id], |r| r.get(0)).unwrap_or(false);
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"success", "data": {"favorited": liked}})))
} 

async fn report_post(
    http_req: HttpRequest,
    path: web::Path<i32>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let post_id = path.into_inner();
    let ip = http_req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let ua = http_req.headers().get("User-Agent").and_then(|h| h.to_str().ok()).map(|s| s.to_string());
    let ua_service = http_req.app_data::<web::Data<crate::services::user_action_service::UserActionService>>().cloned();
    if let Some(svc) = ua_service {
        let req = crate::models::user_action::CreateUserActionRequest {
            user_id: Some(user.id),
            action_type: "Report".to_string(),
            target_type: Some("Post".to_string()),
            target_id: Some(post_id),
            details: None,
            ip_address: ip,
            user_agent: ua,
        };
        let _ = svc.log_user_action(&req).await;
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"success"})))
} 

#[derive(serde::Deserialize)]
struct CommentQuery { page: Option<i32>, size: Option<i32> }

// 获取帖子评论（平面列表）
async fn get_post_comments(
    http_req: HttpRequest,
    path: web::Path<i32>,
    query: web::Query<CommentQuery>,
    comment_service: web::Data<crate::services::comment_service::CommentService>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);
    match comment_service.get_post_comments(post_id, page, size).await {
        Ok((mut comments, total)) => {
            // 计算前缀（优先 PUBLIC_BASE_URL，否则从请求推断）
            let cfg = crate::config::Config::load().unwrap_or_default();
            let mut base_prefix = cfg.public_base_url().map(|s| s.trim_end_matches('/').to_string());
            if base_prefix.is_none() {
                let ci = http_req.connection_info();
                let scheme = ci.scheme();
                let host = ci.host();
                if !host.is_empty() {
                    base_prefix = Some(format!("{}://{}", scheme, host).trim_end_matches('/').to_string());
                }
            }
            if let Some(bp) = base_prefix.as_ref() {
                for c in comments.iter_mut() {
                    if let Some(ref mut avatar) = c.author_avatar {
                        if !avatar.starts_with("http://") && !avatar.starts_with("https://") && avatar.starts_with("/uploads/") {
                            *avatar = format!("{}/{}", bp, avatar.trim_start_matches('/'));
                        }
                    }
                }
            }
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": {"list": comments, "total": total, "page": page, "size": size}
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取帖子评论失败: {}", e)
        })))
    }
} 