use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};
use crate::models::{User, Package, Comment};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Datelike;

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
            "SELECT id, username, email, password_hash, nickname, bio, location, website, skills, role, star, ban_status, 
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
                bio: row.get(5)?,
                location: row.get(6)?,
                website: row.get(7)?,
                skills: row.get(8)?,
                role: match row.get::<_, String>(9)?.to_lowercase().as_str() {
                    "admin" => crate::models::UserRole::Admin,
                    "moderator" => crate::models::UserRole::Moderator,
                    "elder" => crate::models::UserRole::Elder,
                    _ => crate::models::UserRole::User,
                },
                star: row.get(10)?,
                ban_status: match row.get::<_, String>(11)?.as_str() {
                    "suspended" => crate::models::BanStatus::Suspended,
                    "banned" => crate::models::BanStatus::Banned,
                    _ => crate::models::BanStatus::Normal,
                },
                ban_reason: row.get(12)?,
                qq_number: row.get(13)?,
                avatar_url: row.get(14)?,
                login_count: row.get(15)?,
                upload_count: row.get(16)?,
                download_count: row.get(17)?,
                created_at: {
                    let datetime_str = row.get::<_, String>(18)?;
                    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                        chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                    } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                        dt.with_timezone(&chrono::Utc)
                    } else {
                        chrono::Utc::now()
                    }
                },
                last_login: {
                    if let Ok(datetime_str) = row.get::<_, String>(19) {
                        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                            Some(chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc))
                        } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                            Some(dt.with_timezone(&chrono::Utc))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                is_admin: row.get(20)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, nickname, bio, location, website, skills, role, star, ban_status, 
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
                bio: row.get(5)?,
                location: row.get(6)?,
                website: row.get(7)?,
                skills: row.get(8)?,
                role: match row.get::<_, String>(9)?.to_lowercase().as_str() {
                    "admin" => crate::models::UserRole::Admin,
                    "moderator" => crate::models::UserRole::Moderator,
                    "elder" => crate::models::UserRole::Elder,
                    _ => crate::models::UserRole::User,
                },
                star: row.get(10)?,
                ban_status: match row.get::<_, String>(11)?.as_str() {
                    "suspended" => crate::models::BanStatus::Suspended,
                    "banned" => crate::models::BanStatus::Banned,
                    _ => crate::models::BanStatus::Normal,
                },
                ban_reason: row.get(12)?,
                qq_number: row.get(13)?,
                avatar_url: row.get(14)?,
                login_count: row.get(15)?,
                upload_count: row.get(16)?,
                download_count: row.get(17)?,
                created_at: {
                    let datetime_str = row.get::<_, String>(18)?;
                    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                        chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                    } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                        dt.with_timezone(&chrono::Utc)
                    } else {
                        chrono::Utc::now()
                    }
                },
                last_login: {
                    if let Ok(datetime_str) = row.get::<_, String>(19) {
                        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                            Some(chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc))
                        } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                            Some(dt.with_timezone(&chrono::Utc))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                is_admin: row.get(20)?,
            })
        }).optional()?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let user = conn.query_row(
            "SELECT id, username, email, password_hash, nickname, bio, location, website, skills, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, login_count, upload_count, download_count, 
                    created_at, last_login, is_admin 
             FROM users WHERE username = ?",
            params![username],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    nickname: row.get(4)?,
                    bio: row.get(5)?,
                    location: row.get(6)?,
                    website: row.get(7)?,
                    skills: row.get(8)?,
                    role: match row.get::<_, String>(9)?.to_lowercase().as_str() {
                        "admin" => crate::models::UserRole::Admin,
                        "moderator" => crate::models::UserRole::Moderator,
                        "elder" => crate::models::UserRole::Elder,
                        _ => crate::models::UserRole::User,
                    },
                    star: row.get(10)?,
                    ban_status: match row.get::<_, String>(11)?.as_str() {
                        "suspended" => crate::models::BanStatus::Suspended,
                        "banned" => crate::models::BanStatus::Banned,
                        _ => crate::models::BanStatus::Normal,
                    },
                    ban_reason: row.get(12)?,
                    qq_number: row.get(13)?,
                    avatar_url: row.get(14)?,
                    login_count: row.get(15)?,
                    upload_count: row.get(16)?,
                    download_count: row.get(17)?,
                    created_at: {
                        let datetime_str = row.get::<_, String>(18)?;
                        // 尝试解析SQLite默认格式: "YYYY-MM-DD HH:MM:SS"
                        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                            chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                        } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                            // 备用：尝试RFC3339格式
                            dt.with_timezone(&chrono::Utc)
                        } else {
                            // 最后备用：使用当前时间
                            chrono::Utc::now()
                        }
                    },
                    last_login: {
                        if let Ok(datetime_str) = row.get::<_, String>(19) {
                            if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                                Some(chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc))
                            } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                                Some(dt.with_timezone(&chrono::Utc))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    },
                    is_admin: row.get(20)?,
                })
            }
        ).optional()?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().await;
        let user = conn.query_row(
            "SELECT id, username, email, password_hash, nickname, bio, location, website, skills, role, star, ban_status, 
                    ban_reason, qq_number, avatar_url, login_count, upload_count, download_count, 
                    created_at, last_login, is_admin 
             FROM users WHERE email = ?",
            params![email],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    nickname: row.get(4)?,
                    bio: row.get(5)?,
                    location: row.get(6)?,
                    website: row.get(7)?,
                    skills: row.get(8)?,
                    role: match row.get::<_, String>(9)?.to_lowercase().as_str() {
                        "admin" => crate::models::UserRole::Admin,
                        "moderator" => crate::models::UserRole::Moderator,
                        "elder" => crate::models::UserRole::Elder,
                        _ => crate::models::UserRole::User,
                    },
                    star: row.get(10)?,
                    ban_status: match row.get::<_, String>(11)?.as_str() {
                        "suspended" => crate::models::BanStatus::Suspended,
                        "banned" => crate::models::BanStatus::Banned,
                        _ => crate::models::BanStatus::Normal,
                    },
                    ban_reason: row.get(12)?,
                    qq_number: row.get(13)?,
                    avatar_url: row.get(14)?,
                    login_count: row.get(15)?,
                    upload_count: row.get(16)?,
                    download_count: row.get(17)?,
                    created_at: {
                        let datetime_str = row.get::<_, String>(18)?;
                        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                            chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                        } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                            dt.with_timezone(&chrono::Utc)
                        } else {
                            chrono::Utc::now()
                        }
                    },
                    last_login: {
                        if let Ok(datetime_str) = row.get::<_, String>(19) {
                            if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                                Some(chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc))
                            } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                                Some(dt.with_timezone(&chrono::Utc))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    },
                    is_admin: row.get(20)?,
                })
            }
        ).optional()?;
        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> Result<i32> {
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
        
        // 返回新创建用户的ID
        let user_id = conn.last_insert_rowid() as i32;
        Ok(user_id)
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
        
        // 先获取用户名
        let username: String = conn.query_row(
            "SELECT username FROM users WHERE id = ?",
            params![user_id],
            |row| row.get(0)
        )?;
        
        let mut stmt = conn.prepare(
            "SELECT id, name, author, version, description, file_url, file_size, 
                    download_count, like_count, favorite_count, category_id, status, 
                    created_at, updated_at, reviewer_id, reviewed_at, review_comment,
                    is_pinned, is_featured 
             FROM packages WHERE author = ? ORDER BY is_pinned DESC, is_featured DESC, created_at DESC"
        )?;

        let packages = stmt.query_map(params![username], |row| {
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
                    "pending" => crate::models::PackageStatus::Pending,
                    "published" => crate::models::PackageStatus::Active,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: {
                    let datetime_str = row.get::<_, String>(12)?;
                    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                        chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                    } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                        dt.with_timezone(&chrono::Utc)
                    } else {
                        chrono::Utc::now()
                    }
                },
                updated_at: {
                    let datetime_str = row.get::<_, String>(13)?;
                    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                        chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                    } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                        dt.with_timezone(&chrono::Utc)
                    } else {
                        chrono::Utc::now()
                    }
                },
                reviewer_id: row.get(14)?,
                reviewed_at: {
                    if let Ok(datetime_str) = row.get::<_, String>(15) {
                        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S") {
                            Some(chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc))
                        } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&datetime_str) {
                            Some(dt.with_timezone(&chrono::Utc))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                review_comment: row.get(16)?,
                is_pinned: row.get(17).unwrap_or(false),
                is_featured: row.get(18).unwrap_or(false),
                tags: None, // 用户仓库中暂时不处理标签
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
                parent_id: row.get(6)? ,
                likes: row.get(7)?,
                dislikes: row.get(8)?,
                pinned: false, // 默认不置顶
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                author_name: None,
                username: None,
                author_role: None,
                author_avatar: None,
                author_qq: None,
                target_title: None,
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

    // 获取管理员和元老的邮箱地址（用于通知新资源待审核）
    pub async fn get_admin_and_elder_emails(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT email FROM users WHERE (role = 'admin' OR role = 'elder') AND ban_status = 'normal'"
        )?;

        let emails = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(emails)
    }

    // 新增：批量删除用户
    pub async fn batch_delete_users(&self, usernames: Vec<String>) -> Result<()> {
        let conn = self.conn.lock().await;
        for username in usernames {
            // 检查用户是否存在且不是管理员
            let is_admin: bool = conn.query_row(
                "SELECT is_admin FROM users WHERE username = ?",
                params![username],
                |row| row.get(0)
            ).unwrap_or(true); // 如果查询失败，默认认为是管理员以防止误删
            
            if !is_admin {
                conn.execute(
                    "DELETE FROM users WHERE username = ?",
                    params![username]
                )?;
            }
        }
        Ok(())
    }

    // 新增：用户签到
    pub async fn user_check_in(&self, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        // 检查今天是否已经签到
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM user_check_ins WHERE user_id = ? AND check_in_date = ?)",
            params![user_id, today],
            |row| row.get(0)
        ).unwrap_or(false);
        
        if !exists {
            // 创建签到表（如果不存在）
            conn.execute(
                "CREATE TABLE IF NOT EXISTS user_check_ins (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id INTEGER NOT NULL,
                    check_in_date TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    UNIQUE(user_id, check_in_date)
                )",
                []
            )?;
            
            // 插入签到记录
            conn.execute(
                "INSERT INTO user_check_ins (user_id, check_in_date) VALUES (?, ?)",
                params![user_id, today]
            )?;
        }
        
        Ok(())
    }

    // 新增：获取用户连续签到天数
    pub async fn get_check_in_streak(&self, user_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        
        // 确保表存在
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_check_ins (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                check_in_date TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(user_id, check_in_date)
            )",
            []
        )?;
        
        let mut stmt = conn.prepare(
            "SELECT check_in_date FROM user_check_ins 
             WHERE user_id = ? 
             ORDER BY check_in_date DESC"
        )?;
        
        let dates: Vec<String> = stmt.query_map(params![user_id], |row| {
            Ok(row.get::<_, String>(0)?)
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        if dates.is_empty() {
            return Ok(0);
        }
        
        let mut streak = 1;
        let mut current_date = chrono::NaiveDate::parse_from_str(&dates[0], "%Y-%m-%d")
            .map_err(|e| anyhow::anyhow!("日期解析错误: {}", e))?;
        
        for date_str in dates.iter().skip(1) {
            let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|e| anyhow::anyhow!("日期解析错误: {}", e))?;
            
            if current_date - date == chrono::Duration::days(1) {
                streak += 1;
                current_date = date;
            } else {
                break;
            }
        }
        
        Ok(streak)
    }

    // 新增：获取用户本周发布统计
    pub async fn get_weekly_posts_count(&self, user_id: i32) -> Result<Vec<i32>> {
        let conn = self.conn.lock().await;
        let mut weekly_counts = vec![0; 7]; // 周一到周日
        
        // 获取本周的开始和结束日期
        let now = chrono::Utc::now();
        let days_since_monday = now.weekday().num_days_from_monday();
        let monday = now - chrono::Duration::days(days_since_monday as i64);
        
        for i in 0..7 {
            let date = monday + chrono::Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();
            
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM posts WHERE author_id = ? AND DATE(created_at) = ?",
                params![user_id, date_str],
                |row| row.get(0)
            ).unwrap_or(0);
            
            weekly_counts[i as usize] = count as i32;
        }
        
        Ok(weekly_counts)
    }
} 