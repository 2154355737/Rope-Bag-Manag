use anyhow::Result;
use rusqlite::{params, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ForbiddenWordRepository {
    conn: Arc<Mutex<Connection>>,
}

impl ForbiddenWordRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        // 初始化表
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS forbidden_words (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT UNIQUE NOT NULL,
                created_at TEXT NOT NULL
            );",
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn add_word(&self, word: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR IGNORE INTO forbidden_words (word, created_at) VALUES (?, datetime('now'))",
            params![word],
        )?;
        Ok(())
    }

    pub async fn delete_word(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM forbidden_words WHERE id = ?", params![id])?;
        Ok(())
    }

    pub async fn list_words(&self) -> Result<Vec<(i32, String)>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, word FROM forbidden_words ORDER BY id DESC")?;
        let rows = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub async fn contains_forbidden_word(&self, text: &str) -> Result<bool> {
        let words = self.list_words().await?;
        for (_, w) in words {
            if text.contains(&w) {
                return Ok(true);
            }
        }
        Ok(false)
    }
} 