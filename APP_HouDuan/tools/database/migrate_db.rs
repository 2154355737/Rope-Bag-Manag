use rusqlite::Connection;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("data.db")?;
    
    // 需要执行的迁移脚本
    let migration_files = vec![
        "sql/add_package_tags.sql",
        "sql/add_tags_table.sql",
    ];

    for file in migration_files {
        println!("Running migration file: {}", file);
        let sql = fs::read_to_string(file)?;
        for statement in sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("--") {
                println!("Executing: {}", statement);
                if let Err(e) = conn.execute(statement, []) {
                    println!("Warning: {}", e);
                }
            }
        }
    }
    
    println!("Migration completed successfully!");
    Ok(())
} 