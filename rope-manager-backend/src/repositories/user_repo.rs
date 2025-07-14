use anyhow::Result;
use rusqlite::{Connection, params};
use crate::models::User;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct UserRepository {
    conn: Arc<Mutex<Connection>>,
}

impl UserRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, username, password_hash, nickname, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, created_at, last_login, is_admin 
             FROM users ORDER BY created_at DESC"
        )?;

        let users = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                nickname: row.get(3)?,
                role: row.get(4)?,
                star: row.get(5)?,
                ban_status: row.get(6)?,
                ban_reason: row.get(7)?,
                qq_number: row.get(8)?,
                avatar_url: row.get(9)?,
                created_at: row.get(10)?,
                last_login: row.get(11)?,
                is_admin: row.get(12)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, username, password_hash, nickname, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, created_at, last_login, is_admin 
             FROM users WHERE id = ?"
        )?;

        let user = stmt.query_row(params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                nickname: row.get(3)?,
                role: row.get(4)?,
                star: row.get(5)?,
                ban_status: row.get(6)?,
                ban_reason: row.get(7)?,
                qq_number: row.get(8)?,
                avatar_url: row.get(9)?,
                created_at: row.get(10)?,
                last_login: row.get(11)?,
                is_admin: row.get(12)?,
            })
        }).optional()?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, username, password_hash, nickname, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, created_at, last_login, is_admin 
             FROM users WHERE username = ?"
        )?;

        let user = stmt.query_row(params![username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                nickname: row.get(3)?,
                role: row.get(4)?,
                star: row.get(5)?,
                ban_status: row.get(6)?,
                ban_reason: row.get(7)?,
                qq_number: row.get(8)?,
                avatar_url: row.get(9)?,
                created_at: row.get(10)?,
                last_login: row.get(11)?,
                is_admin: row.get(12)?,
            })
        }).optional()?;

        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO users (username, password_hash, nickname, role, star, ban_status, 
                               ban_reason, qq_number, avatar_url, created_at, last_login, is_admin) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                user.username,
                user.password_hash,
                user.nickname,
                user.role,
                user.star,
                user.ban_status,
                user.ban_reason,
                user.qq_number,
                user.avatar_url,
                user.created_at,
                user.last_login,
                user.is_admin,
            ]
        )?;
        Ok(())
    }

    pub async fn update_user(&self, user: &User) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE users SET username = ?, password_hash = ?, nickname = ?, role = ?, 
                    star = ?, ban_status = ?, ban_reason = ?, qq_number = ?, avatar_url = ?, 
                    created_at = ?, last_login = ?, is_admin = ? WHERE id = ?",
            params![
                user.username,
                user.password_hash,
                user.nickname,
                user.role,
                user.star,
                user.ban_status,
                user.ban_reason,
                user.qq_number,
                user.avatar_url,
                user.created_at,
                user.last_login,
                user.is_admin,
                user.id,
            ]
        )?;
        Ok(())
    }

    pub async fn update_last_login(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE users SET last_login = ? WHERE id = ?",
            params![chrono::Utc::now(), user_id]
        )?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM users WHERE id = ?", params![user_id])?;
        Ok(())
    }
} 