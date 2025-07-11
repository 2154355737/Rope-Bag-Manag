use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState};
use crate::utils::{parse_query_params, save_json};
use crate::auth::admin_auth;

// 管理员接口：查询用户所有信息（含密文密码）
#[get("/api/admin/user-info")]
pub async fn admin_user_info(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("缺少目标"),
    };

    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(target) {
        return HttpResponse::Ok().json(user);
    }
    HttpResponse::NotFound().body("用户不存在")
}

// 管理员设置用户昵称/密码
#[get("/api/admin/set-user")]
pub async fn admin_set_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("缺少目标"),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        if let Some(n) = params.get("new_name") {
            user.nickname = n.clone();
        }
        if let Some(p) = params.get("new_password") {
            user.password = p.clone();
        }
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().body("用户信息更新成功");
    }
    HttpResponse::NotFound().body("用户不存在")
}

// 管理员设置用户星级
#[get("/api/admin/set-star")]
pub async fn admin_set_star(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("缺少目标"),
    };
    let star = match params.get("star") {
        Some(s) => s.parse::<u8>().unwrap_or(1),
        None => return HttpResponse::BadRequest().body("缺少星级"),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        user.star = star;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().body("用户星级更新成功");
    }
    HttpResponse::NotFound().body("用户不存在")
}

// 管理员封禁/解封用户
#[get("/api/admin/ban-user")]
pub async fn admin_ban_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("缺少目标"),
    };
    let ban = match params.get("ban") {
        Some(b) => *b == "true",
        None => return HttpResponse::BadRequest().body("缺少封禁状态"),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        user.banned = ban;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().body("用户封禁状态更新成功");
    }
    HttpResponse::NotFound().body("用户不存在")
}

// 管理员添加绳包
#[get("/api/admin/add-rope-package")]
pub async fn admin_add_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let name = match params.get("绳包名称") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().body("缺少名称"),
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

    let mut ropes = data.ropes.lock().unwrap();
    let id = ropes.len() as u32 + 1;
    ropes.insert(id, crate::models::RopePackage {
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

// 管理员删除绳包
#[get("/api/admin/delete-rope-package")]
pub async fn admin_delete_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().body("缺少id"),
    };

    let mut ropes = data.ropes.lock().unwrap();
    if ropes.remove(&id).is_some() {
        save_json("data/data.json", &*ropes);
        return HttpResponse::Ok().body("绳包删除成功");
    }
    HttpResponse::NotFound().body("绳包不存在")
}

// 设置管理员
#[get("/api/set-admin")]
pub async fn set_admin(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if !admin_auth(&req, &data.config, &data.users) {
        return HttpResponse::Forbidden().body("管理员认证失败");
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("缺少目标"),
    };
    let is_admin = match params.get("is_admin") {
        Some(i) => *i == "true",
        None => return HttpResponse::BadRequest().body("缺少管理员状态"),
    };

    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.get_mut(target) {
        user.is_admin = is_admin;
        save_json("data/users.json", &*users);
        return HttpResponse::Ok().body("管理员状态更新成功");
    }
    HttpResponse::NotFound().body("用户不存在")
}
