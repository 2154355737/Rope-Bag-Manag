use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, RawRopePackage, Category};
use crate::utils::{parse_query_params, ApiResponse};
use crate::auth::admin_auth;
use crate::data_manager::DataManager;

// 版本号递增函数
fn bump_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 3 {
        let patch = parts[2].parse::<u32>().unwrap_or(0) + 1;
        format!("{}.{}.{}", parts[0], parts[1], patch)
    } else {
        format!("{}.0.1", version)
    }
}

// 管理员接口：查询用户所有信息（含密文密码）
#[get("/api/admin/user-info")]
pub async fn admin_user_info(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "管理员认证失败".to_string(), 
            data: None 
        });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("username") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少用户名".to_string(), 
            data: None 
        }),
    };

    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    if let Some(user) = users.get(target) {
        return HttpResponse::Ok().json(ApiResponse { 
            code: 0, 
            msg: "查询成功".to_string(), 
            data: Some(user) 
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    })
}

// 管理员设置用户昵称/密码
#[get("/api/admin/set-user")]
pub async fn admin_set_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "管理员认证失败".to_string(), 
            data: None 
        });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少目标".to_string(), 
            data: None 
        }),
    };

    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    if let Some(user) = users.get_mut(target) {
        if let Some(n) = params.get("nickname") {
            user.nickname = n.clone();
        }
        if let Some(p) = params.get("password") {
            user.password = p.clone();
        }
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存用户数据失败".to_string(), 
                data: None 
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "用户信息更新成功".to_string(), 
            data: None 
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    })
}

// 管理员设置用户星级
#[get("/api/admin/set-star")]
pub async fn admin_set_star(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "管理员认证失败".to_string(), 
            data: None 
        });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少目标".to_string(), 
            data: None 
        }),
    };
    let star = match params.get("star") {
        Some(s) => s.parse::<u8>().unwrap_or(1),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少星级".to_string(), 
            data: None 
        }),
    };

    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    if let Some(user) = users.get_mut(target) {
        user.star = star as f32;
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存用户数据失败".to_string(), 
                data: None 
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "用户星级更新成功".to_string(), 
            data: None 
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    })
}

// 管理员封禁/解封用户
#[get("/api/admin/ban-user")]
pub async fn admin_ban_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "管理员认证失败".to_string(), 
            data: None 
        });
    }

    let params = parse_query_params(req.query_string());
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少目标".to_string(), 
            data: None 
        }),
    };
    let ban = match params.get("banned") {
        Some(b) => *b == "true",
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少封禁状态".to_string(), 
            data: None 
        }),
    };

    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    if let Some(user) = users.get_mut(target) {
        user.banned = ban;
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存用户数据失败".to_string(), 
                data: None 
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "用户封禁状态更新成功".to_string(), 
            data: None 
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    })
}

// 管理员添加绳包
#[get("/api/admin/add-rope-package")]
pub async fn admin_add_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "管理员认证失败".to_string(), 
            data: None 
        });
    }

    let params = parse_query_params(req.query_string());
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少名称".to_string(), 
            data: None 
        }),
    };
    let author = match params.get("author") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少作者".to_string(), 
            data: None 
        }),
    };
    let version = match params.get("version") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少版本".to_string(), 
            data: None 
        }),
    };
    let desc = match params.get("desc") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少简介".to_string(), 
            data: None 
        }),
    };
    let url = match params.get("url") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少项目直链".to_string(), 
            data: None 
        }),
    };
    let category = params.get("category").unwrap_or(&"教程".to_string()).clone();
    let status = params.get("status").unwrap_or(&"已发布".to_string()).clone();

    // 读取原始数据库
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载绳包数据失败".to_string(), 
            data: None 
        }),
    };
    
    let new_id = raw_data.绳包列表.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    // 获取当前时间作为上架时间
    let upload_time = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    raw_data.绳包列表.push(RawRopePackage {
        id: new_id,
        绳包名称: name.clone(),
        作者: author.clone(),
        版本: version.clone(),
        简介: desc.clone(),
        项目直链: url.clone(),
        下载次数: 0,
        上架时间: upload_time.clone(),
        分类: category,
        状态: status,
        是否标星: false,
        标星时间: None,
        标星人: None,
    });
    
    // 自动更新数据库配置
    raw_data.数据库配置.数据库名称 = "结绳绳包数据库".to_string();
    raw_data.数据库配置.数据库项目 = raw_data.绳包列表.len() as u32;
    raw_data.数据库配置.数据库版本 = bump_version(&raw_data.数据库配置.数据库版本);
    raw_data.数据库配置.数据库更新时间 = chrono::Local::now().format("%Y%m%d").to_string();
    
    if let Err(_) = data_manager.save_raw_data(&raw_data) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "保存绳包数据失败".to_string(), 
            data: None 
        });
    }
    
    HttpResponse::Ok().json(ApiResponse::<()> { 
        code: 0, 
        msg: "绳包添加成功".to_string(), 
        data: None 
    })
}

