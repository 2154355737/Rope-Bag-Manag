use std::fs::{self, File};
use std::io::Read;
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::path::Path;
use urlencoding::decode;

// ====== 加载函数 ======
pub fn load_json<T: for<'a> Deserialize<'a> + Default>(path: &str) -> T {
    if let Ok(mut f) = File::open(path) {
        let mut s = String::new();
        if f.read_to_string(&mut s).is_ok() {
            return serde_json::from_str(&s).unwrap_or_default();
        }
    }
    T::default()
}

pub fn save_json<T: Serialize>(path: &str, data: &T) {
    if let Ok(s) = serde_json::to_string_pretty(data) {
        let _ = fs::write(path, s);
    }
}

// ====== 日期与时间 ======
pub fn today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn now_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// ====== 查询参数解析 ======
use std::collections::HashMap;

pub fn parse_query_params(query_string: &str) -> HashMap<String, String> {
    query_string
        .split('&')
        .filter_map(|kv| {
            let mut it = kv.splitn(2, '=');
            if let (Some(k), Some(v)) = (it.next(), it.next()) {
                let key = decode(k).unwrap_or_else(|_| k.into()).to_string();
                let val = decode(v).unwrap_or_else(|_| v.into()).to_string();
                Some((key, val))
            } else {
                None
            }
        })
        .collect()
}
#[allow(dead_code)]
pub fn get_param<'a>(params: &'a HashMap<String, String>, key: &str) -> Option<&'a String> {
    params.get(key)
}

#[allow(dead_code)]
pub fn get_param_or_error<'a>(params: &'a HashMap<String, String>, key: &str) -> Result<&'a String, String> {
    params.get(key).ok_or_else(|| format!("缺少{}参数", key))
}

// ====== 日志轮转功能 ======
#[allow(dead_code)]
pub fn rotate_log_file(log_file_path: &str, max_files: u32) -> Result<(), std::io::Error> {
    if !Path::new(log_file_path).exists() {
        return Ok(());
    }

    // 检查文件大小
    let metadata = fs::metadata(log_file_path)?;
    if metadata.len() < 10 * 1024 * 1024 { // 10MB
        return Ok(());
    }

    // 删除最旧的文件
    for i in (max_files..=max_files + 10).rev() {
        let old_file = format!("{}.{}", log_file_path, i);
        if Path::new(&old_file).exists() {
            fs::remove_file(&old_file)?;
        }
    }

    // 重命名现有文件
    for i in (1..=max_files).rev() {
        let old_file = format!("{}.{}", log_file_path, i - 1);
        let new_file = format!("{}.{}", log_file_path, i);
        
        if Path::new(&old_file).exists() {
            fs::rename(&old_file, &new_file)?;
        }
    }

    // 重命名当前日志文件
    let backup_file = format!("{}.1", log_file_path);
    fs::rename(log_file_path, &backup_file)?;

    // 创建新的日志文件
    File::create(log_file_path)?;

    Ok(())
}

// ====== 日志文件清理 ======
#[allow(dead_code)]
pub fn cleanup_old_logs(log_dir: &str, max_age_days: u32) -> Result<(), std::io::Error> {
    if !Path::new(log_dir).exists() {
        return Ok(());
    }

    let now = Local::now();
    let max_age = chrono::Duration::days(max_age_days as i64);

    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    let modified_time: chrono::DateTime<chrono::Local> = chrono::DateTime::from(modified);
                    if now.signed_duration_since(modified_time) > max_age {
                        fs::remove_file(&path)?;
                    }
                }
            }
        }
    }

    Ok(())
}

// ====== 性能监控 ======
#[allow(dead_code)]
pub struct PerformanceTimer {
    start_time: Instant,
    operation_name: String,
}

impl PerformanceTimer {
    #[allow(dead_code)]
    pub fn new(operation_name: &str) -> Self {
        Self {
            start_time: Instant::now(),
            operation_name: operation_name.to_string(),
        }
    }


}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let duration = self.start_time.elapsed();
            log::debug!("操作 '{}' 耗时: {:?}", self.operation_name, duration);
        }
    }
}

// ====== 错误处理工具 ======
#[allow(dead_code)]
pub fn log_error_with_context(error: &dyn std::error::Error, context: &str) {
    log::error!("{}: {}", context, error);
}

#[allow(dead_code)]
pub fn log_warning_with_context(warning: &str, context: &str) {
    log::warn!("{}: {}", context, warning);
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}
