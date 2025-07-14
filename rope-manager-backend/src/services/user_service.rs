use anyhow::Result;
use crate::models::{User, UpdateUserRequest};
use crate::repositories::user_repo::UserRepository;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.user_repo.get_all_users().await
    }

    pub async fn get_user(&self, user_id: i32) -> Result<Option<User>> {
        self.user_repo.find_by_id(user_id).await
    }

    pub async fn update_user(&self, user_id: i32, req: &UpdateUserRequest) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        let user = user.ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        // 更新用户信息
        let mut updated_user = user.clone();
        if let Some(nickname) = &req.nickname {
            updated_user.nickname = nickname.clone();
        }
        if let Some(star) = req.star {
            updated_user.star = star;
        }
        if let Some(ban_status) = &req.ban_status {
            updated_user.ban_status = ban_status.clone();
        }
        if let Some(role) = &req.role {
            updated_user.role = role.clone();
        }
        if let Some(qq_number) = &req.qq_number {
            updated_user.qq_number = qq_number.clone();
        }
        if let Some(avatar_url) = &req.avatar_url {
            updated_user.avatar_url = avatar_url.clone();
        }

        self.user_repo.update_user(&updated_user).await
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        self.user_repo.delete_user(user_id).await
    }
} 