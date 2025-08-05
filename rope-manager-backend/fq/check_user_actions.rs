use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("data.db")?;
    
    println!("🔍 检查user_actions表状态...");
    
    // 检查表是否存在
    let table_exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='user_actions')",
        [],
        |row| row.get(0)
    )?;
    
    if table_exists {
        println!("✅ user_actions表存在");
        
        // 检查表结构
        let mut stmt = conn.prepare("PRAGMA table_info(user_actions)")?;
        let column_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,    // cid
                row.get::<_, String>(1)?, // name  
                row.get::<_, String>(2)?, // type
                row.get::<_, i32>(3)?,    // notnull
                row.get::<_, Option<String>>(4)?, // dflt_value
                row.get::<_, i32>(5)?,    // pk
            ))
        })?;

        println!("📋 表结构:");
        println!("CID\t名称\t\t类型\t\t非空\t默认值\t主键");
        println!("--------------------------------------------------------");
        for column in column_iter {
            let (cid, name, col_type, notnull, dflt_value, pk) = column?;
            println!("{}\t{}\t\t{}\t\t{}\t{:?}\t{}", 
                cid, name, col_type, notnull, dflt_value, pk);
        }
        
        // 检查外键约束
        let mut stmt = conn.prepare("PRAGMA foreign_key_list(user_actions)")?;
        let fk_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,    // id
                row.get::<_, i32>(1)?,    // seq
                row.get::<_, String>(2)?, // table
                row.get::<_, String>(3)?, // from
                row.get::<_, String>(4)?, // to
            ))
        })?;
        
        println!("\n🔗 外键约束:");
        let mut has_fk = false;
        for fk in fk_iter {
            has_fk = true;
            let (id, seq, table, from, to) = fk?;
            println!("外键 {}.{}: {} -> {}.{}", id, seq, from, table, to);
        }
        
        if !has_fk {
            println!("无外键约束");
        }
        
        // 检查表记录数
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_actions",
            [],
            |row| row.get(0)
        )?;
        
        println!("\n📊 表记录数: {}", count);
        
        // 检查最近的记录
        if count > 0 {
            println!("\n📝 最近的3条记录:");
            let mut stmt = conn.prepare(
                "SELECT id, user_id, action_type, target_type, created_at 
                 FROM user_actions ORDER BY id DESC LIMIT 3"
            )?;
            let record_iter = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,      // id
                    row.get::<_, i32>(1)?,      // user_id
                    row.get::<_, String>(2)?,   // action_type
                    row.get::<_, Option<String>>(3)?, // target_type
                    row.get::<_, String>(4)?,   // created_at
                ))
            })?;
            
            for record in record_iter {
                let (id, user_id, action_type, target_type, created_at) = record?;
                println!("ID:{} 用户:{} 行为:{} 目标:{:?} 时间:{}", 
                    id, user_id, action_type, target_type, created_at);
            }
        }
        
    } else {
        println!("❌ user_actions表不存在！");
        println!("正在创建表...");
        
        conn.execute(
            "CREATE TABLE user_actions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
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
    
    // 测试插入一条记录
    println!("\n🧪 测试插入记录...");
    match conn.execute(
        "INSERT INTO user_actions (
            user_id, action_type, target_type, target_id, details, 
            ip_address, user_agent, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        [
            &1 as &dyn rusqlite::ToSql,
            &"test_action" as &dyn rusqlite::ToSql,
            &Some("test_target") as &dyn rusqlite::ToSql,
            &Some(1) as &dyn rusqlite::ToSql,
            &Some("测试记录") as &dyn rusqlite::ToSql,
            &Some("127.0.0.1") as &dyn rusqlite::ToSql,
            &Some("test-agent") as &dyn rusqlite::ToSql,
            &"2025-01-28T12:00:00Z" as &dyn rusqlite::ToSql,
        ],
    ) {
        Ok(rows) => {
            println!("✅ 测试插入成功，影响行数: {}", rows);
            let id = conn.last_insert_rowid();
            println!("新记录ID: {}", id);
            
            // 删除测试记录
            conn.execute("DELETE FROM user_actions WHERE id = ?", [id])?;
            println!("✅ 测试记录已清理");
        },
        Err(e) => {
            println!("❌ 测试插入失败: {}", e);
        }
    }
    
    Ok(())
} 