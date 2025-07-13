use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState, RawRopePackage};
use crate::utils::{parse_query_params, ApiResponse};
use crate::auth::{check_rate_limit, record_api_call};
use crate::data_manager::DataManager;

// 获取数据库数据
#[get("/api/get-data-db")]
pub async fn get_data_db(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let start_time = crate::utils::now_ts();
    
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "get_data_db") {
        return response;
    }

    let data_manager = DataManager::new();
    let raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载数据库失败".to_string(), 
            data: None 
        }),
    };
    
    record_api_call(
        &req.query_string(),
        &data.stats,
        "get_data_db",
        start_time,
        200,
        true,
        None,
    );

    HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "成功".to_string(), 
        data: Some(raw_data) 
    })
}

// 统计绳包下载量
#[get("/api/download-rope-package")]
pub async fn download_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let start_time = crate::utils::now_ts();
    
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "download_rope_package") {
        return response;
    }

    let params = parse_query_params(req.query_string());
    let id = match params.get("id") {
        Some(i) => match i.parse::<u32>() {
            Ok(id) if id > 0 => id,
            _ => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
                code: 1, 
                msg: "无效的资源ID".to_string(), 
                data: None 
            }),
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少资源ID".to_string(), 
            data: None 
        }),
    };

    let data_manager = DataManager::new();
    
    // 加载原始数据
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载数据库失败".to_string(), 
            data: None 
        }),
    };
    
    // 查找并更新下载次数
    let mut found = false;
    for package in &mut raw_data.绳包列表 {
        if package.id == id {
            package.下载次数 += 1;
            found = true;
            break;
        }
    }
    
    if found {
        // 保存更新后的数据
        if let Err(_) = data_manager.save_raw_data(&raw_data) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存数据失败".to_string(), 
                data: None 
            });
        }
        
        // 更新统计
        let mut stats = data.stats.lock().unwrap();
        let id_str = id.to_string();
        *stats.downloads.entry(id_str).or_insert(0) += 1;
        
        record_api_call(
            &req.query_string(),
            &data.stats,
            "download_rope_package",
            start_time,
            200,
            true,
            None,
        );
        
        HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "下载统计成功".to_string(), 
            data: None 
        })
    } else {
        record_api_call(
            &req.query_string(),
            &data.stats,
            "download_rope_package",
            start_time,
            404,
            false,
            Some("绳包不存在".to_string()),
        );
        
        HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "绳包不存在".to_string(), 
            data: None 
        })
    }
}

