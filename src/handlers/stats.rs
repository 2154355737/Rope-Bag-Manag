use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{AppState};
use crate::auth::check_rate_limit;
use crate::utils::ApiResponse;
use crate::storage::DataManager;
use serde::Serialize;
use chrono::Local;
use std::collections::HashMap;

// 仪表盘数据结构
#[derive(Serialize)]
pub struct DashboardData {
    pub user_count: usize,
    pub package_count: usize,
    pub total_downloads: u32,
    pub active_users: usize,
    pub recent_activities: Vec<ActivityItem>,
    pub popular_packages: Vec<PopularPackage>,
    pub system_status: SystemStatus,
    pub download_trends: Vec<DownloadTrend>,
}

#[derive(Serialize)]
pub struct ActivityItem {
    pub id: u32,
    pub type_: String,
    pub icon: String,
    pub title: String,
    pub time: String,
}

#[derive(Serialize)]
pub struct PopularPackage {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub downloads: u32,
}

#[derive(Serialize)]
pub struct SystemStatus {
    pub database: String,
    pub api_service: String,
    pub log_system: String,
    pub storage: String,
}

#[derive(Serialize)]
pub struct DownloadTrend {
    pub date: String,
    pub downloads: u32,
}

// API调用统计数据结构
#[derive(Serialize)]
pub struct ApiCallStats {
    pub total_calls: u32,
    pub success_rate: f64,
    pub avg_response_time_ms: u64,
    pub top_apis: Vec<TopApi>,
    pub recent_calls: Vec<RecentApiCall>,
    pub user_activity: Vec<UserActivity>,
    pub performance_summary: PerformanceSummary,
}

#[derive(Serialize)]
pub struct TopApi {
    pub api_name: String,
    pub call_count: u32,
    pub success_rate: f64,
    pub avg_response_time_ms: u64,
}

#[derive(Serialize)]
pub struct RecentApiCall {
    pub timestamp: u64,
    pub api_name: String,
    pub username: String,
    pub response_time_ms: u64,
    pub success: bool,
    pub status_code: u16,
}

#[derive(Serialize)]
pub struct UserActivity {
    pub username: String,
    pub call_count: u32,
    pub last_activity: u64,
    pub favorite_apis: Vec<String>,
}

#[derive(Serialize)]
pub struct PerformanceSummary {
    pub total_apis: usize,
    pub total_users: usize,
    pub system_uptime_hours: u64,
    pub peak_concurrent_users: u32,
}

// 获取仪表盘数据
#[get("/api/dashboard")]
pub async fn get_dashboard_data(
    _req: actix_web::HttpRequest,
    _data: web::Data<AppState>,
) -> impl Responder {
    println!("开始处理仪表盘数据请求");
    
    let data_manager = DataManager::new();
    
    // 加载用户数据
    let users = match data_manager.load_users() {
        Ok(users) => users,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载用户数据失败".to_string(), 
            data: None 
        }),
    };
    
    // 加载绳包数据
    let ropes = match data_manager.load_packages() {
        Ok(ropes) => ropes,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载绳包数据失败".to_string(), 
            data: None 
        }),
    };
    
    println!("获取到用户数据: {} 个用户", users.len());
    println!("获取到绳包数据: {} 个绳包", ropes.len());

    // 计算用户统计
    let user_count = users.len();
    let active_users = users.values().filter(|user| !user.is_admin).count();

    // 计算绳包统计
    let package_count = ropes.len();
    
    // 计算总下载量
    let total_downloads = ropes.values().map(|pkg| pkg.downloads).sum();

    // 基于真实下载数据生成下载趋势（最近7天）
    let mut download_trends = Vec::new();
    let now = Local::now();
    
    // 计算总下载量
    let total_downloads_from_packages: u32 = ropes.values().map(|pkg| pkg.downloads).sum();
    
    for i in 0..7 {
        let date = now - chrono::Duration::days(i);
        let date_str = date.format("%Y-%m-%d").to_string();
        
        // 基于真实下载量生成每日下载趋势
        let daily_downloads = if total_downloads_from_packages > 0 {
            // 使用总下载量除以7天，再加上一些变化
            (total_downloads_from_packages / 7) + (i as u32 % 3)
        } else {
            // 如果没有下载数据，使用基于绳包数量的模拟数据
            (package_count as u32 / 7) + (i as u32 % 2)
        };
        
        download_trends.push(DownloadTrend {
            date: date_str,
            downloads: daily_downloads,
        });
    }
    
    // 反转数组，让日期按时间顺序排列
    download_trends.reverse();

    // 生成最近活动（基于真实数据）
    let mut recent_activities = Vec::new();
    let mut activity_id = 1;

    // 添加用户注册活动（基于真实用户数据）
    for user in users.values() {
        if !user.is_admin {
            // 根据用户签到天数判断注册时间
            let registration_time = if user.sign_total > 0 {
                format!("{}天前", user.sign_total)
            } else {
                "最近".to_string()
            };
            
            recent_activities.push(ActivityItem {
                id: activity_id,
                type_: "user".to_string(),
                icon: "User".to_string(),
                title: format!("用户 {} 注册", user.username),
                time: registration_time,
            });
            activity_id += 1;
        }
    }

    // 添加绳包发布活动（基于真实上架时间）
    let mut package_activities: Vec<ActivityItem> = Vec::new();
    println!("开始处理绳包活动，绳包数量: {}", ropes.len());
    for pkg in ropes.values() {
        // 计算上架时间距离现在的天数
        let upload_date = chrono::NaiveDate::parse_from_str(&pkg.upload_time, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Local::now().naive_local().date());
        let days_ago = (chrono::Local::now().naive_local().date() - upload_date).num_days();
        
        let time_str = if days_ago == 0 {
            "今天".to_string()
        } else if days_ago == 1 {
            "昨天".to_string()
        } else if days_ago < 7 {
            format!("{}天前", days_ago)
        } else {
            pkg.upload_time.clone()
        };
        
        package_activities.push(ActivityItem {
            id: activity_id,
            type_: "package".to_string(),
            icon: "Package".to_string(),
            title: format!("绳包 {} 发布", pkg.name),
            time: time_str,
        });
        activity_id += 1;
    }

    // 合并活动并按时间排序（这里简化处理，实际应该按真实时间排序）
    recent_activities.extend(package_activities);
    recent_activities.truncate(10); // 只保留最近10个活动

    // 生成热门绳包（基于真实下载数据）
    let mut popular_packages: Vec<PopularPackage> = ropes.values()
        .map(|pkg| PopularPackage {
            id: pkg.id,
            name: pkg.name.clone(),
            author: pkg.author.clone(),
            downloads: pkg.downloads,
        })
        .collect();
    
    // 按下载量排序
    popular_packages.sort_by(|a, b| b.downloads.cmp(&a.downloads));
    popular_packages.truncate(5); // 只保留前5个

    // 系统状态（模拟数据）
    let system_status = SystemStatus {
        database: "正常".to_string(),
        api_service: "运行中".to_string(),
        log_system: "正常".to_string(),
        storage: "正常".to_string(),
    };

    let dashboard_data = DashboardData {
        user_count,
        package_count,
        total_downloads,
        active_users,
        recent_activities,
        popular_packages,
        system_status,
        download_trends,
    };

    println!("仪表盘数据生成完成");
    HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "获取仪表盘数据成功".to_string(), 
        data: Some(dashboard_data) 
    })
}

