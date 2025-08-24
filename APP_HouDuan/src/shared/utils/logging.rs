use std::env;
use tracing_subscriber::EnvFilter;
use crate::config::LoggingConfig;

use tracing_subscriber::fmt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_appender::rolling;
use tracing_appender::non_blocking::WorkerGuard;
use once_cell::sync::OnceCell;

static FILE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

/// 初始化日志系统
pub fn init_logging(config: &LoggingConfig) -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", &config.level);
    }

    // Writer 构建函数
    let build_file_writer = || -> Option<tracing_appender::non_blocking::NonBlocking> {
        if config.file_enabled {
            if let Some(dir) = &config.file_path {
                let appender = rolling::daily(dir, "server.log");
                let (non_blocking, guard) = tracing_appender::non_blocking(appender);
                let _ = FILE_GUARD.set(guard);
                return Some(non_blocking);
            }
        }
        None
    };
    let build_console_writer = || -> Option<fn() -> std::io::Stdout> {
        if config.console_enabled { Some(|| std::io::stdout()) } else { None }
    };

    if config.json_format {
        if config.with_timestamps {
            let builder = fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .json()
                .with_target(config.with_file_info)
                .with_thread_ids(config.with_thread_ids)
                .with_file(config.with_file_info)
                .with_line_number(config.with_file_info)
                .with_level(true);

            match (build_file_writer(), build_console_writer()) {
                (Some(f), Some(c)) => { builder.with_writer(f.and(c)).init(); }
                (Some(f), None) => { builder.with_writer(f).init(); }
                (None, Some(c)) => { builder.with_writer(c).init(); }
                (None, None) => { builder.with_writer(|| std::io::stdout()).init(); }
            }
            return Ok(());
        } else {
            let builder = fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .json()
                .without_time()
                .with_target(config.with_file_info)
                .with_thread_ids(config.with_thread_ids)
                .with_file(config.with_file_info)
                .with_line_number(config.with_file_info)
                .with_level(true);

            match (build_file_writer(), build_console_writer()) {
                (Some(f), Some(c)) => { builder.with_writer(f.and(c)).init(); }
                (Some(f), None) => { builder.with_writer(f).init(); }
                (None, Some(c)) => { builder.with_writer(c).init(); }
                (None, None) => { builder.with_writer(|| std::io::stdout()).init(); }
            }
            return Ok(());
        }
    } else {
        if config.with_timestamps {
            let builder = fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .with_target(config.with_file_info)
                .with_thread_ids(config.with_thread_ids)
                .with_file(config.with_file_info)
                .with_line_number(config.with_file_info)
                .with_level(true);

            match (build_file_writer(), build_console_writer()) {
                (Some(f), Some(c)) => { builder.with_writer(f.and(c)).init(); }
                (Some(f), None) => { builder.with_writer(f).init(); }
                (None, Some(c)) => { builder.with_writer(c).init(); }
                (None, None) => { builder.with_writer(|| std::io::stdout()).init(); }
            }
            return Ok(());
        } else {
            let builder = fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .without_time()
                .with_target(config.with_file_info)
                .with_thread_ids(config.with_thread_ids)
                .with_file(config.with_file_info)
                .with_line_number(config.with_file_info)
                .with_level(true);

            match (build_file_writer(), build_console_writer()) {
                (Some(f), Some(c)) => { builder.with_writer(f.and(c)).init(); }
                (Some(f), None) => { builder.with_writer(f).init(); }
                (None, Some(c)) => { builder.with_writer(c).init(); }
                (None, None) => { builder.with_writer(|| std::io::stdout()).init(); }
            }
            return Ok(());
        }
    }
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