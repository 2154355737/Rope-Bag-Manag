use log::{LevelFilter, Log, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};


// ====== 日志级别枚举 ======
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl From<log::Level> for LogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => LogLevel::Error,
            log::Level::Warn => LogLevel::Warn,
            log::Level::Info => LogLevel::Info,
            log::Level::Debug => LogLevel::Debug,
            log::Level::Trace => LogLevel::Debug,
        }
    }
}

// ====== 日志条目结构 ======
#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

// ====== 日志统计 ======
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStats {
    pub total_logs: u64,
    pub error_count: u64,
    pub warn_count: u64,
    pub info_count: u64,
    pub debug_count: u64,
    pub last_log_time: Option<DateTime<Local>>,
}

// ====== 自定义日志记录器 ======
pub struct CustomLogger {
    console_output: bool,
    file_output: bool,
    log_file: Option<Arc<Mutex<File>>>,
    log_level: LevelFilter,
    stats: Arc<Mutex<LogStats>>,
}

impl CustomLogger {
    pub fn new(console_output: bool, file_output: bool, log_level: LevelFilter) -> io::Result<Self> {
        let log_file = if file_output {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("logs/app.log")?;
            Some(Arc::new(Mutex::new(file)))
        } else {
            None
        };

        // 确保日志目录存在
        if file_output {
            std::fs::create_dir_all("logs")?;
        }

        Ok(Self {
            console_output,
            file_output,
            log_file,
            log_level,
            stats: Arc::new(Mutex::new(LogStats {
                total_logs: 0,
                error_count: 0,
                warn_count: 0,
                info_count: 0,
                debug_count: 0,
                last_log_time: None,
            })),
        })
    }

    #[allow(dead_code)]
    pub fn get_stats(&self) -> LogStats {
        self.stats.lock().unwrap().clone()
    }

    fn write_log(&self, record: &Record) -> io::Result<()> {
        let timestamp = Local::now();
        let level = LogLevel::from(record.level());
        
        // 更新统计信息
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_logs += 1;
            stats.last_log_time = Some(timestamp);
            
            match level {
                LogLevel::Error => stats.error_count += 1,
                LogLevel::Warn => stats.warn_count += 1,
                LogLevel::Info => stats.info_count += 1,
                LogLevel::Debug => stats.debug_count += 1,
            }
        }

        // 格式化日志消息
        let log_entry = LogEntry {
            timestamp,
            level,
            target: record.target().to_string(),
            message: record.args().to_string(),
            module_path: record.module_path().map(|s| s.to_string()),
            file: record.file().map(|s| s.to_string()),
            line: record.line(),
            request_id: None, // 将在请求中间件中设置
            user_id: None,
            ip: None,
            user_agent: None,
        };

        let log_line = format!(
            "[{}] [{}] [{}:{}:{}] {} - {}\n",
            timestamp.format("%Y-%m-%d %H:%M:%S"),
            level.as_str(),
            log_entry.target,
            log_entry.file.as_deref().unwrap_or("unknown"),
            log_entry.line.unwrap_or(0),
            record.args(),
            if let Some(module) = &log_entry.module_path {
                format!("({})", module)
            } else {
                String::new()
            }
        );

        // 控制台输出
        if self.console_output {
            let color = match level {
                LogLevel::Error => "\x1b[31m", // 红色
                LogLevel::Warn => "\x1b[33m",  // 黄色
                LogLevel::Info => "\x1b[32m",  // 绿色
                LogLevel::Debug => "\x1b[36m", // 青色
            };
            print!("{}{}\x1b[0m", color, log_line);
        }

        // 文件输出
        if self.file_output {
            if let Some(file) = &self.log_file {
                let mut file = file.lock().unwrap();
                file.write_all(log_line.as_bytes())?;
                file.flush()?;
            }
        }

        Ok(())
    }
}

impl Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Err(e) = self.write_log(record) {
                eprintln!("Failed to write log: {}", e);
            }
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.log_file {
            if let Ok(mut file) = file.lock() {
                let _ = file.flush();
            }
        }
    }
}

// ====== 日志级别字符串转换 ======
impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

// ====== 日志初始化函数 ======
pub fn init_logger(console_output: bool, file_output: bool, log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let level_filter = match log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let logger = CustomLogger::new(console_output, file_output, level_filter)?;
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level_filter);

    log::info!("日志系统初始化完成 - 级别: {}, 控制台: {}, 文件: {}", 
               log_level, console_output, file_output);

    Ok(())
}

// ====== 便捷日志宏 ======
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    };
}

// ====== 请求日志中间件 ======
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};

use uuid::Uuid;

pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware { service }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = std::time::Instant::now();
        let request_id = Uuid::new_v4().to_string();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let user_agent = "Unknown".to_string();
        let ip = "Unknown".to_string();

        log::info!(
            "请求开始 - ID: {}, 方法: {}, URI: {}, IP: {}, User-Agent: {}",
            request_id, method, uri, ip, user_agent
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            
            log::info!(
                "请求完成 - ID: {}, 状态: {}, 耗时: {:?}",
                request_id, res.status(), duration
            );

            Ok(res)
        })
    }
} 