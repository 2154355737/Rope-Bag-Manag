use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub author_name: Option<String>,
    pub category_id: Option<i32>,
    pub status: PostStatus,
    pub view_count: i32,
    pub like_count: i32,
    pub comment_count: i32,
    pub is_pinned: bool,
    pub is_featured: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostStatus {
    Draft,      // 草稿
    Published,  // 已发布
    Archived,   // 已归档
    Deleted,    // 已删除
}

impl Default for PostStatus {
    fn default() -> Self {
        PostStatus::Draft
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub status: Option<PostStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub status: Option<PostStatus>,
    pub is_pinned: Option<bool>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostListResponse {
    pub list: Vec<Post>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category_id: Option<i32>,
    pub author_id: Option<i32>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_pinned: Option<bool>,
    pub is_featured: Option<bool>,
} 