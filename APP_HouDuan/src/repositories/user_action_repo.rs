use anyhow::Result;
use chrono::{Utc, DateTime};
use rusqlite::{params, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

use crate::models::user_action::{
    UserAction, UserActionWithUser, CreateUserActionRequest, UserActionQueryParams, 
    UserActionStats, DailyActionStat, ActionTypeStat
};

#[derive(Clone)]
pub struct UserActionRepository {
    conn: Arc<Mutex<Connection>>,
}

impl UserActionRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    // 创建用户行为记录
    pub async fn create_user_action(&self, req: &CreateUserActionRequest) -> Result<UserAction> {
        
        let conn = self.conn.lock().await;
        
        // 确保表存在（移除外键约束以支持访客用户）
        
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS user_actions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                action_type TEXT NOT NULL,
                target_type TEXT,
                target_id INTEGER,
                details TEXT,
                ip_address TEXT,
                user_agent TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        ) {
            Ok(_) => println!("表检查成功"),
            Err(e) => println!("表创建错误: {}", e),
        }

        let timestamp = Utc::now();
        let created_at_str = timestamp.to_rfc3339();
        
        
        
        // 对于访客用户(user_id = None)，不检查用户是否存在
        if let Some(user_id) = req.user_id {
            if user_id > 0 {
                // 检查用户是否存在（仅对登录用户）
                match conn.query_row(
                    "SELECT id FROM users WHERE id = ?",
                    params![user_id],
                    |_| Ok(())
                ) {
                    Ok(_) => println!("用户ID {} 验证成功", user_id),
                    Err(e) => {
                        println!("查询用户出错: {}", e);
                        println!("警告: 用户ID {} 在users表中不存在，这可能导致外键约束错误", user_id);
                    }
                }
            }
        } else {
            println!("访客用户，跳过用户验证");
        }
        
        // 插入用户行为记录
        match conn.execute(
            "INSERT INTO user_actions (user_id, action_type, target_type, target_id, details, ip_address, user_agent, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                req.user_id,
                req.action_type,
                req.target_type,
                req.target_id,
                req.details,
                req.ip_address,
                req.user_agent,
                created_at_str
            ]
        ) {
            Ok(_) => {
                
                let action_id = conn.last_insert_rowid() as i32;
                                 let action = UserAction {
                     id: action_id,
                     user_id: req.user_id,
                     action_type: req.action_type.clone(),
                     target_type: req.target_type.clone(),
                     target_id: req.target_id,
                     details: req.details.clone(),
                     ip_address: req.ip_address.clone(),
                     user_agent: req.user_agent.clone(),
                     created_at: timestamp,
                 };
                
                Ok(action)
            },
            Err(e) => {
                
                Err(anyhow::anyhow!("插入用户行为记录失败: {}", e))
            }
        }
    }

    // 获取用户行为记录列表（带用户信息）
    pub async fn get_user_actions_with_user(&self, params: &UserActionQueryParams) -> Result<(Vec<UserActionWithUser>, i64)> {
        let conn = self.conn.lock().await;
        
        // 构建查询条件
        let mut conditions = Vec::new();
        let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(user_id) = params.user_id {
            conditions.push("ua.user_id = ?");
            query_params.push(Box::new(user_id));
        }
        
        if let Some(action_type) = &params.action_type {
            conditions.push("ua.action_type = ?");
            query_params.push(Box::new(action_type.clone()));
        }
        
        if let Some(target_type) = &params.target_type {
            conditions.push("ua.target_type = ?");
            query_params.push(Box::new(target_type.clone()));
        }
        
        if let Some(target_id) = params.target_id {
            conditions.push("ua.target_id = ?");
            query_params.push(Box::new(target_id));
        }
        
        if let Some(start_time) = &params.start_time {
            conditions.push("ua.created_at >= ?");
            query_params.push(Box::new(start_time.clone()));
        }
        
        if let Some(end_time) = &params.end_time {
            conditions.push("ua.created_at <= ?");
            query_params.push(Box::new(end_time.clone()));
        }
        
        // 构建WHERE子句
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // 计算总记录数
        let count_sql = format!("SELECT COUNT(*) FROM user_actions ua {}", where_clause);
        let mut count_stmt = conn.prepare(&count_sql)?;
        
        let total: i64 = count_stmt.query_row(rusqlite::params_from_iter(query_params.iter().map(|p| p.as_ref())), |row| {
            row.get(0)
        })?;
        
        // 分页参数
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;
        
        // 查询记录（LEFT JOIN users表获取用户信息）
        let query_sql = format!(
            "SELECT ua.id, ua.user_id, ua.action_type, ua.target_type, ua.target_id, ua.details, 
                    ua.ip_address, ua.user_agent, ua.created_at,
                    u.username, u.nickname, u.avatar_url
             FROM user_actions ua 
             LEFT JOIN users u ON ua.user_id = u.id
             {} 
             ORDER BY ua.created_at DESC 
             LIMIT ? OFFSET ?",
            where_clause
        );
        
        // 重新创建查询参数，添加分页参数
        let mut all_params: Vec<Box<dyn rusqlite::ToSql>> = query_params;
        all_params.push(Box::new(page_size as i64));
        all_params.push(Box::new(offset as i64));
        
        let mut stmt = conn.prepare(&query_sql)?;
        let action_iter = stmt.query_map(rusqlite::params_from_iter(all_params.iter().map(|p| p.as_ref())), |row| {
            Ok(UserActionWithUser {
                id: row.get(0)?,
                user_id: row.get(1)?,
                action_type: row.get(2)?,
                target_type: row.get(3)?,
                target_id: row.get(4)?,
                details: row.get(5)?,
                ip_address: row.get(6)?,
                user_agent: row.get(7)?,
                created_at: {
                    let time_str: String = row.get(8)?;
                    // 尝试多种时间格式解析
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&time_str) {
                        dt.with_timezone(&Utc)
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S") {
                        DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S%.f") {
                        DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
                    } else {
                        // 如果都解析失败，使用当前时间
                        Utc::now()
                    }
                },
                username: row.get(9).ok(),
                nickname: row.get(10).ok(),
                avatar: row.get(11).ok(),
            })
        })?;
        
        let mut actions = Vec::new();
        for action in action_iter {
            actions.push(action?);
        }
        
        Ok((actions, total))
    }

    // 获取用户行为记录列表（原方法，保持兼容性）
    pub async fn get_user_actions(&self, params: &UserActionQueryParams) -> Result<(Vec<UserAction>, i64)> {
        let conn = self.conn.lock().await;
        
        // 构建查询条件
        let mut conditions = Vec::new();
        let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(user_id) = params.user_id {
            conditions.push("user_id = ?");
            query_params.push(Box::new(user_id));
        }
        
        if let Some(action_type) = &params.action_type {
            conditions.push("action_type = ?");
            query_params.push(Box::new(action_type.clone()));
        }
        
        if let Some(target_type) = &params.target_type {
            conditions.push("target_type = ?");
            query_params.push(Box::new(target_type.clone()));
        }
        
        if let Some(target_id) = params.target_id {
            conditions.push("target_id = ?");
            query_params.push(Box::new(target_id));
        }
        
        // 移除success条件
        
        if let Some(start_time) = &params.start_time {
            conditions.push("created_at >= ?");
            query_params.push(Box::new(start_time.clone()));
        }
        
        if let Some(end_time) = &params.end_time {
            conditions.push("created_at <= ?");
            query_params.push(Box::new(end_time.clone()));
        }
        
        // 构建WHERE子句
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // 计算总记录数
        let count_sql = format!("SELECT COUNT(*) FROM user_actions {}", where_clause);
        let mut count_stmt = conn.prepare(&count_sql)?;
        
        let total: i64 = count_stmt.query_row(rusqlite::params_from_iter(query_params.iter().map(|p| p.as_ref())), |row| {
            row.get(0)
        })?;
        
        // 分页参数
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;
        
        // 查询记录
        let query_sql = format!(
            "SELECT id, user_id, action_type, target_type, target_id, details, 
                    ip_address, user_agent, created_at 
             FROM user_actions 
             {} 
             ORDER BY created_at DESC 
             LIMIT ? OFFSET ?",
            where_clause
        );
        
        // 重新创建查询参数，添加分页参数
        let mut query_params_with_paging: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(user_id) = params.user_id {
            query_params_with_paging.push(Box::new(user_id));
        }
        
        if let Some(action_type) = &params.action_type {
            query_params_with_paging.push(Box::new(action_type.clone()));
        }
        
        if let Some(target_type) = &params.target_type {
            query_params_with_paging.push(Box::new(target_type.clone()));
        }
        
        if let Some(target_id) = params.target_id {
            query_params_with_paging.push(Box::new(target_id));
        }
        
        // 删除对success的引用
        
        if let Some(start_time) = &params.start_time {
            query_params_with_paging.push(Box::new(start_time.clone()));
        }
        
        if let Some(end_time) = &params.end_time {
            query_params_with_paging.push(Box::new(end_time.clone()));
        }
        
        // 添加分页参数
        query_params_with_paging.push(Box::new(page_size));
        query_params_with_paging.push(Box::new(offset));
        
        let mut stmt = conn.prepare(&query_sql)?;
        let rows = stmt.query_map(
            rusqlite::params_from_iter(query_params_with_paging.iter().map(|p| p.as_ref())),
            |row| {
                let created_at_str: String = row.get(8)?;
                let created_at = {
                    // 尝试多种时间格式解析
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&created_at_str) {
                        dt.with_timezone(&Utc)
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S") {
                        DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S%.f") {
                        DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
                    } else {
                        // 如果都解析失败，使用当前时间
                        Utc::now()
                    }
                };
                
                Ok(UserAction {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    action_type: row.get(2)?,
                    target_type: row.get(3)?,
                    target_id: row.get(4)?,
                    details: row.get(5)?,
                    ip_address: row.get(6)?,
                    user_agent: row.get(7)?,
                    created_at,
                })
            },
        )?;
        
        let mut actions = Vec::new();
        for row in rows {
            actions.push(row?);
        }
        
        Ok((actions, total))
    }

    // 删除用户行为记录
    pub async fn delete_user_action(&self, action_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        
        conn.execute("DELETE FROM user_actions WHERE id = ?", params![action_id])?;
        
        Ok(())
    }

    // 批量删除用户行为记录
    pub async fn batch_delete_user_actions(&self, action_ids: &[i32]) -> Result<()> {
        if action_ids.is_empty() {
            return Ok(());
        }
        
        let conn = self.conn.lock().await;
        
        // 构建IN子句的参数占位符
        let placeholders: Vec<String> = (0..action_ids.len()).map(|_| "?".to_string()).collect();
        let in_clause = placeholders.join(", ");
        
        let sql = format!("DELETE FROM user_actions WHERE id IN ({})", in_clause);
        
        let mut stmt = conn.prepare(&sql)?;
        stmt.execute(rusqlite::params_from_iter(action_ids.iter()))?;
        
        Ok(())
    }

    // 获取用户行为统计数据
    pub async fn get_action_stats(&self, params: &UserActionQueryParams) -> Result<UserActionStats> {
        let conn = self.conn.lock().await;
        
        // 构建查询条件
        let mut conditions = Vec::new();
        let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(user_id) = params.user_id {
            conditions.push("user_id = ?");
            query_params.push(Box::new(user_id));
        }
        
        if let Some(start_time) = &params.start_time {
            conditions.push("created_at >= ?");
            query_params.push(Box::new(start_time.clone()));
        }
        
        if let Some(end_time) = &params.end_time {
            conditions.push("created_at <= ?");
            query_params.push(Box::new(end_time.clone()));
        }
        
        // 构建WHERE子句
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        // 总记录数
        let count_sql = format!("SELECT COUNT(*) FROM user_actions {}", where_clause);
        let mut count_stmt = conn.prepare(&count_sql)?;
        
        let total_actions: i64 = count_stmt.query_row(
            rusqlite::params_from_iter(query_params.iter().map(|p| p.as_ref())),
            |row| row.get(0)
        )?;
        
        // 活跃用户数
        let users_sql = format!("SELECT COUNT(DISTINCT user_id) FROM user_actions {}", where_clause);
        let mut users_stmt = conn.prepare(&users_sql)?;
        
        let active_users: i64 = users_stmt.query_row(
            rusqlite::params_from_iter(query_params.iter().map(|p| p.as_ref())),
            |row| row.get(0)
        )?;
        
        // 按日期统计
        let by_day_sql = format!(
            "SELECT date(created_at) as day, COUNT(*) as count 
             FROM user_actions 
             {} 
             GROUP BY day 
             ORDER BY day DESC 
             LIMIT 30",
            where_clause
        );
        let mut by_day_stmt = conn.prepare(&by_day_sql)?;
        
        let by_day_rows = by_day_stmt.query_map(
            rusqlite::params_from_iter(query_params.iter().map(|p| p.as_ref())),
            |row| {
                Ok(DailyActionStat {
                    date: row.get(0)?,
                    count: row.get(1)?,
                })
            }
        )?;
        
        let mut by_day = Vec::new();
        for row in by_day_rows {
            by_day.push(row?);
        }
        
        // 按类型统计
        let by_type_sql = format!(
            "SELECT action_type, COUNT(*) as count 
             FROM user_actions 
             {} 
             GROUP BY action_type 
             ORDER BY count DESC",
            where_clause
        );
        let mut by_type_stmt = conn.prepare(&by_type_sql)?;
        
        let by_type_rows = by_type_stmt.query_map(
            rusqlite::params_from_iter(query_params.iter().map(|p| p.as_ref())),
            |row| {
                Ok(ActionTypeStat {
                    action_type: row.get(0)?,
                    count: row.get(1)?,
                })
            }
        )?;
        
        let mut by_type = Vec::new();
        for row in by_type_rows {
            by_type.push(row?);
        }
        
        Ok(UserActionStats {
            total_actions,
            active_users,
            by_day,
            by_type,
        })
    }

    // 获取用户的点赞列表（使用视图）
    pub async fn get_user_likes(&self, user_id: i32, page: u32, page_size: u32) -> Result<(Vec<UserLikeSummary>, i64)> {
        let conn = self.conn.lock().await;
        let offset = (page - 1) * page_size;
        
        // 查询总数
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_likes_summary WHERE user_id = ?",
            params![user_id],
            |row| row.get(0),
        )?;
        
        // 查询分页数据
        let mut stmt = conn.prepare("
            SELECT user_id, username, like_type, target_id, target_title, target_description, created_at
            FROM user_likes_summary 
            WHERE user_id = ? 
            ORDER BY created_at DESC 
            LIMIT ? OFFSET ?
        ")?;
        
        let likes: Vec<UserLikeSummary> = stmt.query_map(
            params![user_id, page_size, offset],
            |row| {
                Ok(UserLikeSummary {
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    like_type: row.get(2)?,
                    target_id: row.get(3)?,
                    target_title: row.get(4)?,
                    target_description: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )?.collect::<Result<Vec<_>, _>>()?;
        
        Ok((likes, total))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLikeSummary {
    pub user_id: i32,
    pub username: String,
    pub like_type: String,
    pub target_id: i32,
    pub target_title: String,
    pub target_description: Option<String>,
    pub created_at: String,
} 