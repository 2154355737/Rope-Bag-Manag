use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Arc::new(Mutex::new(Connection::open("data.db")?));
    let conn_guard = conn.lock().await;
    
    // 检查packages表结构
    println!("Current packages table schema:");
    let mut stmt = conn_guard.prepare("PRAGMA table_info(packages)")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,     // cid
            row.get::<_, String>(1)?,  // name
            row.get::<_, String>(2)?,  // type
            row.get::<_, i32>(3)?,     // notnull
            row.get::<_, Option<String>>(4)?, // dflt_value
            row.get::<_, i32>(5)?,     // pk
        ))
    })?;
    
    for row in rows {
        let (cid, name, type_name, notnull, dflt_value, pk) = row?;
        println!("{}: {} {} {} {} {}", cid, name, type_name, notnull, 
                dflt_value.unwrap_or("NULL".to_string()), pk);
    }
    
    // 尝试添加字段（如果不存在）
    let fields_to_add = vec![
        ("reviewer_id", "INTEGER DEFAULT NULL"),
        ("reviewed_at", "DATETIME DEFAULT NULL"),
        ("review_comment", "TEXT DEFAULT NULL"),
    ];
    
    for (field_name, field_def) in fields_to_add {
        let sql = format!("ALTER TABLE packages ADD COLUMN {} {}", field_name, field_def);
        match conn_guard.execute(&sql, []) {
            Ok(_) => println!("Added field: {}", field_name),
            Err(e) => {
                if e.to_string().contains("duplicate column name") {
                    println!("Field {} already exists", field_name);
                } else {
                    println!("Error adding field {}: {}", field_name, e);
                }
            }
        }
    }
    
    // 更新现有数据的状态
    match conn_guard.execute("UPDATE packages SET status = 'pending' WHERE status = 'active'", []) {
        Ok(count) => println!("Updated {} packages to pending status", count),
        Err(e) => println!("Error updating package status: {}", e),
    }
    
    // 创建索引
    let indexes = vec![
        "CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status)",
        "CREATE INDEX IF NOT EXISTS idx_packages_reviewer ON packages(reviewer_id)",
    ];
    
    for index_sql in indexes {
        match conn_guard.execute(index_sql, []) {
            Ok(_) => println!("Created index successfully"),
            Err(e) => println!("Error creating index: {}", e),
        }
    }
    
    println!("Database migration completed!");
    Ok(())
} 