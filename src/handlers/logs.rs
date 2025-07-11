use actix_web::{web, HttpResponse, Responder, get};
use crate::models::AppState;
use crate::utils::{parse_query_params, ApiResponse};
use crate::auth::check_rate_limit;
use log::{info, warn, error};
use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
pub struct LogStatsResponse {
    pub total_logs: u64,
    pub error_count: u64,
    pub warn_count: u64,
    pub info_count: u64,
    pub debug_count: u64,
    pub last_log_time: Option<String>,
    pub log_file_size: Option<u64>,
    pub log_file_exists: bool,
}

#[derive(serde::Serialize)]
pub struct LogEntryResponse {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub target: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[get("/api/logs/stats")]
pub async fn get_log_stats(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // 检查限流
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "log_stats") {
        return response;
    }

    // 检查管理员权限
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => {
            warn!("获取日志统计失败: 缺少用户名参数");
            return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None });
        }
    };

    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(username) {
        if !user.is_admin {
            warn!("非管理员用户尝试获取日志统计: {}", username);
            return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "需要管理员权限".to_string(), data: None });
        }
    } else {
        warn!("用户不存在: {}", username);
        return HttpResponse::Unauthorized().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None });
    }

    // 获取日志文件信息
    let log_file_path = "logs/app.log";
    let log_file_exists = Path::new(log_file_path).exists();
    let log_file_size = if log_file_exists {
        fs::metadata(log_file_path).ok().map(|m| m.len())
    } else {
        None
    };

    let stats = LogStatsResponse {
        total_logs: 0, // 这里可以从日志系统获取实际统计
        error_count: 0,
        warn_count: 0,
        info_count: 0,
        debug_count: 0,
        last_log_time: None,
        log_file_size,
        log_file_exists,
    };

    info!("管理员 {} 获取日志统计", username);
    HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(stats) })
}

#[get("/api/logs/entries")]
pub async fn get_log_entries(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // 检查限流
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "log_entries") {
        return response;
    }

    // 检查管理员权限
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => {
            warn!("获取日志条目失败: 缺少用户名参数");
            return HttpResponse::BadRequest().json(ApiResponse::<()> { code: 1, msg: "缺少用户名".to_string(), data: None });
        }
    };

    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(username) {
        if !user.is_admin {
            warn!("非管理员用户尝试获取日志条目: {}", username);
            return HttpResponse::Forbidden().json(ApiResponse::<()> { code: 1, msg: "需要管理员权限".to_string(), data: None });
        }
    } else {
        warn!("用户不存在: {}", username);
        return HttpResponse::Unauthorized().json(ApiResponse::<()> { code: 1, msg: "用户不存在".to_string(), data: None });
    }

    // 获取查询参数
    let limit = params.get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);
    let level = params.get("level").cloned();
    let search = params.get("search").cloned();

    // 读取日志文件
    let log_file_path = "logs/app.log";
    if !Path::new(log_file_path).exists() {
        return HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "日志文件不存在".to_string(), data: None });
    }

    match fs::read_to_string(log_file_path) {
        Ok(content) => {
            let mut entries: Vec<LogEntryResponse> = Vec::new();
            
            for line in content.lines().rev().take(limit) {
                // 简单的日志解析（实际项目中可能需要更复杂的解析）
                if let Some(entry) = parse_log_line(line) {
                    // 应用过滤条件
                    if let Some(ref filter_level) = level {
                        if entry.level.to_lowercase() != filter_level.to_lowercase() {
                            continue;
                        }
                    }
                    
                    if let Some(ref search_term) = search {
                        if !entry.message.to_lowercase().contains(&search_term.to_lowercase()) {
                            continue;
                        }
                    }
                    
                    entries.push(entry);
                }
            }

            info!("管理员 {} 获取日志条目，数量: {}", username, entries.len());
            HttpResponse::Ok().json(ApiResponse { code: 0, msg: "查询成功".to_string(), data: Some(entries) })
        }
        Err(e) => {
            error!("读取日志文件失败: {}", e);
            HttpResponse::InternalServerError().body("读取日志文件失败")
        }
    }
}

#[get("/api/logs/clear")]
pub async fn clear_logs(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // 检查限流
    if let Err(response) = check_rate_limit(&req, &data.config, &data.limiter, &data.global, &data.stats, "clear_logs") {
        return response;
    }

    // 检查管理员权限
    let params = parse_query_params(req.query_string());
    let username = match params.get("username") {
        Some(u) => u,
        None => {
            warn!("清除日志失败: 缺少用户名参数");
            return HttpResponse::BadRequest().body("缺少用户名");
        }
    };

    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(username) {
        if !user.is_admin {
            warn!("非管理员用户尝试清除日志: {}", username);
            return HttpResponse::Forbidden().body("需要管理员权限");
        }
    } else {
        warn!("用户不存在: {}", username);
        return HttpResponse::Unauthorized().body("用户不存在");
    }

    // 清除日志文件
    let log_file_path = "logs/app.log";
    if Path::new(log_file_path).exists() {
        match fs::write(log_file_path, "") {
            Ok(_) => {
                info!("管理员 {} 清除了日志文件", username);
                HttpResponse::Ok().json(ApiResponse::<()> { code: 0, msg: "日志已清除".to_string(), data: None })
            }
            Err(e) => {
                error!("清除日志文件失败: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> { code: 1, msg: "清除日志文件失败".to_string(), data: None })
            }
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> { code: 1, msg: "日志文件不存在".to_string(), data: None })
    }
}

fn parse_log_line(line: &str) -> Option<LogEntryResponse> {
    // 简单的日志行解析
    // 格式: [时间戳] [级别] [目标:文件:行] 消息 - (模块)
    let parts: Vec<&str> = line.split("] ").collect();
    if parts.len() < 3 {
        return None;
    }

    let timestamp = parts[0].trim_start_matches('[').to_string();
    let level_part = parts[1].trim_start_matches('[');
    let level = level_part.to_string();
    
    let message_part = parts[2..].join("] ");
    let message = message_part.split(" - ").next().unwrap_or("").to_string();
    
    // 提取文件信息
    let file_info = if parts.len() > 2 {
        let target_part = parts[2];
        if target_part.contains(':') {
            let target_parts: Vec<&str> = target_part.split(':').collect();
            if target_parts.len() >= 2 {
                let file = target_parts[1].to_string();
                let line_num = target_parts.get(2).and_then(|s| s.parse::<u32>().ok());
                (Some(file), line_num)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    Some(LogEntryResponse {
        timestamp,
        level,
        message,
        target: "unknown".to_string(),
        file: file_info.0,
        line: file_info.1,
    })
} 