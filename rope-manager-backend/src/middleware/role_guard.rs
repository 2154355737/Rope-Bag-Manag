use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use actix_web::body::{MessageBody, BoxBody};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::marker::PhantomData;
use crate::models::UserRole;
use crate::services::auth_service::AuthService;

#[derive(Clone)]
pub struct RoleGuard {
    pub required_roles: Vec<UserRole>,
}

impl<S, B> Transform<S, ServiceRequest> for RoleGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RoleGuardMiddleware<S, B>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RoleGuardMiddleware {
            service: Rc::new(service),
            required_roles: self.required_roles.clone(),
            _phantom: PhantomData,
        })
    }
}

pub struct RoleGuardMiddleware<S, B> {
    service: Rc<S>,
    required_roles: Vec<UserRole>,
    _phantom: PhantomData<B>,
}

impl<S, B> Service<ServiceRequest> for RoleGuardMiddleware<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let required_roles = self.required_roles.clone();
        let service = self.service.clone();
        // 依赖注入 AuthService
        let auth_service = req.app_data::<actix_web::web::Data<AuthService>>().cloned();

        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");
            if let (Some(auth_value), Some(auth_service)) = (auth_header, auth_service) {
                if let Ok(auth_str) = auth_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        if let Ok(user) = auth_service.get_user_from_token(token).await {
                            if required_roles.contains(&user.role) {
                                let res = service.call(req).await?;
                                return Ok(res.map_into_boxed_body());
                            }
                        }
                    }
                }
            }
            let resp = HttpResponse::Forbidden().json(serde_json::json!({
                "code": 403,
                "message": "无权限"
            }));
            let (req, _pl) = req.into_parts();
            let res = ServiceResponse::new(
                req,
                resp.map_into_boxed_body()
            );
            Ok(res)
        })
    }
} 