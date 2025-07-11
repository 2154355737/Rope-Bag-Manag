use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState};
use crate::auth::check_rate_limit;

// 查询所有绳包下载量
#[get("/api/stats/downloads")]
pub async fn stats_downloads(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "stats_downloads") {
        return response;
    }

    let stats = data.stats.lock().unwrap();
    HttpResponse::Ok().json(&stats.downloads)
}

// 查询所有接口访问次数
#[get("/api/stats/api-counts")]
pub async fn stats_api_counts(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "stats_api_counts") {
        return response;
    }

    let stats = data.stats.lock().unwrap();
    HttpResponse::Ok().json(&stats.api_counts)
}

// 获取用户数据库
#[get("/api/get-users-db")]
pub async fn get_users_db(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "get_users_db") {
        return response;
    }

    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}
