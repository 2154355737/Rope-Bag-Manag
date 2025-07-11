use actix_web::{HttpRequest, HttpResponse};
use crate::models::{Users, GlobalLimiter, GlobalCount};
use crate::config::AppConfig;
use crate::utils::{now_ts, parse_query_params};

// ====== 安全 API 限流函数 ======
pub fn check_rate_limit(
    req: &HttpRequest,
    config: &AppConfig,
    limiter: &GlobalLimiter,
    global: &GlobalCount,
    stats: &crate::models::StatsData,
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

// ====== 管理员认证函数 ======
pub fn admin_auth(req: &HttpRequest, _config: &AppConfig, users: &Users) -> bool {
    let params = parse_query_params(req.query_string());
    
    let admin_user = params.get("admin_username");
    let admin_pass = params.get("admin_password");

    if admin_user.is_none() || admin_pass.is_none() {
        return false;
    }

    // 检查配置中的管理员凭据（暂时使用默认值）
    if **admin_user.as_ref().unwrap() == "admin" 
        && **admin_pass.as_ref().unwrap() == "admin" {
        return true;
    }

    // 检查用户表中的管理员
    let users_guard = users.lock().unwrap();
    if let Some(user) = users_guard.get(admin_user.unwrap()) {
        if user.is_admin && **admin_pass.unwrap() == user.password {
            return true;
        }
    }

    false
}