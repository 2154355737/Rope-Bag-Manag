use rusqlite::{Connection, Result};
use std::fs;

fn main() -> Result<()> {
    println!("开始修复SQLite触发器...");
    
    // 读取修复脚本
    let sql_script = fs::read_to_string("fix_sqlite_triggers.sql")
        .expect("无法读取修复脚本文件");
    
    // 连接到数据库
    let conn = Connection::open("data.db")?;
    
    // 执行修复脚本
    conn.execute_batch(&sql_script)?;
    
    println!("SQLite触发器修复完成！");
    
    // 验证触发器是否创建成功
    let trigger_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='trigger' AND name IN ('update_follow_stats_on_insert', 'update_follow_stats_on_delete')",
        [],
        |row| row.get(0)
    )?;
    
    if trigger_count == 2 {
        println!("✅ 触发器创建成功");
    } else {
        println!("❌ 触发器创建失败，找到 {} 个触发器", trigger_count);
    }
    
    Ok(())
} 