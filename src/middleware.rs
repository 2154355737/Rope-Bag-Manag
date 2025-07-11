use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use crate::models::AppState;
use crate::auth::record_api_call;
use crate::utils::now_ts;

pub struct ApiStatsMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ApiStatsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiStatsMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiStatsMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct ApiStatsMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiStatsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let start_time = now_ts();
        let path = req.path().to_string();
        let method = req.method().as_str().to_string();
        let api_name = format!("{} {}", method, path);
        
        // 获取AppState
        let app_state = req.app_data::<actix_web::web::Data<AppState>>().cloned();
        
        let svc = self.service.clone();
        
        Box::pin(async move {
            let response = svc.call(req).await?;
            
            // 记录API调用统计
            if let Some(app_state) = app_state {
                let status_code = response.status().as_u16();
                let success = status_code < 400;
                let error_message = if success { None } else { 
                    Some(format!("HTTP {}", status_code))
                };
                
                // 创建一个简单的请求对象用于记录
                let request_info = format!("{} {}", method, path);
                
                record_api_call(
                    &request_info,
                    &app_state.stats,
                    &api_name,
                    start_time,
                    status_code,
                    success,
                    error_message,
                );
            }
            
            Ok(response)
        })
    }
} 