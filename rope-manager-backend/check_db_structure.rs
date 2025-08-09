use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("data.db")?;
    
    println!("🔍 检查 system_settings 表结构:");
    println!("==================================");
    
    // 获取表结构信息
    let mut stmt = conn.prepare("PRAGMA table_info(system_settings)")?;
    let table_info = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,     // cid
            row.get::<_, String>(1)?,  // name
            row.get::<_, String>(2)?,  // type
            row.get::<_, i32>(3)?,     // notnull
            row.get::<_, Option<String>>(4)?, // dflt_value
            row.get::<_, i32>(5)?,     // pk
        ))
    })?;
    
    println!("列序号 | 列名           | 类型      | 非空 | 默认值 | 主键");
    println!("-------|---------------|-----------|------|--------|-----");
    
    for info in table_info {
        let (cid, name, type_, notnull, dflt_value, pk) = info?;
        println!("{:6} | {:13} | {:9} | {:4} | {:6} | {:4}", 
            cid, name, type_, notnull, 
            dflt_value.unwrap_or("NULL".to_string()), pk);
    }
    
    println!("\n📊 现有配置项数量:");
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM system_settings",
        [],
        |row| row.get(0)
    )?;
    println!("配置项总数: {}", count);
    
    // 显示前5个配置项
    println!("\n📋 前5个配置项:");
    let mut stmt = conn.prepare("SELECT key, value FROM system_settings LIMIT 5")?;
    let settings = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;
    
    for setting in settings {
        let (key, value) = setting?;
        println!("  {} = {}", key, value);
    }
    
    Ok(())
} 