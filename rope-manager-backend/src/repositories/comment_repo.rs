use anyhow::Result;
use rusqlite::{Connection, params};
use crate::models::Comment;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct CommentRepository {
    conn: Arc<Mutex<Connection>>,
}

impl CommentRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn get_comments_by_package(&self, package_id: i32) -> Result<Vec<Comment>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, package_id, content, created_at 
             FROM comments WHERE package_id = ? ORDER BY created_at DESC"
        )?;

        let comments = stmt.query_map(params![package_id], |row| {
            Ok(Comment {
                id: row.get(0)?,
                user_id: row.get(1)?,
                package_id: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(comments)
    }

    pub async fn create_comment(&self, comment: &Comment) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO comments (user_id, package_id, content, created_at) 
             VALUES (?, ?, ?, ?)",
            params![
                comment.user_id,
                comment.package_id,
                comment.content,
                comment.created_at.to_rfc3339(),
            ]
        )?;
        Ok(())
    }
} 