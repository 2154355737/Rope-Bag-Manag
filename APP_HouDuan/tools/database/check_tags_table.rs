use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("data.db")?;
    
    // 检查tags表是否存在
    let table_exists = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='tags'",
        [],
        |row| row.get::<_, String>(0)
    ).is_ok();
    
    if table_exists {
        println!("✅ tags表存在");
        
        // 检查表结构
        let mut stmt = conn.prepare("PRAGMA table_info(tags)")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?, // name
                row.get::<_, String>(2)?, // type
                row.get::<_, i32>(3)?,    // not_null
                row.get::<_, i32>(4)?,    // pk
                row.get::<_, Option<String>>(5)? // default_value
            ))
        })?;
        
        println!("📋 tags表结构:");
        for row in rows {
            let (name, col_type, not_null, pk, default) = row?;
            println!("  - {}: {} (not_null: {}, pk: {}, default: {:?})", 
                    name, col_type, not_null, pk, default);
        }
        
        // 检查数据
        let count: i32 = conn.query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0))?;
        println!("📊 tags表中有 {} 条记录", count);
        
        if count > 0 {
            let mut stmt = conn.prepare("SELECT id, name, description, color, use_count FROM tags LIMIT 5")?;
            let rows = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, i32>(4)?
                ))
            })?;
            
            println!("📝 前5条记录:");
            for row in rows {
                let (id, name, description, color, use_count) = row?;
                println!("  - ID: {}, 名称: {}, 描述: {:?}, 颜色: {:?}, 使用次数: {}", 
                        id, name, description, color, use_count);
            }
        }
    } else {
        println!("❌ tags表不存在");
        
        // 尝试创建表
        println!("🔧 尝试创建tags表...");
        match conn.execute_batch(include_str!("sql/migrate_posts_and_tags.sql")) {
            Ok(_) => println!("✅ tags表创建成功"),
            Err(e) => println!("❌ tags表创建失败: {}", e),
        }
    }
    
    Ok(())
} 