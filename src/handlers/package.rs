use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, RopePackage};
use crate::utils::{parse_query_params, save_json, load_json};
use crate::auth::check_rate_limit;
use serde::Serialize;
use crate::models::{RawDataJson, RawRopePackage, DatabaseConfig};

#[derive(Serialize)]
struct ApiResponse<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

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
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };
    let admin_password = params.get("admin_password");
    let admin_username = &data.config.admin_username;
    let admin_pwd = &data.config.admin_password;
    if username == admin_username {
        match admin_password {
            Some(pwd) if pwd == admin_pwd => {},
            _ => return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员操作需提供正确密码".to_string(), data: None }),
        }
    }
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少name".to_string(), data: None }),
    };
    let author = match params.get("author") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少author".to_string(), data: None }),
    };
    let version = match params.get("version") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少version".to_string(), data: None }),
    };
    let desc = match params.get("desc") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少desc".to_string(), data: None }),
    };
    let url = match params.get("url") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少url".to_string(), data: None }),
    };
    let users = data.users.lock().unwrap();
    let user = match users.get(username) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None }),
    };
    if user.star < 3 {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "用户星级不足".to_string(), data: None });
    }
    drop(users);
    // 读取原始数据库
    let mut raw_data: RawDataJson = load_json("data/data.json");
    let new_id = raw_data.绳包列表.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    raw_data.绳包列表.push(RawRopePackage {
        id: new_id,
        绳包名称: name.clone(),
        作者: author.clone(),
        版本: version.clone(),
        简介: desc.clone(),
        项目直链: url.clone(),
    });
    save_json("data/data.json", &raw_data);
    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包添加成功".to_string(), data: None })
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
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少id".to_string(), data: None }),
    };

    let mut ropes = data.ropes.lock().unwrap();
    if let Some(rope) = ropes.get_mut(&id) {
        rope.downloads += 1;
        save_json("data/data.json", &*ropes);
        
        let mut stats = data.stats.lock().unwrap();
        *stats.downloads.entry(id).or_insert(0) += 1;
        save_json("data/stats.json", &*stats);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "下载统计成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "绳包不存在".to_string(), data: None })
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
    let raw_data: RawDataJson = load_json("data/data.json");
    HttpResponse::Ok().json(ApiResponse { code: 0, msg: "成功".to_string(), data: Some(raw_data) })
}

// 删除绳包（星级大于3）
#[get("/api/delete-rope-package")]
pub async fn delete_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "delete_rope_package") {
        return response;
    }
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };
    let admin_password = params.get("admin_password");
    let admin_username = &data.config.admin_username;
    let admin_pwd = &data.config.admin_password;
    if username == admin_username {
        match admin_password {
            Some(pwd) if pwd == admin_pwd => {},
            _ => return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员操作需提供正确密码".to_string(), data: None }),
        }
    }
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少id".to_string(), data: None }),
    };
    let users = data.users.lock().unwrap();
    let user = match users.get(username) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None }),
    };
    if user.star < 3 {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "用户星级不足".to_string(), data: None });
    }
    drop(users);
    let mut raw_data: RawDataJson = load_json("data/data.json");
    let before = raw_data.绳包列表.len();
    raw_data.绳包列表.retain(|p| p.id != id);
    let after = raw_data.绳包列表.len();
    if before == after {
        return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "绳包不存在".to_string(), data: None });
    }
    save_json("data/data.json", &raw_data);
    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包删除成功".to_string(), data: None })
}

// 修改绳包信息（星级大于3）
#[get("/api/update-rope-package")]
pub async fn update_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "update_rope_package") {
        return response;
    }
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None }),
    };
    let admin_password = params.get("admin_password");
    let admin_username = &data.config.admin_username;
    let admin_pwd = &data.config.admin_password;
    if username == admin_username {
        match admin_password {
            Some(pwd) if pwd == admin_pwd => {},
            _ => return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员操作需提供正确密码".to_string(), data: None }),
        }
    }
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少id".to_string(), data: None }),
    };
    let users = data.users.lock().unwrap();
    let user = match users.get(username) {
        Some(u) => u,
        None => return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None }),
    };
    if user.star < 3 {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "用户星级不足".to_string(), data: None });
    }
    drop(users);
    let mut raw_data: RawDataJson = load_json("data/data.json");
    let mut found = false;
    for rope in &mut raw_data.绳包列表 {
        if rope.id == id {
            if let Some(name) = params.get("name") { rope.绳包名称 = name.clone(); }
            if let Some(author) = params.get("author") { rope.作者 = author.clone(); }
            if let Some(version) = params.get("version") { rope.版本 = version.clone(); }
            if let Some(desc) = params.get("desc") { rope.简介 = desc.clone(); }
            if let Some(url) = params.get("url") { rope.项目直链 = url.clone(); }
            found = true;
            break;
        }
    }
    if !found {
        return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "绳包不存在".to_string(), data: None });
    }
    save_json("data/data.json", &raw_data);
    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包信息修改成功".to_string(), data: None })
}
