use anyhow::Result;
use chrono::Utc;

use crate::models::Comment;
use crate::repositories::comment_repo::CommentRepository;
use crate::repositories::user_repo::UserRepository;
use crate::repositories::package_repo::PackageRepository;
use crate::repositories::user_action_repo::UserActionRepository;
use crate::services::forbidden_word_service::ForbiddenWordService;
use crate::services::notification_service::NotificationService;

#[derive(Clone)]
pub struct CommentService {
    comment_repo: CommentRepository,
    user_repo: UserRepository,
    package_repo: Option<PackageRepository>,
    user_action_repo: Option<UserActionRepository>,
    forbidden_service: Option<ForbiddenWordService>,
    notification_service: Option<NotificationService>,
}

impl CommentService {
    pub fn new(comment_repo: CommentRepository, user_repo: UserRepository) -> Self {
        Self { comment_repo, user_repo, package_repo: None, user_action_repo: None, forbidden_service: None, notification_service: None }
    }

    pub fn with_package_repo(mut self, package_repo: PackageRepository) -> Self {
        self.package_repo = Some(package_repo);
        self
    }

    pub fn with_user_action_repo(mut self, user_action_repo: UserActionRepository) -> Self {
        self.user_action_repo = Some(user_action_repo);
        self
    }

    pub fn with_forbidden_service(mut self, service: ForbiddenWordService) -> Self {
        self.forbidden_service = Some(service);
        self
    }

    pub fn with_notification_service(mut self, service: NotificationService) -> Self {
        self.notification_service = Some(service);
        self
    }

    // 获取所有评论（管理员接口）
    pub async fn get_all_comments(
        &self,
        page: i32,
        size: i32,
        status: Option<&str>,
        target_type: Option<&str>,
        target_id: Option<i32>,
        user_id: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<Comment>, i64)> {
        self.comment_repo.get_all_comments(
            page,
            size,
            status,
            target_type,
            target_id,
            user_id,
            start_date,
            end_date,
            search,
        ).await
    }

    // 获取单个评论详情
    pub async fn get_comment_by_id(&self, comment_id: i32) -> Result<Option<Comment>> {
        self.comment_repo.get_comment_by_id(comment_id).await
    }

    // 创建评论
    pub async fn create_comment(
        &self,
        user_id: i32,
        target_type: String,
        target_id: i32,
        content: String,
        parent_id: Option<i32>,
    ) -> Result<Comment> {
        // 敏感词检测
        if let Some(f_service) = &self.forbidden_service {
            if f_service.contains_forbidden_word(&content).await.unwrap_or(false) {
                return Err(anyhow::anyhow!("评论内容包含违禁词"));
            }
        }

        let now = Utc::now();
        let mut comment = Comment {
            id: 0, // 由数据库生成
            user_id,
            target_type,
            target_id,
            content,
            status: "Active".to_string(),
            parent_id,
            likes: 0,
            pinned: false,
            author_name: None,
            username: None,
            author_role: None,
            author_avatar: None,
            author_qq: None,
            dislikes: 0,
            created_at: now,
            updated_at: now,
            target_title: None,
        };

        // 保存评论
        let comment_id = self.comment_repo.create_comment(&comment).await?;

        // 返回创建的评论（包括ID）
        comment.id = comment_id;

        // 获取用户信息并填充
        if let Some(user) = self.user_repo.find_by_id(user_id).await? {
            comment.author_name = Some(user.nickname.clone().unwrap_or(user.username));
            comment.author_role = Some(user.role.to_string());
            comment.author_avatar = user.avatar_url.clone();
            comment.author_qq = user.qq_number.clone();
        }

        // 给资源作者发送站内通知（仅当评论目标为 Package）
        if comment.target_type.eq_ignore_ascii_case("Package") {
            if let (Some(pkg_repo), Some(notify)) = (&self.package_repo, &self.notification_service) {
                if let Ok(Some(pkg)) = pkg_repo.find_by_id(comment.target_id).await {
                    if let Ok(Some(author_user)) = self.user_repo.find_by_username(&pkg.author).await {
                        if author_user.id != user_id {
                            let link = format!("/resource/{}", pkg.id);
                            let title = "资源收到新评论";
                            let content = format!("您的资源《{}》有一条新评论", pkg.name);
                            if let Err(e) = notify.notify(author_user.id, title, &content, Some(&link), Some("CommentReceived"), Some("Package"), Some(pkg.id)).await {
                                log::error!("发送评论通知失败: {}", e);
                            }
                        }
                    }
                }
            }
        }

        Ok(comment)
    }

