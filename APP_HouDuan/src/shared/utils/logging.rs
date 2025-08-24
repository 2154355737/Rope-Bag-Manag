use std::env;
use tracing_subscriber::EnvFilter;
use crate::config::LoggingConfig;

/// 初始化日志系统
pub fn init_logging(config: &LoggingConfig) -> anyhow::Result<()> {
    // 设置默认环境变量
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", &config.level);
    }

    // 使用简单的格式化订阅器
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(config.with_file_info)
        .with_thread_ids(config.with_thread_ids)
        .with_file(config.with_file_info)
        .with_line_number(config.with_file_info)
        .init();

    Ok(())
}

/// 便捷宏用于记录性能
#[macro_export]
macro_rules! log_execution_time {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        tracing::info!(
            operation = $name,
            duration_ms = duration.as_millis(),
            "Operation completed"
        );
        result
    }};
}

/// 记录系统启动信息
pub fn log_system_info(version: &str, port: u16) {
    tracing::info!(
        version = version,
        port = port,
        target_arch = std::env::consts::ARCH,
        target_os = std::env::consts::OS,
        "🚀 System starting up"
    );
}

/// 记录系统关闭信息
pub fn log_system_shutdown() {
    tracing::info!("🛑 System shutting down gracefully");
}

/// 记录数据库操作
pub fn log_database_operation(operation: &str, table: &str, duration_ms: u128) {
    tracing::debug!(
        operation = operation,
        table = table,
        duration_ms = duration_ms,
        "Database operation completed"
    );
}

/// 记录API请求
pub fn log_api_request(method: &str, path: &str, status: u16, duration_ms: u128) {
    if status >= 400 {
        tracing::warn!(
            method = method,
            path = path,
            status = status,
            duration_ms = duration_ms,
            "API request failed"
        );
    } else {
        tracing::info!(
            method = method,
            path = path,
            status = status,
            duration_ms = duration_ms,
            "API request completed"
        );
    }
}

/// 记录用户操作
pub fn log_user_action(user_id: Option<i64>, action: &str, target: Option<&str>) {
    tracing::info!(
        user_id = user_id,
        action = action,
        target = target,
        "User action recorded"
    );
} 