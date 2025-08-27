use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 自定义序列化：将 Option<Vec<String>> 的 None 序列化为空数组
fn serialize_option_vec<S>(option: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match option {
        Some(v) => v.serialize(serializer),
        None => Vec::<String>::new().serialize(serializer),
    }
}

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
    // 审核相关
    pub review_status: Option<String>,
    pub review_comment: Option<String>,
    pub reviewer_id: Option<i32>,
    pub reviewed_at: Option<DateTime<Utc>>,
    // 新增字段 - 支持前端发布页面
    #[serde(serialize_with = "serialize_option_vec")]
    pub images: Option<Vec<String>>,    // 图片URLs
    pub code_snippet: Option<String>,   // 代码片段
    #[serde(serialize_with = "serialize_option_vec")]
    pub tags: Option<Vec<String>>,      // 标签列表
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
    // 新增字段 - 支持前端发布页面
    pub images: Option<Vec<String>>,    // 图片URLs
    pub code_snippet: Option<String>,   // 代码片段
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
    // 新增字段 - 支持前端发布页面
    pub images: Option<Vec<String>>,    // 图片URLs
    pub code_snippet: Option<String>,   // 代码片段
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