    // 更新评论
    pub async fn update_comment(
        &self,
        comment_id: i32,
        content: Option<String>,
        status: Option<String>,
        is_admin: bool,
    ) -> Result<Comment> {
        // 先获取原有评论
        let mut comment = match self.comment_repo.get_comment_by_id(comment_id).await? {
            Some(c) => c,
            None => return Err(anyhow::anyhow!("评论不存在")),
        };

        // 更新内容
        if let Some(new_content) = content {
            comment.content = new_content;
        }

        // 只有管理员才能更新状态
        if is_admin {
            if let Some(new_status) = status {
                comment.status = new_status;
            }
        }

        // 更新时间
        comment.updated_at = Utc::now();

        // 保存评论
        self.comment_repo.update_comment(&comment).await?;

        Ok(comment)
    }

    // 删除评论
    pub async fn delete_comment(&self, comment_id: i32, is_admin: bool) -> Result<()> {

        if is_admin {
            // 管理员执行物理删除
            self.comment_repo.delete_comment(comment_id).await?;
            return Ok(());
        }

        // 普通用户软删除
        let mut comment = match self.comment_repo.get_comment_by_id(comment_id).await? {
            Some(c) => c,
            None => return Err(anyhow::anyhow!("评论不存在")),
        };
        comment.status = "Deleted".to_string();
        comment.updated_at = Utc::now();
        self.comment_repo.update_comment(&comment).await?;
        Ok(())
    }

    // 获取评论回复
    pub async fn get_comment_replies(&self, comment_id: i32) -> Result<Vec<Comment>> {
        self.comment_repo.get_comment_replies(comment_id).await
    }

    // 批量更新状态
    pub async fn batch_update_status(&self, comment_ids: Vec<i32>, status: String) -> Result<()> {
        // 验证状态值
        if !["Active", "Hidden", "Deleted"].contains(&status.as_str()) {
            return Err(anyhow::anyhow!("无效的状态值"));
        }

        for comment_id in comment_ids {
            let mut comment = match self.comment_repo.get_comment_by_id(comment_id).await? {
                Some(c) => c,
                None => continue, // 评论不存在，跳过
            };

            comment.status = status.clone();
            comment.updated_at = Utc::now();
            self.comment_repo.update_comment(&comment).await?;
        }

        Ok(())
    }

    // 批量删除评论
    pub async fn batch_delete_comments(&self, comment_ids: Vec<i32>) -> Result<()> {
        for comment_id in comment_ids {
            // 使用软删除
            let mut comment = match self.comment_repo.get_comment_by_id(comment_id).await? {
                Some(c) => c,
                None => continue, // 评论不存在，跳过
            };

            comment.status = "Deleted".to_string();
            comment.updated_at = Utc::now();
            self.comment_repo.update_comment(&comment).await?;
        }

        Ok(())
    }

    // 更新评论点赞数
    pub async fn update_comment_likes(&self, comment_id: i32, user_id: i32, like: bool) -> Result<i32> {
        // 获取原有评论
        let mut comment = match self.comment_repo.get_comment_by_id(comment_id).await? {
            Some(c) => c,
            None => return Err(anyhow::anyhow!("评论不存在")),
        };

        // 检查用户是否已经点赞
        let has_liked = self.comment_repo.has_user_liked(comment_id, user_id).await?;

        if like && !has_liked {
            // 点赞（触发器会自动更新likes字段）
            self.comment_repo.add_user_like(comment_id, user_id).await?;
            
            // 记录用户行为
            if let Some(user_action_repo) = &self.user_action_repo {
                let req = crate::models::CreateUserActionRequest {
                    user_id: Some(user_id),
                    action_type: "Like".to_string(),
                    target_type: Some("Comment".to_string()),
                    target_id: Some(comment_id),
                    details: None,
                    ip_address: None,
                    user_agent: None,
                };
                let _ = user_action_repo.create_user_action(&req).await;
            }
        } else if !like && has_liked {
            // 取消点赞（触发器会自动更新likes字段）
            self.comment_repo.remove_user_like(comment_id, user_id).await?;
            
            // 记录用户行为
            if let Some(user_action_repo) = &self.user_action_repo {
                let req = crate::models::CreateUserActionRequest {
                    user_id: Some(user_id),
                    action_type: "Unlike".to_string(),
                    target_type: Some("Comment".to_string()),
                    target_id: Some(comment_id),
                    details: None,
                    ip_address: None,
                    user_agent: None,
                };
                let _ = user_action_repo.create_user_action(&req).await;
            }
        }

        // 获取更新后的点赞数
        let updated_comment = self.comment_repo.get_comment_by_id(comment_id).await?
            .ok_or_else(|| anyhow::anyhow!("评论不存在"))?;

        Ok(updated_comment.likes)
    }

