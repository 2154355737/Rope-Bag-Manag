use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState};
use crate::utils::{parse_query_params, save_json, ApiResponse};
use crate::auth::admin_auth;

// 管理员接口：查询用户所有信息（含密文密码）
#[get("/api/admin/user-info")]
pub async fn admin_user_info(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少目标".to_string(), data: None }),
    };

    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(target) {
        return HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(user) });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 管理员设置用户昵称/密码
#[get("/api/admin/set-user")]
pub async fn admin_set_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少目标".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        if let Some(n) = params.get("nickname") {
            user.nickname = n.clone();
        }
        if let Some(p) = params.get("password") {
            user.password = p.clone();
        }
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "用户信息更新成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 管理员设置用户星级
#[get("/api/admin/set-star")]
pub async fn admin_set_star(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少目标".to_string(), data: None }),
    };
    let star = match params.get("star") {
        Some(s) => s.parse::<u8>().unwrap_or(1),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少星级".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        user.star = star;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "用户星级更新成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 管理员封禁/解封用户
#[get("/api/admin/ban-user")]
pub async fn admin_ban_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少目标".to_string(), data: None }),
    };
    let ban = match params.get("banned") {
        Some(b) => *b == "true",
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少封禁状态".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        user.banned = ban;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "用户封禁状态更新成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 管理员添加绳包
#[get("/api/admin/add-rope-package")]
pub async fn admin_add_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let name = match params.get("绳包名称") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少名称".to_string(), data: None }),
    };
    let author = match params.get("作者") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少作者".to_string(), data: None }),
    };
    let version = match params.get("版本") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少版本".to_string(), data: None }),
    };
    let desc = match params.get("简介") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少简介".to_string(), data: None }),
    };
    let url = match params.get("项目直链") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少项目直链".to_string(), data: None }),
    };

    let mut ropes = data.ropes.lock().unwrap();
    let id = ropes.len() as u32 + 1;
    // 获取当前时间作为上架时间
    let upload_time = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    ropes.insert(id, crate::models::RopePackage {
        id,
        name: name.clone(),
        author: author.clone(),
        version: version.clone(),
        desc: desc.clone(),
        url: url.clone(),
        downloads: 0,
        upload_time,
    });
    save_json("data/data.json", &*ropes);
    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包添加成功".to_string(), data: None })
}

// 管理员删除绳包
#[get("/api/admin/delete-rope-package")]
pub async fn admin_delete_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少id".to_string(), data: None }),
    };

    let mut ropes = data.ropes.lock().unwrap();
    if ropes.remove(&id).is_some() {
        save_json("data/data.json", &*ropes);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包删除成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "绳包不存在".to_string(), data: None })
}

// 设置管理员
#[get("/api/set-admin")]
pub async fn set_admin(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少目标".to_string(), data: None }),
    };
    let is_admin = match params.get("is_admin") {
        Some(i) => *i == "true",
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少管理员状态".to_string(), data: None }),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        user.is_admin = is_admin;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "管理员状态更新成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}
