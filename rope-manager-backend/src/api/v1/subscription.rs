use actix_web::{web,HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::repositories::subscription_repo::SubscriptionRepository;
use crate::middleware::auth::AuthenticatedUser;

#[derive(Deserialize)]
struct SubReq { category_id:i32, enabled:bool }

pub fn configure_routes(cfg:&mut web::ServiceConfig){
    cfg.service(web::scope("/subscriptions")
        .service(web::resource("").route(web::post().to(set_subscription))));
}

async fn set_subscription(
    req: web::Json<SubReq>,
    repo: web::Data<SubscriptionRepository>,
    user: AuthenticatedUser,
)->Result<HttpResponse, actix_web::Error>{
    match repo.set_subscription(user.id, req.category_id, req.enabled).await{
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"code":0}))),
        Err(e)=> Ok(HttpResponse::InternalServerError().json(json!({"code":500,"message":e.to_string()})))
    }
} 