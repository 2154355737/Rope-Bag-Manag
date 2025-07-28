use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse, body::{BoxBody, MessageBody},
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;
use serde_json::json;
use crate::models::user::{User, UserRole, BanStatus};
use crate::utils::jwt::Claims;

/// 权限中间件配置
#[derive(Clone)]
pub struct PermissionConfig {
    /// 需要的角色列表
    pub required_roles: Vec<UserRole>,
    /// 是否需要管理员权限
    pub require_admin: bool,
    /// 是否允许资源所有者访问
    pub allow_owner: bool,
    /// 资源类型 (用于所有权验证)
    pub resource_type: Option<String>,
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self {
            required_roles: vec![],
            require_admin: false,
            allow_owner: false,
            resource_type: None,
        }
    }
}

impl PermissionConfig {
    /// 创建新的权限配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置需要的角色
    pub fn require_roles(mut self, roles: Vec<UserRole>) -> Self {
        self.required_roles = roles;
        self
    }
    
    /// 设置需要管理员权限
    pub fn require_admin(mut self) -> Self {
        self.require_admin = true;
        self
    }
    
    /// 允许资源所有者访问
    pub fn allow_owner(mut self, resource_type: String) -> Self {
        self.allow_owner = true;
        self.resource_type = Some(resource_type);
        self
    }
}

/// 权限验证中间件
pub struct PermissionMiddleware {
    config: PermissionConfig,
}

impl PermissionMiddleware {
    pub fn new(config: PermissionConfig) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PermissionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = PermissionMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddlewareService {
            service: Rc::new(service),
            config: self.config.clone(),
        }))
    }
}

pub struct PermissionMiddlewareService<S> {
    service: Rc<S>,
    config: PermissionConfig,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let config = self.config.clone();
        
        Box::pin(async move {
            // 1. 获取JWT Token
            let token = extract_token(&req);
            if token.is_none() {
                log::warn!("🚫 权限验证失败: 缺少认证Token");
                return Ok(req.into_response(
                    HttpResponse::Unauthorized().json(json!({
                        "code": 401,
                        "message": "需要登录认证"
                    }))
                ));
            }
            
            // 2. 验证JWT Token并构造用户对象
            let claims = match crate::utils::jwt::JwtUtils::new("your_secret_key".to_string()).verify_token(token.unwrap()) {
                Ok(claims) => claims,
                Err(e) => {
                    log::warn!("🚫 JWT验证失败: {}", e);
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized().json(json!({
                            "code": 401,
                            "message": "认证Token无效"
                        }))
                    ));
                }
            };
            
            // 从Claims构造用户对象
            let user_role = match claims.role.as_str() {
                "admin" => UserRole::Admin,
                "moderator" => UserRole::Moderator,
                "elder" => UserRole::Elder,
                _ => UserRole::User,
            };
            
            let is_admin = matches!(user_role, UserRole::Admin);
            
            let user = User {
                id: claims.user_id,
                username: claims.username,
                email: "".to_string(), // 临时设置，实际应从数据库获取
                password_hash: "".to_string(), // 不需要密码
                nickname: None,
                role: user_role,
                star: 0,
                ban_status: BanStatus::Normal, // 默认为正常状态，实际应从数据库获取
                ban_reason: None,
                qq_number: None,
                avatar_url: None,
                login_count: 0,
                upload_count: 0,
                download_count: 0,
                created_at: chrono::Utc::now(),
                last_login: None,
                is_admin,
            };
            
            // 3. 检查用户封禁状态
            if user.ban_status != BanStatus::Normal {
                log::warn!("🚫 封禁用户访问被拦截: {} ({})", user.username, user.ban_status);
                return Ok(req.into_response(
                    HttpResponse::Forbidden().json(json!({
                        "code": 403,
                        "message": match user.ban_status {
                            BanStatus::Suspended => "账户已被暂停",
                            BanStatus::Banned => "账户已被封禁",
                            _ => "账户状态异常"
                        }
                    }))
                ));
            }
            
            // 4. 权限验证
            let permission_result = verify_permission(&user, &config, &req).await;
            if let Err(error_response) = permission_result {
                return Ok(req.into_response(error_response));
            }
            
            // 5. 将用户信息注入请求上下文
            req.extensions_mut().insert(user.clone());
            
