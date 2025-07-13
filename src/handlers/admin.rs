use actix_web::{web, HttpResponse, get, Responder};
use crate::models::{AppState, ApiResponse, BanStatus};
use crate::utils::{parse_query_params};
use crate::auth::admin_auth;
use crate::data_manager::DataManager;

// 版本号递增函数
fn bump_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 3 {
        let patch = parts[2].parse::<u32>().unwrap_or(0) + 1;
        format!("{}.{}.{}", parts[0], parts[1], patch)
    } else {
        version.to_string()
    }
}

// 管理员获取用户信息
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
    let target = match params.get("target") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少目标".to_string(), 
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
            msg: "获取用户信息成功".to_string(), 
            data: Some(user)
        });
    }
    
    return HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    });
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
        Some(s) => s.parse::<u32>().unwrap_or(1),
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

    // 通过用户名查找用户
    let mut target_user = None;
    for (_key, user) in &mut users {
        if user.username == *target {
            target_user = Some(user);
            break;
        }
    }

    if let Some(user) = target_user {
        user.star = star;
        user.update_role();
        
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
    
    return HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    });
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

    // 通过用户名查找用户
    let mut target_user = None;
    for (_key, user) in &mut users {
        if user.username == *target {
            target_user = Some(user);
            break;
        }
    }

    if let Some(user) = target_user {
        if ban {
            user.ban_status = BanStatus::Banned;
            user.ban_reason = Some("管理员封禁".to_string());
        } else {
            user.ban_status = BanStatus::Normal;
            user.ban_reason = None;
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
            msg: "用户封禁状态更新成功".to_string(), 
            data: None 
        });
    }
    
    return HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    });
}

// 获取用户数据库
#[get("/api/get-users-db")]
pub async fn get_users_db(
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

    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    return HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "获取用户数据库成功".to_string(), 
        data: Some(users)
    });
}

// 设置管理员
#[get("/api/admin/set-admin")]
pub async fn set_admin(
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

    // 通过用户名查找用户
    let mut target_user = None;
    for (_key, user) in &mut users {
        if user.username == *target {
            target_user = Some(user);
            break;
        }
    }

    if let Some(user) = target_user {
        user.is_admin = true;
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存用户数据失败".to_string(), 
                data: None 
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "设置管理员成功".to_string(), 
            data: None 
        });
    }
    
    return HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    });
}

// 管理员设置用户信息
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
            msg: "缺少目标用户".to_string(), 
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

    // 通过用户名查找用户
    let mut target_user = None;
    let mut target_key = None;
    for (key, user) in &mut users {
        if user.username == *target {
            target_user = Some(user);
            target_key = Some(key.clone());
            break;
        }
    }

    if let Some(user) = target_user {
        // 更新现有用户 - 密码是可选的
        if let Some(password) = params.get("password") {
            user.password = password.clone();
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
    } else {
        // 创建新用户 - 密码是必需的
        let password = match params.get("password") {
            Some(p) => p,
            None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
                code: 1, 
                msg: "缺少密码".to_string(), 
                data: None 
            }),
        };
        
        match data_manager.create_user(target.clone(), password.clone()) {
            Ok(_) => {
                return HttpResponse::Ok().json(ApiResponse::<()> { 
                    code: 0, 
                    msg: "用户创建成功".to_string(), 
                    data: None 
                });
            },
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                    code: 1, 
                    msg: format!("创建用户失败: {}", e), 
                    data: None 
                });
            }
        }
    }
}

// 管理员设置用户角色
#[get("/api/admin/set-role")]
pub async fn admin_set_role(
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
            msg: "缺少目标用户".to_string(), 
            data: None 
        }),
    };

    let role_str = match params.get("role") {
        Some(r) => r,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少角色参数".to_string(), 
            data: None 
        }),
    };

    let role = match role_str.as_str() {
        "normal" => crate::models::UserRole::Normal,
        "developer" => crate::models::UserRole::Developer,
        "elder" => crate::models::UserRole::Elder,
        _ => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "无效的角色".to_string(), 
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

    // 通过用户名查找用户
    let mut target_user = None;
    for (key, user) in &mut users {
        if user.username == *target {
            target_user = Some(user);
            break;
        }
    }

    if let Some(user) = target_user {
        user.role = role;
        user.permissions = user.get_permissions();
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存用户数据失败".to_string(), 
                data: None 
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "用户角色设置成功".to_string(), 
            data: None 
        });
    }
    
    return HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    });
}

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

    let categories = match data_manager.load_categories() {
        Ok(categories) => categories,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载分类数据失败".to_string(), 
            data: None 
        }),
    };

    return HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "获取分类列表成功".to_string(), 
        data: Some(categories)
    });
}
