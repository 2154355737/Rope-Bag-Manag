use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    pub console_output: bool,
    pub file_output: bool,
    pub log_level: String,
    pub log_file_path: String,
    pub max_file_size: u64, // 字节
    pub max_files: u32,
    pub enable_request_logging: bool,
    pub enable_performance_logging: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LogConfig,
    pub rate_limit: RateLimitConfig,
    pub security: SecurityConfig,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    pub enable_cors: bool,
    pub allowed_origins: Vec<String>,
    pub session_timeout: u64, // 秒
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            console_output: true,
            file_output: true,
            log_level: "info".to_string(),
            log_file_path: "logs/app.log".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            enable_request_logging: true,
            enable_performance_logging: true,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 15201,
            workers: 4,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 60,
            burst_size: 10,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_cors: true,
            allowed_origins: vec!["*".to_string()],
            session_timeout: 3600, // 1小时
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LogConfig::default(),
            rate_limit: RateLimitConfig::default(),
            security: SecurityConfig::default(),
            admin_username: "admin".to_string(),
            admin_password: "admin123".to_string(),
        }
    }
}

pub fn load_config() -> AppConfig {
    // 尝试从文件加载配置
    if let Ok(config_str) = std::fs::read_to_string("data/config.json") {
        if let Ok(config) = serde_json::from_str::<AppConfig>(&config_str) {
            return config;
        }
    }
    
    // 如果文件不存在或解析失败，使用默认配置
    let default_config = AppConfig::default();
    
    // 保存默认配置到文件
    if let Ok(config_str) = serde_json::to_string_pretty(&default_config) {
        let _ = std::fs::write("data/config.json", config_str);
    }
    
    default_config
}

#[allow(dead_code)]
pub fn save_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = serde_json::to_string_pretty(config)?;
    std::fs::write("data/config.json", config_str)?;
    Ok(())
}