// 管理员更新绳包
#[get("/api/admin/update-rope-package")]
pub async fn admin_update_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少id".to_string(), data: None }),
    };
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少名称".to_string(), data: None }),
    };
    let author = match params.get("author") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少作者".to_string(), data: None }),
    };
    let version = match params.get("version") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少版本".to_string(), data: None }),
    };
    let desc = match params.get("desc") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少简介".to_string(), data: None }),
    };
    let url = match params.get("url") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少项目直链".to_string(), data: None }),
    };
    let category = params.get("category").unwrap_or(&"教程".to_string()).clone();
    let status = params.get("status").unwrap_or(&"已发布".to_string()).clone();

    // 读取原始数据库
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "加载绳包数据失败".to_string(), data: None }),
    };
    let mut found = false;
    
    for rope in &mut raw_data.绳包列表 {
        if rope.id == id {
            rope.绳包名称 = name.clone();
            rope.作者 = author.clone();
            rope.版本 = version.clone();
            rope.简介 = desc.clone();
            rope.项目直链 = url.clone();
            rope.分类 = category;
            rope.状态 = status;
            found = true;
            break;
        }
    }
    
    if !found {
        return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "绳包不存在".to_string(), data: None });
    }
    
    // 自动更新数据库配置
    raw_data.数据库配置.数据库名称 = "结绳绳包数据库".to_string();
    raw_data.数据库配置.数据库项目 = raw_data.绳包列表.len() as u32;
    raw_data.数据库配置.数据库版本 = bump_version(&raw_data.数据库配置.数据库版本);
    raw_data.数据库配置.数据库更新时间 = chrono::Local::now().format("%Y%m%d").to_string();
    
    if let Err(_) = data_manager.save_raw_data(&raw_data) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "保存绳包数据失败".to_string(), data: None });
    }
    
    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包更新成功".to_string(), data: None })
}

// 管理员删除绳包
#[get("/api/admin/delete-rope-package")]
pub async fn admin_delete_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "管理员认证失败".to_string(), data: None });
    }

    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => i.parse::<u32>().unwrap_or(0),
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少id".to_string(), data: None }),
    };

    // 读取原始数据库
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "加载绳包数据失败".to_string(), data: None }),
    };
    let before = raw_data.绳包列表.len();
    raw_data.绳包列表.retain(|p| p.id != id);
    let after = raw_data.绳包列表.len();
    
    if before == after {
        return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "绳包不存在".to_string(), data: None });
    }
    
    // 自动更新数据库配置
    raw_data.数据库配置.数据库名称 = "结绳绳包数据库".to_string();
    raw_data.数据库配置.数据库项目 = raw_data.绳包列表.len() as u32;
    raw_data.数据库配置.数据库版本 = bump_version(&raw_data.数据库配置.数据库版本);
    raw_data.数据库配置.数据库更新时间 = chrono::Local::now().format("%Y%m%d").to_string();
    
    if let Err(_) = data_manager.save_raw_data(&raw_data) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "保存绳包数据失败".to_string(), data: None });
    }
    
    HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "绳包删除成功".to_string(), data: None })
}

// 设置管理员
#[get("/api/set-admin")]
pub async fn set_admin(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
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

    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "加载用户数据失败".to_string(), data: None }),
    };
    if let Some(user) = users.get_mut(target) {
        user.is_admin = is_admin;
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "保存用户数据失败".to_string(), data: None });
        }
        return HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "管理员状态更新成功".to_string(), data: None });
    }
    HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None })
}

