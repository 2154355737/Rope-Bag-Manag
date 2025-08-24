use std::sync::Arc;
use tracing::{info, warn, instrument};
use crate::core::domain::post::*;
use crate::core::ports::repositories::*;
use crate::shared::errors::*;
use crate::shared::types::pagination::PaginatedResult;

/// 帖子管理服务
pub struct PostService {
    post_repo: Arc<dyn PostRepository>,
    like_repo: Arc<dyn LikeRepository>,
    bookmark_repo: Arc<dyn BookmarkRepository>,
}

impl PostService {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        like_repo: Arc<dyn LikeRepository>,
        bookmark_repo: Arc<dyn BookmarkRepository>,
    ) -> Self {
        Self {
            post_repo,
            like_repo,
            bookmark_repo,
        }
    }
    
    /// 创建新帖子
    #[instrument(skip(self, request))]
    pub async fn create_post(
        &self,
        user_id: i64,
        request: CreatePostRequest,
    ) -> AppResult<PostDetail> {
        info!("用户 {} 创建帖子: {}", user_id, request.title);
        
        // 创建帖子数据
        let create_data = request.to_create_data(user_id);
        let post = self.post_repo.create(create_data).await?;
        
        info!("帖子创建成功: {} (ID: {})", post.title, post.id);
        
        // 构建详情响应
        Ok(post.to_detail(None, false, false))
    }
    
    /// 获取帖子详情
    #[instrument(skip(self))]
    pub async fn get_post_detail(
        &self,
        post_id: i64,
        user_id: Option<i64>,
    ) -> AppResult<PostDetail> {
        let post = self.post_repo.find_by_id(post_id).await?
            .ok_or_else(|| AppError::NotFound("帖子不存在".to_string()))?;
        
        // 增加查看次数
        if let Err(e) = self.post_repo.increment_view_count(post_id).await {
            warn!("增加查看次数失败: {}", e);
        }
        
        // 获取用户交互状态
        let (is_liked, is_bookmarked) = if let Some(uid) = user_id {
            let liked = self.like_repo.is_liked(uid, "post", post_id).await.unwrap_or(false);
            let bookmarked = self.bookmark_repo.is_bookmarked(uid, "post", post_id).await.unwrap_or(false);
            (liked, bookmarked)
        } else {
            (false, false)
        };
        
        Ok(post.to_detail(None, is_liked, is_bookmarked))
    }
    
    /// 搜索帖子
    #[instrument(skip(self))]
    pub async fn search_posts(
        &self,
        params: PostSearchParams,
    ) -> AppResult<PaginatedResult<PostDetail>> {
        info!("搜索帖子: query={:?}, tag={:?}, page={}, size={}", 
              params.query, params.tag, params.page, params.page_size);
        
        // 参数验证
        if params.page_size > 100 {
            return Err(AppError::Validation("每页最多100条记录".to_string()));
        }
        
        // 执行搜索
        let result = self.post_repo.search(params).await?;
        
        // 转换为详情列表
        let details: Vec<PostDetail> = result.data
            .into_iter()
            .map(|post| post.to_detail(None, false, false))
            .collect();
        
        Ok(PaginatedResult {
            data: details,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
        })
    }
    
    /// 更新帖子
    #[instrument(skip(self, request))]
    pub async fn update_post(
        &self,
        post_id: i64,
        user_id: i64,
        request: UpdatePostRequest,
    ) -> AppResult<PostDetail> {
        let post = self.post_repo.find_by_id(post_id).await?
            .ok_or_else(|| AppError::NotFound("帖子不存在".to_string()))?;
        
        // 权限检查
        if !post.can_edit(user_id, "user") {
            warn!("用户 {} 无权编辑帖子 {}", user_id, post_id);
            return Err(AppError::Forbidden("无权编辑此帖子".to_string()));
        }
        
        // 更新帖子
        let updated_post = self.post_repo.update(post_id, request).await?;
        
        info!("帖子更新成功: {} (ID: {})", updated_post.title, post_id);
        
        Ok(updated_post.to_detail(None, false, false))
    }
    
    /// 点赞帖子
    #[instrument(skip(self))]
    pub async fn like_post(&self, user_id: i64, post_id: i64) -> AppResult<i64> {
        // 检查帖子是否存在
        self.post_repo.find_by_id(post_id).await?
            .ok_or_else(|| AppError::NotFound("帖子不存在".to_string()))?;
        
        // 添加点赞
        self.like_repo.add_like(user_id, "post", post_id).await?;
        
        // 获取最新点赞数
        let like_count = self.like_repo.get_like_count("post", post_id).await?;
        
        // 更新帖子点赞数
        self.post_repo.update_like_count(post_id, like_count).await?;
        
        info!("用户 {} 点赞帖子 {}", user_id, post_id);
        Ok(like_count)
    }
    
    /// 取消点赞帖子
    #[instrument(skip(self))]
    pub async fn unlike_post(&self, user_id: i64, post_id: i64) -> AppResult<i64> {
        // 移除点赞
        self.like_repo.remove_like(user_id, "post", post_id).await?;
        
        // 获取最新点赞数
        let like_count = self.like_repo.get_like_count("post", post_id).await?;
        
        // 更新帖子点赞数
        self.post_repo.update_like_count(post_id, like_count).await?;
        
        info!("用户 {} 取消点赞帖子 {}", user_id, post_id);
        Ok(like_count)
    }
    
    /// 收藏帖子
    #[instrument(skip(self))]
    pub async fn bookmark_post(&self, user_id: i64, post_id: i64) -> AppResult<()> {
        // 检查帖子是否存在
        self.post_repo.find_by_id(post_id).await?
            .ok_or_else(|| AppError::NotFound("帖子不存在".to_string()))?;
        
        // 添加收藏
        self.bookmark_repo.add_bookmark(user_id, "post", post_id).await?;
        
        info!("用户 {} 收藏帖子 {}", user_id, post_id);
        Ok(())
    }
    
    /// 取消收藏帖子
    #[instrument(skip(self))]
    pub async fn unbookmark_post(&self, user_id: i64, post_id: i64) -> AppResult<()> {
        // 移除收藏
        self.bookmark_repo.remove_bookmark(user_id, "post", post_id).await?;
        
        info!("用户 {} 取消收藏帖子 {}", user_id, post_id);
        Ok(())
    }
    
    /// 删除帖子
    #[instrument(skip(self))]
    pub async fn delete_post(&self, post_id: i64, user_id: i64) -> AppResult<()> {
        let post = self.post_repo.find_by_id(post_id).await?
            .ok_or_else(|| AppError::NotFound("帖子不存在".to_string()))?;
        
        // 权限检查
        if !post.can_edit(user_id, "user") {
            warn!("用户 {} 无权删除帖子 {}", user_id, post_id);
            return Err(AppError::Forbidden("无权删除此帖子".to_string()));
        }
        
        // 删除帖子
        self.post_repo.delete(post_id).await?;
        
        info!("帖子删除成功: ID {}", post_id);
        Ok(())
    }
    
    /// 获取用户的帖子列表
    #[instrument(skip(self))]
    pub async fn get_user_posts(
        &self,
        user_id: i64,
        page: i64,
        page_size: i64,
    ) -> AppResult<PaginatedResult<PostDetail>> {
        let result = self.post_repo.find_by_author(user_id, page, page_size).await?;
        
        let details: Vec<PostDetail> = result.data
            .into_iter()
            .map(|post| post.to_detail(None, false, false))
            .collect();
        
        Ok(PaginatedResult {
            data: details,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
        })
    }
    
    /// 健康检查
    pub async fn health_check(&self) -> bool {
        // 这里可以添加服务健康检查逻辑
        true
    }
} 