use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use crate::models::user::UserRole;

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
        let _req = req.clone();
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
    // 尝试从请求中获取Authorization头
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // Bearer token格式
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // 去掉"Bearer "前缀
                
                // 尝试从应用数据中获取JWT工具
                if let Some(jwt_utils) = req.app_data::<actix_web::web::Data<crate::utils::jwt::JwtUtils>>() {
                    // 尝试验证token并提取用户ID
                    match jwt_utils.verify_token(token) {
                        Ok(claims) => {
                            println!("成功从JWT中提取用户ID: {}", claims.user_id);
                            return Some(claims.user_id);
                        }
                        Err(e) => {
                            println!("JWT验证失败: {}", e);
                        }
                    }
                } else {
                    println!("无法获取JWT工具");
                }
            }
        }
    }
    
    // 临时解决方案：如果在登录状态但无法提取用户ID，返回默认管理员ID
    // 注意：这只是为了调试，生产环境中应移除此代码
    println!("无法从JWT提取用户ID，使用默认管理员ID");
    Some(1)  // 使用默认管理员ID
} 