use anyhow::Result;
use crate::models::{User, CreateUserRequest, LoginResponse};
use crate::repositories::user_repo::UserRepository;
use crate::utils::jwt::JwtUtils;
use crate::utils::password::PasswordUtils;
use chrono::Utc;

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt_utils: JwtUtils,
    password_utils: PasswordUtils,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_utils: JwtUtils::new(jwt_secret),
            password_utils: PasswordUtils::new(),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        // 查找用户
        let user = self.user_repo.find_by_username(username).await?;
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

    pub async fn register(&self, req: &CreateUserRequest) -> Result<LoginResponse> {
        // 检查用户名是否已存在
        if let Some(_) = self.user_repo.find_by_username(&req.username).await? {
            return Err(anyhow::anyhow!("用户名已存在"));
        }

        // 加密密码
        let password_hash = self.password_utils.hash_password(&req.password)?;

        // 创建用户
        let user = User {
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

        self.user_repo.create_user(&user).await?;
        // 生成token
        let token = self.jwt_utils.generate_token(&user)?;
        Ok(LoginResponse { user, token })
    }

    pub async fn get_user_from_token(&self, token: &str) -> Result<User> {
        let claims = self.jwt_utils.verify_token(token)?;
        let user = self.user_repo.find_by_id(claims.user_id).await?;
        user.ok_or_else(|| anyhow::anyhow!("用户不存在"))
    }
} 