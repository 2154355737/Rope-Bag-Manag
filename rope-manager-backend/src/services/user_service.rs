use anyhow::Result;
use crate::models::{User, UpdateUserRequest, Package, Comment};
use crate::repositories::user_repo::UserRepository;
use crate::utils::password::PasswordUtils;

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

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        self.user_repo.find_by_id(user_id).await
    }

    pub async fn update_user(&self, user_id: i32, req: &UpdateUserRequest) -> Result<()> {
        let user = self.user_repo.find_by_id(user_id).await?;
        let user = user.ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        // 更新用户信息
        let mut updated_user = user.clone();
        if let Some(nickname) = &req.nickname {
            updated_user.nickname = Some(nickname.clone());
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
            updated_user.qq_number = Some(qq_number.clone());
        }
        if let Some(avatar_url) = &req.avatar_url {
            updated_user.avatar_url = Some(avatar_url.clone());
        }

        self.user_repo.update_user(&updated_user).await
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        self.user_repo.delete_user(user_id).await
    }

    // 新增方法：获取用户资源
    pub async fn get_user_resources(&self, user_id: i32) -> Result<Vec<Package>> {
        self.user_repo.get_user_packages(user_id).await
    }

    // 新增方法：获取用户评论
    pub async fn get_user_comments(&self, user_id: i32) -> Result<Vec<Comment>> {
        self.user_repo.get_user_comments(user_id).await
    }

    // 新增方法：修改密码
    pub async fn change_password(&self, user_id: i32, old_password: &str, new_password: &str) -> Result<()> {
        let user = self.user_repo.find_by_id(user_id).await?;
        let user = user.ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        let password_utils = PasswordUtils::new();

        // 验证旧密码
        if !password_utils.verify_password(old_password, &user.password_hash)? {
            return Err(anyhow::anyhow!("旧密码错误"));
        }

        // 加密新密码
        let new_password_hash = password_utils.hash_password(new_password)?;

        // 更新密码
        self.user_repo.update_password(user_id, &new_password_hash).await
    }

    // 新增方法：增加上传次数
    pub async fn increment_upload_count(&self, user_id: i32) -> Result<()> {
        self.user_repo.increment_upload_count(user_id).await
    }

    // 新增方法：增加下载次数
    pub async fn increment_download_count(&self, user_id: i32) -> Result<()> {
        self.user_repo.increment_download_count(user_id).await
    }

    // 新增方法：获取用户统计信息
    pub async fn get_user_stats(&self) -> Result<(i32, i32, i32)> {
        let users = self.user_repo.get_all_users().await?;
        let total_users = users.len() as i32;
        let active_users = users.iter().filter(|u| u.ban_status == crate::models::BanStatus::Normal).count() as i32;
        let total_uploads: i32 = users.iter().map(|u| u.upload_count).sum();
        let total_downloads: i32 = users.iter().map(|u| u.download_count).sum();
        
        Ok((total_users, active_users, total_uploads + total_downloads))
    }

    // 新增方法：创建用户
    pub async fn create_user(&self, username: &str, email: &str, password: &str, role: &str, star: i32, qq_number: Option<&str>, avatar_url: Option<&str>) -> Result<()> {
        let password_utils = PasswordUtils::new();
        let password_hash = password_utils.hash_password(password)?;

        let user = User {
            id: 0, // 数据库会自动生成ID
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            nickname: None,
            role: match role.to_lowercase().as_str() {
                "admin" => crate::models::UserRole::Admin,
                "moderator" => crate::models::UserRole::Moderator,
                "elder" => crate::models::UserRole::Elder,
                _ => crate::models::UserRole::User,
            },
            star,
            ban_status: crate::models::BanStatus::Normal,
            ban_reason: None,
            qq_number: qq_number.map(|s| s.to_string()),
            avatar_url: avatar_url.map(|s| s.to_string()),
            login_count: 0,
            upload_count: 0,
            download_count: 0,
            created_at: chrono::Utc::now(),
            last_login: None,
            is_admin: role.to_lowercase() == "admin",
        };

        self.user_repo.create_user(&user).await
    }

    // 新增方法：批量删除用户
    pub async fn batch_delete_users(&self, usernames: Vec<String>) -> Result<()> {
        for username in usernames {
            if let Ok(Some(user)) = self.user_repo.find_by_username(&username).await {
                if !user.is_admin {
                    self.user_repo.delete_user(user.id).await?;
                }
            }
        }
        Ok(())
    }
} 