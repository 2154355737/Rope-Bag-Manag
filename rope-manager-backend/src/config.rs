use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub upload_path: String,
    pub max_file_size: usize,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "data/rope_manager.db".to_string());
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key".to_string());
        let upload_path = env::var("UPLOAD_PATH")
            .unwrap_or_else(|_| "uploads".to_string());
        let max_file_size = env::var("MAX_FILE_SIZE")
            .unwrap_or_else(|_| "10485760".to_string()) // 10MB
            .parse::<usize>()
            .unwrap_or(10485760);

        Ok(Config {
            host,
            port,
            database_url,
            jwt_secret,
            upload_path,
            max_file_size,
        })
    }
}
