use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, LoginResponse};
use crate::utils::{parse_query_params, ApiResponse};
use crate::auth::check_rate_limit;
use log::{info, warn};

#[get("/api/login")]
pub async fn login(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // 检查限流
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "login") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    
    let username = match params.get("username") {
        Some(u) => u,
        None => {
            warn!("登录失败: 缺少用户名参数");
            return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None });
        }
    };
    
    let password = match params.get("password") {
        Some(p) => p,
        None => {
            warn!("登录失败: 缺少密码参数");
            return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少密码".to_string(), data: None });
        }
    };

    info!("用户尝试登录: {}", username);

    let users = data.data_manager.get_users();
    
    if let Some(user) = users.get(username) {
        if user.password == *password {
            if user.banned {
                warn!("被封禁用户尝试登录: {}", username);
                return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "用户已被封禁".to_string(), data: None });
            }

            info!("用户登录成功: {}", username);
            let response = LoginResponse {
                username: user.username.clone(),
                nickname: user.nickname.clone(),
                star: user.star,
                banned: user.banned,
                sign_days: user.sign_days,
                sign_total: user.sign_total,
                is_admin: user.is_admin,
            };

            return HttpResponse::Ok().json(ApiResponse { code: 0, msg: "登录成功".to_string(), data: Some(response) });
        }
    }

    warn!("登录失败: 用户名或密码错误 - {}", username);
    HttpResponse::Unauthorized().json(ApiResponse::<()> { code: 1, msg: "用户名或密码错误".to_string(), data: None })
}