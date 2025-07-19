use rusqlite::{params, Connection, OptionalExtension, Result as SqliteResult};
use crate::models::system::{Category, Stats, UserAction, CreateCategoryRequest, UpdateCategoryRequest};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;
use std::path::Path;
use chrono::Utc;
use anyhow::Result;

#[derive(Clone)]
pub struct SystemRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SystemRepository {
    pub fn new(db_path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn get_stats(&self) -> Result<Stats> {
        let conn = self.conn.lock().await;
        
        // 获取用户统计
        let total_users: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        let active_users: i64 = conn.query_row(
            "SELECT COUNT(*) FROM users WHERE last_login > datetime('now', '-7 days')", 
            [], 
            |row| row.get(0)
        )?;
        let new_users_today: i64 = conn.query_row(
            "SELECT COUNT(*) FROM users WHERE created_at > datetime('now', 'start of day')", 
            [], 
            |row| row.get(0)
        )?;

        // 获取绳包统计
        let total_packages: i64 = conn.query_row("SELECT COUNT(*) FROM packages", [], |row| row.get(0))?;
        let new_packages_today: i64 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE created_at > datetime('now', 'start of day')", 
            [], 
            |row| row.get(0)
        )?;

        // 获取评论统计
        let total_comments: i64 = conn.query_row("SELECT COUNT(*) FROM comments", [], |row| row.get(0))?;

        Ok(Stats {
            total_users,
            total_packages,
            total_comments,
            active_users,
            new_users_today,
            new_packages_today,
            system_status: "normal".to_string(),
            uptime: 86400, // 24小时，单位秒
        })
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, enabled, created_at 
             FROM categories ORDER BY created_at ASC"
        )?;

