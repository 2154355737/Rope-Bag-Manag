use actix_web::{web, HttpResponse, HttpRequest, get, post, Responder};
use crate::models::{AppState, User, LoginResponse, ApiResponse};
use crate::utils::parse_query_params;
use crate::data_manager::DataManager;
use serde::{Deserialize};
use chrono::Utc;

// 用户登录
#[get("/api/login")]
pub async fn login(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };
    let password = match params.get("password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少密码".to_string(),
            data: None
        }),
    };

    // 检查系统维护模式
    let settings = match data.data_manager.load_settings() {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "系统配置加载失败".to_string(), 
            data: None 
        }),
    };
    
    // 如果系统处于维护模式，只允许管理员登录
    if settings.system_mode == crate::models::SystemMode::Maintenance {
        let admin_username = &data.config.admin_username;
        if username != admin_username {
            return HttpResponse::ServiceUnavailable().json(ApiResponse::<()> { 
                code: 1, 
                msg: "系统维护中，仅管理员可登录".to_string(), 
                data: None 
            });
        }
    }

    let mut users = match data.data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get_mut(username) {
        // 检查用户是否被封禁
        if user.banned {
            return HttpResponse::Forbidden().json(ApiResponse::<()> {
                code: 1,
                msg: format!("用户已被封禁: {}", user.ban_reason.as_ref().unwrap_or(&"无原因".to_string())),
                data: None
            });
        }

        // 验证密码
        if user.password != *password {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                code: 1,
                msg: "用户名或密码错误".to_string(),
                data: None
            });
        }

        // 构建登录响应（在更新之前）
        let response = LoginResponse {
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            star: user.star,
            role: user.role.clone(),
            banned: user.banned,
            ban_reason: user.ban_reason.clone(),
            qq_number: user.qq_number.clone(),
            avatar_url: user.avatar_url.clone(),
            sign_days: user.sign_days,
            sign_total: user.sign_total,
            last_sign: user.last_sign.clone(),
            permissions: user.get_permissions(),
            is_admin: user.is_admin,
        };

        // 更新最后登录时间
        user.last_login = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // 保存用户数据
        if let Err(_) = data.data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }

        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "登录成功".to_string(),
            data: Some(response)
        });
    }
    
    HttpResponse::Unauthorized().json(ApiResponse::<()> {
        code: 1,
        msg: "用户名或密码错误".to_string(),
        data: None
    })
}

