use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct FollowRepository {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFollow {
    pub id: i32,
    pub follower_id: i32,
    pub followed_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowStats {
    pub user_id: i32,
    pub followers_count: i32,
    pub following_count: i32,
    pub updated_at: DateTime<Utc>,
}

impl Default for FollowStats {
    fn default() -> Self {
        Self {
            user_id: 0,
            followers_count: 0,
            following_count: 0,
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowUser {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub followed_at: DateTime<Utc>,
}

impl FollowRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// 关注用户
    pub async fn follow_user(&self, follower_id: i32, followed_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        
        // 检查是否已经关注
        let already_following: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM user_follows WHERE follower_id = ? AND followed_id = ?)",
            params![follower_id, followed_id],
            |row| row.get(0)
        )?;

        if already_following {
            return Ok(false); // 已经关注了
        }

        // 插入关注关系
        conn.execute(
            "INSERT INTO user_follows (follower_id, followed_id) VALUES (?, ?)",
            params![follower_id, followed_id]
        )?;

        Ok(true)
    }

    /// 取消关注用户
    pub async fn unfollow_user(&self, follower_id: i32, followed_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        
        let rows_affected = conn.execute(
            "DELETE FROM user_follows WHERE follower_id = ? AND followed_id = ?",
            params![follower_id, followed_id]
        )?;

        Ok(rows_affected > 0)
    }

    /// 检查是否关注某用户
    pub async fn is_following(&self, follower_id: i32, followed_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        
        let following: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM user_follows WHERE follower_id = ? AND followed_id = ?)",
            params![follower_id, followed_id],
            |row| row.get(0)
        )?;

        Ok(following)
    }

    /// 获取用户的关注统计
    pub async fn get_follow_stats(&self, user_id: i32) -> Result<FollowStats> {
        let conn = self.conn.lock().await;
        
        let stats = conn.query_row(
            "SELECT user_id, followers_count, following_count, updated_at 
             FROM user_follow_stats WHERE user_id = ?",
            params![user_id],
            |row| {
                Ok(FollowStats {
                    user_id: row.get(0)?,
                    followers_count: row.get(1)?,
                    following_count: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            }
        ).optional()?;

        match stats {
            Some(stats) => Ok(stats),
            None => {
                // 如果不存在统计数据，初始化并返回
                conn.execute(
                    "INSERT OR IGNORE INTO user_follow_stats (user_id) VALUES (?)",
                    params![user_id]
                )?;
                
                Ok(FollowStats {
                    user_id,
                    followers_count: 0,
                    following_count: 0,
                    updated_at: Utc::now(),
                })
            }
        }
    }

    /// 获取用户的关注者列表
    pub async fn get_followers(&self, user_id: i32, offset: i64, limit: i64) -> Result<(Vec<FollowUser>, i64)> {
        let conn = self.conn.lock().await;
        
        // 获取关注者列表
        let mut stmt = conn.prepare(
            "SELECT u.id, u.username, u.nickname, u.avatar_url, u.bio, uf.created_at
             FROM user_follows uf
             JOIN users u ON uf.follower_id = u.id
             WHERE uf.followed_id = ?
             ORDER BY uf.created_at DESC
             LIMIT ? OFFSET ?"
        )?;

        let followers = stmt.query_map(params![user_id, limit, offset], |row| {
            Ok(FollowUser {
                id: row.get(0)?,
                username: row.get(1)?,
                nickname: row.get(2)?,
                avatar_url: row.get(3)?,
                bio: row.get(4)?,
                followed_at: row.get(5)?,
            })
        })?.collect::<rusqlite::Result<Vec<_>>>()?;

        // 获取总数
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_follows WHERE followed_id = ?",
            params![user_id],
            |row| row.get(0)
        )?;

        Ok((followers, total))
    }

    /// 获取用户的关注列表
    pub async fn get_following(&self, user_id: i32, offset: i64, limit: i64) -> Result<(Vec<FollowUser>, i64)> {
        let conn = self.conn.lock().await;
        
        // 获取关注列表
        let mut stmt = conn.prepare(
            "SELECT u.id, u.username, u.nickname, u.avatar_url, u.bio, uf.created_at
             FROM user_follows uf
             JOIN users u ON uf.followed_id = u.id
             WHERE uf.follower_id = ?
             ORDER BY uf.created_at DESC
             LIMIT ? OFFSET ?"
        )?;

        let following = stmt.query_map(params![user_id, limit, offset], |row| {
            Ok(FollowUser {
                id: row.get(0)?,
                username: row.get(1)?,
                nickname: row.get(2)?,
                avatar_url: row.get(3)?,
                bio: row.get(4)?,
                followed_at: row.get(5)?,
            })
        })?.collect::<rusqlite::Result<Vec<_>>>()?;

        // 获取总数
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_follows WHERE follower_id = ?",
            params![user_id],
            |row| row.get(0)
        )?;

        Ok((following, total))
    }

    /// 获取关注的用户的最新内容（动态）
    pub async fn get_following_feed(&self, user_id: i32, _offset: i64, _limit: i64, _content_type: Option<&str>) -> Result<Vec<i32>> {
        let conn = self.conn.lock().await;
        
        // 获取关注的用户ID列表
        let mut stmt = conn.prepare(
            "SELECT followed_id FROM user_follows WHERE follower_id = ?"
        )?;
        
        let following_users: Vec<i32> = stmt.query_map(params![user_id], |row| {
            Ok(row.get::<_, i32>(0)?)
        })?.collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(following_users)
    }

    /// 批量检查关注状态
    pub async fn check_multiple_following(&self, follower_id: i32, user_ids: &[i32]) -> Result<Vec<(i32, bool)>> {
        let conn = self.conn.lock().await;
        
        let mut results = Vec::new();
        
        for &user_id in user_ids {
            let following: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM user_follows WHERE follower_id = ? AND followed_id = ?)",
                params![follower_id, user_id],
                |row| row.get(0)
            )?;
            
            results.push((user_id, following));
        }

        Ok(results)
    }

    /// 获取互相关注的用户（好友）
    pub async fn get_mutual_follows(&self, user_id: i32, offset: i64, limit: i64) -> Result<(Vec<FollowUser>, i64)> {
        let conn = self.conn.lock().await;
        
        // 获取互相关注的用户
        let mut stmt = conn.prepare(
            "SELECT u.id, u.username, u.nickname, u.avatar_url, u.bio, uf1.created_at
             FROM user_follows uf1
             JOIN user_follows uf2 ON uf1.followed_id = uf2.follower_id AND uf1.follower_id = uf2.followed_id
             JOIN users u ON uf1.followed_id = u.id
             WHERE uf1.follower_id = ?
             ORDER BY uf1.created_at DESC
             LIMIT ? OFFSET ?"
        )?;

        let mutual_follows = stmt.query_map(params![user_id, limit, offset], |row| {
            Ok(FollowUser {
                id: row.get(0)?,
                username: row.get(1)?,
                nickname: row.get(2)?,
                avatar_url: row.get(3)?,
                bio: row.get(4)?,
                followed_at: row.get(5)?,
            })
        })?.collect::<rusqlite::Result<Vec<_>>>()?;

        // 获取总数
        let total: i64 = conn.query_row(
            "SELECT COUNT(*)
             FROM user_follows uf1
             JOIN user_follows uf2 ON uf1.followed_id = uf2.follower_id AND uf1.follower_id = uf2.followed_id
             WHERE uf1.follower_id = ?",
            params![user_id],
            |row| row.get(0)
        )?;

        Ok((mutual_follows, total))
    }
} 