            // 6. 记录访问日志
            log::info!("✅ 权限验证通过: {}({}) 访问 {}", 
                user.username, user.role, req.path());
            
            // 7. 继续处理请求
            let res = service.call(req).await?;
            Ok(res.map_into_boxed_body())
        })
    }
}

/// 从请求中提取Token
fn extract_token(req: &ServiceRequest) -> Option<&str> {
    // 1. 优先从Authorization头获取
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(&auth_str[7..]);
            }
        }
    }
    
    // 2. 从Cookie中获取
    if let Some(cookie_header) = req.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("auth_token=") {
                    return Some(&cookie[11..]);
                }
            }
        }
    }
    
    None
}

/// 验证用户权限
async fn verify_permission(
    user: &User, 
    config: &PermissionConfig, 
    req: &ServiceRequest
) -> Result<(), HttpResponse> {
    // 1. 管理员权限检查
    if config.require_admin && user.role != UserRole::Admin {
        log::warn!("🚫 管理员权限不足: {}({}) 尝试访问 {}", 
            user.username, user.role, req.path());
        
        return Err(HttpResponse::Forbidden().json(json!({
            "code": 403,
            "message": "需要管理员权限"
        })));
    }
    
    // 2. 角色权限检查
    if !config.required_roles.is_empty() {
        if !config.required_roles.contains(&user.role) {
            // 如果配置了允许所有者访问，检查资源所有权
            if config.allow_owner {
                let is_owner = check_resource_ownership(user, config, req).await;
                if !is_owner {
                    log::warn!("🚫 角色权限不足且非资源所有者: {}({}) 需要 {:?}", 
                        user.username, user.role, config.required_roles);
                    
                    return Err(HttpResponse::Forbidden().json(json!({
                        "code": 403,
                        "message": "权限不足"
                    })));
                }
            } else {
                log::warn!("🚫 角色权限不足: {}({}) 需要 {:?}", 
                    user.username, user.role, config.required_roles);
                
                return Err(HttpResponse::Forbidden().json(json!({
                    "code": 403,
                    "message": "权限不足"
                })));
            }
        }
    }
    
    Ok(())
}

/// 检查资源所有权
async fn check_resource_ownership(
    user: &User, 
    config: &PermissionConfig, 
    req: &ServiceRequest
) -> bool {
    // 管理员拥有所有资源的权限
    if user.role == UserRole::Admin {
        return true;
    }
    
    // 提取资源ID
    let resource_id = extract_resource_id(req);
    if resource_id.is_none() {
        return false;
    }
    
    let resource_id = resource_id.unwrap();
    let default_type = "unknown".to_string();
    let resource_type = config.resource_type.as_ref().unwrap_or(&default_type);
    
    // TODO: 根据资源类型查询数据库验证所有权
    // 这里需要注入对应的Repository来查询
    // 现在先返回false，后续需要完善
    log::debug!("检查资源所有权: user_id={}, resource_type={}, resource_id={}", 
        user.id, resource_type, resource_id);
    
    false
}

/// 从路径中提取资源ID
fn extract_resource_id(req: &ServiceRequest) -> Option<i32> {
    let path = req.path();
    
    // 从路径中提取数字ID，支持多种路径格式
    // /api/v1/packages/123 -> 123
    // /api/v1/comments/456 -> 456
    let parts: Vec<&str> = path.split('/').collect();
    for part in parts.iter() {
        if let Ok(id) = part.parse::<i32>() {
            return Some(id);
        }
    }
    
    None
}

/// 便捷方法：仅需要登录认证
pub fn require_auth() -> PermissionMiddleware {
    PermissionMiddleware::new(PermissionConfig::new())
}

/// 便捷方法：需要管理员权限
pub fn require_admin() -> PermissionMiddleware {
    PermissionMiddleware::new(PermissionConfig::new().require_admin())
}

/// 便捷方法：需要特定角色
pub fn require_roles(roles: Vec<UserRole>) -> PermissionMiddleware {
    PermissionMiddleware::new(PermissionConfig::new().require_roles(roles))
}

/// 便捷方法：允许资源所有者
pub fn allow_owner(resource_type: &str) -> PermissionMiddleware {
    PermissionMiddleware::new(
        PermissionConfig::new().allow_owner(resource_type.to_string())
    )
} 