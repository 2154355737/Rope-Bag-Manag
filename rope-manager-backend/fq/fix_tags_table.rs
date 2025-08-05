use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("data.db")?;
    
    println!("🔧 正在检查和修复tags表...");
    
    // 直接执行SQL创建表
    let sql = r#"
    -- 创建标签表
    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(50) UNIQUE NOT NULL,
        description TEXT,
        color VARCHAR(20),
        use_count INTEGER DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    -- 创建帖子标签关联表
    CREATE TABLE IF NOT EXISTS post_tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        post_id INTEGER NOT NULL,
        tag_id INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
        FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
        UNIQUE(post_id, tag_id)
    );

    -- 创建索引
    CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
    CREATE INDEX IF NOT EXISTS idx_tags_use_count ON tags(use_count);
    CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags(post_id);
    CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags(tag_id);

    -- 插入一些默认标签
    INSERT OR IGNORE INTO tags (name, description, color, use_count) VALUES 
    ('技术分享', '技术相关的分享和讨论', '#1890ff', 0),
    ('经验交流', '经验和心得的交流', '#52c41a', 0),
    ('问题求助', '遇到问题寻求帮助', '#faad14', 0),
    ('资源推荐', '推荐优质资源', '#f5222d', 0),
    ('教程指南', '教程和指南类内容', '#722ed1', 0),
    ('社区活动', '社区活动和公告', '#13c2c2', 0),
    ('Rust', 'Rust编程语言相关', '#ff4d4f', 0),
    ('Vue', 'Vue.js前端框架相关', '#52c41a', 0),
    ('数据库', '数据库相关技术', '#1890ff', 0),
    ('API', 'API设计和开发', '#722ed1', 0);
    "#;
    
    match conn.execute_batch(sql) {
        Ok(_) => {
            println!("✅ tags表创建/修复成功");
            
            // 验证表是否存在
            let count: i32 = conn.query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0))?;
            println!("📊 tags表中有 {} 条记录", count);
            
            if count > 0 {
                let mut stmt = conn.prepare("SELECT id, name FROM tags LIMIT 5")?;
                let rows = stmt.query_map([], |row| {
                    Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
                })?;
                
                println!("📝 标签列表:");
                for row in rows {
                    let (id, name) = row?;
                    println!("  - ID: {}, 名称: {}", id, name);
                }
            }
        },
        Err(e) => println!("❌ tags表创建失败: {}", e),
    }
    
    Ok(())
} 