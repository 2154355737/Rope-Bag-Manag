use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::models::notification::Notification;

#[derive(Clone)]
pub struct NotificationRepository {
    pub conn: Arc<Mutex<Connection>>,
}

impl NotificationRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let repo = Self { conn: Arc::new(Mutex::new(conn)) };
        futures::executor::block_on(repo.init())?;
        Ok(repo)
    }

    pub async fn init(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notifications (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                link TEXT,
                notif_type TEXT,
                related_type TEXT,
                related_id INTEGER,
                is_read INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub async fn create(&self, n: &Notification) -> Result<i32> {
        let conn = self.conn.lock().await;
        let id: i32 = conn.query_row(
            "INSERT INTO notifications (user_id,title,content,link,notif_type,related_type,related_id,is_read,created_at)
             VALUES (?,?,?,?,?,?,?,?,?) RETURNING id",
            params![
                n.user_id,
                n.title,
                n.content,
                n.link,
                n.notif_type,
                n.related_type,
                n.related_id,
                if n.is_read {1} else {0},
                n.created_at.to_rfc3339(),
            ],
            |row| row.get(0),
        )?;
        Ok(id)
    }

    pub async fn list(&self, user_id: i32, page: i32, size: i32) -> Result<Vec<Notification>> {
        let conn = self.conn.lock().await;
        let offset = (page - 1) * size;
        let mut stmt = conn.prepare(
            "SELECT id,user_id,title,content,link,notif_type,related_type,related_id,is_read,created_at 
             FROM notifications WHERE user_id=? ORDER BY id DESC LIMIT ? OFFSET ?",
        )?;
        let rows = stmt.query_map(params![user_id, size, offset], |row| {
            Ok(Notification {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                link: row.get(4).ok(),
                notif_type: row.get(5).ok(),
                related_type: row.get(6).ok(),
                related_id: row.get(7).ok(),
                is_read: row.get::<_, i32>(8)? != 0,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            })
        })?;
        let mut list = Vec::new();
        for r in rows { list.push(r?); }
        Ok(list)
    }

    pub async fn mark_read(&self, user_id: i32, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("UPDATE notifications SET is_read=1 WHERE id=? AND user_id=?", params![id, user_id])?;
        Ok(())
    }

    pub async fn unread_count(&self, user_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let count: i32 = conn.query_row("SELECT COUNT(*) FROM notifications WHERE user_id=? AND is_read=0", params![user_id], |row| row.get(0))?;
        Ok(count)
    }

    /// 标记该用户所有通知为已读
    pub async fn mark_all_read(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("UPDATE notifications SET is_read=1 WHERE user_id=? AND is_read=0", params![user_id])?;
        Ok(())
    }

    // 新增：查询全站通知（按时间倒序，分页）
    pub async fn list_all(&self, page: i32, size: i32) -> Result<Vec<Notification>> {
        let conn = self.conn.lock().await;
        let offset = (page - 1).max(0) * size.max(1);
        let mut stmt = conn.prepare(
            "SELECT id,user_id,title,content,link,notif_type,related_type,related_id,is_read,created_at \
             FROM notifications ORDER BY id DESC LIMIT ? OFFSET ?",
        )?;
        let rows = stmt.query_map(params![size, offset], |row| {
            Ok(Notification {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                link: row.get(4).ok(),
                notif_type: row.get(5).ok(),
                related_type: row.get(6).ok(),
                related_id: row.get(7).ok(),
                is_read: row.get::<_, i32>(8)? != 0,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            })
        })?;
        let mut list = Vec::new();
        for r in rows { list.push(r?); }
        Ok(list)
    }

    // 新增：全站通知总数
    pub async fn count_all(&self) -> Result<i32> {
        let conn = self.conn.lock().await;
        let count: i32 = conn.query_row("SELECT COUNT(*) FROM notifications", [], |row| row.get(0))?;
        Ok(count)
    }

    /// 删除用户的已读通知
    pub async fn delete_read(&self, user_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let affected = conn.execute("DELETE FROM notifications WHERE user_id=? AND is_read=1", params![user_id])?;
        Ok(affected as i32)
    }

    /// 删除单个通知
    pub async fn delete_by_id(&self, user_id: i32, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM notifications WHERE id=? AND user_id=?", params![id, user_id])?;
        Ok(())
    }
} 