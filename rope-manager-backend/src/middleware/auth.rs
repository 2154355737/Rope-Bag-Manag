use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use crate::models::user::{User, UserRole};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub role: UserRole,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
    
    pub fn is_elder(&self) -> bool {
        matches!(self.role, UserRole::Elder)
    }
}

// 为AuthenticatedUser实现FromRequest特性，使其可以在请求处理器中被提取
impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    
    // 从请求中提取已认证的用户信息
    // 这里为了简化，我们暂时返回一个mock用户
    // 实际项目中，应从JWT中提取用户信息
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // TODO: 从请求头的Authorization中提取JWT，并验证
        // 暂时返回一个mock管理员用户
        ready(Ok(AuthenticatedUser {
            id: 1, // 管理员ID
            username: "admin".to_string(),
            role: UserRole::Admin,
        }))
    }
}

// 提取用户ID的辅助函数
pub fn extract_user_id(req: &HttpRequest) -> Option<i32> {
    // 在实际项目中，应从JWT中提取用户ID
    // 这里为了演示，暂时返回一个固定ID
    Some(1)
} 