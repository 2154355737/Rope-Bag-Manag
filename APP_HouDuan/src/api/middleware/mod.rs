use std::task::{Context, Poll};
use std::time::Instant;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ready, Ready, LocalBoxFuture};
use uuid::Uuid;
use tracing::{info, warn, error};
use actix_web::http::header::{HeaderName, HeaderValue, USER_AGENT};
use actix_web::HttpMessage;

/// 请求跟踪中间件：
/// - 生成并注入 X-Request-Id
/// - 记录方法、路径、状态码、耗时、UA、远端地址
/// - 将请求内日志与请求ID关联
pub struct RequestTracing;

impl RequestTracing {
    pub fn new() -> Self { Self }
}

impl<S, B> Transform<S, ServiceRequest> for RequestTracing
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestTracingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestTracingMiddleware { service }))
    }
}

pub struct RequestTracingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestTracingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let request_id = Uuid::new_v4().to_string();

        let method = req.method().to_string();
        let path = req.path().to_string();
        let conn = req.connection_info().clone();
        let remote = conn.realip_remote_addr().map(|s| s.to_string());
        let user_agent = req
            .headers()
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // 将 request_id 注入 extensions，供后续处理使用
        let req = req;
        req.extensions_mut().insert(request_id.clone());

        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(mut res) => {
                    let status = res.status().as_u16();
                    let elapsed = start.elapsed().as_millis();

                    // 注入响应头
                    if let (Ok(name), Ok(value)) = (
                        HeaderName::from_bytes(b"x-request-id"),
                        HeaderValue::from_str(&request_id),
                    ) {
                        res.headers_mut().insert(name, value);
                    }

                    // 记录日志
                    match status {
                        s if s >= 500 => error!(
                            request_id = %request_id,
                            method = %method,
                            path = %path,
                            status = s,
                            duration_ms = elapsed,
                            remote = remote.as_deref().unwrap_or("-"),
                            user_agent = user_agent.as_deref().unwrap_or("-"),
                            "HTTP request failed (server error)"
                        ),
                        s if s >= 400 => warn!(
                            request_id = %request_id,
                            method = %method,
                            path = %path,
                            status = s,
                            duration_ms = elapsed,
                            remote = remote.as_deref().unwrap_or("-"),
                            user_agent = user_agent.as_deref().unwrap_or("-"),
                            "HTTP request failed (client error)"
                        ),
                        s => info!(
                            request_id = %request_id,
                            method = %method,
                            path = %path,
                            status = s,
                            duration_ms = elapsed,
                            remote = remote.as_deref().unwrap_or("-"),
                            user_agent = user_agent.as_deref().unwrap_or("-"),
                            "HTTP request completed"
                        ),
                    }

                    Ok(res)
                }
                Err(e) => {
                    // 发生错误：记录500并返回错误
                    let elapsed = start.elapsed().as_millis();
                    error!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = 500u16,
                        duration_ms = elapsed,
                        remote = remote.as_deref().unwrap_or("-"),
                        user_agent = user_agent.as_deref().unwrap_or("-"),
                        error = %e,
                        "HTTP request errored"
                    );
                    Err(e)
                }
            }
        })
    }
} 