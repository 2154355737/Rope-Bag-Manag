use actix_web::{web, HttpResponse, get, post, delete};
use serde::Deserialize;
use serde_json::json;
use crate::services::forbidden_word_service::ForbiddenWordService;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::ApiResponse;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/forbidden-words")
            .service(list_words)
            .service(add_word)
            .service(delete_word)
    );
}

#[get("")]
async fn list_words(service: web::Data<ForbiddenWordService>, auth: AuthenticatedUser) -> HttpResponse {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(403, "无权限"));
    }
    match service.list_words().await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse::success(list)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

#[derive(Deserialize)]
struct AddWordReq { word: String }

#[post("")]
async fn add_word(req: web::Json<AddWordReq>, service: web::Data<ForbiddenWordService>, auth: AuthenticatedUser) -> HttpResponse {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(403, "无权限"));
    }
    match service.add_word(&req.word).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(json!({"word": req.word}))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
}

#[delete("/{id}")]
async fn delete_word(path: web::Path<i32>, service: web::Data<ForbiddenWordService>, auth: AuthenticatedUser) -> HttpResponse {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(403, "无权限"));
    }
    let id = path.into_inner();
    match service.delete_word(id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(json!({"id": id}))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())),
    }
} 