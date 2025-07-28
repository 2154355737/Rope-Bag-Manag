use anyhow::Result;
use rusqlite::{Connection, params};
use std::sync::Arc;
use tokio::sync::Mutex;

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

    pub async fn get_subscribed_emails(&self, category_id: i32) -> Result<Vec<String>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT u.email FROM subscriptions s JOIN users u ON s.user_id = u.id WHERE s.category_id = ? AND s.enabled = 1";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![category_id], |r| r.get(0))?;
        Ok(rows.collect::<Result<Vec<String>, _>>()?)
    }
} 