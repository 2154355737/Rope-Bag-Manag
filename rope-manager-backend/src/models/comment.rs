use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub target_type: String, // "Package", "User", "System"
    pub target_id: i32,
    pub content: String,
    pub status: String, // "Active", "Hidden", "Deleted"
    pub parent_id: Option<i32>,
    pub likes: i32,
    pub dislikes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub target_type: String,
    pub target_id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListResponse {
    pub list: Vec<Comment>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub id: i32,
    pub user_id: i32,
    pub author_name: Option<String>,
    pub target_type: String,
    pub target_id: i32,
    pub content: String,
    pub status: String,
    pub parent_id: Option<i32>,
    pub likes: i32,
    pub dislikes: i32,
    pub created_at: String,
    pub updated_at: String,
    pub replies: Option<Vec<CommentResponse>>,
} 