// 更新绳包（星级大于3）
#[get("/api/admin/update-rope-package")]
pub async fn update_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let start_time = crate::utils::now_ts();
    
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "update_rope_package") {
        return response;
    }
    
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少用户名".to_string(), 
            data: None 
        }),
    };
    let admin_password = params.get("admin_password");
    let admin_username = &data.config.admin_username;
    let admin_pwd = &data.config.admin_password;
    if username == admin_username {
        match admin_password {
            Some(pwd) if pwd == admin_pwd => {},
            _ => return HttpResponse::Forbidden().json(ApiResponse::<()> { 
                code: 1, 
                msg: "管理员操作需提供正确密码".to_string(), 
                data: None 
            }),
        }
    }
    let id = match params.get("id") {
        Some(i) => match i.parse::<u32>() {
            Ok(id) if id > 0 => id,
            _ => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
                code: 1, 
                msg: "无效的资源ID".to_string(), 
                data: None 
            }),
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少资源ID".to_string(), 
            data: None 
        }),
    };
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少name".to_string(), 
            data: None 
        }),
    };
    let author = match params.get("author") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少author".to_string(), 
            data: None 
        }),
    };
    let version = match params.get("version") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少version".to_string(), 
            data: None 
        }),
    };
    let desc = match params.get("desc") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少desc".to_string(), 
            data: None 
        }),
    };
    let url = match params.get("url") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少url".to_string(), 
            data: None 
        }),
    };
    let category = match params.get("category") {
        Some(c) => c,
        None => "未分类",
    };
    let status = match params.get("status") {
        Some(s) => s,
        None => "正常",
    };
    
    let data_manager = DataManager::new();
    let user_result = match data_manager.get_user(username) {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户不存在".to_string(), 
            data: None 
        }),
    };
    if user_result.star < 3 {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户星级不足".to_string(), 
            data: None 
        });
    }
    
    // 加载原始数据并更新
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载数据库失败".to_string(), 
            data: None 
        }),
    };
    
    // 分类计数维护
    if let Ok(mut categories) = DataManager::new().load_categories() {
        // 找到原分类
        if let Some(pkg) = raw_data.绳包列表.iter().find(|p| p.id == id) {
            let old_cat = &pkg.分类;
            if old_cat != category {
                // 原分类-1
                if let Some(cat) = categories.iter_mut().find(|c| &c.name == old_cat) {
                    if cat.count > 0 { cat.count -= 1; }
                }
                // 新分类+1
                if let Some(cat) = categories.iter_mut().find(|c| &c.name == category) {
                    cat.count += 1;
                }
                let _ = DataManager::new().save_categories(&categories);
            }
        }
    }
    
    let updates = RawRopePackage {
        id,
        绳包名称: name.clone(),
        作者: author.clone(),
        版本: version.clone(),
        简介: desc.clone(),
        项目直链: url.clone(),
        下载次数: 0,
        上架时间: chrono::Local::now().format("%Y-%m-%d").to_string(),
        分类: category.to_string(),
        状态: status.to_string(),
        是否标星: false,
        标星时间: None,
        标星人: None,
    };
    
    let mut found = false;
    for package in &mut raw_data.绳包列表 {
        if package.id == id {
            *package = updates;
            found = true;
            break;
        }
    }
    
    if found {
        if let Err(_) = data_manager.save_raw_data(&raw_data) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存数据失败".to_string(), 
                data: None 
            });
        }
        
        record_api_call(
            &req.query_string(),
            &data.stats,
            "update_rope_package",
            start_time,
            200,
            true,
            None,
        );
        
        HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "绳包更新成功".to_string(), 
            data: None 
        })
    } else {
        record_api_call(
            &req.query_string(),
            &data.stats,
            "update_rope_package",
            start_time,
            404,
            false,
            Some("绳包不存在".to_string()),
        );
        
        HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "绳包不存在".to_string(), 
            data: None 
        })
    }
}

// 删除绳包（星级大于3）
#[get("/api/admin/delete-rope-package")]
pub async fn delete_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let start_time = crate::utils::now_ts();
    
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
        Some(i) => match i.parse::<u32>() {
            Ok(id) if id > 0 => id,
            _ => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
                code: 1, 
                msg: "无效的资源ID".to_string(), 
                data: None 
            }),
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少资源ID".to_string(), 
            data: None 
        }),
    };
    let data_manager = DataManager::new();
    let user_result = match data_manager.get_user(username) {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户不存在".to_string(), 
            data: None 
        }),
    };
    if user_result.star < 3 {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户星级不足".to_string(), 
            data: None 
        });
    }
    
    // 加载原始数据并删除
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载数据库失败".to_string(), 
            data: None 
        }),
    };
    let mut deleted_category = None;
    if let Some(pkg) = raw_data.绳包列表.iter().find(|p| p.id == id) {
        deleted_category = Some(pkg.分类.clone());
    }
    let initial_len = raw_data.绳包列表.len();
    raw_data.绳包列表.retain(|p| p.id != id);
    // 分类计数-1
    if let Some(cat_name) = deleted_category {
        if let Ok(mut categories) = data_manager.load_categories() {
            if let Some(cat) = categories.iter_mut().find(|c| c.name == cat_name) {
                if cat.count > 0 {
                    cat.count -= 1;
                }
            }
            let _ = data_manager.save_categories(&categories);
        }
    }
    
    if raw_data.绳包列表.len() < initial_len {
        if let Err(_) = data_manager.save_raw_data(&raw_data) {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
                code: 1, 
                msg: "保存数据失败".to_string(), 
                data: None 
            });
        }
        
        record_api_call(
            &req.query_string(),
            &data.stats,
            "delete_rope_package",
            start_time,
            200,
            true,
            None,
        );
        
        HttpResponse::Ok().json(ApiResponse::<()> { 
            code: 0, 
            msg: "绳包删除成功".to_string(), 
            data: None 
        })
    } else {
        record_api_call(
            &req.query_string(),
            &data.stats,
            "delete_rope_package",
            start_time,
            404,
            false,
            Some("绳包不存在".to_string()),
        );
        
        HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "绳包不存在".to_string(), 
            data: None 
        })
    }
}

