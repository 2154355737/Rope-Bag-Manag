use anyhow::Result;
use crate::models::{User, CreateUserRequest, LoginResponse};
use crate::repositories::user_repo::UserRepository;
use crate::utils::jwt::JwtUtils;
use crate::utils::password::PasswordUtils;
use chrono::Utc;
use crate::repositories::email_verification_repo::EmailVerificationRepository;
use crate::services::email_service::EmailService;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt_utils: JwtUtils,
    password_utils: PasswordUtils,
    email_repo: EmailVerificationRepository,
    email_service: Arc<RwLock<EmailService>>,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String, email_repo: EmailVerificationRepository, email_service: Arc<RwLock<EmailService>>) -> Self {
        Self {
            user_repo,
            jwt_utils: JwtUtils::new(jwt_secret),
            password_utils: PasswordUtils::new(),
            email_repo,
            email_service,
        }
    }

    /// 用户名/邮箱 + 密码登录
    pub async fn login(&self, username_or_email: &str, password: &str) -> Result<LoginResponse> {
        // 首先尝试用户名登录，再尝试邮箱登录
        let user = if username_or_email.contains('@') {
            self.user_repo.find_by_email(username_or_email).await?
        } else {
            self.user_repo.find_by_username(username_or_email).await?
        };
        
        let user = user.ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        // 验证密码
        if !self.password_utils.verify_password(password, &user.password_hash)? {
            return Err(anyhow::anyhow!("密码错误"));
        }

        // 检查用户状态
        if user.ban_status != crate::models::BanStatus::Normal {
            return Err(anyhow::anyhow!("用户已被封禁"));
        }

        // 生成JWT token
        let token = self.jwt_utils.generate_token(&user)?;

        // 更新最后登录时间
        self.user_repo.update_last_login(user.id).await?;

        Ok(LoginResponse { user, token })
    }

    /// 邮箱验证码登录
    pub async fn login_by_email_code(&self, email: &str, code: &str) -> Result<LoginResponse> {
        // 验证邮箱验证码
        if !self.email_repo.verify(email, code).await? {
            return Err(anyhow::anyhow!("验证码错误或已过期"));
        }

        // 查找用户
        let user = self.user_repo.find_by_email(email).await?;
        let user = user.ok_or_else(|| anyhow::anyhow!("该邮箱未注册账户"))?;

        // 检查用户状态
        if user.ban_status != crate::models::BanStatus::Normal {
            return Err(anyhow::anyhow!("用户已被封禁"));
        }

        // 生成JWT token
        let token = self.jwt_utils.generate_token(&user)?;

        // 更新最后登录时间
        self.user_repo.update_last_login(user.id).await?;

        Ok(LoginResponse { user, token })
    }

    /// 发送登录验证码
    pub async fn send_login_code(&self, email: &str) -> Result<()> {
        // 检查邮箱是否已注册
        let user = self.user_repo.find_by_email(email).await?;
        if user.is_none() {
            return Err(anyhow::anyhow!("该邮箱未注册账户"));
        }

        // 生成6位验证码
        let code = format!("{:06}", rand::random::<u32>() % 1_000_000);
        let expires = chrono::Utc::now() + chrono::Duration::minutes(5);
        
        // 保存验证码
        self.email_repo.create(None, email, &code, expires).await?;
        
        // 发送验证码邮件
        {
            let es = self.email_service.read().await;
            es.send_verification_code(email, &code).await?;
        }
        
        Ok(())
    }

    /// 用户注册（需要邮箱验证）
    pub async fn register(&self, req: &CreateUserRequest) -> Result<LoginResponse> {
        // 验证邮箱验证码
        if !self.email_repo.verify(&req.email, &req.verification_code).await? {
            return Err(anyhow::anyhow!("邮箱验证码错误或已过期"));
        }

        // 检查用户名是否已存在
        if let Some(_) = self.user_repo.find_by_username(&req.username).await? {
            return Err(anyhow::anyhow!("用户名已存在"));
        }

        // 检查邮箱是否已存在
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            return Err(anyhow::anyhow!("邮箱已被注册"));
        }

        // 加密密码
        let password_hash = self.password_utils.hash_password(&req.password)?;

        // 创建用户
        let mut user = User {
            id: 0, // 数据库会自动生成
            username: req.username.clone(),
            email: req.email.clone(),
            password_hash,
            nickname: req.nickname.clone(),
            role: crate::models::UserRole::User,
            star: 1,
            ban_status: crate::models::BanStatus::Normal,
            ban_reason: None,
            qq_number: req.qq_number.clone(),
            avatar_url: None,
            login_count: 0,
            upload_count: 0,
            download_count: 0,
            created_at: Utc::now(),
            last_login: None,
            is_admin: false,
        };

        let user_id = self.user_repo.create_user(&user).await?;
        user.id = user_id;
        
        // 生成token
        let token = self.jwt_utils.generate_token(&user)?;
        Ok(LoginResponse { user, token })
    }

    pub async fn send_register_code(&self, email: &str) -> Result<()> {
        let code = format!("{:06}", rand::random::<u32>() % 1_000_000);
        let expires = chrono::Utc::now() + chrono::Duration::minutes(5);
        self.email_repo.create(None, email, &code, expires).await?;
        {
            let es = self.email_service.read().await;
            es.send_verification_code(email, &code).await?;
        }
        Ok(())
    }

    pub async fn send_reset_link(&self, email: &str) -> Result<()> {
        // 首先验证邮箱是否存在注册用户
        let user = self.user_repo.find_by_email(email).await?;
        if user.is_none() {
            return Err(anyhow::anyhow!("该邮箱未注册账户"));
        }
        
        // 生成重置令牌
        let link_token = Uuid::new_v4().to_string();
        let expires = chrono::Utc::now() + chrono::Duration::minutes(15);
        
        // 保存重置令牌到数据库
        let user_id = user.map(|u| u.id);
        self.email_repo.create(user_id, email, &link_token, expires).await?;
        
        // 构建重置链接（应该是前端的重置页面地址）
        let reset_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
        // 对邮箱地址进行URL编码，避免@符号等特殊字符导致邮件服务认为链接无效
        let encoded_email = urlencoding::encode(email);
        let link = format!("{}/auth/reset-password?token={}&email={}", reset_url, link_token, encoded_email);
        
        // 发送重置邮件
        {
            let es = self.email_service.read().await;
            es.send_password_reset(email, &link).await?;
        }
        Ok(())
    }

    pub async fn verify_email_code(&self, email: &str, code: &str) -> Result<bool> {
        self.email_repo.verify(email, code).await
    }

    pub async fn reset_password_with_token(&self, email: &str, token: &str, new_password: &str) -> Result<()> {
        // 验证重置令牌
        let is_valid = self.email_repo.verify(email, token).await?;
        if !is_valid {
            return Err(anyhow::anyhow!("重置令牌无效或已过期"));
        }

        // 查找用户
        let user = self.user_repo.find_by_email(email).await?;
        if let Some(user) = user {
            // 加密新密码
            let password_hash = bcrypt::hash(new_password, 12)?;
            
            // 更新用户密码
            self.user_repo.update_password(user.id, &password_hash).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("用户不存在"))
        }
    }

    pub async fn get_user_from_token(&self, token: &str) -> Result<User> {
        let claims = self.jwt_utils.verify_token(token)?;
        let user = self.user_repo.find_by_id(claims.user_id).await?;
        user.ok_or_else(|| anyhow::anyhow!("用户不存在"))
    }
} 