// 分类管理相关API
// 获取分类列表
#[get("/api/admin/categories")]
pub async fn get_categories(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }
    match data_manager.load_categories() {
        Ok(categories) => HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "查询成功".to_string(),
            data: Some(categories)
        }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载分类数据失败".to_string(),
            data: None
        })
    }
}

// 添加分类
#[get("/api/admin/add-category")]
pub async fn add_category(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }
    let params = parse_query_params(req.query_string());
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少分类名称".to_string(),
            data: None
        }),
    };
    let description = match params.get("description") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少分类描述".to_string(),
            data: None
        }),
    };
    let enabled = match params.get("enabled") {
        Some(e) => *e == "true",
        None => true,
    };
    let mut categories = match data_manager.load_categories() {
        Ok(c) => c,
        Err(_) => Vec::new(),
    };
    let new_id = categories.iter().map(|c| c.id).max().unwrap_or(0) + 1;
    let new_category = Category {
        id: new_id,
        name: name.clone(),
        description: description.clone(),
        enabled,
        count: 0,
    };
    categories.push(new_category);
    if let Err(_) = data_manager.save_categories(&categories) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存分类失败".to_string(),
            data: None
        });
    }
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: "分类添加成功".to_string(),
        data: None
    })
}

// 更新分类
#[get("/api/admin/update-category")]
pub async fn update_category(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }
    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => match i.parse::<u32>() {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
                code: 1,
                msg: "无效的分类ID".to_string(),
                data: None
            }),
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少分类ID".to_string(),
            data: None
        }),
    };
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少分类名称".to_string(),
            data: None
        }),
    };
    let description = match params.get("description") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少分类描述".to_string(),
            data: None
        }),
    };
    let enabled = match params.get("enabled") {
        Some(e) => *e == "true",
        None => true,
    };
    let mut categories = match data_manager.load_categories() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载分类数据失败".to_string(),
            data: None
        }),
    };
    let mut found = false;
    for cat in &mut categories {
        if cat.id == id {
            cat.name = name.clone();
            cat.description = description.clone();
            cat.enabled = enabled;
            found = true;
            break;
        }
    }
    if !found {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "分类不存在".to_string(),
            data: None
        });
    }
    if let Err(_) = data_manager.save_categories(&categories) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存分类失败".to_string(),
            data: None
        });
    }
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: "分类更新成功".to_string(),
        data: None
    })
}

// 删除分类
#[get("/api/admin/delete-category")]
pub async fn delete_category(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    if !admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }
    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => match i.parse::<u32>() {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
                code: 1,
                msg: "无效的分类ID".to_string(),
                data: None
            }),
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少分类ID".to_string(),
            data: None
        }),
    };
    let mut categories = match data_manager.load_categories() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载分类数据失败".to_string(),
            data: None
        }),
    };
    // 找到要删除的分类名
    let (del_name, orig_len) = {
        let orig_len = categories.len();
        let del_name = categories.iter().find(|cat| cat.id == id).map(|cat| cat.name.clone());
        categories.retain(|cat| cat.id != id);
        (del_name, orig_len)
    };
    if categories.len() == orig_len {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "分类不存在".to_string(),
            data: None
        });
    }
    // 处理资源归类
    if let Some(del_name) = del_name {
        if let Ok(mut raw_data) = data_manager.load_raw_data() {
            let mut changed = false;
            for pkg in &mut raw_data.绳包列表 {
                if pkg.分类 == del_name {
                    pkg.分类 = "未分类".to_string();
                    changed = true;
                }
            }
            if changed {
                let _ = data_manager.save_raw_data(&raw_data);
            }
        }
        // 更新“未分类”计数
        if let Some(unclassified) = categories.iter_mut().find(|cat| cat.name == "未分类") {
            if let Ok(raw_data) = data_manager.load_raw_data() {
                unclassified.count = raw_data.绳包列表.iter().filter(|pkg| pkg.分类 == "未分类").count() as u32;
            }
        }
    }
    if let Err(_) = data_manager.save_categories(&categories) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存分类失败".to_string(),
            data: None
        });
    }
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: "分类删除成功".to_string(),
        data: None
    })
}
