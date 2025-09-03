use rusqlite::Connection;
use anyhow::Result;
use log::info;

pub struct DatabaseRepairService {
    db_path: String,
}

impl DatabaseRepairService {
    pub fn new(db_path: String) -> Self {
        Self { db_path }
    }

    pub fn repair_database(&self) -> Result<()> {
        info!("开始数据库修复...");

        let conn = Connection::open(&self.db_path)?;

        // 添加posts表的新字段
        self.add_posts_fields(&conn)?;
        
        // 更新现有数据
        self.update_existing_data(&conn)?;
        
        // 创建索引
        self.create_indexes(&conn)?;
        
        // 更新系统设置
        self.update_system_settings(&conn)?;

        info!("数据库修复完成");
        Ok(())
    }

    fn add_posts_fields(&self, conn: &Connection) -> Result<()> {
        info!("添加posts表字段...");

        let fields_to_add = vec![
            ("review_status", "TEXT DEFAULT 'pending'"),
            ("review_comment", "TEXT"),
            ("reviewer_id", "INTEGER"),
            ("reviewed_at", "DATETIME"),
            ("images", "TEXT DEFAULT '[]'"),
            ("code_snippet", "TEXT"),
            ("tags", "TEXT DEFAULT '[]'"),
            ("author_name", "TEXT"),
        ];

        for (field_name, field_def) in fields_to_add {
            // 检查字段是否已存在
            let exists: i32 = conn.prepare("SELECT COUNT(*) FROM pragma_table_info('posts') WHERE name = ?1")?
                .query_row([field_name], |row| row.get(0))?;

            if exists == 0 {
                let sql = format!("ALTER TABLE posts ADD COLUMN {} {}", field_name, field_def);
                conn.execute(&sql, [])?;
                info!("添加字段: {}", field_name);
            } else {
                info!("字段已存在: {}", field_name);
            }
        }

        Ok(())
    }

    fn update_existing_data(&self, conn: &Connection) -> Result<()> {
        info!("更新现有数据...");

        // 更新author_name字段
        conn.execute(
            "UPDATE posts SET author_name = (
                SELECT username FROM users WHERE users.id = posts.author_id
            ) WHERE author_name IS NULL",
            []
        )?;

        // 确保tags和images字段有默认值
        conn.execute("UPDATE posts SET tags = '[]' WHERE tags IS NULL", [])?;
        conn.execute("UPDATE posts SET images = '[]' WHERE images IS NULL", [])?;

        info!("数据更新完成");
        Ok(())
    }

    fn create_indexes(&self, conn: &Connection) -> Result<()> {
        info!("创建索引...");

        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_posts_review_status ON posts(review_status)",
            "CREATE INDEX IF NOT EXISTS idx_posts_reviewer_id ON posts(reviewer_id)",
            "CREATE INDEX IF NOT EXISTS idx_posts_reviewed_at ON posts(reviewed_at)",
            "CREATE INDEX IF NOT EXISTS idx_posts_author_name ON posts(author_name)",
        ];

        for index_sql in indexes {
            conn.execute(index_sql, [])?;
        }

        info!("索引创建完成");
        Ok(())
    }

    fn update_system_settings(&self, conn: &Connection) -> Result<()> {
        info!("更新系统设置...");

        // 创建system_settings表（如果不存在）
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                description TEXT,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            []
        )?;

        // 记录修复时间
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, description, updated_at) 
             VALUES (?1, ?2, ?3, datetime('now'))",
            ["database_repaired", "true", "数据库修复完成时间"]
        )?;

        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, description, updated_at) 
             VALUES (?1, ?2, ?3, datetime('now'))",
            ["posts_fields_updated", "true", "帖子字段更新完成时间"]
        )?;

        info!("系统设置更新完成");
        Ok(())
    }
} 