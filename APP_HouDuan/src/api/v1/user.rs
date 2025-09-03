use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use crate::services::user_service::UserService;
use crate::models::UpdateUserRequest;
use crate::utils::auth_helper::AuthHelper;
#[macro_use] use crate::utils::auth_helper;
use crate::services::user_action_service::UserActionService;
use crate::services::package_service::PackageService;
use crate::services::post_service::PostService;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::io::Write;

#[derive(serde::Deserialize)]
struct GetUsersQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    search: Option<String>,
    role: Option<String>,
    ban_status: Option<String>,
}

#[derive(serde::Deserialize)]
struct UpdateProfileRequest {
    nickname: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    website: Option<String>,
    skills: Option<String>,
    avatar_url: Option<String>,
}

#[derive(serde::Serialize)]
struct WeeklyReportData {
    total_posts: i32,
    completed_projects: i32,
    current_streak: i32,
    today_activity: f32,
    weekly_posts: Vec<i32>,
    achievements: Vec<Achievement>,
}

#[derive(serde::Serialize)]
struct Achievement {
    id: i32,
    name: String,
    icon: String,
    description: String,
    earned_at: Option<String>,
}

#[derive(serde::Serialize)]
struct UserActivityStats {
    posts_count: i32,
    resources_count: i32,
    comments_count: i32,
    total_views: i32,
    total_likes: i32,
    total_downloads: i32,
    level: String,
    experience: i32,
    next_level_exp: i32,
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
                web::resource("/upload-avatar")
                    .route(web::post().to(upload_avatar))
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
            .service(
                web::resource("/{id}/profile")
                    .route(web::get().to(get_user_profile))
            )
            .service(
                web::resource("/{id}/posts")
                    .route(web::get().to(get_user_posts))
            )
            .service(
                web::resource("/{id}/resources")
                    .route(web::get().to(get_user_resources))
            )
            .service(
                web::resource("/{id}/latest-content")
                    .route(web::get().to(get_user_latest_content))
            )
    );

    // 新增：/me 别名集合
    cfg.service(
        web::scope("/me")
            .service(
                web::resource("")
                    .route(web::get().to(get_current_user_profile))
                    .route(web::put().to(update_current_user_profile))
            )
            .service(
                web::resource("/stats")
                    .route(web::get().to(get_my_stats))
            )
            .service(
                web::resource("/activity-stats")
                    .route(web::get().to(get_my_activity_stats))
            )
            .service(
                web::resource("/weekly-report")
                    .route(web::get().to(get_my_weekly_report))
            )
            .service(
                web::resource("/achievements")
                    .route(web::get().to(get_my_achievements))
            )
            .service(
                web::resource("/check-in")
                    .route(web::post().to(user_check_in))
            )
            .service(
                web::resource("/resources")
                    .route(web::get().to(get_my_resources))
            )
            .service(
                web::resource("/posts")
                    .route(web::get().to(get_my_posts))
            )
            .service(
                web::resource("/comments")
                    .route(web::get().to(get_my_comments))
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

async fn get_user_profile(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            // 获取真实的统计数据
            use rusqlite::Connection;
            let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            
            let posts_count: i64 = conn.query_row("SELECT COUNT(*) FROM posts WHERE author_id = ? AND status = 'Published'", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
            let resources_count: i64 = conn.query_row("SELECT COUNT(*) FROM packages WHERE author = ? AND status = 'active'", rusqlite::params![user.username], |r| r.get(0)).unwrap_or(0);
            let total_views: i64 = conn.query_row("SELECT COALESCE(SUM(view_count),0) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
            let total_likes: i64 = conn.query_row("SELECT COALESCE(SUM(like_count),0) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
            
            // 构建完整的用户资料信息，包括统计数据
            let profile = json!({
                "id": user.id,
                "username": user.username,
                "nickname": user.nickname,
                "avatar_url": if let Some(avatar) = user.avatar_url {
                    if avatar.starts_with("http://") || avatar.starts_with("https://") {
                        avatar
                    } else if avatar.starts_with("/") {
                        format!("http://localhost:15201{}", avatar)
                    } else {
                        format!("http://localhost:15201/{}", avatar)
                    }
                } else {
                    format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", user.id)
                },
                "bio": user.bio,
                "location": user.location,
                "website": user.website,
                "skills": user.skills,
                "followers_count": 0, // TODO: 从关注表获取
                "following_count": 0, // TODO: 从关注表获取
                "posts_count": posts_count as i32,
                "resources_count": resources_count as i32,
                "total_likes": total_likes as i32,
                "total_views": total_views as i32,
                "total_downloads": user.download_count,
                "created_at": user.created_at.format("%Y-%m-%d").to_string(),
                "is_following": false // TODO: 根据当前用户判断是否关注
            });
            
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": profile
            })))
        },
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
        Ok(Some(mut user)) => {
            if let Some(avatar) = &mut user.avatar_url {
                if !avatar.starts_with("http://") && !avatar.starts_with("https://") && avatar.starts_with("/uploads/") {
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
                        *avatar = format!("{}/{}", bp, avatar.trim_start_matches('/'));
                    }
                }
            }
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": user
            })))
        },
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

