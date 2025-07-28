use anyhow::Result;
use rusqlite::{Connection, params};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{Utc, DateTime};

#[derive(Clone)]
pub struct EmailVerificationRepository {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug)]
pub struct EmailVerification {
    pub id: i32,
    pub user_id: Option<i32>,
    pub email: String,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

impl EmailVerificationRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    pub async fn create(&self, user_id: Option<i32>, email: &str, code: &str, expires_at: DateTime<Utc>) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("INSERT INTO email_verifications (user_id,email,code,expires_at) VALUES (?,?,?,?)",
            params![user_id, email, code, expires_at.to_rfc3339()])?;
        Ok(())
    }

    pub async fn verify(&self, email: &str, code: &str) -> Result<bool> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, expires_at, used FROM email_verifications WHERE email=? AND code=? ORDER BY id DESC LIMIT 1")?;
        let row = stmt.query_row(params![email, code], |r| {
            Ok((r.get::<_, i32>(0)?, r.get::<_, String>(1)?, r.get::<_, i32>(2)?))
        });
        if let Ok((id, exp, used)) = row {
            if used == 1 { return Ok(false); }
            let exp_time = DateTime::parse_from_rfc3339(&exp)?.with_timezone(&Utc);
            if Utc::now() > exp_time { return Ok(false); }
            conn.execute("UPDATE email_verifications SET used=1 WHERE id=?", params![id])?;
            return Ok(true);
        }
        Ok(false)
    }
} 