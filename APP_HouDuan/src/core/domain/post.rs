use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 帖子实体 - 核心业务对象
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub like_count: i64,
    pub view_count: i64,
    pub comment_count: i64,
    pub status: PostStatus,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub tags: Option<String>, // JSON array as string
    pub images: Option<String>, // JSON array as string
    pub code_snippet: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 帖子状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "post_status", rename_all = "lowercase")]
pub enum PostStatus {
    Draft,     // 草稿
    Published, // 已发布
    Archived,  // 已归档
    Banned,    // 已封禁
}

/// 创建帖子请求DTO
#[derive(Debug, Validate, Deserialize)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 1, max = 10000))]
    pub content: String,
    
    pub tags: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub code_snippet: Option<String>,
}

/// 更新帖子请求DTO
#[derive(Debug, Validate, Deserialize)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 10000))]
    pub content: Option<String>,
    
    pub tags: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub code_snippet: Option<String>,
}

/// 帖子搜索参数
#[derive(Debug, Clone)]
pub struct PostSearchParams {
    pub query: Option<String>,
    pub tag: Option<String>,
    pub status: Option<PostStatus>,
    pub author_id: Option<i64>,
    pub is_featured: Option<bool>,
    pub page: i64,
    pub page_size: i64,
}

/// 帖子详情响应DTO
#[derive(Debug, Serialize)]
pub struct PostDetail {
    #[serde(flatten)]
    pub post: Post,
    pub author: Option<super::resource::UserProfile>,
    pub is_liked: bool,
    pub is_bookmarked: bool,
}

/// 创建帖子数据
#[derive(Debug, Clone)]
pub struct CreatePostData {
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub status: PostStatus,
    pub tags: Option<String>,
    pub images: Option<String>,
    pub code_snippet: Option<String>,
}

impl Post {
    /// 检查帖子是否可以被查看
    pub fn is_viewable(&self) -> bool {
        matches!(self.status, PostStatus::Published)
    }
    
    /// 检查用户是否可以编辑此帖子
    pub fn can_edit(&self, user_id: i64, user_role: &str) -> bool {
        self.author_id == user_id || user_role == "admin" || user_role == "moderator"
    }
    
    /// 增加查看次数
    pub fn increment_view_count(&mut self) {
        self.view_count += 1;
    }
    
    /// 转换为详情响应
    pub fn to_detail(
        self,
        author: Option<super::resource::UserProfile>,
        is_liked: bool,
        is_bookmarked: bool,
    ) -> PostDetail {
        PostDetail {
            post: self,
            author,
            is_liked,
            is_bookmarked,
        }
    }
}

impl CreatePostRequest {
    /// 转换为创建数据
    pub fn to_create_data(self, author_id: i64) -> CreatePostData {
        CreatePostData {
            title: self.title,
            content: self.content,
            author_id,
            status: PostStatus::Published, // 帖子默认发布
            tags: self.tags.map(|t| serde_json::to_string(&t).unwrap_or_default()),
            images: self.images.map(|i| serde_json::to_string(&i).unwrap_or_default()),
            code_snippet: self.code_snippet,
        }
    }
}

impl Default for PostStatus {
    fn default() -> Self {
        PostStatus::Draft
    }
} 