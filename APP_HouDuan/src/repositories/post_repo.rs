use rusqlite::{Connection, Result, Row};
use std::path::Path;
use crate::models::post::{Post, PostStatus};
use chrono::{DateTime, Utc};
use serde_json;

#[derive(Clone)]
pub struct PostRepository {
    db_path: String,
}

impl PostRepository {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        Ok(PostRepository {
            db_path: db_path.as_ref().to_string_lossy().to_string(),
        })
    }

    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn map_row_to_post(row: &Row) -> Result<Post> {
        let status_str: String = row.get(6)?;
        let status = match status_str.as_str() {
            "Draft" => PostStatus::Draft,
            "Published" => PostStatus::Published,
            "Archived" => PostStatus::Archived,
            "Deleted" => PostStatus::Deleted,
            _ => PostStatus::Draft,
        };

        // 处理标签
        let tags_str: Option<String> = row.get(20).ok();
        let tags = tags_str.and_then(|s| {
            if s.is_empty() {
                None
            } else {
                serde_json::from_str(&s).ok()
            }
        });

        // 处理图片
        let images_str: Option<String> = row.get(18).ok();
        let images = images_str.and_then(|s| {
            if s.is_empty() {
                None
            } else {
                serde_json::from_str(&s).ok()
            }
        });

        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            author_id: row.get(3)?,
            author_name: row.get(4)?,
            category_id: row.get(5).ok(),
            status,
            view_count: row.get(7)?,
            like_count: row.get(8)?,
            comment_count: row.get(9)?,
            is_pinned: row.get(10)?,
            is_featured: row.get(11)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(12)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(12, "created_at".to_string(), rusqlite::types::Type::Text))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(13)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(13, "updated_at".to_string(), rusqlite::types::Type::Text))?
                .with_timezone(&Utc),
            review_status: row.get(14).ok(),
            review_comment: row.get(15).ok(),
            reviewer_id: row.get(16).ok(),
            reviewed_at: row.get::<_, Option<String>>(17)?.and_then(|s| {
                DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))
            }),
            images,
            code_snippet: row.get(19).ok(),
            tags,
        })
    }

    /// 获取帖子排行榜（按浏览量排序）
    pub fn get_post_ranking(&self, page: u32, page_size: u32) -> Result<(Vec<Post>, i64)> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * page_size;

        // 首先获取总数
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM posts WHERE status = 'Published'",
            [],
            |row| row.get(0),
        )?;

        // 获取排行数据，按浏览量降序排列
        let query = "
            SELECT id, title, content, author_id, author_name, category_id, status,
                   view_count, like_count, comment_count, is_pinned, is_featured,
                   created_at, updated_at, review_status, review_comment, reviewer_id,
                   reviewed_at, images, code_snippet, tags
            FROM posts 
            WHERE status = 'Published'
            ORDER BY view_count DESC, like_count DESC, created_at DESC
            LIMIT ? OFFSET ?
        ";

        let mut stmt = conn.prepare(query)?;
        let posts_iter = stmt.query_map([page_size, offset], Self::map_row_to_post)?;

        let mut posts = Vec::new();
        for post_result in posts_iter {
            posts.push(post_result?);
        }

        Ok((posts, total))
    }

    /// 根据ID获取帖子详情
    pub fn find_by_id(&self, post_id: i32) -> Result<Option<Post>> {
        let conn = self.get_connection()?;
        
        let query = "
            SELECT id, title, content, author_id, author_name, category_id, status,
                   view_count, like_count, comment_count, is_pinned, is_featured,
                   created_at, updated_at, review_status, review_comment, reviewer_id,
                   reviewed_at, images, code_snippet, tags
            FROM posts 
            WHERE id = ?
        ";

        match conn.query_row(query, [post_id], Self::map_row_to_post) {
            Ok(post) => Ok(Some(post)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
} 