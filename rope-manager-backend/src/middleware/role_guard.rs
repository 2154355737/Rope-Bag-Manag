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
            println!("🔍 角色守卫检查 - 路径: {}", req.path());
            println!("🔍 需要角色: {:?}", required_roles);
            
            let auth_header = req.headers().get("Authorization");
            println!("🔍 Authorization header: {:?}", auth_header);
            
            if let (Some(auth_value), Some(auth_service)) = (auth_header, auth_service) {
                if let Ok(auth_str) = auth_value.to_str() {
                    println!("🔍 Authorization string: {}", auth_str);
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        println!("🔍 JWT Token: {}", token);
                        
                        match auth_service.get_user_from_token(token).await {
                            Ok(user) => {
                                println!("🔍 用户信息: {:?}", user);
                                println!("🔍 用户角色: {:?}", user.role);
                                
                                if required_roles.contains(&user.role) {
                                    println!("✅ 权限验证通过");
                                    let res = service.call(req).await?;
                                    return Ok(res.map_into_boxed_body());
                                } else {
                                    println!("❌ 权限不足 - 需要: {:?}, 用户角色: {:?}", required_roles, user.role);
                                }
                            },
                            Err(e) => {
                                println!("❌ Token验证失败: {}", e);
                            }
                        }
                    } else {
                        println!("❌ Token格式错误 - 不是Bearer格式");
                    }
                } else {
                    println!("❌ Authorization header不是有效字符串");
                }
            } else {
                println!("❌ 缺少Authorization header或AuthService");
            }
            
            println!("❌ 返回403 Forbidden");
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