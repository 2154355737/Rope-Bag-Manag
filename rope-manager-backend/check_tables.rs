use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // 连接数据库
    let conn = Connection::open("data.db")?;
    
    // 查询所有表
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
    let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;
    
    println!("数据库中的表:");
    for table in tables {
        let table_name = table?;
        println!("- {}", table_name);
        
        // 查询表结构
        let mut pragma_stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
        let columns = pragma_stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?, // cid
                row.get::<_, String>(1)?, // name
                row.get::<_, String>(2)?, // type
                row.get::<_, bool>(3)?, // notnull
                row.get::<_, Option<String>>(4)?, // dflt_value
                row.get::<_, i32>(5)?, // pk
            ))
        })?;
        
        println!("  列信息:");
        for column in columns {
            let (cid, name, type_, notnull, dflt_value, pk) = column?;
            println!("  - {}: {} (类型: {}, 非空: {}, 默认值: {:?}, 主键: {})",
                    cid, name, type_, notnull, dflt_value, pk);
        }
        
        println!();
    }
    
    // 检查user_actions表是否存在
    let user_actions_exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='user_actions')",
        [],
        |row| row.get(0),
    )?;
    
    if user_actions_exists {
        println!("user_actions表存在");
    } else {
        println!("user_actions表不存在!");
    }
    
    Ok(())
} 