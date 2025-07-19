use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::user_service::UserService;
use crate::models::UpdateUserRequest;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_users))
            .route(web::post().to(create_user))
    )
    .service(
        web::resource("/batch")
            .route(web::delete().to(batch_delete_users))
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(get_user))
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user))
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
        web::resource("/change-password")
            .route(web::post().to(change_password))
    );
}

async fn get_users(
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    match user_service.get_users().await {
        Ok(users) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": users,
                "total": users.len(),
                "page": 1,
                "pageSize": users.len(),
                "totalPages": 1
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
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
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
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
}

async fn delete_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
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
}

// 新增方法：获取当前用户资料
async fn get_current_user_profile(
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // TODO: 从JWT token获取当前用户ID
    let current_user_id = 1; // 临时硬编码
    
    match user_service.get_user_by_id(current_user_id).await {
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
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // TODO: 从JWT token获取当前用户ID
    let current_user_id = 1; // 临时硬编码
    
    match user_service.update_user(current_user_id, &req).await {
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
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // TODO: 从JWT token获取当前用户ID
    let current_user_id = 1; // 临时硬编码
    
    match user_service.get_user_resources(current_user_id).await {
        Ok(resources) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": resources,
                "total": resources.len(),
                "page": 1,
                "pageSize": resources.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 新增方法：获取我的评论
async fn get_my_comments(
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    // TODO: 从JWT token获取当前用户ID
    let current_user_id = 1; // 临时硬编码
    
    match user_service.get_user_comments(current_user_id).await {
        Ok(comments) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": comments,
                "total": comments.len(),
                "page": 1,
                "pageSize": comments.len()
            }
        }))),
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

// 新增方法：批量删除用户
async fn batch_delete_users(
    req: web::Json<serde_json::Value>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let usernames = req["usernames"].as_array()
        .map(|arr| arr.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
        .unwrap_or_default();

    if usernames.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "请选择要删除的用户"
        })));
    }

    match user_service.batch_delete_users(usernames).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "批量删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 