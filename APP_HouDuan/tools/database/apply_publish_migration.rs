use anyhow::Result;
use rusqlite::Connection;

fn main() -> Result<()> {
    println!("🔧 开始应用发布页面数据库迁移...");
    
    // 连接数据库
    let mut conn = Connection::open("data.db")?;
    
    // 开始事务
    let tx = conn.transaction()?;
    
    // 检查并添加packages表的新字段
    println!("📦 更新packages表结构...");
    
    // 检查screenshots字段是否存在
    let screenshots_exists: bool = tx.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('packages') WHERE name='screenshots'",
        [],
        |row| row.get::<_, i32>(0).map(|count| count > 0)
    )?;
    
    if !screenshots_exists {
        tx.execute("ALTER TABLE packages ADD COLUMN screenshots TEXT DEFAULT NULL", [])?;
        println!("✅ 添加screenshots字段");
    } else {
        println!("⏭️ screenshots字段已存在");
    }
    
    // 检查cover_image字段是否存在
    let cover_image_exists: bool = tx.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('packages') WHERE name='cover_image'",
        [],
        |row| row.get::<_, i32>(0).map(|count| count > 0)
    )?;
    
    if !cover_image_exists {
        tx.execute("ALTER TABLE packages ADD COLUMN cover_image TEXT DEFAULT NULL", [])?;
        println!("✅ 添加cover_image字段");
    } else {
        println!("⏭️ cover_image字段已存在");
    }
    
    // 检查requirements字段是否存在
    let requirements_exists: bool = tx.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('packages') WHERE name='requirements'",
        [],
        |row| row.get::<_, i32>(0).map(|count| count > 0)
    )?;
    
    if !requirements_exists {
        tx.execute("ALTER TABLE packages ADD COLUMN requirements TEXT DEFAULT NULL", [])?;
        println!("✅ 添加requirements字段");
    } else {
        println!("⏭️ requirements字段已存在");
    }
    
    // 检查posts表是否存在
    let posts_table_exists: bool = tx.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='posts'",
        [],
        |row| row.get::<_, i32>(0).map(|count| count > 0)
    )?;
    
    if posts_table_exists {
        println!("📝 更新posts表结构...");
        
        // 检查images字段是否存在
        let images_exists: bool = tx.query_row(
            "SELECT COUNT(*) FROM pragma_table_info('posts') WHERE name='images'",
            [],
            |row| row.get::<_, i32>(0).map(|count| count > 0)
        )?;
        
        if !images_exists {
            tx.execute("ALTER TABLE posts ADD COLUMN images TEXT DEFAULT NULL", [])?;
            println!("✅ 添加posts.images字段");
        } else {
            println!("⏭️ posts.images字段已存在");
        }
        
        // 检查code_snippet字段是否存在
        let code_snippet_exists: bool = tx.query_row(
            "SELECT COUNT(*) FROM pragma_table_info('posts') WHERE name='code_snippet'",
            [],
            |row| row.get::<_, i32>(0).map(|count| count > 0)
        )?;
        
        if !code_snippet_exists {
            tx.execute("ALTER TABLE posts ADD COLUMN code_snippet TEXT DEFAULT NULL", [])?;
            println!("✅ 添加posts.code_snippet字段");
        } else {
            println!("⏭️ posts.code_snippet字段已存在");
        }
        
        // 检查tags字段是否存在（posts表可能缺少）
        let tags_exists: bool = tx.query_row(
            "SELECT COUNT(*) FROM pragma_table_info('posts') WHERE name='tags'",
            [],
            |row| row.get::<_, i32>(0).map(|count| count > 0)
        )?;
        
        if !tags_exists {
            tx.execute("ALTER TABLE posts ADD COLUMN tags TEXT DEFAULT NULL", [])?;
            println!("✅ 添加posts.tags字段");
        } else {
            println!("⏭️ posts.tags字段已存在");
        }
    } else {
        println!("⚠️ posts表不存在，跳过posts表迁移");
    }
    
    // 确保schema_migrations表存在
    tx.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL
        )",
        []
    )?;
    
    // 记录迁移
    tx.execute(
        "INSERT OR REPLACE INTO schema_migrations (version, applied_at) VALUES (?, datetime('now'))",
        ["add_package_publish_fields"]
    )?;
    
    // 提交事务
    tx.commit()?;
    
    println!("🎉 发布页面数据库迁移完成！");
    println!("📊 验证迁移结果...");
    
    // 验证packages表结构
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare("SELECT sql FROM sqlite_master WHERE type='table' AND name='packages'")?;
    let table_sql: String = stmt.query_row([], |row| row.get(0))?;
    println!("📦 packages表结构:");
    println!("{}", table_sql);
    
    if posts_table_exists {
        let mut stmt = conn.prepare("SELECT sql FROM sqlite_master WHERE type='table' AND name='posts'")?;
        let table_sql: String = stmt.query_row([], |row| row.get(0))?;
        println!("📝 posts表结构:");
        println!("{}", table_sql);
    }
    
    Ok(())
} 