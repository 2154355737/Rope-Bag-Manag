use actix_web::{web, HttpRequest, HttpResponse};
use crate::shared::utils::jwt as jwt_util;
use crate::infrastructure::database::repositories::{CommentRepository, SqlxLikeRepository};
use crate::core::ports::repositories::LikeRepository;
use serde::Deserialize;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .route("/{id}/like", web::post().to(like_comment))
            .route("/{id}/helpful", web::post().to(mark_helpful))
            .route("/{id}/reply", web::post().to(reply_comment))
            .route("/{id}/report", web::post().to(report_comment))
    );
}

async fn like_comment(app: web::Data<crate::core::AppState>, req: HttpRequest, path: web::Path<i64>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let id = path.into_inner();
    let c_repo = CommentRepository::new(app.db.pool());
    let c = c_repo.find_by_id(id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let c = match c { Some(v) => v, None => return Ok(HttpResponse::NotFound().json(serde_json::json!({"code":404, "message":"评论不存在"}))) };
    let like_repo = SqlxLikeRepository::new(app.db.pool());
    like_repo.add_like(uid, "comment", id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let count = like_repo.get_like_count("comment", id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let _ = c_repo.update_like_count(id, count).await;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"like_count": count}})))
}

async fn mark_helpful(app: web::Data<crate::core::AppState>, req: HttpRequest, path: web::Path<i64>) -> Result<HttpResponse, actix_web::Error> {
    let _uid = extract_uid(&req, &app)?;
    let id = path.into_inner();
    let c_repo = CommentRepository::new(app.db.pool());
    let _ = c_repo.mark_helpful(id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已标记有用"})))
}

#[derive(Deserialize)]
struct ReplyReq { content: String }

async fn reply_comment(app: web::Data<crate::core::AppState>, req: HttpRequest, path: web::Path<i64>, payload: web::Json<ReplyReq>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let parent_id = path.into_inner();
    let c_repo = CommentRepository::new(app.db.pool());
    let c = c_repo.find_by_id(parent_id).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let c = match c { Some(v) => v, None => return Ok(HttpResponse::NotFound().json(serde_json::json!({"code":404, "message":"评论不存在"}))) };
    if payload.content.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1, "message":"内容不能为空"})))
    }
    let item = c_repo.create(uid, &c.target_type, c.target_id, &payload.content, Some(parent_id)).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": item})))
}

async fn report_comment(app: web::Data<crate::core::AppState>, req: HttpRequest, path: web::Path<i64>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let id = path.into_inner();
    // 记录用户举报行为
    let _ = sqlx::query("INSERT INTO user_actions (user_id, action_type, target_type, target_id, details) VALUES (?, 'report_comment', 'comment', ?, '{}')")
        .bind(uid).bind(id)
        .execute(app.db.pool()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已举报"})))
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