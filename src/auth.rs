use actix_web::{HttpRequest, HttpResponse};
use crate::models::{GlobalLimiter, GlobalCount, ApiCallRecord, ApiPerformance, StatsData};
use crate::config::AppConfig;
use crate::utils::{now_ts, parse_query_params};
use std::sync::{Arc, Mutex};


// ====== 安全 API 限流函数 ======
pub fn check_rate_limit(
    req: &HttpRequest,
    config: &AppConfig,
    limiter: &GlobalLimiter,
    global: &GlobalCount,
    stats: &Arc<Mutex<StatsData>>,
    api_name: &str,
) -> Result<(), HttpResponse> {
    let params = parse_query_params(req.query_string());
    
    // 对于注册和登录接口，允许没有用户名参数
    let username = if api_name == "register" || api_name == "login" {
        params.get("username").cloned().unwrap_or_else(|| "anonymous".to_string())
    } else {
        params.get("username")
            .cloned()
            .ok_or_else(|| HttpResponse::Forbidden().body("缺少用户名参数"))?
    };

    let now = now_ts();

    // 检查是否启用限流
    if !config.rate_limit.enabled {
        return Ok(());
    }

    // 检查用户级限流
    let mut limiter_guard = limiter.lock().unwrap();
    let window = 60; // 1分钟窗口
    let limit = config.rate_limit.requests_per_minute;

    let entry = limiter_guard.entry(username.clone())
        .and_modify(|(t, c)| {
            if now - *t >= window as u64 {
                *t = now;
                *c = 0;
            }
        })
        .or_insert((now, 0));

    if entry.1 >= limit {
        return Err(HttpResponse::TooManyRequests().body("请求频率过高"));
    }
    entry.1 += 1;
    drop(limiter_guard);

    // 检查全局限流
    let mut global_guard = global.lock().unwrap();
    let g_window = 60; // 1分钟窗口
    let g_limit = config.rate_limit.burst_size * 10; // 突发限制

    if now - global_guard.0 >= g_window as u64 {
        global_guard.0 = now;
        global_guard.1 = 0;
    }

    if global_guard.1 >= g_limit {
        return Err(HttpResponse::TooManyRequests().body("系统请求频率过高"));
    }
    global_guard.1 += 1;
    drop(global_guard);

    // 更新统计
    let mut stats_guard = stats.lock().unwrap();
    *stats_guard.api_counts.entry(api_name.to_string()).or_insert(0) += 1;
    drop(stats_guard);

    Ok(())
}

// ====== 记录API调用统计 ======
pub fn record_api_call(
    req_info: &str,
    stats: &Arc<Mutex<StatsData>>,
    api_name: &str,
    start_time: u64,
    status_code: u16,
    success: bool,
    error_message: Option<String>,
) {
    // 从请求信息中解析参数
    let username = if req_info.contains("username=") {
        // 简单的参数解析
        if let Some(username_start) = req_info.find("username=") {
            let username_part = &req_info[username_start..];
            if let Some(username_end) = username_part.find('&') {
                username_part[9..username_end].to_string()
            } else {
                username_part[9..].to_string()
            }
        } else {
            "anonymous".to_string()
        }
    } else {
        "anonymous".to_string()
    };
    
    // 简化IP地址和User-Agent获取
    let ip_address = "unknown".to_string();
    let user_agent = "unknown".to_string();
    
    let end_time = now_ts();
    let response_time_ms = end_time - start_time;
    
    let api_call_record = ApiCallRecord {
        timestamp: end_time,
        api_name: api_name.to_string(),
        username,
        ip_address,
        user_agent,
        response_time_ms,
        status_code,
        success,
        error_message,
    };
    
    let mut stats_guard = stats.lock().unwrap();
    
    // 添加API调用记录
    stats_guard.api_calls.push(api_call_record.clone());

    // 记录API最后调用时间
    stats_guard.api_last_used.insert(api_name.to_string(), end_time);
    
    // 限制记录数量，保留最近1000条记录
    if stats_guard.api_calls.len() > 1000 {
        stats_guard.api_calls.remove(0);
    }
    
    // 更新API性能统计
    let performance = stats_guard.api_performance
        .entry(api_name.to_string())
        .or_insert_with(ApiPerformance::default);
    
    performance.total_calls += 1;
    if success {
        performance.success_calls += 1;
    } else {
        performance.failed_calls += 1;
    }
    
    // 更新响应时间统计
    if performance.total_calls == 1 {
        performance.min_response_time_ms = response_time_ms;
        performance.max_response_time_ms = response_time_ms;
        performance.avg_response_time_ms = response_time_ms;
    } else {
        performance.min_response_time_ms = performance.min_response_time_ms.min(response_time_ms);
        performance.max_response_time_ms = performance.max_response_time_ms.max(response_time_ms);
        
        // 计算平均响应时间
        let total_time = performance.avg_response_time_ms * ((performance.total_calls - 1) as u64) + response_time_ms;
        performance.avg_response_time_ms = total_time / (performance.total_calls as u64);
    }
    
    performance.last_called = end_time;
    performance.unique_users.insert(api_call_record.username.clone());
    
    drop(stats_guard);
}

// ====== 管理员认证函数 ======
pub fn admin_auth(req: &HttpRequest, config: &AppConfig, data_manager: &crate::storage::DataManager) -> bool {
    let params = parse_query_params(req.query_string());
    let admin_user = params.get("admin_username");
    let admin_pass = params.get("admin_password");
    if admin_user.is_none() || admin_pass.is_none() {
        return false;
    }
    // 检查配置中的管理员凭据
    if admin_user.unwrap() == &config.admin_username 
        && admin_pass.unwrap() == &config.admin_password {
        return true;
    }
    // 检查用户表中的管理员
    if let Ok(users) = data_manager.load_users() {
        if let Some(user) = users.get(admin_user.unwrap()) {
            if user.is_admin && admin_pass.unwrap() == &user.password {
                return true;
            }
        }
    }
    false
}