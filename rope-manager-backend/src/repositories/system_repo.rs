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

    // 获取资源记录列表
    pub async fn get_resource_records(&self) -> Result<Vec<crate::models::ResourceRecord>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT r.id, r.resource_id, r.resource_type, p.name as resource_name, 
                r.user_id, u.username as user_name, r.action, r.ip_address,
                r.old_data, r.new_data, r.timestamp, r.created_at, r.updated_at,
                COALESCE(p.file_size, 0) as file_size,
                COALESCE(p.download_count, 0) as download_count
         FROM resource_records r
         LEFT JOIN packages p ON r.resource_id = p.id AND r.resource_type = 'Package'
         LEFT JOIN users u ON r.user_id = u.id
         ORDER BY r.timestamp DESC"
        )?;

        let records = stmt.query_map([], |row| {
            Ok(crate::models::ResourceRecord {
                id: row.get(0)?,
                resource_id: row.get(1)?,
                resource_type: row.get(2)?,
                resource_name: row.get(3)?,
                user_id: row.get(4)?,
                user_name: row.get(5)?,
                action: row.get(6)?,
                ip_address: row.get(7)?,
                old_data: row.get(8)?,
                new_data: row.get(9)?,
                timestamp: row.get(10)?,
                file_size: row.get(13).ok(),
                download_count: row.get(14).ok(),
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    // 添加记录资源操作的方法
    pub async fn log_resource_action(&self, record: &crate::models::CreateResourceRecordRequest, user_id: i32) -> Result<i64> {
        let conn = self.conn.lock().await;
        
        let timestamp = chrono::Utc::now().timestamp();
        
        // 尝试获取客户端IP地址，这里简化处理
        let ip_address = "127.0.0.1"; // 实际应用中应从请求中获取
        
        conn.execute(
            "INSERT INTO resource_records (resource_id, resource_type, user_id, action, ip_address, old_data, new_data, timestamp, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))",
            params![
                record.resource_id,
                record.resource_type,
                user_id,
                record.action,
                ip_address,
                record.old_data.as_deref().unwrap_or(""),
                record.new_data.as_deref().unwrap_or(""),
                timestamp
            ]
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    // 添加删除资源记录方法
    pub async fn delete_resource_record(&self, record_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        
        conn.execute(
            "DELETE FROM resource_records WHERE id = ?",
            [record_id]
        )?;
        
        Ok(())
    }

    // 添加批量删除资源记录方法
    pub async fn batch_delete_resource_records(&self, record_ids: &[i32]) -> Result<usize> {
        let conn = self.conn.lock().await;
        
        let placeholders = record_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
            
        let query = format!("DELETE FROM resource_records WHERE id IN ({})", placeholders);
        
        let mut stmt = conn.prepare(&query)?;
        
        let params: Vec<&dyn rusqlite::ToSql> = record_ids
            .iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect();
        
        let count = stmt.execute(&params[..])?;
        
        Ok(count)
    }

    // 获取资源操作统计
    pub async fn get_resource_action_stats(&self) -> Result<crate::models::ResourceActionStats> {
        let conn = self.conn.lock().await;
        
        // 获取各种操作计数
        let mut stmt = conn.prepare(
            "SELECT 
                COUNT(CASE WHEN action = 'Create' THEN 1 END) as create_count,
                COUNT(CASE WHEN action = 'Update' THEN 1 END) as update_count,
                COUNT(CASE WHEN action = 'Delete' THEN 1 END) as delete_count,
                COUNT(CASE WHEN action = 'Download' THEN 1 END) as download_count
             FROM resource_records"
        )?;
        
        let counts = stmt.query_row([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?
            ))
        })?;
        
        // 获取每日统计
        let mut daily_stmt = conn.prepare(
            "SELECT 
                date(created_at) as day,
                COUNT(*) as count
             FROM resource_records
             GROUP BY date(created_at)
             ORDER BY day DESC
             LIMIT 30"
        )?;
        
        let daily_stats = daily_stmt.query_map([], |row| {
            Ok(crate::models::DailyStats {
                date: row.get(0)?,
                count: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(crate::models::ResourceActionStats {
            create_count: counts.0,
            update_count: counts.1,
            delete_count: counts.2,
            download_count: counts.3,
            by_day: daily_stats,
        })
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