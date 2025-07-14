use rusqlite::{Connection, Result};
use crate::models::{Stats, Category, UserAction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SystemRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SystemRepository {
    pub fn new(db_path: &str) -> Result<Self> {
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
} 