use anyhow::Result;
use rusqlite::{Connection, params};
use crate::models::Comment;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;

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

    // 获取所有评论（管理员接口）
    pub async fn get_all_comments(
        &self,
        page: i32,
        size: i32,
        status: Option<&str>,
        target_type: Option<&str>,
        target_id: Option<i32>,
        user_id: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<(Vec<Comment>, i64)> {
        let conn = self.conn.lock().await;
        
        // 构建查询条件
        let mut where_clauses = Vec::new();
        let mut params_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(s) = status {
            where_clauses.push("status = ?");
            params_values.push(Box::new(s.to_string()));
        }
        
        if let Some(t) = target_type {
            where_clauses.push("target_type = ?");
            params_values.push(Box::new(t.to_string()));
        }
        
        if let Some(id) = target_id {
            where_clauses.push("target_id = ?");
            params_values.push(Box::new(id));
        }
        
        if let Some(id) = user_id {
            where_clauses.push("user_id = ?");
            params_values.push(Box::new(id));
        }
        
        if let Some(start) = start_date {
            where_clauses.push("created_at >= ?");
            params_values.push(Box::new(start.to_string()));
        }
        
        if let Some(end) = end_date {
            where_clauses.push("created_at <= ?");
            params_values.push(Box::new(end.to_string()));
        }
        
        let where_clause = if !where_clauses.is_empty() {
            format!("WHERE {}", where_clauses.join(" AND "))
        } else {
            "".to_string()
        };
        
        // 查询总记录数
        let count_sql = format!("SELECT COUNT(*) FROM comments {}", where_clause);
        
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_values.iter()
            .map(|p| &**p as &dyn rusqlite::ToSql)
            .collect();

        let total: i64 = conn.query_row(
            &count_sql,
            &params_refs[..],
            |row| row.get(0)
        )?;
        
        // 获取分页数据
        let sql = format!(
            "SELECT id, user_id, target_type, target_id, content, status, parent_id, 
                    likes, dislikes, created_at, updated_at 
             FROM comments {} ORDER BY created_at DESC LIMIT ? OFFSET ?",
            where_clause
        );
        
        // 创建一个新的参数列表
        let mut all_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        // 添加现有的参数
        for param in params_values {
            all_params.push(param);
        }
        
        // 添加分页参数
        all_params.push(Box::new(size));
        all_params.push(Box::new((page - 1) * size));
        
        // 转换为引用
        let all_params_refs: Vec<&dyn rusqlite::ToSql> = all_params.iter()
            .map(|p| &**p as &dyn rusqlite::ToSql)
            .collect();

        let mut stmt = conn.prepare(&sql)?;
        let comment_iter = stmt.query_map(&all_params_refs[..], |row| {
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
        })?;
        
        let mut comments = Vec::new();
        for comment in comment_iter {
            comments.push(comment?);
        }
        
        Ok((comments, total))
    }

    // 获取评论（兼容旧方法，使用target_id替代package_id）
    pub async fn get_comments_by_package(&self, package_id: i32) -> Result<Vec<Comment>> {
        self.get_comments_by_target("Package", package_id, 1, 100).await.map(|(comments, _)| comments)
    }

    // 获取特定目标的评论
    pub async fn get_comments_by_target(
        &self,
        target_type: &str,
        target_id: i32,
        page: i32,
        size: i32
    ) -> Result<(Vec<Comment>, i64)> {
        let conn = self.conn.lock().await;
        
        // 计算总记录数
        let count_sql = "SELECT COUNT(*) FROM comments WHERE target_type = ? AND target_id = ? AND status != 'Deleted'";
        let total: i64 = conn.query_row(
            count_sql,
            params![target_type, target_id],
            |row| row.get(0)
        )?;
        
        // 获取评论列表
        let sql = "SELECT id, user_id, target_type, target_id, content, status, parent_id, 
                          likes, dislikes, created_at, updated_at 
                   FROM comments 
                   WHERE target_type = ? AND target_id = ? AND status != 'Deleted' AND parent_id IS NULL 
                   ORDER BY created_at DESC 
                   LIMIT ? OFFSET ?";
        
        let mut stmt = conn.prepare(sql)?;
        let comment_iter = stmt.query_map(
            params![target_type, target_id, size, (page - 1) * size],
            |row| {
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
            }
        )?;
        
        let mut comments = Vec::new();
        for comment in comment_iter {
            comments.push(comment?);
        }
        
        Ok((comments, total))
    }

    // 获取单个评论
    pub async fn get_comment_by_id(&self, comment_id: i32) -> Result<Option<Comment>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, user_id, target_type, target_id, content, status, parent_id, 
                          likes, dislikes, created_at, updated_at 
                   FROM comments 
                   WHERE id = ?";
        
        let mut stmt = conn.prepare(sql)?;
        let comment = stmt.query_row(params![comment_id], |row| {
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
        });
        
        match comment {
            Ok(comment) => Ok(Some(comment)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // 创建评论
    pub async fn create_comment(&self, comment: &Comment) -> Result<i32> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO comments (user_id, target_type, target_id, content, status, parent_id, 
                                  likes, dislikes, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                comment.user_id,
                comment.target_type,
                comment.target_id,
                comment.content,
                comment.status,
                comment.parent_id,
                comment.likes,
                comment.dislikes,
                comment.created_at.to_rfc3339(),
                comment.updated_at.to_rfc3339(),
            ]
        )?;
        
        // 获取最后插入的ID
        let id = conn.last_insert_rowid() as i32;
        Ok(id)
    }

    // 更新评论
    pub async fn update_comment(&self, comment: &Comment) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE comments 
             SET content = ?, status = ?, likes = ?, dislikes = ?, updated_at = ? 
             WHERE id = ?",
            params![
                comment.content,
                comment.status,
                comment.likes,
                comment.dislikes,
                comment.updated_at.to_rfc3339(),
                comment.id,
            ]
        )?;
        
        Ok(())
    }

    // 删除评论（物理删除）
    pub async fn delete_comment(&self, comment_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM comments WHERE id = ?", params![comment_id])?;
        Ok(())
    }

    // 获取评论回复
    pub async fn get_comment_replies(&self, comment_id: i32) -> Result<Vec<Comment>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, user_id, target_type, target_id, content, status, parent_id, 
                          likes, dislikes, created_at, updated_at 
                   FROM comments 
                   WHERE parent_id = ? AND status != 'Deleted' 
                   ORDER BY created_at ASC";
        
        let mut stmt = conn.prepare(sql)?;
        let comment_iter = stmt.query_map(params![comment_id], |row| {
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
        })?;
        
        let mut replies = Vec::new();
        for reply in comment_iter {
            replies.push(reply?);
        }
        
        Ok(replies)
    }

    // 获取用户的评论
    pub async fn get_user_comments(&self, user_id: i32, page: i32, size: i32) -> Result<(Vec<Comment>, i64)> {
        let conn = self.conn.lock().await;
        
        // 计算总记录数
        let count_sql = "SELECT COUNT(*) FROM comments WHERE user_id = ? AND status != 'Deleted'";
        let total: i64 = conn.query_row(
            count_sql,
            params![user_id],
            |row| row.get(0)
        )?;
        
        // 获取评论列表
        let sql = "SELECT id, user_id, target_type, target_id, content, status, parent_id, 
                          likes, dislikes, created_at, updated_at 
                   FROM comments 
                   WHERE user_id = ? AND status != 'Deleted' 
                   ORDER BY created_at DESC 
                   LIMIT ? OFFSET ?";
        
        let mut stmt = conn.prepare(sql)?;
        let comment_iter = stmt.query_map(
            params![user_id, size, (page - 1) * size],
            |row| {
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
            }
        )?;
        
        let mut comments = Vec::new();
        for comment in comment_iter {
            comments.push(comment?);
        }
        
        Ok((comments, total))
    }

    // 检查用户是否已点赞
    pub async fn has_user_liked(&self, comment_id: i32, user_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM comment_likes WHERE comment_id = ? AND user_id = ?",
            params![comment_id, user_id],
            |row| row.get(0)
        )?;
        
        Ok(count > 0)
    }

    // 添加用户点赞
    pub async fn add_user_like(&self, comment_id: i32, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR IGNORE INTO comment_likes (comment_id, user_id, created_at) VALUES (?, ?, ?)",
            params![comment_id, user_id, Utc::now().to_rfc3339()]
        )?;
        
        Ok(())
    }

    // 移除用户点赞
    pub async fn remove_user_like(&self, comment_id: i32, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "DELETE FROM comment_likes WHERE comment_id = ? AND user_id = ?",
            params![comment_id, user_id]
        )?;
        
        Ok(())
    }

    // 检查用户是否已点踩
    pub async fn has_user_disliked(&self, comment_id: i32, user_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM comment_dislikes WHERE comment_id = ? AND user_id = ?",
            params![comment_id, user_id],
            |row| row.get(0)
        )?;
        
        Ok(count > 0)
    }

    // 添加用户点踩
    pub async fn add_user_dislike(&self, comment_id: i32, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR IGNORE INTO comment_dislikes (comment_id, user_id, created_at) VALUES (?, ?, ?)",
            params![comment_id, user_id, Utc::now().to_rfc3339()]
        )?;
        
        Ok(())
    }

    // 移除用户点踩
    pub async fn remove_user_dislike(&self, comment_id: i32, user_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "DELETE FROM comment_dislikes WHERE comment_id = ? AND user_id = ?",
            params![comment_id, user_id]
        )?;
        
        Ok(())
    }
} 