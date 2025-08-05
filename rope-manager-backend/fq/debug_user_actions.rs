use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 调试用户行为记录问题...");
    
    // 1. 测试数据库连接
    println!("📋 步骤1: 测试数据库连接");
    let conn = Connection::open("data.db")?;
    println!("✅ 数据库连接成功");
    
    // 2. 检查表是否存在
    println!("📋 步骤2: 检查user_actions表");
    let table_exists = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='user_actions'");
    match table_exists {
        Ok(mut stmt) => {
            let exists = stmt.query_row([], |row| {
                let name: String = row.get(0)?;
                Ok(name)
            });
            match exists {
                Ok(name) => println!("✅ 表 {} 存在", name),
                Err(_) => {
                    println!("❌ user_actions表不存在，尝试创建...");
                    // 创建表
                    conn.execute(
                        "CREATE TABLE IF NOT EXISTS user_actions (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            user_id INTEGER,
                            action_type TEXT NOT NULL,
                            target_type TEXT,
                            target_id INTEGER,
                            details TEXT,
                            ip_address TEXT,
                            user_agent TEXT,
                            created_at TEXT NOT NULL
                        )",
                        [],
                    )?;
                    println!("✅ user_actions表创建成功");
                }
            }
        },
        Err(e) => {
            println!("❌ 查询表失败: {}", e);
            return Err(e.into());
        }
    }
    
    // 3. 测试查询记录
    println!("📋 步骤3: 测试查询记录");
    let count_result = conn.prepare("SELECT COUNT(*) FROM user_actions");
    match count_result {
        Ok(mut stmt) => {
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            println!("✅ 表中共有 {} 条记录", count);
        },
        Err(e) => {
            println!("❌ 查询记录数失败: {}", e);
            return Err(e.into());
        }
    }
    
    // 4. 测试分页查询
    println!("📋 步骤4: 测试分页查询");
    let query_result = conn.prepare(
        "SELECT id, user_id, action_type, target_type, target_id, details, 
                ip_address, user_agent, created_at 
         FROM user_actions 
         ORDER BY created_at DESC 
         LIMIT ? OFFSET ?"
    );
    
    match query_result {
        Ok(mut stmt) => {
            let rows = stmt.query_map([5, 0], |row| {
                Ok((
                    row.get::<_, i32>(0)?,        // id
                    row.get::<_, Option<i32>>(1)?, // user_id
                    row.get::<_, String>(2)?,      // action_type
                    row.get::<_, String>(8)?,      // created_at
                ))
            })?;
            
            println!("✅ 分页查询成功，结果:");
            for (i, row) in rows.enumerate() {
                match row {
                    Ok((id, user_id, action_type, created_at)) => {
                        println!("  {}. ID:{} 用户:{:?} 行为:{} 时间:{}", 
                            i + 1, id, user_id, action_type, created_at);
                    },
                    Err(e) => {
                        println!("  ❌ 解析行失败: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("❌ 分页查询失败: {}", e);
            return Err(e.into());
        }
    }
    
    // 5. 测试时间解析
    println!("📋 步骤5: 测试时间解析");
    let time_test = conn.prepare("SELECT created_at FROM user_actions LIMIT 1");
    match time_test {
        Ok(mut stmt) => {
            let time_result = stmt.query_row([], |row| {
                let time_str: String = row.get(0)?;
                Ok(time_str)
            });
            
            match time_result {
                Ok(time_str) => {
                    println!("✅ 获取到时间字符串: {}", time_str);
                    
                    // 尝试解析时间
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&time_str) {
                        println!("✅ RFC3339 时间解析成功: {}", dt);
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S") {
                        println!("✅ SQLite 时间解析成功: {}", dt);
                    } else {
                        println!("⚠️ 时间解析失败，但这不会阻止查询");
                    }
                },
                Err(e) => {
                    println!("⚠️ 没有记录或时间获取失败: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ 时间测试失败: {}", e);
        }
    }
    
    println!("\n✅ 调试完成！数据库层面看起来正常。");
    println!("💡 问题可能在于:");
    println!("   1. 服务层的错误处理");
    println!("   2. HTTP请求参数解析");
    println!("   3. 权限验证");
    println!("   4. 路由冲突");
    
    Ok(())
} 