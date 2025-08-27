use env_logger::{Builder, Env};
use log::LevelFilter;
use std::io::Write;
use chrono::Local;

/// 初始化日志系统
pub fn init_logger() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let level = match record.level() {
                log::Level::Error => "\x1b[31m[ERROR]\x1b[0m", // 红色
                log::Level::Warn  => "\x1b[33m[WARN]\x1b[0m",  // 黄色
                log::Level::Info  => "\x1b[32m[INFO]\x1b[0m",  // 绿色
                log::Level::Debug => "\x1b[36m[DEBUG]\x1b[0m", // 青色
                log::Level::Trace => "\x1b[37m[TRACE]\x1b[0m", // 白色
            };
            
            // 获取模块路径的简化版本
            let module = record.module_path().unwrap_or("unknown");
            let module_short = simplify_module_path(module);
            
            writeln!(
                buf,
                "{} {} \x1b[90m[{}]\x1b[0m {}",
                timestamp,
                level,
                module_short,
                record.args()
            )
        })
        .filter_level(LevelFilter::Info) // 默认只显示Info及以上级别
        .init();
}

/// 简化模块路径显示
fn simplify_module_path(module: &str) -> String {
    let parts: Vec<&str> = module.split("::").collect();
    match parts.len() {
        0 => "unknown".to_string(),
        1 => parts[0].to_string(),
        2 => format!("{}::{}", parts[0], parts[1]),
        _ => {
            // 只保留最后两个部分
            let len = parts.len();
            format!("{}::{}", parts[len-2], parts[len-1])
        }
    }
}

/// 性能相关的日志宏
#[macro_export]
macro_rules! log_performance {
    ($operation:expr, $duration:expr) => {
        if $duration.as_millis() > 100 {
            log::warn!("⚡ {} took {}ms (slow)", $operation, $duration.as_millis());
        } else {
            log::debug!("⚡ {} took {}ms", $operation, $duration.as_millis());
        }
    };
}

/// API请求日志宏
#[macro_export]
macro_rules! log_api_request {
    ($method:expr, $path:expr) => {
        log::info!("🌐 {} {}", $method, $path);
    };
    ($method:expr, $path:expr, $user_id:expr) => {
        log::info!("🌐 {} {} (user: {})", $method, $path, $user_id);
    };
}

/// 数据库操作日志宏
#[macro_export]
macro_rules! log_db_operation {
    ($operation:expr, $table:expr) => {
        log::debug!("🗄️ {} on table: {}", $operation, $table);
    };
    ($operation:expr, $table:expr, $id:expr) => {
        log::debug!("🗄️ {} on table: {} (id: {})", $operation, $table, $id);
    };
}

/// 邮件发送日志宏
#[macro_export]
macro_rules! log_email {
    (success, $to:expr, $subject:expr) => {
        log::info!("📧 Email sent successfully: {} -> '{}'", $to, $subject);
    };
    (error, $to:expr, $subject:expr, $error:expr) => {
        log::error!("📧 Email failed: {} -> '{}', error: {}", $to, $subject, $error);
    };
}

/// 用户行为日志宏
#[macro_export]
macro_rules! log_user_action {
    ($user_id:expr, $action:expr) => {
        log::info!("👤 User {} performed: {}", $user_id, $action);
    };
    ($user_id:expr, $action:expr, $target:expr) => {
        log::info!("👤 User {} performed: {} on {}", $user_id, $action, $target);
    };
}

/// 安全相关日志宏
#[macro_export]
macro_rules! log_security {
    (suspicious, $ip:expr, $action:expr) => {
        log::warn!("🔒 Suspicious activity from {}: {}", $ip, $action);
    };
    (blocked, $ip:expr, $reason:expr) => {
        log::warn!("🚫 Blocked access from {}: {}", $ip, $reason);
    };
    (auth_failed, $user:expr, $ip:expr) => {
        log::warn!("🔐 Authentication failed for '{}' from {}", $user, $ip);
    };
}

/// 系统启动和关闭日志
pub fn log_system_start(version: &str, port: &str) {
    log::info!("🚀 绳包管理器后端服务启动");
    log::info!("📦 Version: {}", version);
    log::info!("🌐 Server: http://127.0.0.1:{}", port);
    log::info!("📋 API 文档: http://127.0.0.1:{}/swagger-ui/", port);
}

pub fn log_system_stop() {
    log::info!("�� 绳包管理器后端服务停止");
} 