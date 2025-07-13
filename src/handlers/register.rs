use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, User};
use crate::utils::{parse_query_params, ApiResponse};
use crate::auth::check_rate_limit;

#[get("/api/register")]
pub async fn register(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // 检查限流
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "register") {
        return response;
    }

    // 检查系统维护模式
    let settings = match data.data_manager.load_settings() {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "系统配置加载失败".to_string(), 
            data: None 
        }),
    };
    
    // 如果系统处于维护模式，禁止注册
    if settings.system_mode == crate::models::SystemMode::Maintenance {
        return HttpResponse::ServiceUnavailable().json(ApiResponse::<()> { 
            code: 1, 
            msg: "系统维护中，暂时禁止注册".to_string(), 
            data: None 
        });
    }
    
    // 检查注册功能是否启用
    if !settings.feature_flags.enable_registration {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户注册功能已禁用".to_string(), 
            data: None 
        });
    }

    let params = parse_query_params(req.query_string());
    
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };
    
    let password = match params.get("password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少密码".to_string(), data: None }),
    };
    
    let nickname = match params.get("nickname") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少昵称".to_string(), data: None }),
    };

    let users = data.data_manager.get_users();
    if users.contains_key(username) {
        return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "用户已存在".to_string(), data: None });
    }

    let new_user = User {
        username: username.clone(),
        password: password.clone(),
        nickname: nickname.clone(),
        star: 1.0,
        role: crate::models::UserRole::Normal,
        permissions: crate::models::UserPermissions::default(),
        banned: false,
        ban_reason: None,
        ban_time: None,
        qq_number: None,
        avatar_url: None,
        sign_records: Vec::new(),
        sign_days: 0,
        sign_total: 0,
        last_sign: String::new(),
        register_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        last_login: String::new(),
        upload_count: 0,
        download_count: 0,
        is_admin: false,
    };

    if let Err(e) = data.data_manager.update_user(username.clone(), new_user) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: format!("注册失败: {}", e), data: None });
    }

    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "注册成功".to_string(), data: None })
}