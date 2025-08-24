use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use crate::shared::utils::jwt as jwt_util;
use crate::infrastructure::database::repositories::{SqlxLikeRepository, UserRepository};
use crate::core::ports::repositories::LikeRepository;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/profile", web::get().to(get_profile))
            .route("/profile", web::put().to(update_profile))
            .route("/my-likes", web::get().to(my_likes))
    );
}

async fn get_profile(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = UserRepository::new(app.db.pool());
    if let Some(u) = repo.find_by_id(uid).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))? {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"id": u.id, "username": u.username, "email": u.email, "nickname": u.nickname, "avatar_url": u.avatar_url, "bio": u.bio, "role": u.role, "status": u.status, "created_at": u.created_at}})))
    }
    Ok(HttpResponse::Unauthorized().json(serde_json::json!({"code":401, "message":"未登录或用户不存在，请重新登录"})))
}

#[derive(Deserialize)]
struct UpdateProfileReq { nickname: Option<String>, avatar_url: Option<String>, bio: Option<String> }

async fn update_profile(app: web::Data<crate::core::AppState>, req: HttpRequest, payload: web::Json<UpdateProfileReq>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = UserRepository::new(app.db.pool());
    let _ = repo.update_profile(uid, payload.nickname.as_deref(), payload.avatar_url.as_deref(), payload.bio.as_deref(), None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    if let Some(u) = repo.find_by_id(uid).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))? {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"更新成功", "data": {"id": u.id, "username": u.username, "email": u.email, "nickname": u.nickname, "avatar_url": u.avatar_url, "bio": u.bio, "role": u.role, "status": u.status, "created_at": u.created_at}})))
    }
    Ok(HttpResponse::NotFound().json(serde_json::json!({"code":404, "message":"用户不存在"})))
}

#[derive(Deserialize)]
struct MyLikesQuery { page: Option<i64>, pageSize: Option<i64>, targetType: Option<String> }

async fn my_likes(app: web::Data<crate::core::AppState>, req: HttpRequest, query: web::Query<MyLikesQuery>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.pageSize.unwrap_or(20).clamp(1, 100);
    let target_type_opt = query.targetType.as_deref();

    let repo = SqlxLikeRepository::new(app.db.pool());
    let page_data = repo.find_user_likes(uid, target_type_opt, page, page_size)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": page_data})))
}

fn extract_uid(req: &HttpRequest, app: &crate::core::AppState) -> Result<i64, actix_web::Error> {
    if let Some(h) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = h.to_str() {
            if let Some(t) = s.strip_prefix("Bearer ") {
                if let Some(uid) = jwt_util::verify(t, &app.config.jwt) { return Ok(uid); }
            }
        }
    }
    Err(actix_web::error::ErrorUnauthorized("未登录"))
} 