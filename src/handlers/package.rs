use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, RopePackage};
use crate::utils::{parse_query_params, save_json};
use crate::auth::check_rate_limit;

// 添加绳包（星级大于3）
#[get("/api/add-rope-package")]
pub async fn add_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "add_rope_package") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("缺少用户名"),
    };
    let name = match params.get("绳包名称") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().body("缺少绳包名称"),
    };
    let author = match params.get("作者") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().body("缺少作者"),
    };
    let version = match params.get("版本") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().body("缺少版本"),
    };
    let desc = match params.get("简介") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().body("缺少简介"),
    };
    let url = match params.get("项目直链") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("缺少项目直链"),
    };

    let users = data.users.lock().unwrap();
    let user = match users.get(username) {
        Some(u) => u,
        None => return HttpResponse::NotFound().body("用户不存在"),
    };

    // 检查用户星级（暂时使用固定值3）
    if user.star < 3 {
        return HttpResponse::Forbidden().body("用户星级不足");
    }
    drop(users);

    let mut ropes = data.ropes.lock().unwrap();
    let id = ropes.len() as u32 + 1;
    ropes.insert(id, RopePackage {
        id,
        name: name.clone(),
        author: author.clone(),
        version: version.clone(),
        desc: desc.clone(),
        url: url.clone(),
        downloads: 0,
    });
    save_json("data/data.json", &*ropes);
    HttpResponse::Ok().body("绳包添加成功")
}

// 统计绳包下载量
#[get("/api/download-rope-package")]
pub async fn download_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "download_rope_package") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().body("缺少id"),
    };

    let mut ropes = data.ropes.lock().unwrap();
    if let Some(rope) = ropes.get_mut(&id) {
        rope.downloads += 1;
        save_json("data/data.json", &*ropes);
        
        let mut stats = data.stats.lock().unwrap();
        *stats.downloads.entry(id).or_insert(0) += 1;
        save_json("data/stats.json", &*stats);
        return HttpResponse::Ok().body("下载统计成功");
    }
    HttpResponse::NotFound().body("绳包不存在")
}

// 获取所有绳包
#[get("/api/get-data-db")]
pub async fn get_data_db(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "get_data_db") {
        return response;
    }

    let ropes = data.ropes.lock().unwrap();
    HttpResponse::Ok().json(&*ropes)
}
