use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use crate::services::user_service::UserService;
use crate::models::UpdateUserRequest;
use crate::utils::auth_helper::AuthHelper;
#[macro_use] use crate::utils::auth_helper;
use crate::services::user_action_service::UserActionService;
use crate::services::package_service::PackageService;
use crate::services::post_service::PostService;

#[derive(serde::Deserialize)]
struct GetUsersQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    search: Option<String>,
    role: Option<String>,
    ban_status: Option<String>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::get().to(get_users))
                    .route(web::post().to(create_user))
            )
            .service(
                web::resource("/batch")
                    .route(web::delete().to(batch_delete_users))
            )
            .service(
                web::resource("/profile")
                    .route(web::get().to(get_current_user_profile))
                    .route(web::put().to(update_current_user_profile))
            )
            .service(
                web::resource("/my-resources")
                    .route(web::get().to(get_my_resources))
            )
            .service(
                web::resource("/my-comments")
                    .route(web::get().to(get_my_comments))
            )
            .service(
                web::resource("/my-likes")
                    .route(web::get().to(get_my_likes))
            )
            .service(
                web::resource("/my-likes/stats")
                    .route(web::get().to(get_my_likes_stats))
            )
            .service(
                web::resource("/change-password")
                    .route(web::post().to(change_password))
            )
            .service(
                web::resource("/{id}/comments")
                    .route(web::get().to(crate::api::v1::comment::get_user_comments))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user))
            )
    );
}

async fn get_users(
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
    query: web::Query<GetUsersQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证管理员权限
    match AuthHelper::require_admin(&http_req) {
        Ok(_admin_user) => {
            match user_service.get_users().await {
                Ok(mut users) => {
                    // 过滤
                    if let Some(s) = &query.search { let k = s.to_lowercase(); users.retain(|u| u.username.to_lowercase().contains(&k) || u.email.to_lowercase().contains(&k)); }
                    if let Some(r) = &query.role { let rk = r.to_lowercase(); users.retain(|u| format!("{:?}", u.role).to_lowercase()==rk || match u.role { crate::models::UserRole::Admin=>"admin",crate::models::UserRole::Moderator=>"moderator",crate::models::UserRole::Elder=>"elder",crate::models::UserRole::User=>"user",}.eq(&rk)); }
                    if let Some(bs) = &query.ban_status { let bsk = bs.to_lowercase(); users.retain(|u| match u.ban_status { crate::models::BanStatus::Normal=>"normal", crate::models::BanStatus::Suspended=>"suspended", crate::models::BanStatus::Banned=>"banned"}.eq(&bsk)); }
                    // 分页
                    let page = query.page.unwrap_or(1).max(1);
                    let page_size = query.page_size.unwrap_or(20).max(1);
                    let total = users.len() as i32;
                    let start = ((page - 1) * page_size) as usize;
                    let end = std::cmp::min(start + page_size as usize, users.len());
                    let page_list = if start < users.len() { users[start..end].to_vec() } else { Vec::new() };
                    Ok(HttpResponse::Ok().json(json!({
                        "code": 0,
                        "message": "success",
                        "data": {
                            "list": page_list,
                            "total": total,
                            "page": page,
                            "pageSize": page_size,
                            "totalPages": ((total as f64)/(page_size as f64)).ceil() as i32
                        }
                    })))
                },
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "code": 500,
                    "message": e.to_string()
                })))
            }
        },
        Err(_) => Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "需要管理员权限"
        })))
    }
}

async fn get_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": user
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "用户不存在"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn update_user(
    path: web::Path<i32>,
    req: web::Json<UpdateUserRequest>,
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证管理员权限
    match AuthHelper::require_admin(&http_req) {
        Ok(_admin_user) => {
            let user_id = path.into_inner();
            match user_service.update_user(user_id, &req).await {
                Ok(_) => Ok(HttpResponse::Ok().json(json!({
                    "code": 0,
                    "message": "更新成功"
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "code": 500,
                    "message": e.to_string()
                })))
            }
        },
        Err(_) => Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "需要管理员权限"
        })))
    }
}