    // 检查用户是否已点赞评论
    pub async fn check_comment_like_status(&self, comment_id: i32, user_id: i32) -> Result<bool> {
        let is_liked = self.comment_repo.has_user_liked(comment_id, user_id).await?;
        Ok(is_liked)
    }

    // 更新评论点踩数
    pub async fn update_comment_dislikes(&self, comment_id: i32, user_id: i32, dislike: bool) -> Result<i32> {
        // 获取原有评论
        let mut comment = match self.comment_repo.get_comment_by_id(comment_id).await? {
            Some(c) => c,
            None => return Err(anyhow::anyhow!("评论不存在")),
        };

        // 检查用户是否已经点踩
        let has_disliked = self.comment_repo.has_user_disliked(comment_id, user_id).await?;

        if dislike && !has_disliked {
            // 点踩
            comment.dislikes += 1;
            self.comment_repo.add_user_dislike(comment_id, user_id).await?;
        } else if !dislike && has_disliked {
            // 取消点踩
            comment.dislikes = comment.dislikes.saturating_sub(1);
            self.comment_repo.remove_user_dislike(comment_id, user_id).await?;
        }

        // 保存评论
        self.comment_repo.update_comment(&comment).await?;

        Ok(comment.dislikes)
    }

    // 获取资源评论
    pub async fn get_package_comments(&self, package_id: i32, page: i32, size: i32) -> Result<(Vec<Comment>, i64)> {
        self.comment_repo.get_comments_by_target("Package", package_id, page, size).await
    }

    // 获取用户评论
    pub async fn get_user_comments(&self, user_id: i32, page: i32, size: i32) -> Result<(Vec<Comment>, i64)> {
        self.comment_repo.get_user_comments(user_id, page, size).await
    }

    // 置顶评论（仅资源作者、管理员和元老可用）
    pub async fn pin_comment(&self, comment_id: i32, user_id: i32, pinned: bool) -> Result<Comment> {
        // 获取评论信息
        let comment = self.comment_repo.get_comment_by_id(comment_id).await?
            .ok_or_else(|| anyhow::anyhow!("评论不存在"))?;
        
        // 获取用户信息
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        
        // 权限检查：管理员、元老或资源作者可以置顶评论
        let is_admin = matches!(user.role, crate::models::UserRole::Admin | crate::models::UserRole::Elder);
        let mut is_resource_author = false;
        
        if comment.target_type == "Package" {
            // 获取资源信息检查作者
            if let Some(package_repo) = &self.package_repo {
                if let Ok(Some(package)) = package_repo.find_by_id(comment.target_id).await {
                    is_resource_author = package.author == user.username;
                }
            }
        }
        
        if !is_admin && !is_resource_author {
            return Err(anyhow::anyhow!("只有管理员、元老或资源作者可以置顶评论"));
        }
        
        // 设置置顶状态
        self.comment_repo.set_comment_pinned(
            comment_id, 
            &comment.target_type, 
            comment.target_id, 
            pinned
        ).await?;
        
        // 返回更新后的评论
        let updated_comment = self.comment_repo.get_comment_by_id(comment_id).await?
            .ok_or_else(|| anyhow::anyhow!("获取更新后的评论失败"))?;
        
        Ok(updated_comment)
    }
} 