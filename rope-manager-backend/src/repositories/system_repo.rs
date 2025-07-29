use std::{fs, path::Path, sync::Arc};
use rusqlite::{params, Connection, OptionalExtension, Result as SqliteResult};
use tokio::sync::Mutex;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;


// 导入所需模型
use crate::models::user_action::UserAction;
use crate::models::system::{Category, CreateCategoryRequest, UpdateCategoryRequest};

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

    pub async fn get_stats(&self) -> Result<crate::models::Stats> {
        let conn = self.conn.lock().await;
        
        // 获取总用户数
        let total_users: i64 = match conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0)) {
            Ok(count) => count,
            Err(_) => 0, // 表可能不存在或其他错误
        };
        
        // 获取总包数
        let total_packages: i64 = match conn.query_row("SELECT COUNT(*) FROM packages", [], |row| row.get(0)) {
            Ok(count) => count,
            Err(_) => 0,
        };
        
        // 获取总评论数
        let total_comments: i64 = match conn.query_row("SELECT COUNT(*) FROM comments", [], |row| row.get(0)) {
            Ok(count) => count,
            Err(_) => 0,
        };
        
        // 获取活跃用户数（30天内登录过的）
        let active_users: i64 = match conn.query_row(
            "SELECT COUNT(DISTINCT user_id) FROM user_actions WHERE action_type = 'Login' AND timestamp > datetime('now', '-30 day')",
            [], 
            |row| row.get(0)
        ) {
            Ok(count) => count,
            Err(_) => 0,
        };
        
        // 获取今日新增用户数
        let new_users_today: i64 = match conn.query_row(
            "SELECT COUNT(*) FROM users WHERE created_at > datetime('now', 'start of day')", 
            [], 
            |row| row.get(0)
        ) {
            Ok(count) => count,
            Err(_) => 0,
        };

        // 获取今日新增包数
        let new_packages_today: i64 = match conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE created_at > datetime('now', 'start of day')", 
            [], 
            |row| row.get(0)
        ) {
            Ok(count) => count,
            Err(_) => 0,
        };
        
        // 创建Stats对象
        let stats = crate::models::Stats {
            total_users: total_users as i32,
            total_packages: total_packages as i32,
            total_comments: total_comments as i32,
            active_users: active_users as i32,
            new_users_today: new_users_today as i32,
            new_packages_today: new_packages_today as i32,
            system_status: "Running".to_string(),
            uptime: 0, // 暂时不计算运行时间
        };
        
        Ok(stats)
    }

    // 修复get_categories方法
    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let conn = self.conn.lock().await;
        
        // 检查表是否存在，不存在则创建
        conn.execute(
            "CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                enabled INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT
            )",
            []
        )?;
        
        // 检查表中是否有数据，没有则插入默认分类
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))?;
        if count == 0 {
            conn.execute(
                "INSERT INTO categories (name, description, enabled, created_at)
                VALUES ('默认分类', '系统默认分类', 1, datetime('now'))",
                []
            )?;
        }
        
        // 获取表结构信息
        let mut has_description = false;
        let mut has_enabled = false;
        let mut has_updated_at = false;
        
        let mut stmt = conn.prepare("PRAGMA table_info(categories)")?;
        let cols = stmt.query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?;
        
        for col_result in cols {
            if let Ok(col_name) = col_result {
                if col_name == "description" {
                    has_description = true;
                } else if col_name == "enabled" {
                    has_enabled = true;
                } else if col_name == "updated_at" {
                    has_updated_at = true;
                }
            }
        }
        
        // 构建查询
        let sql = format!(
            "SELECT id, name, {} {} subscription_locked, created_at {} FROM categories",
            if has_description { "description," } else { "NULL as description," },
            if has_enabled { "enabled," } else { "1 as enabled," },
            if has_updated_at { ", updated_at" } else { ", NULL as updated_at" }
        );
        
        let mut stmt = conn.prepare(&sql)?;
        
        let rows = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3).unwrap_or(true),
                subscription_locked: row.get(4).unwrap_or(false),
                created_at: row.get(5)?,
                updated_at: row.get(6).ok(),
            })
        })?;
        
        let mut categories = Vec::new();
        for category_result in rows {
            match category_result {
                Ok(category) => categories.push(category),
                Err(e) => {
                    eprintln!("Error fetching category: {}", e);
                    // 跳过错误的记录但不中断整个查询
                    continue;
                }
            }
        }

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
    pub async fn get_logs(&self, page: Option<u32>, page_size: Option<u32>, level: Option<&str>, search: Option<&str>) -> Result<(Vec<crate::services::admin_service::SystemLog>, i64)> {
        let conn = self.conn.lock().await;
        
        // 创建日志表（如果不存在）
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                level TEXT NOT NULL,
                message TEXT NOT NULL,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                details TEXT
            )",
            [],
        )?;
        
        // 构建查询条件
        let mut conditions = Vec::new();
        let mut params_vec: Vec<String> = Vec::new();
        let mut params_ref: Vec<&dyn rusqlite::ToSql> = Vec::new();
        
        if let Some(l) = level {
            conditions.push("level = ?");
            params_vec.push(l.to_string());
        }
        
        if let Some(s) = search {
            conditions.push("(message LIKE ? OR details LIKE ?)");
            let search_pattern = format!("%{}%", s);
            params_vec.push(search_pattern.clone());
            params_vec.push(search_pattern);
        }
        
        // 更新参数引用
        for param in &params_vec {
            params_ref.push(param as &dyn rusqlite::ToSql);
        }
        
        // 构建查询语句
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // 获取总记录数
        let count_query = format!("SELECT COUNT(*) FROM system_logs {}", where_clause);
        let total: i64 = conn.query_row(&count_query, &params_ref[..], |row| row.get(0))?;
        
        // 构建分页查询
        let limit_offset = match (page_size, page) {
            (Some(limit), Some(page_num)) => {
                let offset = (page_num - 1) * limit;
                format!("LIMIT {} OFFSET {}", limit, offset)
            }
            (Some(limit), None) => format!("LIMIT {}", limit),
            _ => String::new(),
        };
        
        // 构建完整查询
        let query = format!(
            "SELECT id, level, message, timestamp, details 
             FROM system_logs 
             {} 
             ORDER BY timestamp DESC 
             {}",
            where_clause, limit_offset
        );
        
        // 执行查询
        let mut stmt = conn.prepare(&query)?;
        
        let logs = stmt.query_map(&params_ref[..], |row| {
            Ok(crate::services::admin_service::SystemLog {
                id: row.get(0)?,
                level: row.get(1)?,
                message: row.get(2)?,
                timestamp: row.get(3)?,
                details: row.get(4)?,
            })
        })?
        .filter_map(|result| result.ok())
        .collect::<Vec<_>>();
        
        Ok((logs, total))
    }

    // 添加记录日志的方法
    pub async fn add_log(&self, level: &str, message: &str, details: Option<&str>) -> Result<i64> {
        let conn = self.conn.lock().await;
        
        // 创建日志表（如果不存在）
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                level TEXT NOT NULL,
                message TEXT NOT NULL,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                details TEXT
            )",
            [],
        )?;
        
        // 插入日志
        conn.execute(
            "INSERT INTO system_logs (level, message, details) VALUES (?, ?, ?)",
            params![level, message, details],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    // 完善创建备份方法
    pub async fn create_backup(&self, backup_type: &str, description: Option<&str>, user_id: Option<i32>) -> Result<crate::models::system::BackupInfo> {
        // 生成唯一备份ID
        let backup_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let timestamp_str = timestamp.format("%Y%m%d_%H%M%S").to_string();
        
        // 创建备份文件名
        let filename = format!("backup_{}_{}.db", backup_type.to_lowercase(), timestamp_str);
        let file_path = format!("backups/{}", filename);
        
        // 确保备份目录存在
        if let Some(parent) = Path::new(&file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // 获取当前数据库路径
        let db_path = "data.db";
        let mut status = "Success";
        let mut file_size = 0;
        
        // 执行数据库备份
        if Path::new(db_path).exists() {
            // 为防止备份过程中数据库变化，先创建临时连接并在事务中执行备份
            let conn = self.conn.lock().await;
            conn.execute("BEGIN IMMEDIATE", [])?;
            
            // 执行备份
            let result = fs::copy(db_path, &file_path);
            
            // 结束事务
            conn.execute("END", [])?;
            
            match result {
                Ok(size) => {
                    file_size = size;
                },
                Err(e) => {
                    status = "Failed";
                    eprintln!("Backup failed: {}", e);
                }
            }
        } else {
            status = "Failed";
            eprintln!("Database file not found");
        }
        
        // 记录备份信息到数据库
        let backup_info = self.record_backup_info(
            &backup_id, 
            &filename, 
            &file_path, 
            file_size, 
            backup_type, 
            status, 
            description, 
            &timestamp.to_rfc3339(), 
            user_id
        ).await?;
        
        Ok(backup_info)
    }

    // 记录备份信息到数据库
    async fn record_backup_info(
        &self,
        id: &str,
        filename: &str,
        file_path: &str,
        file_size: u64,
        backup_type: &str,
        status: &str,
        description: Option<&str>,
        backup_time: &str,
        created_by: Option<i32>
    ) -> Result<crate::models::system::BackupInfo> {
        let conn = self.conn.lock().await;
        
        // 确保备份表存在
        conn.execute(
            "CREATE TABLE IF NOT EXISTS backups (
                id TEXT PRIMARY KEY,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                backup_type TEXT NOT NULL,
                status TEXT NOT NULL,
                description TEXT,
                backup_time TEXT NOT NULL,
                created_by INTEGER,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            []
        )?;
        
        // 插入备份记录
        conn.execute(
            "INSERT INTO backups (id, filename, file_path, file_size, backup_type, status, description, backup_time, created_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                id,
                filename,
                file_path,
                file_size as i64,
                backup_type,
                status,
                description,
                backup_time,
                created_by
            ]
        )?;
        
        // 获取用户名（如果有用户ID）
        let created_by_name = if let Some(user_id) = created_by {
            conn.query_row(
                "SELECT username FROM users WHERE id = ?",
                [user_id],
                |row| row.get(0)
            ).ok()
        } else {
            None
        };
        
        // 构建并返回备份信息
        Ok(crate::models::system::BackupInfo {
            id: id.to_string(),
            filename: filename.to_string(),
            file_path: file_path.to_string(),
            file_size,
            backup_type: backup_type.to_string(),
            status: status.to_string(),
            description: description.map(|s| s.to_string()),
            backup_time: backup_time.to_string(),
            created_by,
            created_by_name,
        })
    }

    // 完善获取备份列表方法
    pub async fn get_backups(&self, limit: Option<u32>, offset: Option<u32>) -> Result<(Vec<crate::models::system::BackupInfo>, i64)> {
        let conn = self.conn.lock().await;
        
        // 确保备份表存在
        conn.execute(
            "CREATE TABLE IF NOT EXISTS backups (
                id TEXT PRIMARY KEY,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                backup_type TEXT NOT NULL,
                status TEXT NOT NULL,
                description TEXT,
                backup_time TEXT NOT NULL,
                created_by INTEGER,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            []
        )?;
        
        // 获取总记录数
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM backups",
            [],
            |row| row.get(0)
        )?;
        
        // 构建查询语句
        let mut sql = "SELECT b.id, b.filename, b.file_path, b.file_size, b.backup_type, 
                     b.status, b.description, b.backup_time, b.created_by, u.username
                  FROM backups b
                  LEFT JOIN users u ON b.created_by = u.id
                  ORDER BY b.backup_time DESC".to_string();
        
        // 添加分页
        if let (Some(limit), Some(offset)) = (limit, offset) {
            sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
        } else if let Some(limit) = limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        
        // 执行查询
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(crate::models::system::BackupInfo {
                id: row.get(0)?,
                filename: row.get(1)?,
                file_path: row.get(2)?,
                file_size: row.get::<_, i64>(3)? as u64,
                backup_type: row.get(4)?,
                status: row.get(5)?,
                description: row.get(6)?,
                backup_time: row.get(7)?,
                created_by: row.get(8)?,
                created_by_name: row.get(9)?,
            })
        })?;
        
        // 收集结果
        let mut backups = Vec::new();
        for row in rows {
            backups.push(row?);
        }
        
        Ok((backups, total))
    }

    // 获取备份详情
    pub async fn get_backup_details(&self, backup_id: &str) -> Result<crate::models::system::BackupInfo> {
        let conn = self.conn.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT b.id, b.filename, b.file_path, b.file_size, b.backup_type, 
                    b.status, b.description, b.backup_time, b.created_by, u.username
             FROM backups b
             LEFT JOIN users u ON b.created_by = u.id
             WHERE b.id = ?"
        )?;
        
        let backup = stmt.query_row([backup_id], |row| {
            Ok(crate::models::system::BackupInfo {
                id: row.get(0)?,
                filename: row.get(1)?,
                file_path: row.get(2)?,
                file_size: row.get::<_, i64>(3)? as u64,
                backup_type: row.get(4)?,
                status: row.get(5)?,
                description: row.get(6)?,
                backup_time: row.get(7)?,
                created_by: row.get(8)?,
                created_by_name: row.get(9)?,
            })
        })?;
        
        Ok(backup)
    }

    // 完善删除备份方法
    pub async fn delete_backup(&self, backup_id: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 先获取备份文件路径
        let file_path: String = conn.query_row(
            "SELECT file_path FROM backups WHERE id = ?",
            [backup_id],
            |row| row.get(0)
        )?;
        
        // 删除文件
        if Path::new(&file_path).exists() {
            fs::remove_file(&file_path)?;
        }
        
        // 删除数据库记录
        conn.execute("DELETE FROM backups WHERE id = ?", [backup_id])?;
        
        Ok(())
    }

    // 批量删除备份
    pub async fn batch_delete_backups(&self, backup_ids: &[String]) -> Result<usize> {
        if backup_ids.is_empty() {
            return Ok(0);
        }
        
        let conn = self.conn.lock().await;
        
        // 获取所有要删除的文件路径
        let placeholder = backup_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!("SELECT file_path FROM backups WHERE id IN ({})", placeholder);
        
        let mut stmt = conn.prepare(&query)?;
        let file_paths = stmt.query_map(
            rusqlite::params_from_iter(backup_ids.iter()),
            |row| row.get::<_, String>(0)
        )?;
        
        // 删除文件
        for file_path_result in file_paths {
            let file_path = file_path_result?;
            if Path::new(&file_path).exists() {
                fs::remove_file(&file_path).ok(); // 忽略错误，继续删除其他文件
            }
        }
        
        // 从数据库中删除记录
        let delete_query = format!("DELETE FROM backups WHERE id IN ({})", placeholder);
        let deleted = conn.execute(
            &delete_query,
            rusqlite::params_from_iter(backup_ids.iter())
        )?;
        
        Ok(deleted as usize)
    }

    // 添加恢复备份方法
    pub async fn restore_backup(&self, backup_id: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 获取备份文件路径和状态
        let (file_path, status): (String, String) = conn.query_row(
            "SELECT file_path, status FROM backups WHERE id = ?",
            [backup_id],
            |row| Ok((row.get(0)?, row.get(1)?))
        )?;
        
        // 检查备份状态
        if status != "Success" {
            return Err(anyhow::anyhow!("Cannot restore from a failed backup"));
        }
        
        // 检查备份文件是否存在
        if !Path::new(&file_path).exists() {
            return Err(anyhow::anyhow!("Backup file not found"));
        }
        
        // 备份当前数据库（恢复前备份）
        let pre_restore_backup_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let pre_restore_filename = format!("pre_restore_backup_{}.db", timestamp);
        let pre_restore_path = format!("backups/{}", pre_restore_filename);
        
        // 确保备份目录存在
        if let Some(parent) = Path::new(&pre_restore_path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 备份当前数据库
        let db_path = "data.db";
        if Path::new(db_path).exists() {
            fs::copy(db_path, &pre_restore_path)?;
            
            // 记录这次恢复前的备份
            self.record_backup_info(
                &pre_restore_backup_id,
                &pre_restore_filename,
                &pre_restore_path,
                fs::metadata(&pre_restore_path)?.len(),
                "PreRestore",
                "Success",
                Some("Automatic backup before restore operation"),
                &Utc::now().to_rfc3339(),
                None
            ).await?;
        }
        
        // 执行恢复操作（关闭当前连接并复制备份文件）
        drop(conn); // 释放锁，关闭连接
        
        // 复制备份文件到数据库位置
        fs::copy(&file_path, db_path)?;
        
        // 重新建立数据库连接并记录恢复操作
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO system_logs (level, message, details, timestamp) 
             VALUES (?, ?, ?, datetime('now'))",
            params![
                "INFO",
                format!("数据库已从备份 {} 恢复", backup_id),
                format!("Restored from backup file: {}", file_path),
            ]
        )?;
        
        Ok(())
    }

    // 获取备份统计信息
    pub async fn get_backup_stats(&self) -> Result<crate::models::system::BackupStats> {
        let conn = self.conn.lock().await;
        
        // 确保备份表存在
        conn.execute(
            "CREATE TABLE IF NOT EXISTS backups (
                id TEXT PRIMARY KEY,
                filename TEXT NOT NULL,
                file_path TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                backup_type TEXT NOT NULL,
                status TEXT NOT NULL,
                description TEXT,
                backup_time TEXT NOT NULL,
                created_by INTEGER,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            []
        )?;
        
        // 总备份数
        let total_backups: i32 = conn.query_row(
            "SELECT COUNT(*) FROM backups",
            [],
            |row| row.get(0)
        )?;
        
        // 成功备份数
        let success_backups: i32 = conn.query_row(
            "SELECT COUNT(*) FROM backups WHERE status = 'Success'",
            [],
            |row| row.get(0)
        )?;
        
        // 失败备份数
        let failed_backups: i32 = conn.query_row(
            "SELECT COUNT(*) FROM backups WHERE status != 'Success'",
            [],
            |row| row.get(0)
        )?;
        
        // 总大小
        let total_size: i64 = conn.query_row(
            "SELECT SUM(file_size) FROM backups",
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        // 最后一次备份时间
        let last_backup_time: Option<String> = conn.query_row(
            "SELECT backup_time FROM backups WHERE status = 'Success' ORDER BY backup_time DESC LIMIT 1",
            [],
            |row| row.get(0)
        ).ok();
        
        // 下一次计划备份时间（如果有备份调度配置的话）
        let next_scheduled_backup: Option<String> = None; // 这需要备份调度功能，可以在后续实现
        
        Ok(crate::models::system::BackupStats {
            total_backups,
            success_backups,
            failed_backups,
            total_size: total_size as u64,
            last_backup_time,
            next_scheduled_backup,
        })
    }

    // 添加获取备份文件下载路径方法
    pub async fn get_backup_path(&self, backup_id: &str) -> Result<String> {
        let conn = self.conn.lock().await;
        
        let (file_path, status): (String, String) = conn.query_row(
            "SELECT file_path, status FROM backups WHERE id = ?",
            [backup_id],
            |row| Ok((row.get(0)?, row.get(1)?))
        )?;
        
        // 检查备份状态和文件存在性
        if status != "Success" {
            return Err(anyhow::anyhow!("Cannot download a failed backup"));
        }
        
        if !Path::new(&file_path).exists() {
            return Err(anyhow::anyhow!("Backup file not found"));
        }
        
        Ok(file_path)
    }

    // 新增方法：获取公告
    pub async fn get_announcements(&self) -> Result<Vec<crate::services::admin_service::Announcement>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, title, content, priority, start_time, end_time FROM announcements ORDER BY priority DESC";
        let mut stmt = conn.prepare(sql)?;
        let announcements = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let title: String = row.get(1)?;
            let content: String = row.get(2)?;
            let priority: i32 = row.get(3)?;
            let start_time: String = row.get(4)?;
            let end_time: Option<String> = row.get(5).ok();
            Ok(crate::services::admin_service::Announcement {
                id,
                title,
                content,
                type_: String::new(),
                priority,
                enabled: true,
                start_time,
                end_time,
                created_at: String::new(),
                updated_at: String::new(),
            })
        })?
        .filter_map(|result| result.ok())
        .collect::<Vec<_>>();
        Ok(announcements)
    }

    // 新增方法：创建公告
    pub async fn create_announcement(&self, title: &str, content: &str, priority: i32, start_time: &str, end_time: Option<&str>) -> Result<crate::services::admin_service::Announcement> {
        let conn = self.conn.lock().await;
        
        // 首先检查表结构
        let mut _has_type = false;
        let mut _has_enabled = false;
        let mut _has_start_time = false;
        let mut _has_end_time = false;
        
        // 获取表的列信息
        let mut stmt = conn.prepare("PRAGMA table_info(announcements)")?;
        let cols = stmt.query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?;
        
        // 检查列是否存在
        for col_result in cols {
            if let Ok(col_name) = col_result {
                if col_name == "type" {
                    _has_type = true;
                } else if col_name == "enabled" {
                    _has_enabled = true;
                } else if col_name == "start_time" {
                    _has_start_time = true;
                } else if col_name == "end_time" {
                    _has_end_time = true;
                }
            }
        }
        
        // 默认值设置
        let type_ = "Info";
        let enabled = true;
        let now = chrono::Utc::now().to_rfc3339();
        let start_time_str = start_time.to_string();
        let end_time_opt = end_time.map(|s| s.to_string());
        
        // 构建适合当前表结构的INSERT语句
        let sql = String::from("INSERT INTO announcements (title, content, priority, start_time, end_time) VALUES (?, ?, ?, ?, ?)");
        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&title, &content, &priority, &start_time_str, &end_time_opt];
        
        conn.execute(&sql, params.as_slice())?;

        let id = conn.last_insert_rowid() as i32;
        
        Ok(crate::services::admin_service::Announcement {
            id,
            title: title.to_string(),
            content: content.to_string(),
            type_: type_.to_string(),
            priority,
            enabled,
            start_time: start_time_str,
            end_time: end_time_opt,
            created_at: String::new(),
            updated_at: String::new(),
        })
    }

    // 新增方法：更新公告
    pub async fn update_announcement(&self, id: i32, title: &str, content: &str, priority: i32, start_time: &str, end_time: Option<&str>) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 首先检查表结构
        let mut _has_type = false;
        let mut _has_enabled = false;
        let mut _has_start_time = false;
        let mut _has_end_time = false;
        
        // 获取表的列信息
        let mut stmt = conn.prepare("PRAGMA table_info(announcements)")?;
        let cols = stmt.query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?;
        
        // 检查列是否存在
        for col_result in cols {
            if let Ok(col_name) = col_result {
                if col_name == "type" {
                    _has_type = true;
                } else if col_name == "enabled" {
                    _has_enabled = true;
                } else if col_name == "start_time" {
                    _has_start_time = true;
                } else if col_name == "end_time" {
                    _has_end_time = true;
                }
            }
        }
        
        // 构建UPDATE语句
        let sql = String::from("UPDATE announcements SET title = ?, content = ?, priority = ?, start_time = ?, end_time = ? WHERE id = ?");
        let end_time_str = end_time.unwrap_or("");
        let params: Vec<&dyn rusqlite::ToSql> = vec![&title, &content, &priority, &start_time, &end_time_str, &id];
        
        conn.execute(&sql, params.as_slice())?;

        Ok(())
    }

    // 新增方法：删除公告
    pub async fn delete_announcement(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        
        let sql = "DELETE FROM announcements WHERE id = ?";
        
        conn.execute(sql, params![id])?;
        
        Ok(())
    }

    // 获取单个公告
    pub async fn get_announcement_by_id(&self, id: i32) -> Result<Option<crate::services::admin_service::Announcement>> {
        let conn = self.conn.lock().await;
        
        // 首先检查表结构
        let mut _has_type = false;
        let mut _has_enabled = false;
        let mut _has_start_time = false;
        let mut _has_end_time = false;
        
        // 获取表的列信息
        let mut stmt = conn.prepare("PRAGMA table_info(announcements)")?;
        let cols = stmt.query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?;
        
        // 检查列是否存在
        for col_result in cols {
            if let Ok(col_name) = col_result {
                if col_name == "type" {
                    _has_type = true;
                } else if col_name == "enabled" {
                    _has_enabled = true;
                } else if col_name == "start_time" {
                    _has_start_time = true;
                } else if col_name == "end_time" {
                    _has_end_time = true;
                }
            }
        }
        
        // 使用动态SQL，根据实际存在的列构建查询
        let sql = format!(
            "SELECT id, title, content, {} priority, {} {} {} created_at, updated_at 
             FROM announcements 
             WHERE id = ?",
            if _has_type { "type," } else { "'' as type," },
            if _has_enabled { "enabled," } else { "1 as enabled," },
            if _has_start_time { "start_time," } else { "created_at as start_time," },
            if _has_end_time { "end_time," } else { "NULL as end_time," }
        );
        
        let announcement = match conn.query_row(&sql, [id], |row| {
            let id: i32 = row.get(0)?;
            let title: String = row.get(1)?;
            let content: String = row.get(2)?;
            let type_: String = row.get(3).unwrap_or_else(|_| "Info".to_string());
            let priority: i32 = row.get(4)?;
            let enabled: bool = row.get(5).unwrap_or(true);
            let start_time: String = row.get(6).unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());
            let end_time: Option<String> = row.get(7).ok();
            let created_at: String = row.get(8)?;
            let updated_at: String = row.get(9)?;
            
            Ok(crate::services::admin_service::Announcement {
                id,
                title,
                content,
                type_,
                priority,
                enabled,
                start_time,
                end_time,
                created_at,
                updated_at
            })
        }) {
            Ok(a) => Some(a),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => return Err(e.into()),
        };
        
        Ok(announcement)
    }

    // 批量更新公告状态
    pub async fn batch_update_announcement_status(&self, ids: &[i32], enabled: bool) -> Result<usize> {
        if ids.is_empty() {
            return Ok(0);
        }
        
        let conn = self.conn.lock().await;
        
        // 创建占位符
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!("UPDATE announcements SET enabled = ?, updated_at = datetime('now') WHERE id IN ({})", placeholders);
        
        // 准备参数
        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&enabled];
        for id in ids {
            params.push(id);
        }
        
        let updated = conn.execute(&sql, params.as_slice())?;
        
        Ok(updated as usize)
    }

    // 批量删除公告
    pub async fn batch_delete_announcements(&self, ids: &[i32]) -> Result<usize> {
        if ids.is_empty() {
            return Ok(0);
        }
        
        let conn = self.conn.lock().await;
        
        // 创建占位符
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!("DELETE FROM announcements WHERE id IN ({})", placeholders);
        
        // 准备参数
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        for id in ids {
            params.push(id);
        }
        
        let deleted = conn.execute(&sql, params.as_slice())?;
        
        Ok(deleted as usize)
    }

    // 查询有效公告
    pub async fn get_active_announcements(&self) -> Result<Vec<crate::services::admin_service::Announcement>> {
        let conn = self.conn.lock().await;
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let sql = "SELECT id, title, content, priority, start_time, end_time FROM announcements WHERE start_time <= ? AND (end_time IS NULL OR end_time > ?) ORDER BY priority DESC";
        let mut stmt = conn.prepare(sql)?;
        let announcements = stmt.query_map([&now, &now], |row| {
            let id: i32 = row.get(0)?;
            let title: String = row.get(1)?;
            let content: String = row.get(2)?;
            let priority: i32 = row.get(3)?;
            let start_time: String = row.get(4)?;
            let end_time: Option<String> = row.get(5).ok();
            Ok(crate::services::admin_service::Announcement {
                id,
                title,
                content,
                type_: String::new(),
                priority,
                enabled: true,
                start_time,
                end_time,
                created_at: String::new(),
                updated_at: String::new(),
            })
        })?
        .filter_map(|result| result.ok())
        .collect::<Vec<_>>();
        Ok(announcements)
    }

    // 获取所有系统设置
    pub async fn get_all_settings(&self) -> Result<HashMap<String, String>> {
        let conn = self.conn.lock().await;
        
        // 确保设置表存在
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;
        
        // 查询所有设置
        let mut stmt = conn.prepare("SELECT key, value FROM system_settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        
        let mut settings = HashMap::new();
        for row_result in rows {
            if let Ok((key, value)) = row_result {
                settings.insert(key, value);
            }
        }
        
        // 设置默认值
        if !settings.contains_key("primary_color") {
            settings.insert("primary_color".to_string(), "#409EFF".to_string());
            // 保存默认值到数据库
            conn.execute(
                "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
                params!["primary_color", "#409EFF"],
            )?;
        }
        
        if !settings.contains_key("secondary_color") {
            settings.insert("secondary_color".to_string(), "#67C23A".to_string());
            conn.execute(
                "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
                params!["secondary_color", "#67C23A"],
            )?;
        }
        
        if !settings.contains_key("dark_mode") {
            settings.insert("dark_mode".to_string(), "false".to_string());
            conn.execute(
                "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
                params!["dark_mode", "false"],
            )?;
        }
        
        if !settings.contains_key("font_size") {
            settings.insert("font_size".to_string(), "14px".to_string());
            conn.execute(
                "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
                params!["font_size", "14px"],
            )?;
        }
        
        if !settings.contains_key("language") {
            settings.insert("language".to_string(), "zh-CN".to_string());
            conn.execute(
                "INSERT OR REPLACE INTO system_settings (key, value) VALUES (?, ?)",
                params!["language", "zh-CN"],
            )?;
        }
        
        Ok(settings)
    }

    // 新增方法：获取主题设置
    pub async fn get_theme_settings(&self) -> Result<crate::services::admin_service::ThemeSettings> {
        let settings = self.get_all_settings().await?;
        
        let primary_color = settings.get("primary_color").cloned().unwrap_or_else(|| "#409EFF".to_string());
        let secondary_color = settings.get("secondary_color").cloned().unwrap_or_else(|| "#67C23A".to_string());
        let dark_mode = settings.get("dark_mode")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false);
        let font_size = settings.get("font_size").cloned().unwrap_or_else(|| "14px".to_string());
        let language = settings.get("language").cloned().unwrap_or_else(|| "zh-CN".to_string());

        Ok(crate::services::admin_service::ThemeSettings {
            primary_color,
            secondary_color,
            dark_mode,
            font_size,
            language,
        })
    }

    // 新增方法：更新主题设置
    pub async fn update_theme_settings(&self, primary_color: &str, secondary_color: &str, dark_mode: bool, font_size: &str, language: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 确保设置表存在
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;
        
        // 更新或插入主题设置
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params!["primary_color", primary_color],
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params!["secondary_color", secondary_color],
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params!["dark_mode", &dark_mode.to_string()],
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params!["font_size", font_size],
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params!["language", language],
        )?;

        Ok(())
    }

    // 获取设置值
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().await;
        
        let value = conn.query_row(
            "SELECT value FROM system_settings WHERE key = ?",
            params![key],
            |row| row.get::<_, String>(0)
        ).optional()?;
        
        Ok(value)
    }

    // 更新设置值
    pub async fn update_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        
        conn.execute(
            "INSERT OR REPLACE INTO system_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params![key, value],
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

    // 修复get_category_by_id方法
    pub async fn get_category_by_id(&self, id: i32) -> Result<Option<Category>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, description, enabled, subscription_locked, created_at, updated_at FROM categories WHERE id = ?";
        println!("[SQL] get_category_by_id: {}", sql);
        
        let mut stmt = conn.prepare(sql)?;
        
        let category = match stmt.query_row(rusqlite::params![id], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3)?,
                subscription_locked: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
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

    // 修复create_category方法
    pub async fn create_category(&self, req: &CreateCategoryRequest) -> Result<Category> {
        let conn = self.conn.lock().await;
        let sql = "INSERT INTO categories (name, description, enabled, subscription_locked, created_at) VALUES (?, ?, ?, ?, datetime('now'))";
        println!("[SQL] create_category: {}", sql);
        
        let enabled = req.enabled.unwrap_or(true);
        let subscription_locked = req.subscription_locked.unwrap_or(false);
        
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(rusqlite::params![
            req.name,
            req.description,
            enabled,
            subscription_locked
        ])?;
        
        let id = conn.last_insert_rowid() as i32;
        let now = chrono::Utc::now().to_rfc3339();
        
        let category = Category {
            id,
            name: req.name.clone(),
            description: req.description.clone(),
            enabled,
            subscription_locked,
            created_at: now,
            updated_at: None,
        };
        
        Ok(category)
    }

    // 修复update_category方法
    pub async fn update_category(&self, id: i32, req: &UpdateCategoryRequest) -> Result<Category> {
        // 先检查分类是否存在
        let category = self.get_category_by_id(id).await?
            .ok_or_else(|| anyhow::anyhow!("分类不存在"))?;
        
        let conn = self.conn.lock().await;
        let sql = "UPDATE categories SET name = ?, description = ?, enabled = ?, subscription_locked = ?, updated_at = datetime('now') WHERE id = ?";
        println!("[SQL] update_category: {}", sql);
        
        let mut stmt = conn.prepare(sql)?;
        stmt.execute(rusqlite::params![
            req.name.as_ref().unwrap_or(&category.name),
            req.description.as_ref().or(category.description.as_ref()),
            req.enabled.unwrap_or(category.enabled),
            req.subscription_locked.unwrap_or(category.subscription_locked),
            id
        ])?;
        
        // 获取更新后的分类
        let now = chrono::Utc::now().to_rfc3339();
        let updated_category = Category {
            id,
            name: req.name.clone().unwrap_or(category.name),
            description: req.description.clone().or(category.description),
            enabled: req.enabled.unwrap_or(category.enabled),
            subscription_locked: req.subscription_locked.unwrap_or(category.subscription_locked),
            created_at: category.created_at,
            updated_at: Some(now),
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

    // 获取社区设置
    pub async fn get_community_settings(&self) -> Result<crate::models::system::CommunitySettings> {
        let mut settings = crate::models::system::CommunitySettings::default();
        
        // 从数据库读取配置，如果不存在则使用默认值
        if let Ok(Some(title)) = self.get_setting("site_title").await {
            settings.site_title = title;
        }
        if let Ok(Some(subtitle)) = self.get_setting("site_subtitle").await {
            settings.site_subtitle = subtitle;
        }
        if let Ok(Some(description)) = self.get_setting("site_description").await {
            settings.site_description = description;
        }
        if let Ok(Some(welcome)) = self.get_setting("welcome_message").await {
            settings.welcome_message = welcome;
        }
        if let Ok(Some(announcement)) = self.get_setting("announcement").await {
            settings.announcement = Some(announcement);
        }
        if let Ok(Some(footer)) = self.get_setting("footer_text").await {
            settings.footer_text = footer;
        }
        if let Ok(Some(email)) = self.get_setting("contact_email").await {
            settings.contact_email = email;
        }
        if let Ok(Some(github)) = self.get_setting("github_link").await {
            settings.github_link = Some(github);
        }
        if let Ok(Some(qq)) = self.get_setting("qq_group").await {
            settings.qq_group = Some(qq);
        }
        if let Ok(Some(wechat)) = self.get_setting("wechat_group").await {
            settings.wechat_group = Some(wechat);
        }

        Ok(settings)
    }

    // 更新社区设置
    pub async fn update_community_settings(&self, request: &crate::models::system::UpdateCommunitySettingsRequest) -> Result<()> {
        if let Some(ref title) = request.site_title {
            self.update_setting("site_title", title).await?;
        }
        if let Some(ref subtitle) = request.site_subtitle {
            self.update_setting("site_subtitle", subtitle).await?;
        }
        if let Some(ref description) = request.site_description {
            self.update_setting("site_description", description).await?;
        }
        if let Some(ref welcome) = request.welcome_message {
            self.update_setting("welcome_message", welcome).await?;
        }
        if let Some(ref announcement) = request.announcement {
            if announcement.is_empty() {
                // 如果公告为空，则删除该设置
                let conn = self.conn.lock().await;
                conn.execute("DELETE FROM system_settings WHERE key = ?", params!["announcement"])?;
            } else {
                self.update_setting("announcement", announcement).await?;
            }
        }
        if let Some(ref footer) = request.footer_text {
            self.update_setting("footer_text", footer).await?;
        }
        if let Some(ref email) = request.contact_email {
            self.update_setting("contact_email", email).await?;
        }
        if let Some(ref github) = request.github_link {
            if github.is_empty() {
                let conn = self.conn.lock().await;
                conn.execute("DELETE FROM system_settings WHERE key = ?", params!["github_link"])?;
            } else {
                self.update_setting("github_link", github).await?;
            }
        }
        if let Some(ref qq) = request.qq_group {
            if qq.is_empty() {
                let conn = self.conn.lock().await;
                conn.execute("DELETE FROM system_settings WHERE key = ?", params!["qq_group"])?;
            } else {
                self.update_setting("qq_group", qq).await?;
            }
        }
        if let Some(ref wechat) = request.wechat_group {
            if wechat.is_empty() {
                let conn = self.conn.lock().await;
                conn.execute("DELETE FROM system_settings WHERE key = ?", params!["wechat_group"])?;
            } else {
                self.update_setting("wechat_group", wechat).await?;
            }
        }

        Ok(())
    }
} 