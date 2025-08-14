use actix_web::{web, get, HttpResponse};
use serde_json::json;
use crate::models::ApiResponse;
use crate::models::CommentListResponse;
use crate::services::comment_service::CommentService;
use crate::services::admin_service::AdminService;
use actix_web::post;

#[derive(serde::Deserialize)]
pub struct PublicCommentQuery {
    pub target_type: Option<String>,
    pub target_id: Option<i32>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(serde::Deserialize)]
struct AppLaunchReq { user_id: Option<i32>, device_id: Option<String>, app_version: Option<String>, platform: Option<String> }

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/public")
            .service(get_public_comments)
            .service(get_public_banners)
            .service(app_launch)
    );
}

#[post("/app/launch")]
async fn app_launch(req: web::Json<AppLaunchReq>, admin_service: web::Data<AdminService>) -> HttpResponse {
    match admin_service.record_app_launch(req.user_id, req.device_id.as_deref(), req.app_version.as_deref(), req.platform.as_deref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({"code":0, "message":"ok"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
}

#[get("/comments")]
async fn get_public_comments(
    query: web::Query<PublicCommentQuery>,
    comment_service: web::Data<CommentService>,
) -> HttpResponse {
    // 校验 target_type
    let ttype = match &query.target_type { Some(t) if !t.is_empty() => t.to_lowercase(), _ => String::new() };
    if !(ttype == "post" || ttype == "package") {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, "必须提供合法的 target_type: Post 或 Package"));
    }
    let target_id = match query.target_id { Some(id) if id > 0 => id, _ => {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, "必须提供有效的 target_id"));
    }};

    let page = query.page.unwrap_or(1);
    let mut size = query.size.unwrap_or(20);
    if size > 100 { size = 100; }

    // 仅 Active
    let status = Some("Active");
    let db_target_type = if ttype == "post" { "Post" } else { "Package" };
    let target_type = Some(db_target_type);

    match comment_service.get_all_comments(page, size, status, target_type, Some(target_id), None, None, None, None).await {
        Ok((comments, total)) => {
            let resp = CommentListResponse { list: comments, total, page, size };
            HttpResponse::Ok().json(ApiResponse::success(resp))
        },
        Err(e) => {
            log::error!("公开获取评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &format!("获取评论失败: {}", e)))
        }
    }
}

#[get("/banners")]
async fn get_public_banners(
    admin_service: web::Data<AdminService>,
) -> HttpResponse {
    match admin_service.get_active_banners().await {
        Ok(banners) => HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": banners
        })),
        Err(e) => {
            log::error!("获取活动轮播图失败: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": "获取轮播图失败"
            }))
        }
    }
} 