// 获取API调用统计
#[get("/api/stats/api-calls")]
pub async fn get_api_call_stats(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "get_api_call_stats") {
        return response;
    }

    let data_manager = DataManager::new();
    
    // 加载API性能数据
    let api_performance = match data_manager.load_api_performance() {
        Ok(performance) => performance,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载API性能数据失败".to_string(), 
            data: None 
        }),
    };

    // 加载API调用记录
    let api_calls = match data_manager.load_api_calls() {
        Ok(calls) => calls,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载API调用记录失败".to_string(), 
            data: None 
        }),
    };

    // 加载API最后使用时间
    let api_last_used = match data_manager.load_api_last_used() {
        Ok(last_used) => last_used,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载API最后使用时间失败".to_string(), 
            data: None 
        }),
    };
    
    // 计算总体统计
    let total_calls: u32 = api_performance.values().map(|p| p.total_calls).sum();
    let total_success: u32 = api_performance.values().map(|p| p.success_calls).sum();
    let success_rate = if total_calls > 0 {
        (total_success as f64 / total_calls as f64) * 100.0
    } else {
        0.0
    };
    
    let avg_response_time = if total_calls > 0 {
        let total_time: u64 = api_performance.values()
            .map(|p| p.avg_response_time_ms * p.total_calls as u64)
            .sum();
        total_time / total_calls as u64
    } else {
        0
    };
    
    // 生成热门API列表
    let mut top_apis: Vec<TopApi> = api_performance
        .iter()
        .map(|(api_name, perf)| {
            let api_success_rate = if perf.total_calls > 0 {
                (perf.success_calls as f64 / perf.total_calls as f64) * 100.0
            } else {
                0.0
            };
            
            TopApi {
                api_name: api_name.clone(),
                call_count: perf.total_calls,
                success_rate: api_success_rate,
                avg_response_time_ms: perf.avg_response_time_ms,
            }
        })
        .collect();
    
    // 按调用次数排序
    top_apis.sort_by(|a, b| b.call_count.cmp(&a.call_count));
    top_apis.truncate(10);
    
    // 生成最近调用记录
    let recent_calls: Vec<RecentApiCall> = api_calls
        .iter()
        .rev() // 最新的在前
        .take(20)
        .map(|call| RecentApiCall {
            timestamp: call.timestamp,
            api_name: call.api_name.clone(),
            username: call.username.clone(),
            response_time_ms: call.response_time_ms,
            success: call.success,
            status_code: call.status_code,
        })
        .collect();
    
    // 生成用户活动统计
    let mut user_activity_map: HashMap<String, (u32, u64, Vec<String>)> = HashMap::new();
    
    for call in &api_calls {
        let entry = user_activity_map.entry(call.username.clone()).or_insert((0, 0, Vec::new()));
        entry.0 += 1; // 调用次数
        entry.1 = entry.1.max(call.timestamp); // 最后活动时间
        if !entry.2.contains(&call.api_name) {
            entry.2.push(call.api_name.clone());
        }
    }
    
    let mut user_activity: Vec<UserActivity> = user_activity_map
        .into_iter()
        .map(|(username, (call_count, last_activity, favorite_apis))| {
            UserActivity {
                username,
                call_count,
                last_activity,
                favorite_apis: favorite_apis.into_iter().take(5).collect(),
            }
        })
        .collect();
    
    // 按调用次数排序
    user_activity.sort_by(|a, b| b.call_count.cmp(&a.call_count));
    user_activity.truncate(10);
    
    // 性能摘要
    let performance_summary = PerformanceSummary {
        total_apis: api_performance.len(),
        total_users: user_activity.len(),
        system_uptime_hours: 24, // 简化处理
        peak_concurrent_users: user_activity.iter().map(|u| u.call_count).max().unwrap_or(0),
    };
    
    let api_call_stats = ApiCallStats {
        total_calls,
        success_rate,
        avg_response_time_ms: avg_response_time,
        top_apis,
        recent_calls,
        user_activity,
        performance_summary,
    };
    // 新增：组装带api_last_used的返回结构
    let mut resp = serde_json::to_value(&api_call_stats).unwrap();
    resp["api_last_used"] = serde_json::to_value(api_last_used).unwrap();
    HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "获取API调用统计成功".to_string(), 
        data: Some(resp)
    })
}

