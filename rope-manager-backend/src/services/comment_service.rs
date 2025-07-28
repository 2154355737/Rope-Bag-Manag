use anyhow::Result;
use chrono::Utc;

use crate::models::Comment;
use crate::repositories::comment_repo::CommentRepository;
use crate::repositories::user_repo::UserRepository;
use crate::services::forbidden_word_service::ForbiddenWordService;

#[derive(Clone)]
pub struct CommentService {
    comment_repo: CommentRepository,
    user_repo: UserRepository,
    forbidden_service: Option<ForbiddenWordService>,
}

impl CommentService {
    pub fn new(comment_repo: CommentRepository, user_repo: UserRepository) -> Self {
        Self { comment_repo, user_repo, forbidden_service: None }
    }

    pub fn with_forbidden_service(mut self, service: ForbiddenWordService) -> Self {
        self.forbidden_service = Some(service);
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
            author_name: None,
            author_role: None,
            author_avatar: None,
            author_qq: None,
            dislikes: 0,
            created_at: now,
            updated_at: now,
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
            // 点赞
            comment.likes += 1;
            self.comment_repo.add_user_like(comment_id, user_id).await?;
        } else if !like && has_liked {
            // 取消点赞
            comment.likes = comment.likes.saturating_sub(1);
            self.comment_repo.remove_user_like(comment_id, user_id).await?;
        }

        // 保存评论
        self.comment_repo.update_comment(&comment).await?;

        Ok(comment.likes)
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
} 