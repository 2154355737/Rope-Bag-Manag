use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::time::Instant;
use crate::utils::auth_helper::AuthHelper;

pub struct ApiLogger;

impl<S, B> Transform<S, ServiceRequest> for ApiLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiLoggerMiddleware { service }))
    }
}

pub struct ApiLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ApiLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let query = req.query_string().to_string();
        
        // 获取用户信息（如果已认证）
        let user_info = if let Ok(user) = AuthHelper::verify_user(req.request()) {
            Some(format!("user:{}", user.id))
        } else {
            None
        };
        
        // 获取客户端IP
        let client_ip = req.connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status();
            
            // 构建完整路径
            let full_path = if query.is_empty() {
                path
            } else {
                format!("{}?{}", path, query)
            };
            
            // 根据状态码和响应时间选择日志级别
            let log_level = if status.is_server_error() {
                "ERROR"
            } else if status.is_client_error() {
                "WARN"
            } else if duration.as_millis() > 1000 {
                "WARN"
            } else {
                "INFO"
            };
            
            let duration_ms = duration.as_millis();
            let performance_indicator = if duration_ms > 1000 {
                "🐌" // 慢请求
            } else if duration_ms > 500 {
                "⚠️"  // 中等
            } else {
                "⚡" // 快速
            };
            
            match log_level {
                "ERROR" => {
                    log::error!(
                        "🌐 {} {} {} {}ms {} [{}]",
                        method, full_path, status, duration_ms, performance_indicator,
                        user_info.as_deref().unwrap_or(&client_ip)
                    );
                }
                "WARN" => {
                    log::warn!(
                        "🌐 {} {} {} {}ms {} [{}]",
                        method, full_path, status, duration_ms, performance_indicator,
                        user_info.as_deref().unwrap_or(&client_ip)
                    );
                }
                _ => {
                    log::info!(
                        "🌐 {} {} {} {}ms {} [{}]",
                        method, full_path, status, duration_ms, performance_indicator,
                        user_info.as_deref().unwrap_or(&client_ip)
                    );
                }
            }

            Ok(res)
        })
    }
} 