use rusqlite::{Connection, Result};
use serde_json;

fn main() -> Result<()> {
    println!("开始测试IP封禁表...");
    
    // 连接数据库
    let conn = Connection::open("data.db")?;
    println!("✅ 数据库连接成功");
    
    // 检查表是否存在
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='ip_bans'")?;
    let table_exists = stmt.exists([])?;
    println!("ip_bans表存在: {}", table_exists);
    
    if table_exists {
        // 检查表结构
        let mut stmt = conn.prepare("PRAGMA table_info(ip_bans)")?;
        let columns = stmt.query_map([], |row| {
            Ok(format!("{}: {}", row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })?.collect::<Result<Vec<_>>>()?;
        
        println!("表结构:");
        for column in columns {
            println!("  {}", column);
        }
        
        // 尝试查询数据
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM ip_bans")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        println!("IP封禁记录数量: {}", count);
        
        // 尝试获取所有记录
        let mut stmt = conn.prepare(
            "SELECT id, ip_address, reason, ban_type, duration_hours, created_at, expires_at, is_active, created_by, notes 
             FROM ip_bans 
             ORDER BY created_at DESC"
        )?;
        
        let bans = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i32>(0)?,
                "ip_address": row.get::<_, String>(1)?,
                "reason": row.get::<_, String>(2)?,
                "ban_type": row.get::<_, String>(3)?,
                "duration_hours": row.get::<_, Option<i32>>(4)?,
                "created_at": row.get::<_, String>(5)?,
                "expires_at": row.get::<_, Option<String>>(6)?,
                "is_active": row.get::<_, bool>(7)?,
                "created_by": row.get::<_, String>(8)?,
                "notes": row.get::<_, Option<String>>(9)?
            }))
        })?.collect::<Result<Vec<_>>>()?;
        
        println!("查询到的记录:");
        for ban in bans {
            println!("  {}", ban);
        }
    } else {
        println!("❌ ip_bans表不存在，需要创建表");
        
        // 尝试创建表
        conn.execute_batch(include_str!("sql/create_ip_ban_tables.sql"))?;
        println!("✅ 已创建ip_bans表");
    }
    
    Ok(())
} 