use std::sync::Arc;
use tracing::{info, warn, instrument};
use crate::core::domain::resource::*;
use crate::core::ports::repositories::*;
use crate::shared::errors::*;
use crate::shared::types::pagination::PaginatedResult;

/// 资源管理服务
pub struct ResourceService {
    resource_repo: Arc<dyn ResourceRepository>,
    like_repo: Arc<dyn LikeRepository>,
    bookmark_repo: Arc<dyn BookmarkRepository>,
}

impl ResourceService {
    pub fn new(
        resource_repo: Arc<dyn ResourceRepository>,
        like_repo: Arc<dyn LikeRepository>,
        bookmark_repo: Arc<dyn BookmarkRepository>,
    ) -> Self {
        Self {
            resource_repo,
            like_repo,
            bookmark_repo,
        }
    }
    
    /// 创建新资源
    #[instrument(skip(self, request))]
    pub async fn create_resource(
        &self,
        user_id: i64,
        request: CreateResourceRequest,
        file_path: String,
    ) -> AppResult<ResourceDetail> {
        info!("用户 {} 创建资源: {}", user_id, request.title);
        
        // 验证资源名唯一性
        if self.resource_repo.exists_by_name(&request.name).await? {
            warn!("资源名 {} 已存在", request.name);
            return Err(AppError::Business(format!("资源名 {} 已存在", request.name)));
        }
        
        // 创建资源数据
        let create_data = request.to_create_data(user_id, file_path);
        let resource = self.resource_repo.create(create_data).await?;
        
        info!("资源创建成功: {} (ID: {})", resource.title, resource.id);
        
        // 构建详情响应
        Ok(resource.to_detail(None, None, false, false, None))
    }
    
    /// 获取资源详情
    #[instrument(skip(self))]
    pub async fn get_resource_detail(
        &self,
        resource_id: i64,
        user_id: Option<i64>,
    ) -> AppResult<ResourceDetail> {
        let resource = self.resource_repo.find_by_id(resource_id).await?
            .ok_or_else(|| AppError::NotFound("资源不存在".to_string()))?;
        
        // 增加查看次数
        if let Err(e) = self.resource_repo.increment_view_count(resource_id).await {
            warn!("增加查看次数失败: {}", e);
        }
        
        // 获取用户交互状态
        let (is_liked, is_bookmarked) = if let Some(uid) = user_id {
            let liked = self.like_repo.is_liked(uid, "package", resource_id).await.unwrap_or(false);
            let bookmarked = self.bookmark_repo.is_bookmarked(uid, "package", resource_id).await.unwrap_or(false);
            (liked, bookmarked)
        } else {
            (false, false)
        };
        
        // 构建下载URL（如果资源可下载）
        let download_url = if resource.is_downloadable() {
            Some(format!("/api/v1/resources/{}/download", resource_id))
        } else {
            None
        };
        
        Ok(resource.to_detail(None, None, is_liked, is_bookmarked, download_url))
    }
    
    /// 搜索资源
    #[instrument(skip(self))]
    pub async fn search_resources(
        &self,
        params: ResourceSearchParams,
    ) -> AppResult<PaginatedResult<ResourceDetail>> {
        info!("搜索资源: query={:?}, category={:?}, page={}, size={}", 
              params.query, params.category_id, params.page, params.page_size);
        
        // 参数验证
        if params.page_size > 100 {
            return Err(AppError::Validation("每页最多100条记录".to_string()));
        }
        
        // 执行搜索
        let result = self.resource_repo.search(params).await?;
        
        // 转换为详情列表
        let details: Vec<ResourceDetail> = result.data
            .into_iter()
            .map(|resource| resource.to_detail(None, None, false, false, None))
            .collect();
        
        Ok(PaginatedResult {
            data: details,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
        })
    }
    
