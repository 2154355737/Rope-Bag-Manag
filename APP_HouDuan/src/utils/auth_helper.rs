use actix_web::{HttpRequest, HttpResponse, Result};
use serde_json::json;
use crate::models::user::{User, UserRole, BanStatus};
use crate::utils::jwt::JwtUtils;

/// 权限验证错误
#[derive(Debug)]
pub enum AuthError {
    TokenMissing,
    TokenInvalid,
    UserBanned,
    InsufficientRole,
    AdminRequired,
}

impl AuthError {
    pub fn to_response(&self) -> HttpResponse {
        match self {
            AuthError::TokenMissing => HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "需要登录认证"
            })),
            AuthError::TokenInvalid => HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "认证Token无效"
            })),
            AuthError::UserBanned => HttpResponse::Forbidden().json(json!({
                "code": 403,
                "message": "账户已被封禁或暂停"
            })),
            AuthError::InsufficientRole => HttpResponse::Forbidden().json(json!({
                "code": 403,
                "message": "权限不足"
            })),
            AuthError::AdminRequired => HttpResponse::Forbidden().json(json!({
                "code": 403,
                "message": "需要管理员权限"
            })),
        }
    }
}

/// 权限验证工具
pub struct AuthHelper;

impl AuthHelper {
    /// 从请求中提取Token
    pub fn extract_token(req: &HttpRequest) -> Option<String> {
        // 1. 优先从Authorization头获取
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    return Some(auth_str[7..].to_string());
                }
            }
        }
        
        // 2. 从Cookie中获取
        if let Some(cookie_header) = req.headers().get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if cookie.starts_with("auth_token=") {
                        return Some(cookie[11..].to_string());
                    }
                }
            }
        }
        
        None
    }

    /// 验证并获取用户信息
    pub fn verify_user(req: &HttpRequest) -> Result<User, AuthError> {
        // 1. 提取Token
        let token = Self::extract_token(req).ok_or(AuthError::TokenMissing)?;
        
        // 2. 从app_data中获取JWT Utils
        let jwt_utils = req.app_data::<actix_web::web::Data<std::sync::Arc<JwtUtils>>>()
            .ok_or(AuthError::TokenInvalid)?;
        let claims = jwt_utils.verify_token(&token).map_err(|_| AuthError::TokenInvalid)?;
        
        // 3. 构造用户对象
        let user = User {
            id: claims.user_id,
            username: claims.username.clone(),
            email: String::new(), // Claims中没有email字段，使用空字符串
            password_hash: String::new(),
            nickname: None, // Claims中没有nickname字段，使用None
            bio: None,
            location: None,
            website: None,
            skills: None,
            role: match claims.role.as_str() {
                "admin" => UserRole::Admin,
                "moderator" => UserRole::Moderator,
                "elder" => UserRole::Elder,
                _ => UserRole::User,
            },
            star: 0,
            ban_status: BanStatus::Normal,
            ban_reason: None,
            qq_number: None,
            avatar_url: None,
            login_count: 0,
            upload_count: 0,
            download_count: 0,
            created_at: chrono::Utc::now(),
            last_login: None,
            is_admin: claims.role == "admin",
        };
        
        // 4. 检查封禁状态
        if user.ban_status != BanStatus::Normal {
            return Err(AuthError::UserBanned);
        }
        
        Ok(user)
    }

    /// 要求管理员权限
    pub fn require_admin(req: &HttpRequest) -> Result<User, AuthError> {
        let user = Self::verify_user(req)?;
        
        if user.role != UserRole::Admin {
            return Err(AuthError::AdminRequired);
        }
        
        Ok(user)
    }

    /// 要求特定角色权限
    pub fn require_roles(req: &HttpRequest, required_roles: &[UserRole]) -> Result<User, AuthError> {
        let user = Self::verify_user(req)?;
        
        if !required_roles.contains(&user.role) {
            return Err(AuthError::InsufficientRole);
        }
        
        Ok(user)
    }

    /// 检查是否为管理员
    pub fn is_admin(req: &HttpRequest) -> bool {
        Self::verify_user(req).map(|user| user.role == UserRole::Admin).unwrap_or(false)
    }

    /// 从请求中提取用户ID
    pub fn extract_user_id(req: &HttpRequest) -> Option<i32> {
        Self::verify_user(req).ok().map(|user| user.id)
    }

    /// 从请求中获取用户名
    pub fn get_username(req: &HttpRequest) -> Option<String> {
        Self::verify_user(req).ok().map(|user| user.username)
    }

    /// 检查是否为资源所有者或管理员
    pub fn require_owner_or_admin(req: &HttpRequest, resource_owner_id: i32) -> Result<User, AuthError> {
        let user = Self::verify_user(req)?;
        
        // 管理员拥有所有权限
        if user.role == UserRole::Admin {
            return Ok(user);
        }
        
        // 检查是否为资源所有者
        if user.id == resource_owner_id {
            return Ok(user);
        }
        
        Err(AuthError::InsufficientRole)
    }

    /// 记录安全事件
    pub async fn log_security_event(
        user: Option<&User>,
        operation: &str,
        details: &str,
        req: &HttpRequest,
    ) {
        let ip = Self::get_client_ip(req).unwrap_or_else(|| "unknown".to_string());
        let user_agent = Self::get_user_agent(req).unwrap_or_else(|| "unknown".to_string());
        
        log::warn!(
            "🛡️ 安全事件: {} | 用户: {:?} | IP: {} | UA: {} | 详情: {}",
            operation,
            user.map(|u| &u.username),
            ip,
            user_agent,
            details
        );
    }

    /// 获取客户端IP
    fn get_client_ip(req: &HttpRequest) -> Option<String> {
        // X-Forwarded-For
        if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
            if let Ok(forwarded_str) = forwarded_for.to_str() {
                let ip = forwarded_str.split(',').next().unwrap_or("").trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
        
        // X-Real-IP
        if let Some(real_ip) = req.headers().get("X-Real-IP") {
            if let Ok(ip_str) = real_ip.to_str() {
                return Some(ip_str.to_string());
            }
        }
        
        // Peer address
        if let Some(peer_addr) = req.peer_addr() {
            return Some(peer_addr.ip().to_string());
        }
        
        None
    }

    /// 获取User-Agent
    fn get_user_agent(req: &HttpRequest) -> Option<String> {
        req.headers()
            .get("User-Agent")
            .and_then(|ua| ua.to_str().ok())
            .map(|s| s.to_string())
    }
}

/// 便捷宏：要求登录认证
#[macro_export]
macro_rules! require_auth {
    ($req:expr) => {
        match crate::utils::auth_helper::AuthHelper::verify_user($req) {
            Ok(user) => user,
            Err(e) => return Ok(e.to_response()),
        }
    };
}

/// 便捷宏：要求管理员权限
#[macro_export]
macro_rules! require_admin {
    ($req:expr) => {
        match crate::utils::auth_helper::AuthHelper::require_admin($req) {
            Ok(user) => user,
            Err(e) => return Ok(e.to_response()),
        }
    };
}

/// 便捷宏：要求特定角色
#[macro_export]
macro_rules! require_roles {
    ($req:expr, $roles:expr) => {
        match crate::utils::auth_helper::AuthHelper::require_roles($req, $roles) {
            Ok(user) => user,
            Err(e) => return Ok(e.to_response()),
        }
    };
}

/// 便捷宏：要求资源所有者或管理员
#[macro_export]
macro_rules! require_owner_or_admin {
    ($req:expr, $owner_id:expr) => {
        match crate::utils::auth_helper::AuthHelper::require_owner_or_admin($req, $owner_id) {
            Ok(user) => user,
            Err(e) => return Ok(e.to_response()),
        }
    };
} 