// 新增：我的统计
async fn get_my_stats(
    http_req: HttpRequest,
    post_service: web::Data<PostService>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let posts: i64 = conn.query_row("SELECT COUNT(*) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let resources: i64 = conn.query_row("SELECT COUNT(*) FROM packages WHERE author = (SELECT username FROM users WHERE id = ?)", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let views: i64 = conn.query_row("SELECT COALESCE(SUM(view_count),0) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let likes: i64 = conn.query_row("SELECT COALESCE(SUM(like_count),0) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"posts": posts, "resources": resources, "views": views, "likes": likes}})))
}

// 新增：我的周报（占位）
async fn get_my_weekly_report(
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    // 获取本周的统计数据
    let total_posts: i64 = conn.query_row("SELECT COUNT(*) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let total_resources: i64 = conn.query_row("SELECT COUNT(*) FROM packages WHERE author = (SELECT username FROM users WHERE id = ?)", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    
    // 获取本周每天的发布数量（示例数据）
    let weekly_posts = vec![2, 1, 3, 0, 2, 4, 1]; // 周一到周日的发布数量
    
    // 计算连续签到天数（示例）
    let current_streak = 12;
    
    // 今日活跃度（示例）
    let today_activity = 85.0;
    
    let report_data = WeeklyReportData {
        total_posts: total_posts as i32,
        completed_projects: total_resources as i32,
        current_streak,
        today_activity,
        weekly_posts,
        achievements: vec![
            Achievement {
                id: 1,
                name: "初学者".to_string(),
                icon: "🌱".to_string(),
                description: "完成第一个课程".to_string(),
                earned_at: Some("2024-01-15T10:30:00Z".to_string()),
            },
            Achievement {
                id: 2,
                name: "勤奋学习".to_string(),
                icon: "📚".to_string(),
                description: "连续学习7天".to_string(),
                earned_at: Some("2024-01-20T15:45:00Z".to_string()),
            },
        ],
    };
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": report_data
    })))
}

// 新增：我的活动统计
async fn get_my_activity_stats(
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let posts: i64 = conn.query_row("SELECT COUNT(*) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let resources: i64 = conn.query_row("SELECT COUNT(*) FROM packages WHERE author = (SELECT username FROM users WHERE id = ?)", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let comments: i64 = conn.query_row("SELECT COUNT(*) FROM comments WHERE user_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let views: i64 = conn.query_row("SELECT COALESCE(SUM(view_count),0) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let likes: i64 = conn.query_row("SELECT COALESCE(SUM(like_count),0) FROM posts WHERE author_id = ?", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);
    let downloads: i64 = conn.query_row("SELECT COALESCE(SUM(download_count),0) FROM packages WHERE author = (SELECT username FROM users WHERE id = ?)", rusqlite::params![user.id], |r| r.get(0)).unwrap_or(0);

    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "posts_count": posts as i32,
            "resources_count": resources as i32,
            "comments_count": comments as i32,
            "total_views": views as i32,
            "total_likes": likes as i32,
            "total_downloads": downloads as i32,
            "level": "初级开发者", // 示例级别
            "experience": 1200, // 示例经验值
            "next_level_exp": 2000 // 示例下一级所需经验
        }
    })))
}

// 新增：我的成就
async fn get_my_achievements(
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let mut achievements: Vec<Achievement> = Vec::new();

    // 示例：获取用户已完成的成就
    // 实际应用中，这些成就应该从数据库或配置文件加载
    let completed_achievements = vec![
        Achievement { id: 1, name: "新手入门".to_string(), icon: "🎉".to_string(), description: "完成首次登录".to_string(), earned_at: None },
        Achievement { id: 2, name: "点赞达人".to_string(), icon: "👍".to_string(), description: "点赞超过100次".to_string(), earned_at: None },
        Achievement { id: 3, name: "资源分享".to_string(), icon: "📦".to_string(), description: "分享超过5个资源".to_string(), earned_at: None },
    ];

    // 实际应用中，这里需要从数据库或配置文件加载所有可能的成就
    // 例如：let all_achievements = load_all_achievements();
    // 然后过滤出用户已完成的成就

    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "list": completed_achievements,
            "total": completed_achievements.len() as i32
        }
    })))
}

