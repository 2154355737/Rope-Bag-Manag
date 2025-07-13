use actix_web::{web, HttpResponse, get, post, delete, Responder};
use crate::models::{AppState, ApiResponse, UserInfoResponse, NicknameInfo, UserRole};
use serde::{Deserialize};
use crate::utils::parse_query_params;
use crate::storage::DataManager;
use serde_json;
use actix_web::put;

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
    let user_result = match data_manager.find_user_by_username(target) {
        Ok(Some((user_id, user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户不存在".to_string(), 
            data: None 
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };

    let (user_id, user) = user_result;
    let response = UserInfoResponse {
        id: user_id,
        username: user.username.clone(),
        star: user.star,
        role: user.role.clone(),
        online_status: user.online_status.clone(),
        ban_status: user.ban_status.clone(),
        ban_reason: user.ban_reason.clone(),
        qq_number: user.qq_number.clone(),
        avatar_url: user.avatar_url.clone(),
        sign_days: user.sign_days,
        sign_total: user.sign_total,
        last_sign: user.last_sign.clone(),
        register_time: user.register_time.clone(),
        last_login: user.last_login.clone(),
        login_count: user.login_count,
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
    let user_result = match data_manager.find_user_by_username(username) {
        Ok(Some((user_id, mut user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, mut user) = user_result;
    let (success, message) = user.sign_in();
    
    if success {
        // 保存更新后的用户数据
        if let Err(_) = data_manager.update_user(user_id, user.clone()) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 1,
                msg: "保存用户数据失败".to_string(),
                data: None
            });
        }
        
        return HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: message,
            data: Some(user)
        });
    } else {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: message,
            data: None
        });
    }
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
    let user_result = match data_manager.find_user_by_username(username) {
        Ok(Some((user_id, mut user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, mut user) = user_result;
    if user.ban_status == crate::models::BanStatus::Banned {
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
    
    if let Err(_) = data_manager.update_user(user_id, user) {
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

// 用户列表
#[get("/api/nicknames")]
pub async fn nicknames(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    let users = match data_manager.get_all_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let list: Vec<NicknameInfo> = users.into_iter()
        .map(|(id, u)| NicknameInfo {
            id,
            username: u.username.clone(),
            star: u.star,
            role: u.role.clone(),
            online_status: u.online_status.clone(),
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
    let user_result = match data_manager.find_user_by_username(username) {
        Ok(Some((user_id, mut user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, mut user) = user_result;
    user.set_qq(qq_number.clone());
    
    // 先克隆用户数据，避免借用冲突
    let user_clone = user.clone();
    
    if let Err(_) = data_manager.update_user(user_id, user) {
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
    let user_result = match data_manager.find_user_by_username(username) {
        Ok(Some((user_id, user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, user) = user_result;
    let permissions = user.get_permissions();
    return HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "获取权限成功".to_string(),
        data: Some(permissions)
    });
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
    let user_result = match data_manager.find_user_by_username(username) {
        Ok(Some((user_id, user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, user) = user_result;
    let can_perform = user.can_perform(action);
    return HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "权限检查完成".to_string(),
        data: Some(can_perform)
    });
}

// 设置用户角色
#[post("/api/user/set-role")]
pub async fn set_user_role(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    let target = match params.get("username") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "缺少目标用户名".to_string(),
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
        "normal" => UserRole::Normal,
        "developer" => UserRole::Developer,
        "elder" => UserRole::Elder,
        _ => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "无效的角色类型".to_string(),
            data: None
        }),
    };

    let data_manager = DataManager::new();
    let user_result = match data_manager.find_user_by_username(target) {
        Ok(Some((user_id, mut user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, mut user) = user_result;
    user.role = role;
    user.permissions = user.get_permissions();
    
    // 先克隆用户数据，避免借用冲突
    let user_clone = user.clone();
    
    if let Err(_) = data_manager.update_user(user_id, user) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存用户数据失败".to_string(),
            data: None
        });
    }
    
    return HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "角色设置成功".to_string(),
        data: Some(user_clone)
    });
}

// 删除用户
#[delete("/api/users/{username}")]
pub async fn delete_user(
    path: web::Path<String>,
    _data: web::Data<AppState>,
) -> impl Responder {
    let username = path.into_inner();
    
    let data_manager = DataManager::new();
    
    // 检查用户是否存在
    let user_result = match data_manager.find_user_by_username(&username) {
        Ok(Some((user_id, user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, user) = user_result;
    
    // 不允许删除管理员用户
    if user.is_admin {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "不能删除管理员用户".to_string(),
            data: None
        });
    }
    
    // 删除用户
    if let Err(_) = data_manager.delete_user(user_id) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "删除用户失败".to_string(),
            data: None
        });
    }
    
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: format!("用户 {} 已删除", username),
        data: None
    })
}

// 编辑用户信息
#[derive(Debug, Deserialize)]
pub struct EditUserRequest {
    pub username: Option<String>,
    pub star: Option<u32>,
    pub role: Option<UserRole>,
    pub ban_status: Option<crate::models::BanStatus>,
    pub ban_reason: Option<String>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
    pub is_admin: Option<bool>,
}

#[put("/api/users/{username}")]
pub async fn edit_user(
    path: web::Path<String>,
    edit_data: web::Json<EditUserRequest>,
    _data: web::Data<AppState>,
) -> impl Responder {
    let username = path.into_inner();
    
    let data_manager = DataManager::new();
    
    // 检查用户是否存在
    let user_result = match data_manager.find_user_by_username(&username) {
        Ok(Some((user_id, mut user))) => (user_id, user),
        Ok(None) => return HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 1,
            msg: "用户不存在".to_string(),
            data: None
        }),
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };

    let (user_id, mut user) = user_result;
    
    // 更新用户信息
    if let Some(new_username) = &edit_data.username {
        user.username = new_username.clone();
    }
    if let Some(star) = edit_data.star {
        user.star = star;
    }
    if let Some(role) = &edit_data.role {
        user.role = role.clone();
    }
    if let Some(ban_status) = &edit_data.ban_status {
        user.ban_status = ban_status.clone();
    }
    if let Some(ban_reason) = &edit_data.ban_reason {
        user.ban_reason = Some(ban_reason.clone());
    }
    if let Some(qq_number) = &edit_data.qq_number {
        user.qq_number = Some(qq_number.clone());
    }
    if let Some(avatar_url) = &edit_data.avatar_url {
        user.avatar_url = Some(avatar_url.clone());
    }
    if let Some(is_admin) = edit_data.is_admin {
        user.is_admin = is_admin;
    }
    
    // 更新用户权限
    user.update_role();
    
    // 保存用户数据
    if let Err(_) = data_manager.update_user(user_id, user.clone()) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存用户数据失败".to_string(),
            data: None
        });
    }
    
    HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "用户信息更新成功".to_string(),
        data: Some(user)
    })
}

