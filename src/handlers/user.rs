use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, UserInfoResponse, NicknameInfo};
use crate::utils::{parse_query_params, save_json, today};
use crate::auth::check_rate_limit;
use crate::utils::ApiResponse;

// 查询用户信息（不含密码）
#[get("/api/user-info")]
pub async fn user_info(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "user_info") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少目标用户".to_string(), data: None }),
    };

    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(target) {
        let response = UserInfoResponse {
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            star: user.star,
            banned: user.banned,
            sign_days: user.sign_days,
            sign_total: user.sign_total,
        };
        return HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(response) });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 用户签到
#[get("/api/sign-in")]
pub async fn sign_in(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "sign_in") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(username) {
        if user.banned {
            return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "用户已被封禁".to_string(), data: None });
        }
        let today_str = today();
        if user.last_sign == today_str {
            return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "今日已签到".to_string(), data: None });
        }
        user.sign_total += 1;
        user.last_sign = today_str;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "签到成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 修改密码
#[get("/api/change-password")]
pub async fn change_password(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "change_password") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };
    let old_password = match params.get("old_password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少旧密码".to_string(), data: None }),
    };
    let new_password = match params.get("new_password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少新密码".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(username) {
        if user.password != *old_password {
            return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "旧密码错误".to_string(), data: None });
        }
        user.password = new_password.clone();
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "密码修改成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 修改昵称
#[get("/api/change-nickname")]
pub async fn change_nickname(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "change_nickname") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };
    let nickname = match params.get("nickname") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少昵称".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(username) {
        user.nickname = nickname.clone();
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "昵称修改成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 用户昵称列表及星级
#[get("/api/nicknames")]
pub async fn nicknames(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "nicknames") {
        return response;
    }

    let users = data.users.lock().unwrap();
    let list: Vec<NicknameInfo> = users.values()
        .map(|u| NicknameInfo {
            nickname: u.nickname.clone(),
            star: u.star,
        })
        .collect();
    HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(list) })
}