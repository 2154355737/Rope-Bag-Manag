use anyhow::Result;
use crate::models::Comment;
use crate::repositories::comment_repo::CommentRepository;
use chrono::Utc;

#[derive(Clone)]
pub struct CommunityService {
    comment_repo: CommentRepository,
}

impl CommunityService {
    pub fn new(comment_repo: CommentRepository) -> Self {
        Self { comment_repo }
    }

    pub async fn get_comments(&self, package_id: i32) -> Result<Vec<Comment>> {
        self.comment_repo.get_comments_by_package(package_id).await
    }

    pub async fn create_comment(&self, package_id: i32, content: &str) -> Result<Comment> {
        // 这里应该从JWT token中获取用户ID，暂时使用默认值
        let user_id = 1; // TODO: 从认证中获取

        let comment = Comment {
            id: 0, // 数据库会自动生成
            user_id,
            package_id,
            content: content.to_string(),
            created_at: Utc::now(),
        };

        self.comment_repo.create_comment(&comment).await?;
        Ok(comment)
    }
} 