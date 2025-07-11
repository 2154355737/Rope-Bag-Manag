use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, User};
use crate::utils::{parse_query_params, save_json};
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

    let params = parse_query_params(req.query_string());
    
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("缺少用户名"),
    };
    
    let password = match params.get("password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().body("缺少密码"),
    };
    
    let nickname = match params.get("nickname") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().body("缺少昵称"),
    };

    let mut users = data.users.lock().unwrap();
    
    if users.contains_key(username) {
        return HttpResponse::BadRequest().body("用户已存在");
    }

    let new_user = User {
        username: username.clone(),
        password: password.clone(),
        nickname: nickname.clone(),
        star: 1,
        banned: false,
        sign_days: 0,
        sign_total: 0,
        last_sign: String::new(),
        is_admin: false,
    };

    users.insert(username.clone(), new_user);
    save_json("data/users.json", &*users);

    HttpResponse::Ok().body("注册成功")
}