pub mod user_repo;
pub mod package_repo;
pub mod system_repo;
pub mod comment_repo;
pub mod user_action_repo; // 添加用户行为记录仓库
pub mod forbidden_word_repo; // 违禁词仓库
pub mod email_verification_repo;
pub mod subscription_repo;
pub mod mail_repo; // 添加邮件仓库
pub mod download_security_repo; // 添加下载安全仓库

pub use user_repo::*;
pub use package_repo::*;
pub use comment_repo::*;
pub use system_repo::*;

pub use email_verification_repo::*;
pub use subscription_repo::*;

use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn get_connection() -> Result<Connection> {
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "data.db".to_string());
    
    // 确保数据库目录存在
    if let Some(parent) = PathBuf::from(&db_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?;
    }
    
    Connection::open(&db_path)
} 