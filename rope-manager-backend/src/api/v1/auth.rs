use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde_json::json;
use crate::models::{CreateUserRequest, LoginRequest, EmailLoginRequest, SendCodeRequest};
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
            .service(web::resource("/login-by-email").route(web::post().to(login_by_email)))
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/user-info").route(web::get().to(get_user_info)))
            .service(web::resource("/verify").route(web::get().to(verify_auth)))  // 新增认证验证接口
            .service(web::resource("/logout").route(web::post().to(logout)))    // 新增退出登录接口
            .service(web::resource("/send-register-code").route(web::post().to(send_register_code)))
            .service(web::resource("/send-login-code").route(web::post().to(send_login_code)))
            .service(web::resource("/verify-code").route(web::post().to(verify_code)))
            .service(web::resource("/reset-request").route(web::post().to(reset_request)))
            .service(web::resource("/reset-password").route(web::post().to(reset_password)))
    );
}

async fn login(
    req: web::Json<LoginRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 添加调试日志
    println!("接收到登录请求: username='{}', password长度={}", req.username, req.password.len());
    
    match auth_service.login(&req.username, &req.password).await {
        Ok(response) => {
            println!("登录成功: 用户ID={}, 角色={:?}", response.user.id, response.user.role);
            
            // 设置HttpOnly Cookie
            let cookie = actix_web::cookie::Cookie::build("auth_token", &response.token)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::hours(24))
                .same_site(actix_web::cookie::SameSite::Lax)
                .http_only(true)
                .secure(cfg!(not(debug_assertions))) // 生产环境使用HTTPS
                .finish();
            
            Ok(HttpResponse::Ok()
                .cookie(cookie)
                .json(json!({
                    "code": 0,
                    "message": "登录成功",
                    "data": response
                })))
        },
        Err(e) => {
            println!("登录失败: {}", e);
            Ok(HttpResponse::BadRequest().json(json!({
                "code": 1,
                "message": e.to_string()
            })))
        }
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
    // 先尝试从Authorization头获取token
    let mut token: Option<&str> = None;
    
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                token = Some(&auth_str[7..]);
            }
        }
    }
    
    // 如果Authorization头没有token，尝试从Cookie获取
    if token.is_none() {
        if let Some(cookie_header) = req.headers().get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if cookie.starts_with("auth_token=") {
                        token = Some(&cookie[11..]);
                        break;
                    }
                }
            }
        }
    }
    
    if let Some(token_str) = token {
        match auth_service.get_user_from_token(token_str).await {
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
        Ok(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "缺少认证信息"
        })))
    }
}

/// 邮箱验证码登录
async fn login_by_email(
    req: web::Json<EmailLoginRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.login_by_email_code(&req.email, &req.code).await {
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

/// 发送注册验证码
async fn send_register_code(
    req: web::Json<SendCodeRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.send_register_code(&req.email).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"验证码已发送"}))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
}

/// 发送登录验证码
async fn send_login_code(
    req: web::Json<SendCodeRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    match auth_service.send_login_code(&req.email).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0, "message":"验证码已发送"}))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({"code":1,"message":e.to_string()})))
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

/// 验证认证状态（简化版本，只返回是否有效）
async fn verify_auth(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 先尝试从Authorization头获取token
    let mut token: Option<&str> = None;
    
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                token = Some(&auth_str[7..]);
            }
        }
    }
    
    // 如果Authorization头没有token，尝试从Cookie获取
    if token.is_none() {
        if let Some(cookie_header) = req.headers().get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if cookie.starts_with("auth_token=") {
                        token = Some(&cookie[11..]);
                        break;
                    }
                }
            }
        }
    }
    
    if let Some(token_str) = token {
        match auth_service.get_user_from_token(token_str).await {
            Ok(_user) => Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "认证有效"
            }))),
            Err(_) => Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "认证无效"
            })))
        }
    } else {
        Ok(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "缺少认证信息"
        })))
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

/// 退出登录（清除HttpOnly Cookie）
async fn logout() -> Result<HttpResponse, actix_web::Error> {
    // 创建一个过期的Cookie来清除客户端的HttpOnly Cookie
    let mut response = HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "退出登录成功"
    }));
    
    // 设置过期的auth_token Cookie来清除客户端认证状态
    let cookie = actix_web::http::header::HeaderValue::from_str(
        "auth_token=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax"
    ).unwrap();
    
    response.headers_mut().insert(
        actix_web::http::header::SET_COOKIE,
        cookie
    );
    
    Ok(response)
} 