    /// 更新资源
    #[instrument(skip(self, request))]
    pub async fn update_resource(
        &self,
        resource_id: i64,
        user_id: i64,
        request: UpdateResourceRequest,
    ) -> AppResult<ResourceDetail> {
        let resource = self.resource_repo.find_by_id(resource_id).await?
            .ok_or_else(|| AppError::NotFound("资源不存在".to_string()))?;
        
        // 权限检查
        if !resource.can_edit(user_id, "user") {
            warn!("用户 {} 无权编辑资源 {}", user_id, resource_id);
            return Err(AppError::Forbidden("无权编辑此资源".to_string()));
        }
        
        // 更新资源
        let updated_resource = self.resource_repo.update(resource_id, request).await?;
        
        info!("资源更新成功: {} (ID: {})", updated_resource.title, resource_id);
        
        Ok(updated_resource.to_detail(None, None, false, false, None))
    }
    
    /// 点赞资源
    #[instrument(skip(self))]
    pub async fn like_resource(&self, user_id: i64, resource_id: i64) -> AppResult<i64> {
        // 检查资源是否存在
        self.resource_repo.find_by_id(resource_id).await?
            .ok_or_else(|| AppError::NotFound("资源不存在".to_string()))?;
        
        // 添加点赞
        self.like_repo.add_like(user_id, "package", resource_id).await?;
        
        // 获取最新点赞数
        let like_count = self.like_repo.get_like_count("package", resource_id).await?;
        
        // 更新资源点赞数
        self.resource_repo.update_like_count(resource_id, like_count).await?;
        
        info!("用户 {} 点赞资源 {}", user_id, resource_id);
        Ok(like_count)
    }
    
    /// 取消点赞资源
    #[instrument(skip(self))]
    pub async fn unlike_resource(&self, user_id: i64, resource_id: i64) -> AppResult<i64> {
        // 移除点赞
        self.like_repo.remove_like(user_id, "package", resource_id).await?;
        
        // 获取最新点赞数
        let like_count = self.like_repo.get_like_count("package", resource_id).await?;
        
        // 更新资源点赞数
        self.resource_repo.update_like_count(resource_id, like_count).await?;
        
        info!("用户 {} 取消点赞资源 {}", user_id, resource_id);
        Ok(like_count)
    }
    
    /// 收藏资源
    #[instrument(skip(self))]
    pub async fn bookmark_resource(&self, user_id: i64, resource_id: i64) -> AppResult<()> {
        // 检查资源是否存在
        self.resource_repo.find_by_id(resource_id).await?
            .ok_or_else(|| AppError::NotFound("资源不存在".to_string()))?;
        
        // 添加收藏
        self.bookmark_repo.add_bookmark(user_id, "package", resource_id).await?;
        
        info!("用户 {} 收藏资源 {}", user_id, resource_id);
        Ok(())
    }
    
    /// 取消收藏资源
    #[instrument(skip(self))]
    pub async fn unbookmark_resource(&self, user_id: i64, resource_id: i64) -> AppResult<()> {
        // 移除收藏
        self.bookmark_repo.remove_bookmark(user_id, "package", resource_id).await?;
        
        info!("用户 {} 取消收藏资源 {}", user_id, resource_id);
        Ok(())
    }
    
    /// 删除资源
    #[instrument(skip(self))]
    pub async fn delete_resource(&self, resource_id: i64, user_id: i64) -> AppResult<()> {
        let resource = self.resource_repo.find_by_id(resource_id).await?
            .ok_or_else(|| AppError::NotFound("资源不存在".to_string()))?;
        
        // 权限检查
        if !resource.can_edit(user_id, "user") {
            warn!("用户 {} 无权删除资源 {}", user_id, resource_id);
            return Err(AppError::Forbidden("无权删除此资源".to_string()));
        }
        
        // 删除资源
        self.resource_repo.delete(resource_id).await?;
        
        info!("资源删除成功: ID {}", resource_id);
        Ok(())
    }
    
    /// 获取用户的资源列表
    #[instrument(skip(self))]
    pub async fn get_user_resources(
        &self,
        user_id: i64,
        page: i64,
        page_size: i64,
    ) -> AppResult<PaginatedResult<ResourceDetail>> {
        let result = self.resource_repo.find_by_author(user_id, page, page_size).await?;
        
        let details: Vec<ResourceDetail> = result.data
            .into_iter()
            .map(|resource| resource.to_detail(None, None, false, false, None))
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