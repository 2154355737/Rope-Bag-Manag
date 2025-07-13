use actix_web::{web, HttpResponse, get, Responder};
use crate::models::{AppState, LoginResponse, ApiResponse};
use crate::utils::parse_query_params;

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

    // 查找用户
    let user_result = match data.data_manager.get_user(username) {
        Some(user) => (user.id, user),
        None => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 1,
            msg: "用户名或密码错误".to_string(),
            data: None
        }),
    };

    let (user_id, mut user) = user_result;

    // 检查用户是否被封禁
    if user.ban_status == crate::models::BanStatus::Banned {
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
        id: user_id,
        username: user.username.clone(),
        star: user.star,
        role: user.role.clone(),
        online_status: user.online_status.clone(),
        ban_status: user.ban_status.clone(),
        ban_reason: user.ban_reason.clone(),
        qq_number: user.qq_number.clone(),
        avatar_url: user.avatar_url.clone(),
        sign_days: user.sign_days,
        sign_total: user.sign_total,
        last_sign: user.last_sign.clone(),
        permissions: user.get_permissions(),
        is_admin: user.is_admin,
    };

    // 更新用户状态
    user.set_online_status(crate::models::OnlineStatus::Online);
    user.increment_login_count();
    
    // 保存用户数据
    if let Err(_) = data.data_manager.update_user(user.username.clone(), user) {
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

// 用户注册
#[get("/api/register")]
pub async fn register(
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

    // 检查用户名长度
    if username.len() < 2 || username.len() > 20 {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "用户名长度必须在2-20个字符之间".to_string(),
            data: None
        });
    }

    // 检查密码长度
    if password.len() < 6 {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "密码长度不能少于6个字符".to_string(),
            data: None
        });
    }

    // 创建新用户
    match data.data_manager.create_user(username.to_string(), password.to_string()) {
        Ok(user_id) => {
            return HttpResponse::Ok().json(ApiResponse {
                code: 0,
                msg: "注册成功".to_string(),
                data: Some(user_id.to_string())
            });
        },
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                code: 1,
                msg: format!("注册失败: {}", e),
                data: None
            });
        }
    }
}

