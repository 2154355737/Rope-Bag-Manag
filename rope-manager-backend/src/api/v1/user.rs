use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::user_service::UserService;
use crate::models::UpdateUserRequest;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_users))
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(get_user))
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user))
    );
}

async fn get_users(
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    match user_service.get_users().await {
        Ok(users) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": users,
                "total": users.len(),
                "page": 1,
                "size": users.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn get_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    match user_service.get_user(user_id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": user
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "用户不存在"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn update_user(
    path: web::Path<i32>,
    req: web::Json<UpdateUserRequest>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    match user_service.update_user(user_id, &req).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "更新成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn delete_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = path.into_inner();
    match user_service.delete_user(user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
} 