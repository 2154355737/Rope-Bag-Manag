use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::user_action::{
    UserAction, CreateUserActionRequest, UserActionQueryParams, 
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
        
        let timestamp = Utc::now();
        let created_at_str = timestamp.to_rfc3339();
        
        conn.execute(
            "INSERT INTO user_actions (
                user_id, action_type, target_type, target_id, details, 
                ip_address, user_agent, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                req.user_id,
                req.action_type,
                req.target_type,
                req.target_id,
                req.details,
                req.ip_address,
                req.user_agent,
                created_at_str
            ],
        )?;
        
        let id = conn.last_insert_rowid() as i32;
        
        Ok(UserAction {
            id,
            user_id: req.user_id,
            action_type: req.action_type.clone(),
            target_type: req.target_type.clone(),
            target_id: req.target_id,
            details: req.details.clone(),
            ip_address: req.ip_address.clone(),
            user_agent: req.user_agent.clone(),
            created_at: timestamp,
        })
    }

    // 获取用户行为记录列表
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
                let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc);
                
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
} 