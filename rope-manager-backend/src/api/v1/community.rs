use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::community_service::CommunityService;
use crate::models::CreateCommentRequest;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/comments/{package_id}")
            .route(web::get().to(get_comments))
            .route(web::post().to(create_comment))
    );
}

async fn get_comments(
    path: web::Path<i32>,
    community_service: web::Data<CommunityService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match community_service.get_comments(package_id).await {
        Ok(comments) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": comments,
                "total": comments.len(),
                "page": 1,
                "size": comments.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn create_comment(
    path: web::Path<i32>,
    req: web::Json<CreateCommentRequest>,
    community_service: web::Data<CommunityService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match community_service.create_comment(package_id, &req.content).await {
        Ok(comment) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "评论成功",
            "data": comment
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
} 