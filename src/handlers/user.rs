use actix_web::{web, HttpResponse, HttpRequest, get, post, Responder};
use crate::models::{AppState, UserRole, ApiResponse, UserInfoResponse, NicknameInfo};
use serde::{Deserialize};
use crate::data_manager::DataManager;
use crate::utils::{parse_query_params, now_ts, record_api_call};
use crate::auth::admin_auth;
use chrono::Utc;

// 查询用户信息（不含密码）
#[get("/api/user-info")]
pub async fn user_info(
    req: actix_web::HttpRequest,
    _data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let target = match params.get("username") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少用户名".to_string(), 
            data: None 
        }),
    };

    let data_manager = DataManager::new();
    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    if let Some(user) = users.get(target) {
        let response = UserInfoResponse {
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            star: user.star,
            role: user.role.clone(),
            banned: user.banned,
            ban_reason: user.ban_reason.clone(),
            qq_number: user.qq_number.clone(),
            avatar_url: user.avatar_url.clone(),
            sign_days: user.sign_days,
            sign_total: user.sign_total,
            last_sign: user.last_sign.clone(),
            register_time: user.register_time.clone(),
            last_login: user.last_login.clone(),
            upload_count: user.upload_count,
            download_count: user.download_count,
            permissions: user.get_permissions(),
            is_admin: user.is_admin,
        };
        return HttpResponse::Ok().json(ApiResponse { 
            code: 0, 
            msg: "查询成功".to_string(), 
            data: Some(response) 
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> { 
        code: 1, 
        msg: "用户不存在".to_string(), 
        data: None 
    })
}

// 用户签到（兼容旧接口）
#[get("/api/sign-in")]
pub async fn sign_in(
    req: actix_web::HttpRequest,
    _data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };

    let data_manager = DataManager::new();
    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get_mut(username) {
        let (success, message) = user.sign_in();
        
        if success {
            // 先克隆用户数据，避免借用冲突
            let user_clone = user.clone();
            
            // 保存更新后的用户数据
            if let Err(_) = data_manager.save_users(&users) {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 1,
                    msg: "保存用户数据失败".to_string(),
                    data: None
                });
            }
            
            return HttpResponse::Ok().json(ApiResponse {
                code: 0,
                msg: message,
                data: Some(user_clone)
            });
        } else {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                code: 1,
                msg: message,
                data: None
            });
        }
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 修改密码
#[get("/api/change-password")]
pub async fn change_password(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };
    let old_password = match params.get("old_password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少旧密码".to_string(),
            data: None
        }),
    };
    let new_password = match params.get("new_password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少新密码".to_string(),
            data: None
        }),
    };

    let data_manager = DataManager::new();
    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get_mut(username) {
        if user.banned {
            return HttpResponse::Forbidden().json(ApiResponse::<()> {
                code: 1,
                msg: "用户已被封禁".to_string(),
                data: None
            });
        }
        
        if user.password != *old_password {
            return HttpResponse::Forbidden().json(ApiResponse::<()> {
                code: 1,
                msg: "旧密码错误".to_string(),
                data: None
            });
        }
        
        user.password = new_password.clone();
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse::<()> {
            code: 0,
            msg: "密码修改成功".to_string(),
            data: None
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 修改昵称
#[get("/api/change-nickname")]
pub async fn change_nickname(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };
    let nickname = match params.get("nickname") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少昵称".to_string(),
            data: None
        }),
    };

    let data_manager = DataManager::new();
    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get_mut(username) {
        if user.banned {
            return HttpResponse::Forbidden().json(ApiResponse::<()> {
                code: 1,
                msg: "用户已被封禁".to_string(),
                data: None
            });
        }
        
        user.nickname = nickname.clone();
        
        // 先克隆用户数据，避免借用冲突
        let user_clone = user.clone();
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "昵称修改成功".to_string(),
            data: Some(user_clone)
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 用户昵称列表及星级
#[get("/api/nicknames")]
pub async fn nicknames(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let list: Vec<NicknameInfo> = users.values()
        .map(|u| NicknameInfo {
            nickname: u.nickname.clone(),
            star: u.star,
            role: u.role.clone(),
            avatar_url: u.avatar_url.clone(),
        })
        .collect();

    HttpResponse::Ok().json(ApiResponse { 
        code: 0,
        msg: "查询成功".to_string(), 
        data: Some(list) 
    })
}

// 绑定QQ号码
#[post("/api/user/bind-qq")]
pub async fn bind_qq(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };
    
    let qq_number = match params.get("qq_number") {
        Some(q) => q,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少QQ号码".to_string(),
            data: None
        }),
    };

    // 检查系统维护模式
    let settings = match data.data_manager.load_settings() {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "系统配置加载失败".to_string(), 
            data: None 
        }),
    };
    
    // 如果系统处于维护模式，禁止QQ绑定
    if settings.system_mode == crate::models::SystemMode::Maintenance {
        return HttpResponse::ServiceUnavailable().json(ApiResponse::<()> { 
            code: 1, 
            msg: "系统维护中，暂时禁止QQ绑定".to_string(), 
            data: None 
        });
    }
    
    // 检查QQ绑定功能是否启用
    if !settings.feature_flags.enable_qq_binding {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "QQ绑定功能已禁用".to_string(), 
            data: None 
        });
    }

    let data_manager = DataManager::new();
    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get_mut(username) {
        user.set_qq(qq_number.clone());
        
        // 先克隆用户数据，避免借用冲突
        let user_clone = user.clone();
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "QQ绑定成功".to_string(),
            data: Some(user_clone)
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 获取用户权限
#[get("/api/user/permissions")]
pub async fn get_user_permissions(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };

    let data_manager = DataManager::new();
    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get(username) {
        let permissions = user.get_permissions();
        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "获取权限成功".to_string(),
            data: Some(permissions)
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 检查用户权限
#[get("/api/user/check-permission")]
pub async fn check_user_permission(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少用户名".to_string(),
            data: None
        }),
    };
    
    let action = match params.get("action") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少操作类型".to_string(),
            data: None
        }),
    };

    let data_manager = DataManager::new();
    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(user) = users.get(username) {
        let can_perform = user.can_perform(action);
        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: if can_perform { "权限检查通过".to_string() } else { "权限不足".to_string() },
            data: Some(can_perform)
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 设置用户身份（仅管理员）
#[post("/api/user/set-role")]
pub async fn set_user_role(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
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
            msg: "缺少身份类型".to_string(),
            data: None
        }),
    };
    
    let admin_username = match params.get("admin_username") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少管理员用户名".to_string(),
            data: None
        }),
    };
    
    let admin_password = match params.get("admin_password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少管理员密码".to_string(),
            data: None
        }),
    };

    // 验证管理员权限
    let data_manager = DataManager::new();
    if !crate::auth::admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }

    let role = match role_str.as_str() {
        "normal" => UserRole::Normal,
        "developer" => UserRole::Developer,
        "elder" => UserRole::Elder,
        _ => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "无效的身份类型".to_string(),
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
        user.role = role;
        user.permissions = user.get_permissions();
        
        // 先克隆用户数据，避免借用冲突
        let user_clone = user.clone();
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "用户身份设置成功".to_string(),
            data: Some(user_clone)
        });
    }
    
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 1,
        msg: "用户不存在".to_string(),
        data: None
    })
}

