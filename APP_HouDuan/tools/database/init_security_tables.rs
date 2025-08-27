use rusqlite::Connection;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 初始化安全相关数据库表...");
    
    // 读取SQL文件
    let sql_content = fs::read_to_string("sql/create_security_actions_tables.sql")?;
    
    // 连接数据库
    let conn = Connection::open("data.db")?;
    
    // 执行SQL语句
    conn.execute_batch(&sql_content)?;
    
    println!("✅ 安全表初始化完成！");
    println!("📋 已创建的表：");
    println!("   - ip_bans (IP封禁表)");
    println!("   - security_actions (安全操作记录表)");
    println!("   - security_config (安全配置表)");
    println!("   - ip_whitelist (IP白名单表)");
    
    // 验证表是否创建成功
    let tables = vec!["ip_bans", "security_actions", "security_config", "ip_whitelist"];
    for table in tables {
        let count: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'", table),
            [],
            |row| row.get(0),
        )?;
        
        if count > 0 {
            println!("   ✅ {} 表创建成功", table);
        } else {
            println!("   ❌ {} 表创建失败", table);
        }
    }
    
    Ok(())
} 