// 新增：用户签到
async fn user_check_in(
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };

    match user_service.user_check_in(user.id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "签到成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 新增：我的帖子
async fn get_my_posts(
    http_req: HttpRequest,
    query: web::Query<serde_json::Value>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) { Ok(u) => u, Err(e) => return Ok(e.to_response()) };
    let page = query.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as i32;
    let page_size = query.get("pageSize").and_then(|v| v.as_u64()).unwrap_or(10) as i32;
    let params = crate::models::PostQueryParams { page: Some((page as u32)), page_size: Some((page_size as u32)), category_id: None, author_id: Some(user.id), status: None, search: None, tags: None, is_pinned: None, is_featured: None };
    match post_service.get_posts(params).await {
        Ok(resp) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": resp}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
} 

// 新增：头像上传
async fn upload_avatar(
    mut payload: Multipart,
    http_req: HttpRequest,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = match AuthHelper::verify_user(&http_req) {
        Ok(user) => user,
        Err(e) => return Ok(e.to_response()),
    };

    while let Some(mut field) = payload.try_next().await.map_err(actix_web::error::ErrorBadRequest)? {
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.and_then(|cd| cd.get_filename()) {
            // 检查文件类型
            if !filename.to_lowercase().ends_with(".jpg") 
                && !filename.to_lowercase().ends_with(".jpeg") 
                && !filename.to_lowercase().ends_with(".png") 
                && !filename.to_lowercase().ends_with(".gif") {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "code": 400,
                    "message": "只支持 JPG, PNG, GIF 格式的图片"
                })));
            }

            // 生成唯一文件名
            let file_extension = filename.split('.').last().unwrap_or("jpg");
            let new_filename = format!("avatar_{}_{}.{}", user.username, chrono::Utc::now().timestamp(), file_extension);
            
            // 创建上传目录: /uploads/结绳社区/头像/用户名/
            let upload_dir = std::path::Path::new("./uploads/结绳社区/头像").join(&user.username);
            if !upload_dir.exists() {
                std::fs::create_dir_all(&upload_dir).map_err(actix_web::error::ErrorInternalServerError)?;
            }
            
            let file_path = upload_dir.join(&new_filename);
            let mut file = std::fs::File::create(&file_path).map_err(actix_web::error::ErrorInternalServerError)?;
            
            // 读取并写入文件数据，同时检查文件大小
            let mut file_size = 0;
            const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
            
            while let Some(chunk) = field.try_next().await.map_err(actix_web::error::ErrorBadRequest)? {
                file_size += chunk.len();
                if file_size > MAX_FILE_SIZE {
                    // 删除已创建的文件
                    let _ = std::fs::remove_file(&file_path);
                    return Ok(HttpResponse::BadRequest().json(json!({
                        "code": 400,
                        "message": "文件大小不能超过5MB"
                    })));
                }
                file.write_all(&chunk).map_err(actix_web::error::ErrorInternalServerError)?;
            }
            
            // 生成访问URL
            let cfg = crate::config::Config::load().unwrap_or_default();
            let base_prefix = if let Some(base) = cfg.public_base_url() { base.trim_end_matches('/') } else { "" };
            let avatar_url = if base_prefix.is_empty() {
                format!("/uploads/结绳社区/头像/{}/{}", user.username, new_filename)
            } else {
                format!("{}/uploads/结绳社区/头像/{}/{}", base_prefix, user.username, new_filename)
            };
            
            // 更新用户头像URL
            let update_req = UpdateUserRequest {
                email: None,
                nickname: None,
                star: None,
                ban_status: None,
                ban_reason: None,
                role: None,
                qq_number: None,
                avatar_url: Some(avatar_url.clone()),
                bio: None,
                location: None,
                website: None,
                skills: None,
            };
            
            match user_service.update_user(user.id, &update_req).await {
                Ok(_) => {
                    return Ok(HttpResponse::Ok().json(json!({
                        "code": 0,
                        "message": "头像上传成功",
                        "data": {
                            "avatar_url": avatar_url
                        }
                    })));
                },
                Err(e) => {
                    // 删除上传的文件
                    let _ = std::fs::remove_file(&file_path);
                    return Ok(HttpResponse::InternalServerError().json(json!({
                        "code": 500,
                        "message": format!("更新头像失败: {}", e)
                    })));
                }
            }
        }
    }
    
    Ok(HttpResponse::BadRequest().json(json!({
        "code": 400,
        "message": "未找到有效的图片文件"
    })))
}

