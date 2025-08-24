use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub status: String,
    pub likes_count: i64,
    pub dislikes_count: i64,
    pub is_pinned: i64,
    pub helpful_count: Option<i64>,
    pub is_helpful: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub likes_count: i64,
    pub helpful_count: i64,
    pub created_at: DateTime<Utc>,
}

impl From<Comment> for CommentResponse {
    fn from(c: Comment) -> Self {
        Self {
            id: c.id,
            user_id: c.user_id,
            content: c.content,
            parent_id: c.parent_id,
            likes_count: c.likes_count,
            helpful_count: c.helpful_count.unwrap_or(0),
            created_at: c.created_at,
        }
    }
} 