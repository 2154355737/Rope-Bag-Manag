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
        let mut token: Option<&str> = None;
        
        // 1. 优先从Authorization头中获取Bearer Token
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    token = Some(&auth_str[7..]);
                }
            }
        }
        
        // 2. 如果Authorization头中没有token，则从Cookie中获取
        if token.is_none() {
            if let Some(cookie_header) = req.headers().get("Cookie") {
                if let Ok(cookie_str) = cookie_header.to_str() {
                    // 解析Cookie字符串
                    for cookie in cookie_str.split(';') {
                        let cookie = cookie.trim();
                        if cookie.starts_with("auth_token=") {
                            token = Some(&cookie[11..]); // "auth_token=".len() = 11
                            break;
                        }
                    }
                }
            }
        }
        
        // 3. 验证获取到的token
        if let Some(token_value) = token {
                    // 从应用数据中获取 JwtUtils
                    if let Some(jwt_utils) = req.app_data::<actix_web::web::Data<std::sync::Arc<crate::utils::jwt::JwtUtils>>>() {
                match jwt_utils.verify_token(token_value) {
                            Ok(claims) => {
                                let role = match claims.role.as_str() {
                                    "admin" => UserRole::Admin,
                                    "moderator" => UserRole::Moderator,
                                    "elder" => UserRole::Elder,
                                    _ => UserRole::User,
                                };
                                return ready(Ok(AuthenticatedUser {
                                    id: claims.user_id,
                                    username: claims.username,
                                    role,
                                }));
                            }
                            Err(e) => {
                                log::warn!("JWT验证失败: {}", e);
                            }
                        }
                    } else {
                        log::warn!("JwtUtils 未注入，无法验证Token");
            }
        }

        // 未携带或验证失败时，返回未认证错误
        ready(Err(actix_web::error::ErrorUnauthorized("未认证用户")))
    }
}

// 提取用户ID的辅助函数
pub fn extract_user_id(req: &HttpRequest) -> Option<i32> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                
                if let Some(jwt_utils) = req.app_data::<actix_web::web::Data<std::sync::Arc<crate::utils::jwt::JwtUtils>>>() {
                    if let Ok(claims) = jwt_utils.verify_token(token) {
                            return Some(claims.user_id);
                    }
                }
            }
        }
    }
    
    None
} 