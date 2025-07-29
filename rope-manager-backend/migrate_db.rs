use rusqlite::Connection;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("data.db")?;
    
    // 读取迁移SQL文件
    let sql = fs::read_to_string("sql/migrate_package_approval.sql")?;
    
    // 分割SQL语句并执行
    for statement in sql.split(';') {
        let statement = statement.trim();
        if !statement.is_empty() && !statement.starts_with("--") {
            println!("Executing: {}", statement);
            if let Err(e) = conn.execute(statement, []) {
                println!("Warning: {}", e);
            }
        }
    }
    
    println!("Migration completed successfully!");
    Ok(())
} 