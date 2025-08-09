use anyhow::Result;
use rusqlite::{Connection, params};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use serde_json::json;

#[derive(Clone)]
pub struct SubscriptionRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SubscriptionRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    pub async fn set_subscription(&self, user_id: i32, category_id: i32, enabled: bool) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 检查分类是否被锁定
        let sql = "SELECT subscription_locked FROM categories WHERE id = ?";
        let locked: i32 = conn.query_row(sql, params![category_id], |row| row.get(0))?;
        
        if locked == 1 {
            return Err(anyhow::anyhow!("该分类已被锁定，无法修改订阅状态"));
        }
        
        conn.execute("INSERT OR REPLACE INTO subscriptions (user_id,category_id,enabled) VALUES (?,?,?)",
            params![user_id, category_id, if enabled {1} else {0}])?;
        Ok(())
    }

    pub async fn get_enabled_categories(&self, user_id: i32) -> Result<Vec<i32>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT category_id FROM subscriptions WHERE user_id=? AND enabled=1")?;
        let rows = stmt.query_map(params![user_id], |r| r.get(0))?;
        Ok(rows.collect::<Result<Vec<i32>, _>>()?)
    }

    pub async fn get_user_subscriptions(&self, user_id: i32) -> Result<HashMap<i32, bool>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT category_id, enabled FROM subscriptions WHERE user_id=?")?;
        let rows = stmt.query_map(params![user_id], |r| {
            Ok((r.get::<_, i32>(0)?, r.get::<_, i32>(1)? == 1))
        })?;
        let mut subscriptions = HashMap::new();
        for row in rows {
            let (category_id, enabled) = row?;
            subscriptions.insert(category_id, enabled);
        }
        Ok(subscriptions)
    }

    pub async fn get_subscribed_emails(&self, category_id: i32) -> Result<Vec<String>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT u.email FROM subscriptions s JOIN users u ON s.user_id = u.id WHERE s.category_id = ? AND s.enabled = 1";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![category_id], |r| r.get(0))?;
        Ok(rows.collect::<Result<Vec<String>, _>>()?)
    }

    // 新增：获取订阅某分类的用户ID列表（用于站内通知）
    pub async fn get_subscribed_user_ids(&self, category_id: i32) -> Result<Vec<i32>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT user_id FROM subscriptions WHERE category_id = ? AND enabled = 1";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![category_id], |r| r.get(0))?;
        Ok(rows.collect::<Result<Vec<i32>, _>>()?)
    }

    // 管理员功能 - 统计分类订阅数量
    pub async fn count_category_subscriptions(&self, category_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let sql = "SELECT COUNT(*) FROM subscriptions WHERE category_id = ? AND enabled = 1";
        let count: i32 = conn.query_row(sql, params![category_id], |r| r.get(0))?;
        Ok(count)
    }

    // 管理员功能 - 获取分类订阅者详细信息
    pub async fn get_category_subscribers(&self, category_id: i32) -> Result<Vec<serde_json::Value>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT u.id as user_id, u.username, u.nickname, u.email, s.created_at as subscribed_at 
                   FROM subscriptions s 
                   JOIN users u ON s.user_id = u.id 
                   WHERE s.category_id = ? AND s.enabled = 1 
                   ORDER BY s.created_at DESC";
        
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![category_id], |row| {
            Ok(json!({
                "user_id": row.get::<_, i32>(0)?,
                "username": row.get::<_, String>(1)?,
                "nickname": row.get::<_, Option<String>>(2)?.unwrap_or_else(|| "未设置".to_string()),
                "email": row.get::<_, String>(3)?,
                "subscribed_at": row.get::<_, String>(4)?
            }))
        })?;
        
        let mut subscribers = Vec::new();
        for row in rows {
            subscribers.push(row?);
        }
        Ok(subscribers)
    }

    // 管理员功能 - 获取全部订阅统计
    pub async fn get_all_subscription_stats(&self) -> Result<Vec<serde_json::Value>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT c.id, c.name, c.description, COUNT(s.user_id) as subscription_count
                   FROM categories c 
                   LEFT JOIN subscriptions s ON c.id = s.category_id AND s.enabled = 1
                   GROUP BY c.id, c.name, c.description
                   ORDER BY subscription_count DESC";
        
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(json!({
                "category_id": row.get::<_, i32>(0)?,
                "category_name": row.get::<_, String>(1)?,
                "category_description": row.get::<_, String>(2)?,
                "subscription_count": row.get::<_, i32>(3)?
            }))
        })?;
        
        let mut stats = Vec::new();
        for row in rows {
            stats.push(row?);
        }
        Ok(stats)
    }
} 