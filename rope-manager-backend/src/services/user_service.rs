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

    // 新增：按用户名查询用户
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        self.user_repo.find_by_username(username).await
    }

    // 新增：按邮箱查询用户
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        self.user_repo.find_by_email(email).await
    }

    pub async fn update_user(&self, user_id: i32, req: &UpdateUserRequest) -> Result<()> {
        let user = self.user_repo.find_by_id(user_id).await?;
        let user = user.ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        let mut updated_user = user.clone();
        if let Some(email) = &req.email { updated_user.email = email.clone(); }
        if let Some(nickname) = &req.nickname { updated_user.nickname = Some(nickname.clone()); }
        if let Some(star) = req.star { updated_user.star = star; }
        if let Some(ban_status) = &req.ban_status { updated_user.ban_status = ban_status.clone(); }
        if let Some(ban_reason) = &req.ban_reason { updated_user.ban_reason = Some(ban_reason.clone()); }
        if let Some(role) = &req.role { updated_user.role = role.clone(); }
        if let Some(qq_number) = &req.qq_number { updated_user.qq_number = Some(qq_number.clone()); }
        if let Some(avatar_url) = &req.avatar_url { updated_user.avatar_url = Some(avatar_url.clone()); }
        self.user_repo.update_user(&updated_user).await
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<()> {
        self.user_repo.delete_user(user_id).await
    }

    pub async fn get_user_resources(&self, user_id: i32) -> Result<Vec<Package>> {
        self.user_repo.get_user_packages(user_id).await
    }

    pub async fn get_user_comments(&self, user_id: i32) -> Result<Vec<Comment>> {
        self.user_repo.get_user_comments(user_id).await
    }

    pub async fn change_password(&self, user_id: i32, old_password: &str, new_password: &str) -> Result<()> {
        let user = self.user_repo.find_by_id(user_id).await?;
        let user = user.ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        let password_utils = PasswordUtils::new();
        if !password_utils.verify_password(old_password, &user.password_hash)? { return Err(anyhow::anyhow!("旧密码错误")); }
        let new_password_hash = password_utils.hash_password(new_password)?;
        self.user_repo.update_password(user_id, &new_password_hash).await
    }

    pub async fn increment_upload_count(&self, user_id: i32) -> Result<()> { self.user_repo.increment_upload_count(user_id).await }
    pub async fn increment_download_count(&self, user_id: i32) -> Result<()> { self.user_repo.increment_download_count(user_id).await }

    pub async fn get_user_stats(&self) -> Result<(i32, i32, i32)> {
        let users = self.user_repo.get_all_users().await?;
        let total_users = users.len() as i32;
        let active_users = users.iter().filter(|u| u.ban_status == crate::models::BanStatus::Normal).count() as i32;
        let total_uploads: i32 = users.iter().map(|u| u.upload_count).sum();
        let total_downloads: i32 = users.iter().map(|u| u.download_count).sum();
        Ok((total_users, active_users, total_uploads + total_downloads))
    }

    pub async fn create_user(&self, username: &str, email: &str, password: &str, role: &str, star: i32, qq_number: Option<&str>, avatar_url: Option<&str>) -> Result<()> {
        let password_utils = PasswordUtils::new();
        let password_hash = password_utils.hash_password(password)?;
        let user = User {
            id: 0,
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            nickname: None,
            bio: None,
            location: None,
            website: None,
            skills: None,
            role: match role.to_lowercase().as_str() { "admin" => crate::models::UserRole::Admin, "moderator" => crate::models::UserRole::Moderator, "elder" => crate::models::UserRole::Elder, _ => crate::models::UserRole::User },
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
        self.user_repo.create_user(&user).await.map(|_| ())
    }

    pub async fn batch_delete_users(&self, usernames: Vec<String>) -> Result<()> { 
        self.user_repo.batch_delete_users(usernames).await 
    }

    // 新增：用户签到
    pub async fn user_check_in(&self, user_id: i32) -> Result<()> {
        self.user_repo.user_check_in(user_id).await
    }

    // 新增：获取用户连续签到天数
    pub async fn get_user_check_in_streak(&self, user_id: i32) -> Result<i32> {
        self.user_repo.get_check_in_streak(user_id).await
    }

    // 新增：获取用户今日活跃度
    pub async fn get_user_today_activity(&self, user_id: i32) -> Result<f32> {
        // 基于用户今日的各种活动计算活跃度
        // 这里可以根据发帖、评论、点赞等行为计算
        Ok(85.0) // 示例值
    }

    // 新增：获取用户本周发布统计
    pub async fn get_user_weekly_posts(&self, user_id: i32) -> Result<Vec<i32>> {
        self.user_repo.get_weekly_posts_count(user_id).await
    }

    // 新增：按ID批量删除
    pub async fn batch_delete_users_by_ids(&self, ids: Vec<i32>) -> Result<()> {
        for id in ids { if let Some(user) = self.user_repo.find_by_id(id).await? { if !user.is_admin { self.user_repo.delete_user(user.id).await?; } } }
        Ok(())
    }
} 