        let categories = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub async fn get_user_actions(&self) -> Result<Vec<UserAction>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, action_type, target_type, target_id, details, 
                    ip_address, user_agent, created_at 
             FROM user_actions ORDER BY created_at DESC LIMIT 100"
        )?;

        let actions = stmt.query_map([], |row| {
            Ok(UserAction {
                id: row.get(0)?,
                user_id: row.get(1)?,
                action_type: row.get(2)?,
                target_type: row.get(3)?,
                target_id: row.get(4)?,
                details: row.get(5)?,
                ip_address: row.get(6)?,
                user_agent: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(actions)
    }

    // 新增方法：获取系统日志
    pub async fn get_logs(&self) -> Result<Vec<crate::services::admin_service::SystemLog>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, level, message, timestamp, details 
             FROM system_logs ORDER BY timestamp DESC LIMIT 100"
        )?;

        let logs = stmt.query_map([], |row| {
            Ok(crate::services::admin_service::SystemLog {
                id: row.get(0)?,
                level: row.get(1)?,
                message: row.get(2)?,
                timestamp: row.get(3)?,
                details: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(logs)
    }

    // 新增方法：创建备份
    pub async fn create_backup(&self, file_path: &str) -> Result<()> {
        // 确保备份目录存在
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // 简单复制数据库文件作为备份
        let db_path = "data.db";
        if Path::new(db_path).exists() {
            fs::copy(db_path, file_path).map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        
        Ok(())
    }

    // 新增方法：获取备份列表
    pub async fn get_backups(&self) -> Result<Vec<crate::services::admin_service::BackupInfo>> {
        let backup_dir = "backups";
        if !Path::new(backup_dir).exists() {
            return Ok(vec![]);
        }

        let mut backups = Vec::new();
        for entry in fs::read_dir(backup_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "db") {
                let metadata = fs::metadata(&path)?;
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let backup_id = file_name.replace("backup_", "").replace(".db", "");
                
                backups.push(crate::services::admin_service::BackupInfo {
                    backup_id,
                    file_path: path.to_str().unwrap().to_string(),
                    size: metadata.len(),
                    created_at: Utc::now().to_rfc3339(),
                });
            }
        }

        Ok(backups)
    }

    // 新增方法：获取备份文件路径
    pub async fn get_backup_path(&self, backup_id: &str) -> Result<String> {
        let backup_path = format!("backups/backup_{}.db", backup_id);
        if Path::new(&backup_path).exists() {
            Ok(backup_path)
        } else {
            Err(anyhow::anyhow!("Backup not found"))
        }
    }

    // 新增方法：删除备份
    pub async fn delete_backup(&self, backup_id: &str) -> Result<()> {
        let backup_path = format!("backups/backup_{}.db", backup_id);
        if Path::new(&backup_path).exists() {
            fs::remove_file(backup_path)?;
        }
        Ok(())
    }

    // 新增方法：获取公告
    pub async fn get_announcements(&self) -> Result<Vec<crate::services::admin_service::Announcement>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, title, content, priority, created_at, updated_at 
             FROM announcements ORDER BY priority DESC, created_at DESC"
        )?;

        let announcements = stmt.query_map([], |row| {
            Ok(crate::services::admin_service::Announcement {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                priority: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(announcements)
    }

    // 新增方法：创建公告
    pub async fn create_announcement(&self, title: &str, content: &str, priority: i32) -> Result<crate::services::admin_service::Announcement> {
        let conn = self.conn.lock().await;
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO announcements (title, content, priority, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?)",
            [title, content, &priority.to_string(), &now, &now]
        )?;

        let id = conn.last_insert_rowid() as i32;
        
        Ok(crate::services::admin_service::Announcement {
            id,
            title: title.to_string(),
            content: content.to_string(),
            priority,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    // 新增方法：更新公告
    pub async fn update_announcement(&self, id: i32, title: &str, content: &str, priority: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "UPDATE announcements SET title = ?, content = ?, priority = ?, updated_at = ? 
             WHERE id = ?",
            [title, content, &priority.to_string(), &now, &id.to_string()]
        )?;

        Ok(())
    }

    // 新增方法：删除公告
    pub async fn delete_announcement(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM announcements WHERE id = ?", [id])?;
        Ok(())
    }

    // 新增方法：获取主题设置
    pub async fn get_theme_settings(&self) -> Result<crate::services::admin_service::ThemeSettings> {
        let conn = self.conn.lock().await;
        
        // 从系统设置表获取主题配置
        let primary_color: String = conn.query_row(
            "SELECT value FROM system_settings WHERE key = 'primary_color'", 
            [], 
            |row| row.get(0)
        ).unwrap_or("#409EFF".to_string());
        
        let secondary_color: String = conn.query_row(
            "SELECT value FROM system_settings WHERE key = 'secondary_color'", 
            [], 
            |row| row.get(0)
        ).unwrap_or("#67C23A".to_string());
        
        let dark_mode: bool = conn.query_row(
            "SELECT value FROM system_settings WHERE key = 'dark_mode'", 
            [], 
            |row| row.get(0)
        ).unwrap_or(false);
        
        let font_size: String = conn.query_row(
            "SELECT value FROM system_settings WHERE key = 'font_size'", 
            [], 
            |row| row.get(0)
        ).unwrap_or("14px".to_string());
        
        let language: String = conn.query_row(
            "SELECT value FROM system_settings WHERE key = 'language'", 
            [], 
            |row| row.get(0)
        ).unwrap_or("zh-CN".to_string());

        Ok(crate::services::admin_service::ThemeSettings {
            primary_color,
            secondary_color,
            dark_mode,
            font_size,
            language,
        })
    }

    // 新增方法：更新主题设置
    pub async fn update_theme_settings(&self, primary_color: &str, secondary_color: &str, dark_mode: bool) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 更新或插入主题设置
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
            ["primary_color", primary_color]
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
            ["secondary_color", secondary_color]
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
            ["dark_mode", &dark_mode.to_string()]
        )?;

        Ok(())
    }

    // 新增方法：获取资源记录
    pub async fn get_resource_records(&self) -> Result<Vec<crate::services::admin_service::ResourceRecord>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, type, file_size, download_count, created_at, updated_at 
             FROM packages ORDER BY created_at DESC"
        )?;

        let records = stmt.query_map([], |row| {
            Ok(crate::services::admin_service::ResourceRecord {
                id: row.get(0)?,
                resource_name: row.get(1)?,
                resource_type: row.get(2)?,
                file_size: row.get(3)?,
                download_count: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    // 获取单个分类
    pub async fn get_category_by_id(&self, id: i32) -> Result<Option<Category>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, description, enabled, created_at FROM categories WHERE id = ?";
        println!("[SQL] get_category_by_id: {}", sql);
        
        let mut stmt = conn.prepare(sql)?;
        
        let category = match stmt.query_row(rusqlite::params![id], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3)?,
                created_at: row.get(4)?,
            })
        }) {
            Ok(c) => Some(c),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => {
                println!("[ERROR] get_category_by_id error: {}", e);
                return Err(e.into());
            }
        };
        
        Ok(category)
    }

    // 创建分类
    pub async fn create_category(&self, req: &CreateCategoryRequest) -> Result<Category> {
        let conn = self.conn.lock().await;
        let sql = "INSERT INTO categories (name, description, enabled) VALUES (?, ?, ?)";
        println!("[SQL] create_category: {}", sql);
        
        let now = chrono::Utc::now();
        
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(rusqlite::params![
            req.name,
            req.description,
            req.enabled
        ])?;
        
        let id = conn.last_insert_rowid() as i32;
        
        let category = Category {
            id,
            name: req.name.clone(),
            description: req.description.clone(),
            enabled: req.enabled,
            created_at: now,
        };
        
        Ok(category)
    }

    // 更新分类
    pub async fn update_category(&self, id: i32, req: &UpdateCategoryRequest) -> Result<Category> {
        // 先检查分类是否存在
        let category = self.get_category_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("分类不存在"))?;
        
        let conn = self.conn.lock().await;
        let sql = "UPDATE categories SET name = ?, description = ?, enabled = ? WHERE id = ?";
        println!("[SQL] update_category: {}", sql);
        
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(rusqlite::params![
            req.name.as_ref().unwrap_or(&category.name),
            req.description.as_ref().or(category.description.as_ref()),
            req.enabled.unwrap_or(category.enabled),
            id
        ])?;
        
        // 获取更新后的分类
        let updated_category = Category {
            id,
            name: req.name.clone().unwrap_or(category.name),
            description: req.description.clone().or(category.description),
            enabled: req.enabled.unwrap_or(category.enabled),
            created_at: category.created_at,
        };
        
        Ok(updated_category)
    }

    // 删除分类
    pub async fn delete_category(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        let sql = "DELETE FROM categories WHERE id = ?";
        println!("[SQL] delete_category: {}", sql);
        
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(rusqlite::params![id])?;
        
        Ok(())
    }
} 