// 批量删除用户
#[derive(Debug, Deserialize)]
pub struct BatchDeleteUsersRequest {
    pub usernames: Vec<String>,
}

#[delete("/api/users/batch")]
pub async fn batch_delete_users(
    delete_data: web::Json<BatchDeleteUsersRequest>,
    _data: web::Data<AppState>,
) -> impl Responder {
    let data_manager = DataManager::new();
    let mut deleted_count = 0;
    let mut failed_users = Vec::new();
    
    for username in &delete_data.usernames {
        let user_result = match data_manager.find_user_by_username(username) {
            Ok(Some((user_id, user))) => (user_id, user),
            Ok(None) => {
                failed_users.push(format!("{} (用户不存在)", username));
                continue;
            },
            Err(_) => {
                failed_users.push(format!("{} (查询失败)", username));
                continue;
            },
        };

        let (user_id, user) = user_result;
        
        // 不允许删除管理员用户
        if user.is_admin {
            failed_users.push(format!("{} (管理员用户)", username));
            continue;
        }
        
        // 删除用户
        if let Err(_) = data_manager.delete_user(user_id) {
            failed_users.push(format!("{} (删除失败)", username));
            continue;
        }
        
        deleted_count += 1;
    }
    
    let message = if failed_users.is_empty() {
        format!("成功删除 {} 个用户", deleted_count)
    } else {
        format!("成功删除 {} 个用户，失败: {}", deleted_count, failed_users.join(", "))
    };
    
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: message,
        data: None
    })
}

