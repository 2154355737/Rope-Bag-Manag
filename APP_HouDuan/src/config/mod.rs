use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub jwt: JwtConfig,
    pub cache: CacheConfig,
    pub email: EmailConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    pub upload_path: String,
    pub temp_path: String,
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub default_ttl: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
    pub from_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_enabled: bool,
    pub file_path: Option<String>,
    pub console_enabled: bool,
    pub json_format: bool,
    pub with_file_info: bool,
    pub with_thread_ids: bool,
    pub with_timestamps: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        let s = Config::builder()
            // 默认配置
            .add_source(File::with_name("config/default"))
            // 环境特定配置
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // 本地配置（优先级最高）
            .add_source(File::with_name("config/local").required(false))
            // 环境变量覆盖
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        s.try_deserialize()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 15201,
                workers: num_cpus::get(),
            },
            database: DatabaseConfig {
                url: "sqlite:./data/app.db".to_string(),
                max_connections: 20,
                min_connections: 5,
                acquire_timeout: 30,
                idle_timeout: 300,
                max_lifetime: 1800,
            },
            storage: StorageConfig {
                upload_path: "./uploads".to_string(),
                temp_path: "./temp".to_string(),
                max_file_size: 50 * 1024 * 1024, // 50MB
                allowed_extensions: vec![
                    "zip".to_string(),
                    "rar".to_string(),
                    "7z".to_string(),
                    "tar".to_string(),
                    "gz".to_string(),
                    "jpg".to_string(),
                    "jpeg".to_string(),
                    "png".to_string(),
                    "gif".to_string(),
                    "webp".to_string(),
                ],
            },
            jwt: JwtConfig {
                secret: "your-secret-key-change-in-production".to_string(),
                expires_in: 86400,      // 24小时
                refresh_expires_in: 604800, // 7天
            },
            cache: CacheConfig {
                redis_url: "redis://127.0.0.1:6379".to_string(),
                default_ttl: 3600, // 1小时
            },
            email: EmailConfig {
                smtp_host: "smtp.qq.com".to_string(),
                smtp_port: 587,
                smtp_username: "".to_string(),
                smtp_password: "".to_string(),
                from_address: "noreply@jieshen.com".to_string(),
                from_name: "绳包管理器".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_enabled: true,
                file_path: Some("./logs".to_string()),
                console_enabled: true,
                json_format: false,
                with_file_info: true,
                with_thread_ids: true,
                with_timestamps: true,
            },
        }
    }
} 