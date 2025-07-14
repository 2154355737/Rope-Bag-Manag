// 暂时注释掉认证中间件，避免复杂的类型问题
// TODO: 重新实现认证中间件

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }
} 