// 获取API性能详情
#[get("/api/stats/api-performance")]
pub async fn get_api_performance(
    _req: actix_web::HttpRequest,
    _data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&_req, &_data.config, &_data.limiter, &_data.global, &_data.stats, "get_api_performance") {
        return response;
    }

    let data_manager = DataManager::new();
    
    // 加载API性能数据
    let api_performance = match data_manager.load_api_performance() {
        Ok(performance) => performance,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载API性能数据失败".to_string(), 
            data: None 
        }),
    };
    
    // 转换HashSet为Vec以便序列化
    let mut performance_data: HashMap<String, serde_json::Value> = HashMap::new();
    
    for (api_name, perf) in &api_performance {
        let unique_users_count = perf.unique_users.len();
        let performance_json = serde_json::json!({
            "total_calls": perf.total_calls,
            "success_calls": perf.success_calls,
            "failed_calls": perf.failed_calls,
            "avg_response_time_ms": perf.avg_response_time_ms,
            "min_response_time_ms": perf.min_response_time_ms,
            "max_response_time_ms": perf.max_response_time_ms,
            "last_called": perf.last_called,
            "unique_users_count": unique_users_count,
            "success_rate": if perf.total_calls > 0 {
                (perf.success_calls as f64 / perf.total_calls as f64) * 100.0
            } else {
                0.0
            }
        });
        
        performance_data.insert(api_name.clone(), performance_json);
    }
    
    HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "获取API性能详情成功".to_string(), 
        data: Some(&performance_data) 
    })
}

// 获取实时API调用记录
#[get("/api/stats/recent-calls")]
pub async fn get_recent_calls(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "get_recent_calls") {
        return response;
    }

    let data_manager = DataManager::new();
    
    // 加载API调用记录
    let api_calls = match data_manager.load_api_calls() {
        Ok(calls) => calls,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> { 
            code: 1, 
            msg: "加载API调用记录失败".to_string(), 
            data: None 
        }),
    };
    
    // 获取最近100条调用记录
    let recent_calls: Vec<&crate::models::ApiCallRecord> = api_calls
        .iter()
        .rev()
        .take(100)
        .collect();
    
    HttpResponse::Ok().json(ApiResponse { 
        code: 0, 
        msg: "获取最近调用记录成功".to_string(), 
        data: Some(&recent_calls) 
    })
}

// 查询所有绳包下载量
#[get("/api/stats/downloads")]
pub async fn stats_downloads(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "stats_downloads") {
        return response;
    }

    let data_manager = DataManager::new();
    match data_manager.load_downloads() {
        Ok(downloads) => HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(downloads) }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "加载下载数据失败".to_string(), data: None }),
    }
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

    let data_manager = DataManager::new();
    match data_manager.load_api_counts() {
        Ok(api_counts) => HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(api_counts) }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "加载API计数数据失败".to_string(), data: None }),
    }
}

// 获取用户数据库
#[get("/api/get-users-db")]
pub async fn get_users_db(
    _req: actix_web::HttpRequest,
    _data: web::Data<AppState>,
) -> impl Responder {
    // 管理员接口，跳过限流检查
    let data_manager = DataManager::new();
    match data_manager.load_users() {
        Ok(users) => HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(users) }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "加载用户数据失败".to_string(), data: None }),
    }
}