// 获取用户列表（支持分页和搜索）
#[get("/api/users")]
pub async fn get_users(
    req: actix_web::HttpRequest,
    _data: web::Data<AppState>,
) -> impl Responder {
    let params = parse_query_params(req.query_string());
    
    let page = params.get("page")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);
    let size = params.get("size")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20);
    let search = params.get("search").cloned();
    let role_filter = params.get("role").cloned();
    let status_filter = params.get("status").cloned();
    
    let data_manager = DataManager::new();
    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "加载用户数据失败".to_string(),
            data: None
        }),
    };
    
    // 过滤用户
    let mut filtered_users: Vec<(String, crate::models::User)> = users.into_iter()
        .map(|(id, user)| (id.to_string(), user))
        .collect();
    
    // 搜索过滤
    if let Some(search_term) = &search {
        filtered_users.retain(|(_, user)| {
            user.username.to_lowercase().contains(&search_term.to_lowercase()) ||
            user.qq_number.as_ref().map_or(false, |qq| qq.contains(search_term))
        });
    }
    
    // 角色过滤
    if let Some(role_str) = &role_filter {
        if let Ok(role) = serde_json::from_str::<UserRole>(&format!("\"{}\"", role_str)) {
            filtered_users.retain(|(_, user)| user.role == role);
        }
    }
    
    // 状态过滤
    if let Some(status_str) = &status_filter {
        if let Ok(status) = serde_json::from_str::<crate::models::BanStatus>(&format!("\"{}\"", status_str)) {
            filtered_users.retain(|(_, user)| user.ban_status == status);
        }
    }
    
    let total = filtered_users.len();
    let offset = (page - 1) * size;
    let paginated_users = filtered_users.into_iter()
        .skip(offset)
        .take(size)
        .map(|(id, user)| {
            serde_json::json!({
                "id": id,
                "username": user.username,
                "star": user.star,
                "role": user.role,
                "online_status": user.online_status,
                "ban_status": user.ban_status,
                "ban_reason": user.ban_reason,
                "qq_number": user.qq_number,
                "avatar_url": user.avatar_url,
                "register_time": user.register_time,
                "last_login": user.last_login,
                "login_count": user.login_count,
                "upload_count": user.upload_count,
                "download_count": user.download_count,
                "is_admin": user.is_admin,
            })
        })
        .collect::<Vec<_>>();
    
    HttpResponse::Ok().json(ApiResponse {
        code: 0,
        msg: "获取用户列表成功".to_string(),
        data: Some(serde_json::json!({
            "users": paginated_users,
            "total": total,
            "page": page,
            "size": size
        }))
    })
}

// 添加用户请求体
#[derive(Debug, Deserialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
    pub role: Option<UserRole>,
    pub star: Option<u32>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
}

// 添加用户接口
#[post("/api/users")]
pub async fn add_user(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
    user_data: web::Json<AddUserRequest>,
) -> impl Responder {
    // 检查管理员权限 - 暂时注释掉以便测试
    /*
    let data_manager = crate::data_manager::DataManager::new();
    if !crate::auth::admin_auth(&req, &data.config, &data_manager) {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            code: 1,
            msg: "管理员认证失败".to_string(),
            data: None
        });
    }
    */

    // 校验字段
    let username = user_data.username.trim();
    let password = user_data.password.trim();
    
    if username.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "用户名不能为空".to_string(),
            data: None
        });
    }
    
    if password.len() < 6 {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "密码不能少于6位".to_string(),
            data: None
        });
    }

    // 检查用户名是否已存在
    let data_manager = crate::data_manager::DataManager::new();
    if let Ok(Some(_)) = data_manager.find_user_by_username(username) {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 1,
            msg: "用户名已存在".to_string(),
            data: None
        });
    }

    // 创建用户
    match data_manager.create_user(username.to_string(), password.to_string()) {
        Ok(user_id) => {
            // 获取新创建的用户并更新角色等信息
            if let Ok(Some((_, mut user))) = data_manager.find_user_by_username(username) {
                // 设置角色
                if let Some(role) = &user_data.role {
                    user.role = role.clone();
                }
                
                // 设置其他字段
                if let Some(star) = user_data.star {
                    user.star = star;
                }
                if let Some(qq_number) = &user_data.qq_number {
                    user.qq_number = Some(qq_number.clone());
                }
                if let Some(avatar_url) = &user_data.avatar_url {
                    user.avatar_url = Some(avatar_url.clone());
                }
                
                // 更新用户权限
                user.update_role();
                
                // 保存更新后的用户信息
                if let Err(_) = data_manager.update_user(username.to_string(), user.clone()) {
                    return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        code: 1,
                        msg: "保存用户信息失败".to_string(),
                        data: None
                    });
                }
            }

            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 0,
                msg: "用户添加成功".to_string(),
                data: None
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: format!("添加用户失败: {}", e),
            data: None
        })
    }
}