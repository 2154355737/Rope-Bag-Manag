use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde_json::json;
use crate::models::{CreateUserRequest, LoginRequest};
use crate::services::auth_service::AuthService;
use serde::Deserialize;

#[derive(Deserialize)]
struct EmailReq { email: String }

#[derive(Deserialize)]
struct VerifyCodeReq { 
    email: String, 
    code: String 
}

#[derive(Deserialize)]
struct ResetPasswordReq {
    email: String,
    token: String,
    new_password: String,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/user-info").route(web::get().to(get_user_info)))
            .service(web::resource("/send-code").route(web::post().to(send_code)))
            .service(web::resource("/verify-code").route(web::post().to(verify_code)))
            .service(web::resource("/reset-request").route(web::post().to(reset_request)))
            .service(web::resource("/reset-password").route(web::post().to(reset_password)))
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
        Ok(response) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "注册成功",
            "data": response
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

async fn send_code(
    req: web::Json<EmailReq>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.send_register_code(&req.email).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"邮件已发送"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
}

async fn verify_code(
    req: web::Json<VerifyCodeReq>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.verify_email_code(&req.email, &req.code).await {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({"code":0,"message":"验证码正确"}))),
        Ok(false) => Ok(HttpResponse::BadRequest().json(json!({"code":400,"message":"验证码错误或已过期"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
}

async fn reset_request(
    req: web::Json<EmailReq>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.send_reset_link(&req.email).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0,"message":"重置邮件已发送"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
}

async fn reset_password(
    req: web::Json<ResetPasswordReq>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.reset_password_with_token(&req.email, &req.token, &req.new_password).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0,"message":"密码重置成功"}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":400,"message":e.to_string()})))
    }
} 