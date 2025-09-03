use rusqlite::{Connection, Result};
use std::fs;

fn main() -> Result<()> {
    println!("开始修复关注表...");
    
    // 读取SQL脚本
    let sql_script = fs::read_to_string("fix_follow_tables.sql")
        .expect("无法读取修复脚本文件");
    
    // 连接到数据库
    let conn = Connection::open("data.db")?;
    
    // 执行SQL脚本
    conn.execute_batch(&sql_script)?;
    
    println!("关注表修复完成！");
    
    // 验证表是否创建成功
    let table_exists: bool = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='user_follows'",
        [],
        |_| Ok(true)
    ).unwrap_or(false);
    
    if table_exists {
        println!("✅ user_follows 表创建成功");
    } else {
        println!("❌ user_follows 表创建失败");
    }
    
    let stats_table_exists: bool = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='user_follow_stats'",
        [],
        |_| Ok(true)
    ).unwrap_or(false);
    
    if stats_table_exists {
        println!("✅ user_follow_stats 表创建成功");
    } else {
        println!("❌ user_follow_stats 表创建失败");
    }
    
    Ok(())
} 