// 获取特定用户的帖子
async fn get_user_posts(
    path: web::Path<i32>,
    query: web::Query<serde_json::Value>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    let page = query.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as i32;
    let page_size = query.get("pageSize").and_then(|v| v.as_u64()).unwrap_or(10) as i32;
    let params = crate::models::PostQueryParams { 
        page: Some((page as u32)), 
        page_size: Some((page_size as u32)), 
        category_id: None, 
        author_id: Some(user_id), 
        status: Some("Published".to_string()), // 只显示已发布的帖子
        search: None, 
        tags: None, 
        is_pinned: None, 
        is_featured: None 
    };
    match post_service.get_posts(params).await {
        Ok(resp) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": resp}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
}

// 获取特定用户的资源
async fn get_user_resources(
    path: web::Path<i32>,
    query: web::Query<serde_json::Value>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    let page = query.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
    let page_size = query.get("pageSize").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
    
    // 通过用户ID获取用户名
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let username: String = match conn.query_row("SELECT username FROM users WHERE id = ?", rusqlite::params![user_id], |r| r.get(0)) {
        Ok(name) => name,
        Err(_) => return Ok(HttpResponse::NotFound().json(json!({"code":404, "message": "用户不存在"})))
    };
    
    match package_service.get_packages_advanced(page, page_size, None, Some(username), Some("active".to_string())).await {
        Ok((list, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0, 
            "message": "success", 
            "data": {
                "list": list,
                "total": total,
                "page": page,
                "pageSize": page_size,
                "totalPages": (total as f64 / page_size as f64).ceil() as u32
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
    }
}

// 获取特定用户的最新内容（帖子和资源混合）
async fn get_user_latest_content(
    path: web::Path<i32>,
    query: web::Query<serde_json::Value>,
    post_service: web::Data<PostService>,
    package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    let limit = query.get("limit").and_then(|v| v.as_u64()).unwrap_or(6) as u32;
    
    // 获取用户名
    use rusqlite::Connection;
    let conn = crate::repositories::get_connection().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let username: String = match conn.query_row("SELECT username FROM users WHERE id = ?", rusqlite::params![user_id], |r| r.get(0)) {
        Ok(name) => name,
        Err(_) => return Ok(HttpResponse::NotFound().json(json!({"code":404, "message": "用户不存在"})))
    };
    
    let mut content_list = Vec::new();
    
    // 获取用户的帖子（最新的几条）
    let post_params = crate::models::PostQueryParams {
        page: Some(1),
        page_size: Some(limit),
        category_id: None,
        author_id: Some(user_id),
        status: Some("Published".to_string()),
        search: None,
        tags: None,
        is_pinned: None,
        is_featured: None
    };
    
    if let Ok(posts_resp) = post_service.get_posts(post_params).await {
        for post in posts_resp.list {
            content_list.push(json!({
                "id": post.id,
                "type": "post",
                "title": post.title,
                "description": post.content.chars().take(100).collect::<String>() + if post.content.len() > 100 { "..." } else { "" },
                "stats": {
                    "views": post.view_count,
                    "likes": post.like_count,
                    "comments": post.comment_count
                },
                "created_at": post.created_at,
                "tags": post.tags.unwrap_or_default()
            }));
        }
    }
    
    // 获取用户的资源（最新的几条）
    if let Ok((resources_list, _total)) = package_service.get_packages_advanced(1, limit, None, Some(username), Some("active".to_string())).await {
        for resource in resources_list {
            content_list.push(json!({
                "id": resource.id,
                "type": "resource",
                "title": resource.name,
                "description": resource.description.unwrap_or_default(),
                "stats": {
                    "downloads": resource.download_count,
                    "likes": resource.like_count,
                    "rating": 4.5 // 暂时固定值，后续可以从评分表获取
                },
                "created_at": resource.created_at,
                "tags": [] // 暂时为空，后续可以从标签关联表获取
            }));
        }
    }
    
    // 按创建时间排序
    content_list.sort_by(|a, b| {
        let a_time = a["created_at"].as_str().unwrap_or("");
        let b_time = b["created_at"].as_str().unwrap_or("");
        b_time.cmp(a_time) // 降序排列
    });
    
    // 限制返回数量
    content_list.truncate(limit as usize);
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "list": content_list,
            "total": content_list.len()
        }
    })))
} 