async fn delete_user(
    path: web::Path<i32>,
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证管理员权限
    match AuthHelper::require_admin(&http_req) {
        Ok(_admin_user) => {
            let user_id = path.into_inner();
            match user_service.delete_user(user_id).await {
                Ok(_) => Ok(HttpResponse::Ok().json(json!({
                    "code": 0,
                    "message": "删除成功"
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "code": 500,
                    "message": e.to_string()
                })))
            }
        },
        Err(_) => Ok(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "需要管理员权限"
        })))
    }
}

// 新增方法：获取当前用户资料
async fn get_current_user_profile(
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证并获取当前用户ID
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    match user_service.get_user_by_id(user.id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": user
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "用户不存在"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 新增方法：更新当前用户资料
async fn update_current_user_profile(
    req: web::Json<UpdateUserRequest>,
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证并获取当前用户ID
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    match user_service.update_user(user.id, &req).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "资料更新成功"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

// 新增方法：获取我的资源
async fn get_my_resources(
    http_req: HttpRequest,
    query: web::Query<serde_json::Value>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证并获取当前用户ID
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    // 解析分页参数
    let page = query.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as i32;
    let page_size = query.get("pageSize").and_then(|v| v.as_u64()).unwrap_or(10) as i32;
    
    match user_service.get_user_resources(user.id).await {
        Ok(resources) => {
            let total = resources.len();
            let start = ((page - 1) * page_size) as usize;
            let end = std::cmp::min(start + page_size as usize, total);
            let paginated_resources = if start < total {
                resources[start..end].to_vec()
            } else {
                Vec::new()
            };
            
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": {
                    "list": paginated_resources,
                    "total": total,
                    "page": page,
                    "pageSize": page_size,
                    "totalPages": (total as f64 / page_size as f64).ceil() as i32
                }
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 新增方法：获取我的评论
async fn get_my_comments(
    http_req: HttpRequest,
    query: web::Query<serde_json::Value>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 验证用户认证并获取当前用户ID
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };
    
    // 解析分页参数，支持size和pageSize两种参数名
    let page = query.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as i32;
    let page_size = query.get("size")
        .or_else(|| query.get("pageSize"))
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as i32;
    
    match user_service.get_user_comments(user.id).await {
        Ok(comments) => {
            let total = comments.len();
            let start = ((page - 1) * page_size) as usize;
            let end = std::cmp::min(start + page_size as usize, total);
            let paginated_comments = if start < total {
                comments[start..end].to_vec()
            } else {
                Vec::new()
            };
            
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": {
                    "list": paginated_comments,
                    "total": total,
                    "page": page,
                    "pageSize": page_size,
                    "totalPages": (total as f64 / page_size as f64).ceil() as i32
                }
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 新增方法：修改密码
async fn change_password(
    req: web::Json<serde_json::Value>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // TODO: 从JWT token获取当前用户ID
    let current_user_id = 1; // 临时硬编码
    
    let old_password = req["old_password"].as_str().unwrap_or("");
    let new_password = req["new_password"].as_str().unwrap_or("");
    
    match user_service.change_password(current_user_id, old_password, new_password).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "密码修改成功"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

// 新增方法：创建用户
async fn create_user(
    req: web::Json<serde_json::Value>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = req["username"].as_str().unwrap_or("");
    let email = req["email"].as_str().unwrap_or("");
    let password = req["password"].as_str().unwrap_or("");
    let role = req["role"].as_str().unwrap_or("user");
    let star = req["star"].as_i64().unwrap_or(1) as i32;
    let qq_number = req["qq_number"].as_str();
    let avatar_url = req["avatar_url"].as_str();

    if username.is_empty() || password.is_empty() || email.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "用户名、邮箱和密码不能为空"
        })));
    }

    // 转换角色格式
    let role_lower = role.to_lowercase();
    match user_service.create_user(username, email, password, &role_lower, star, qq_number, avatar_url).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "用户创建成功"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

// 批量删除用户（支持 ids 或 usernames）
async fn batch_delete_users(
    req: web::Json<serde_json::Value>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 优先按ids删除
    if let Some(ids_val) = req.get("ids") {
        if let Some(arr) = ids_val.as_array() {
            let ids: Vec<i32> = arr.iter().filter_map(|v| v.as_i64().map(|x| x as i32)).collect();
            if !ids.is_empty() {
                match user_service.batch_delete_users_by_ids(ids).await {
                    Ok(_) => return Ok(HttpResponse::Ok().json(json!({"code":0,"message":"批量删除成功"}))),
                    Err(e) => return Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
                }
            }
        }
    }
    // 兼容按用户名删除
    let usernames = req["usernames"].as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<String>>())
        .unwrap_or_default();
    if usernames.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({"code":400,"message":"缺少 ids 或 usernames"})));
    }
    match user_service.batch_delete_users(usernames).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0,"message":"批量删除成功"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
} 

