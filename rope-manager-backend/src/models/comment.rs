use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub package_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentListResponse {
    pub list: Vec<Comment>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
} 