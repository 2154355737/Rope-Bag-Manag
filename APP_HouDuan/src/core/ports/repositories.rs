// 仓储接口定义
// 定义所有仓储的trait接口

use async_trait::async_trait;
use crate::core::domain::*;
use crate::shared::errors::AppResult;
use crate::shared::types::pagination::PaginatedResult;
use serde::Serialize;

/// 资源仓储接口
#[async_trait]
pub trait ResourceRepository: Send + Sync {
    /// 创建资源
    async fn create(&self, data: resource::CreateResourceData) -> AppResult<resource::Resource>;
    
    /// 根据ID查找资源
    async fn find_by_id(&self, id: i64) -> AppResult<Option<resource::Resource>>;
    
    /// 根据名称查找资源
    async fn find_by_name(&self, name: &str) -> AppResult<Option<resource::Resource>>;
    
    /// 检查资源名是否存在
    async fn exists_by_name(&self, name: &str) -> AppResult<bool>;
    
    /// 更新资源
    async fn update(&self, id: i64, data: resource::UpdateResourceRequest) -> AppResult<resource::Resource>;
    
    /// 更新资源状态
    async fn update_status(&self, id: i64, status: resource::ResourceStatus) -> AppResult<resource::Resource>;
    
    /// 搜索资源
    async fn search(&self, params: resource::ResourceSearchParams) -> AppResult<PaginatedResult<resource::Resource>>;
    
    /// 获取用户的资源列表
    async fn find_by_author(&self, author_id: i64, page: i64, page_size: i64) -> AppResult<PaginatedResult<resource::Resource>>;
    
    /// 获取热门资源
    async fn find_popular(&self, limit: i64) -> AppResult<Vec<resource::Resource>>;
    
    /// 增加下载次数
    async fn increment_download_count(&self, id: i64) -> AppResult<()>;
    
    /// 增加查看次数
    async fn increment_view_count(&self, id: i64) -> AppResult<()>;
    
    /// 更新点赞数
    async fn update_like_count(&self, id: i64, count: i64) -> AppResult<()>;
    
    /// 删除资源
    async fn delete(&self, id: i64) -> AppResult<()>;
}

/// 帖子仓储接口
#[async_trait]
pub trait PostRepository: Send + Sync {
    /// 创建帖子
    async fn create(&self, data: post::CreatePostData) -> AppResult<post::Post>;
    
    /// 根据ID查找帖子
    async fn find_by_id(&self, id: i64) -> AppResult<Option<post::Post>>;
    
    /// 更新帖子
    async fn update(&self, id: i64, data: post::UpdatePostRequest) -> AppResult<post::Post>;
    
    /// 更新帖子状态
    async fn update_status(&self, id: i64, status: post::PostStatus) -> AppResult<post::Post>;
    
    /// 搜索帖子
    async fn search(&self, params: post::PostSearchParams) -> AppResult<PaginatedResult<post::Post>>;
    
    /// 获取用户的帖子列表
    async fn find_by_author(&self, author_id: i64, page: i64, page_size: i64) -> AppResult<PaginatedResult<post::Post>>;
    
    /// 获取热门帖子
    async fn find_popular(&self, limit: i64) -> AppResult<Vec<post::Post>>;
    
    /// 增加查看次数
    async fn increment_view_count(&self, id: i64) -> AppResult<()>;
    
    /// 更新点赞数
    async fn update_like_count(&self, id: i64, count: i64) -> AppResult<()>;
    
    /// 删除帖子
    async fn delete(&self, id: i64) -> AppResult<()>;
}

/// 点赞仓储接口
#[async_trait]
pub trait LikeRepository: Send + Sync {
    /// 添加点赞
    async fn add_like(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()>;
    
    /// 移除点赞
    async fn remove_like(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()>;
    
    /// 检查是否已点赞
    async fn is_liked(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<bool>;
    
    /// 获取目标的点赞数
    async fn get_like_count(&self, target_type: &str, target_id: i64) -> AppResult<i64>;
    
    /// 获取用户的点赞列表
    async fn find_user_likes(&self, user_id: i64, target_type: Option<&str>, page: i64, page_size: i64) -> AppResult<PaginatedResult<UserLike>>;
}

/// 收藏仓储接口
#[async_trait]
pub trait BookmarkRepository: Send + Sync {
    /// 添加收藏
    async fn add_bookmark(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()>;
    
    /// 移除收藏
    async fn remove_bookmark(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()>;
    
    /// 检查是否已收藏
    async fn is_bookmarked(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<bool>;
    
    /// 获取用户的收藏列表
    async fn find_user_bookmarks(&self, user_id: i64, target_type: Option<&str>, page: i64, page_size: i64) -> AppResult<PaginatedResult<UserBookmark>>;
}

/// 用户点赞记录
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserLike {
    pub id: i64,
    pub user_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 用户收藏记录
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserBookmark {
    pub id: i64,
    pub user_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
} 