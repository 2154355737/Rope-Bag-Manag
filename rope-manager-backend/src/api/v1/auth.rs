use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use crate::services::auth_service::AuthService;
use crate::models::{LoginRequest, CreateUserRequest, LoginResponse};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login))
    )
    .service(
        web::resource("/register")
            .route(web::post().to(register))
    )
    .service(
        web::resource("/user-info")
            .route(web::get().to(get_user_info))
    );
}

async fn login(
    req: web::Json<LoginRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.login(&req.username, &req.password).await {
        Ok(response) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "登录成功",
            "data": response
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 1,
            "message": e.to_string()
        })))
    }
}

async fn register(
    req: web::Json<CreateUserRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.register(&req).await {
        Ok(user) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "注册成功",
            "data": user
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 1,
            "message": e.to_string()
        })))
    }
}

async fn get_user_info(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 从请求头获取token
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                match auth_service.get_user_from_token(token).await {
                    Ok(user) => Ok(HttpResponse::Ok().json(json!({
                        "code": 0,
                        "message": "success",
                        "data": user
                    }))),
                    Err(_) => Ok(HttpResponse::Unauthorized().json(json!({
                        "code": 401,
                        "message": "无效的token"
                    })))
                }
            } else {
                Ok(HttpResponse::BadRequest().json(json!({
                    "code": 400,
                    "message": "无效的认证格式"
                })))
            }
        } else {
            Ok(HttpResponse::BadRequest().json(json!({
                "code": 400,
                "message": "无效的认证头"
            })))
        }
    } else {
        Ok(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "缺少认证头"
        })))
    }
} 