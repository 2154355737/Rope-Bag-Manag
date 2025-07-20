use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};
use crate::models::{User, Package, Comment};
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
            "SELECT id, username, email, password_hash, nickname, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, login_count, upload_count, download_count, 
                    created_at, last_login, is_admin 
             FROM users ORDER BY created_at DESC"
        )?;

        let users = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                role: match row.get::<_, String>(5)?.to_lowercase().as_str() {
                    "admin" => crate::models::UserRole::Admin,
                    "moderator" => crate::models::UserRole::Moderator,
                    "elder" => crate::models::UserRole::Elder,
                    _ => crate::models::UserRole::User,
                },
                star: row.get(6)?,
                ban_status: match row.get::<_, String>(7)?.as_str() {
                    "suspended" => crate::models::BanStatus::Suspended,
                    "banned" => crate::models::BanStatus::Banned,
                    _ => crate::models::BanStatus::Normal,
                },
                ban_reason: row.get(8)?,
                qq_number: row.get(9)?,
                avatar_url: row.get(10)?,
                login_count: row.get(11)?,
                upload_count: row.get(12)?,
                download_count: row.get(13)?,
                created_at: row.get(14)?,
                last_login: row.get(15)?,
                is_admin: row.get(16)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, nickname, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, login_count, upload_count, download_count, 
                    created_at, last_login, is_admin 
             FROM users WHERE id = ?"
        )?;

        let user = stmt.query_row(params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                role: match row.get::<_, String>(5)?.to_lowercase().as_str() {
                    "admin" => crate::models::UserRole::Admin,
                    "moderator" => crate::models::UserRole::Moderator,
                    "elder" => crate::models::UserRole::Elder,
                    _ => crate::models::UserRole::User,
                },
                star: row.get(6)?,
                ban_status: match row.get::<_, String>(7)?.as_str() {
                    "suspended" => crate::models::BanStatus::Suspended,
                    "banned" => crate::models::BanStatus::Banned,
                    _ => crate::models::BanStatus::Normal,
                },
                ban_reason: row.get(8)?,
                qq_number: row.get(9)?,
                avatar_url: row.get(10)?,
                login_count: row.get(11)?,
                upload_count: row.get(12)?,
                download_count: row.get(13)?,
                created_at: row.get(14)?,
                last_login: row.get(15)?,
                is_admin: row.get(16)?,
            })
        }).optional()?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, username, email, password_hash, nickname, role, star, ban_status, \
                    ban_reason, qq_number, avatar_url, login_count, upload_count, download_count, \
                    created_at, last_login, is_admin \
             FROM users WHERE username = ?";
        println!("[SQL] find_by_username: {} | username={}", sql, username);
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR] prepare failed: {}", e);
                return Err(e.into());
            }
        };
        let user = match stmt.query_row(params![username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                role: match row.get::<_, String>(5)?.to_lowercase().as_str() {
                    "admin" => crate::models::UserRole::Admin,
                    "moderator" => crate::models::UserRole::Moderator,
                    "elder" => crate::models::UserRole::Elder,
                    _ => crate::models::UserRole::User,
                },
                star: row.get(6)?,
                ban_status: match row.get::<_, String>(7)?.as_str() {
                    "suspended" => crate::models::BanStatus::Suspended,
                    "banned" => crate::models::BanStatus::Banned,
                    _ => crate::models::BanStatus::Normal,
                },
                ban_reason: row.get(8)?,
                qq_number: row.get(9)?,
                avatar_url: row.get(10)?,
                login_count: row.get(11)?,
                upload_count: row.get(12)?,
                download_count: row.get(13)?,
                created_at: row.get(14)?,
                last_login: row.get(15)?,
                is_admin: row.get(16)?,
            })
        }) {
            Ok(val) => Some(val),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => {
                println!("[ERROR] query_row failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[SQL] find_by_username result: {:?}", user);
        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO users (username, email, password_hash, nickname, role, star, ban_status, 
                               ban_reason, qq_number, avatar_url, login_count, upload_count, download_count, 
                               created_at, last_login, is_admin) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                user.username,
                user.email,
                user.password_hash,
                user.nickname,
                match user.role {
                    crate::models::UserRole::Admin => "admin",
                    crate::models::UserRole::Moderator => "moderator",
                    crate::models::UserRole::Elder => "elder",
                    crate::models::UserRole::User => "user",
                },
                user.star,
                match user.ban_status {
                    crate::models::BanStatus::Normal => "normal",
                    crate::models::BanStatus::Suspended => "suspended",
                    crate::models::BanStatus::Banned => "banned",
                },
                user.ban_reason,
                user.qq_number,
                user.avatar_url,
                user.login_count,
                user.upload_count,
                user.download_count,
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
                match user.role {
                    crate::models::UserRole::Admin => "admin",
                    crate::models::UserRole::Moderator => "moderator",
                    crate::models::UserRole::Elder => "elder",
                    crate::models::UserRole::User => "user",
                },
                user.star,
                match user.ban_status {
                    crate::models::BanStatus::Normal => "normal",
                    crate::models::BanStatus::Suspended => "suspended",
                    crate::models::BanStatus::Banned => "banned",
                },
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
            "UPDATE users SET last_login = ?, login_count = login_count + 1 WHERE id = ?",
            params![chrono::Utc::now().to_rfc3339(), user_id]
        )?;
        Ok(())
    }

    // 新增方法：更新上传次数
    pub async fn increment_upload_count(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE users SET upload_count = upload_count + 1 WHERE id = ?",
            params![user_id]
        )?;
        Ok(())
    }

    // 新增方法：更新下载次数
    pub async fn increment_download_count(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE users SET download_count = download_count + 1 WHERE id = ?",
            params![user_id]
        )?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM users WHERE id = ?", params![user_id])?;
        Ok(())
    }

    // 新增方法：获取用户资源
    pub async fn get_user_packages(&self, user_id: i32) -> Result<Vec<Package>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, author, version, description, file_url, file_size, 
                    download_count, like_count, favorite_count, category_id, status, 
                    created_at, updated_at 
             FROM packages WHERE author_id = ? ORDER BY created_at DESC"
        )?;

        let packages = stmt.query_map(params![user_id], |row| {
            Ok(Package {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                version: row.get(3)?,
                description: row.get(4)?,
                file_url: row.get(5)?,
                file_size: row.get(6)?,
                download_count: row.get(7)?,
                like_count: row.get(8)?,
                favorite_count: row.get(9)?,
                category_id: row.get(10)?,
                status: match row.get::<_, String>(11)?.as_str() {
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Active,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(packages)
    }

    // 新增方法：获取用户评论
    pub async fn get_user_comments(&self, user_id: i32) -> Result<Vec<Comment>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, user_id, target_type, target_id, content, status, parent_id, 
                    likes, dislikes, created_at, updated_at 
             FROM comments WHERE user_id = ? ORDER BY created_at DESC"
        )?;

        let comments = stmt.query_map(params![user_id], |row| {
            Ok(Comment {
                id: row.get(0)?,
                user_id: row.get(1)?,
                target_type: row.get(2)?,
                target_id: row.get(3)?,
                content: row.get(4)?,
                status: row.get(5)?,
                parent_id: row.get(6)?,
                likes: row.get(7)?,
                dislikes: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(comments)
    }

    // 新增方法：更新密码
    pub async fn update_password(&self, user_id: i32, password_hash: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE users SET password_hash = ? WHERE id = ?",
            params![password_hash, user_id]
        )?;
        Ok(())
    }
} 