// 标星绳包（仅管理员）
#[post("/api/user/star-package")]
pub async fn star_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let package_id = match params.get("package_id") {
        Some(p) => match p.parse::<u32>() {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
                code: 1,
                msg: "无效的绳包ID".to_string(),
                data: None
            }),
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少绳包ID".to_string(),
            data: None
        }),
    };
    
    let admin_username = match params.get("admin_username") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少管理员用户名".to_string(),
            data: None
        }),
    };
    
    let admin_password = match params.get("admin_password") {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少管理员密码".to_string(),
            data: None
        }),
    };

    // 验证管理员权限
    let data_manager = DataManager::new();
    if !crate::auth::admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }

    // 加载绳包数据
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载绳包数据失败".to_string(),
            data: None
        }),
    };

    // 查找并标星绳包
    let mut package_author = None;
    for package in &mut raw_data.绳包列表 {
        if package.id == package_id {
            package.是否标星 = true;
            package.标星时间 = Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
            package.标星人 = Some(admin_username.clone());
            package_author = Some(package.作者.clone());
            break;
        }
    }

    if package_author.is_none() {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "绳包不存在".to_string(),
            data: None
        });
    }

    // 保存绳包数据
    if let Err(_) = data_manager.save_raw_data(&raw_data) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存绳包数据失败".to_string(),
            data: None
        });
    }

    // 给作者增加星级
    let mut users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    if let Some(author) = users.get_mut(&package_author.unwrap()) {
        author.add_star(1.0);
        
        if let Err(_) = data_manager.save_users(&users) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }
    }

    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: "绳包标星成功，作者星级+1".to_string(),
        data: None
    })
}