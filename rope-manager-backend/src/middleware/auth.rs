use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpRequest, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use crate::utils::jwt::JwtUtils;

pub struct AuthMiddleware {
    jwt_utils: JwtUtils,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_utils: JwtUtils::new(jwt_secret),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_utils: self.jwt_utils.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_utils: JwtUtils,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        // 检查是否需要认证
        let auth_header = req.headers().get("Authorization");
        let jwt_utils = self.jwt_utils.clone();
        let service = self.service.clone();

        Box::pin(async move {
            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        if let Ok(_claims) = jwt_utils.verify_token(token) {
                            // Token有效，继续处理请求
                            let res = service.call(req).await?;
                            return Ok(res);
                        }
                    }
                }
            }

            // 认证失败，返回401
            let (http_req, _) = req.into_parts();
            let res = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "code": 401,
                    "message": "需要认证"
                }))
                .map_into_boxed_body();
            Ok(ServiceResponse::new(http_req, res))
        })
    }
} 