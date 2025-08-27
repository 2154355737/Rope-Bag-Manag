use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub file: FileConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub bcrypt_cost: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub upload_path: String,
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
    pub temp_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: Option<String>,
    pub max_files: usize,
    pub max_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u64,
}

// MailConfig已移至models/mail.rs，此处保留是为了向后兼容
// 实际的邮件配置现在存储在数据库中

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 15201,
                workers: 4,
                timeout: 30,
            },
            database: DatabaseConfig {
                url: "data.db".to_string(),
                max_connections: 10,
                timeout: 30,
            },
            auth: AuthConfig {
                jwt_secret: "your-secret-key-change-in-production".to_string(),
                jwt_expiration: 86400, // 24 hours
                bcrypt_cost: 12,
            },
            file: FileConfig {
                upload_path: "uploads".to_string(),
                max_file_size: 10485760, // 10MB
                allowed_extensions: vec![
                    "zip".to_string(),
                    "rar".to_string(),
                    "7z".to_string(),
                    "tar".to_string(),
                    "gz".to_string(),
                ],
                temp_path: "temp".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: Some("logs/app.log".to_string()),
                max_files: 5,
                max_size: 10485760, // 10MB
            },
            cors: CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec![
                    "Content-Type".to_string(),
                    "Authorization".to_string(),
                    "X-Requested-With".to_string(),
                ],
                max_age: 3600,
            },
// 邮件配置已移至数据库，config.toml不再包含邮件配置
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {

        // 1. 尝试从多种常见路径加载配置文件
        let candidate_paths = [
            "config.toml",                                     // 当前工作目录
            "./config.toml",                                   // 显式当前目录
            "rope-manager-backend/config.toml",                // 项目根目录启动时
            "./rope-manager-backend/config.toml",              // 同上（显式）
        ];

        for path in &candidate_paths {
            if let Ok(config) = Self::load_from_file(path) {
                return Ok(config);
            }
        }

        // 2. 尝试从环境变量加载
        if let Ok(config) = Self::load_from_env() {
            return Ok(config);
        }

        // 3. 使用默认配置
        Ok(Self::default())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Err("配置文件不存在".into());
        }

        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();

        // 服务器配置
        if let Ok(host) = env::var("HOST") {
            config.server.host = host;
        }
        if let Ok(port) = env::var("PORT") {
            config.server.port = port.parse().unwrap_or(15201);
        }
        if let Ok(workers) = env::var("WORKERS") {
            config.server.workers = workers.parse().unwrap_or(4);
        }
        if let Ok(timeout) = env::var("TIMEOUT") {
            config.server.timeout = timeout.parse().unwrap_or(30);
        }

        // 数据库配置
        if let Ok(url) = env::var("DATABASE_URL") {
            config.database.url = url;
        }
        if let Ok(max_conn) = env::var("DATABASE_MAX_CONNECTIONS") {
            config.database.max_connections = max_conn.parse().unwrap_or(10);
        }

        // 认证配置
        if let Ok(secret) = env::var("JWT_SECRET") {
            config.auth.jwt_secret = secret;
        }
        if let Ok(expiration) = env::var("JWT_EXPIRATION") {
            config.auth.jwt_expiration = expiration.parse().unwrap_or(86400);
        }
        if let Ok(cost) = env::var("BCRYPT_COST") {
            config.auth.bcrypt_cost = cost.parse().unwrap_or(12);
        }

        // 文件配置
        if let Ok(upload_path) = env::var("UPLOAD_PATH") {
            config.file.upload_path = upload_path;
        }
        if let Ok(max_size) = env::var("MAX_FILE_SIZE") {
            config.file.max_file_size = max_size.parse().unwrap_or(10485760);
        }
        if let Ok(temp_path) = env::var("TEMP_PATH") {
            config.file.temp_path = temp_path;
        }

        // 日志配置
        if let Ok(level) = env::var("LOG_LEVEL") {
            config.logging.level = level;
        }
        if let Ok(file_path) = env::var("LOG_FILE") {
            config.logging.file_path = Some(file_path);
        }

// 邮件配置环境变量已移除，配置现在存储在数据库中

        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = Self::default();
        config.save_to_file("config.toml")?;
        Ok(())
    }

    // 便捷方法
    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn jwt_secret(&self) -> &str {
        &self.auth.jwt_secret
    }

    pub fn upload_path(&self) -> &str {
        &self.file.upload_path
    }

    pub fn max_file_size(&self) -> usize {
        self.file.max_file_size
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
