pub mod user_repo;
pub mod package_repo;
pub mod comment_repo;
pub mod system_repo;

pub use user_repo::*;
pub use package_repo::*;
pub use comment_repo::*;
pub use system_repo::*;

use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_database(db_path: &str) -> Result<()> {
    // 确保数据目录存在
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(db_path)?;
    
    // 创建表
    conn.execute_batch(include_str!("../sql/init.sql"))?;
    
    Ok(())
} 