// 添加绳包
#[get("/api/add-rope-package")]
pub async fn add_rope_package(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let start_time = crate::utils::now_ts();
    
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "add_rope_package") {
        return response;
    }
    
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少用户名".to_string(), 
            data: None 
        }),
    };
    let admin_password = params.get("admin_password");
    let admin_username = &data.config.admin_username;
    let admin_pwd = &data.config.admin_password;
    if username == admin_username {
        match admin_password {
            Some(pwd) if pwd == admin_pwd => {},
            _ => return HttpResponse::Forbidden().json(ApiResponse::<()> { 
                code: 1, 
                msg: "管理员操作需提供正确密码".to_string(), 
                data: None 
            }),
        }
    }
    let name = match params.get("name") {
        Some(n) => n,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少name".to_string(), 
            data: None 
        }),
    };
    let author = match params.get("author") {
        Some(a) => a,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少author".to_string(), 
            data: None 
        }),
    };
    let version = match params.get("version") {
        Some(v) => v,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少version".to_string(), 
            data: None 
        }),
    };
    let desc = match params.get("desc") {
        Some(d) => d,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少desc".to_string(), 
            data: None 
        }),
    };
    let url = match params.get("url") {
        Some(u) => u,
        None => return HttpResponse::BadRequest().json(ApiResponse::<()> { 
            code: 1, 
            msg: "缺少url".to_string(), 
            data: None 
        }),
    };
    let category = match params.get("category") {
        Some(c) => c,
        None => "未分类",
    };
    let status = match params.get("status") {
        Some(s) => s,
        None => "正常",
    };
    
    let data_manager = DataManager::new();
    // 分类计数+1
    if let Ok(mut categories) = data_manager.load_categories() {
        for cat in &mut categories {
            if &cat.name == category {
                cat.count += 1;
            }
        }
        let _ = data_manager.save_categories(&categories);
    }
    
    let user_result = match data_manager.get_user(username) {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户不存在".to_string(), 
            data: None 
        }),
    };
    if user_result.star < 3 {
        return HttpResponse::Forbidden().json(ApiResponse::<()> { 
            code: 1, 
            msg: "用户星级不足".to_string(), 
            data: None 
        });
    }

    // 获取新ID
    let raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载数据库失败".to_string(), 
            data: None 
        }),
    };
    let new_id = raw_data.绳包列表.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    
    let package = RawRopePackage {
        id: new_id,
        绳包名称: name.clone(),
        作者: author.clone(),
        版本: version.clone(),
        简介: desc.clone(),
        项目直链: url.clone(),
        下载次数: 0,
        上架时间: chrono::Local::now().format("%Y-%m-%d").to_string(),
        分类: category.to_string(),
        状态: status.to_string(),
        是否标星: false,
        标星时间: None,
        标星人: None,
    };
    
    // 加载原始数据并添加
    let mut raw_data = match data_manager.load_raw_data() {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载数据库失败".to_string(), 
            data: None 
        }),
    };
    
    raw_data.绳包列表.push(package);
    
    if let Err(_) = data_manager.save_raw_data(&raw_data) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "保存数据失败".to_string(), 
            data: None 
        });
    }
    
    record_api_call(
        &req.query_string(),
        &data.stats,
        "add_rope_package",
        start_time,
        200,
        true,
        None,
    );
    
    HttpResponse::Ok().json(ApiResponse::<()> { 
        code: 0, 
        msg: "绳包添加成功".to_string(), 
        data: None 
    })
}
