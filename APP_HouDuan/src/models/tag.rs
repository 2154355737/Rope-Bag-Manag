use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub use_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagListResponse {
    pub list: Vec<Tag>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub search: Option<String>,
    pub sort_by: Option<String>, // "name", "use_count", "created_at"
    pub sort_order: Option<String>, // "asc", "desc"
}

// 帖子标签关联表模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTag {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTagRequest {
    pub post_id: i32,
    pub tag_ids: Vec<i32>,
} 