#[derive(serde::Deserialize)]
struct MyLikesQuery { target: Option<String>, page: Option<i32>, page_size: Option<i32> }

// GET /users/my-likes  
async fn get_my_likes(
    http_req: HttpRequest,
    query: web::Query<MyLikesQuery>,
    ua_service: web::Data<UserActionService>,
    package_service: web::Data<PackageService>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).max(1);
    let target = query.target.clone().unwrap_or_default().to_lowercase();

    match target.as_str() {
        "post" => {
            // 返回用户点赞的帖子
            match post_service.get_user_liked_posts(user.id, page, page_size).await {
                Ok((posts, total)) => Ok(HttpResponse::Ok().json(json!({
                    "code": 0,
                    "message": "success",
                    "data": { "list": posts, "total": total, "page": page, "page_size": page_size }
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
            }
        }
        "package" => {
            // 返回用户点赞的资源包（兼容旧版本）
            use rusqlite::Connection;
            let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            let total: i64 = conn.query_row("SELECT COUNT(*) FROM package_likes WHERE user_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
            let offset = (page - 1).max(0) * page_size.max(1);
            let sql = "SELECT p.id, p.name, p.author, p.description, p.like_count, p.download_count, pl.created_at FROM packages p JOIN package_likes pl ON pl.package_id = p.id WHERE pl.user_id = ? ORDER BY pl.created_at DESC LIMIT ? OFFSET ?";
            let mut stmt = conn.prepare(sql).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            let mut items = Vec::new();
            let rows = stmt.query_map(rusqlite::params![user.id, page_size, offset], |row| {
                Ok(serde_json::json!({
                    "type": "Package",
                    "id": row.get::<_, i32>(0)?,
                    "name": row.get::<_, String>(1)?,
                    "author": row.get::<_, String>(2)?,
                    "description": row.get::<_, String>(3)?,
                    "like_count": row.get::<_, i32>(4)?,
                    "download_count": row.get::<_, i32>(5)?,
                    "created_at": row.get::<_, String>(6)?,
                }))
            }).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            for r in rows { items.push(r.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?); }
            Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"list": items, "total": total, "page": page, "page_size": page_size}})))
        }
        _ => {
            // 默认返回所有类型的点赞（使用新的统一视图）
            match ua_service.get_user_likes(user.id, page as u32, page_size as u32).await {
                Ok((likes, total)) => Ok(HttpResponse::Ok().json(json!({
                    "code": 0,
                    "message": "success",
                    "data": { "list": likes, "total": total, "page": page, "page_size": page_size }
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
            }
        }
    }
}

// GET /users/my-likes/stats
async fn get_my_likes_stats(
    http_req: HttpRequest,
    _ua_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let like_pkg: i64 = conn.query_row("SELECT COUNT(*) FROM package_likes WHERE user_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let like_post: i64 = conn.query_row("SELECT COUNT(*) FROM post_likes WHERE user_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    // 浏览统计此处无likes表，先返回0，后续可接入posts view日志表
    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "like_total": like_pkg + like_post,
            "like_by_type": {"package": like_pkg, "post": like_post},
            "view_total": 0,
            "view_by_type": {"post": 0}
        }
    })))
} 