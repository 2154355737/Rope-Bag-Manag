use actix_web::{web, get, HttpResponse, HttpRequest};
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
            .service(get_public_banners)
            .service(get_public_announcements)
            .service(get_public_announcement_detail)
    );

    // 顶层别名，便于前端直接访问 /announcements 与 /announcements/{id}
    cfg.service(get_public_announcements);
    cfg.service(get_public_announcement_detail);
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
    http_req: HttpRequest,
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

    let db_target_type = if ttype == "post" { "Post" } else { "Package" };

    match comment_service.get_top_level_comments(db_target_type, target_id, page, size).await {
        Ok((mut comments, total)) => {
            // 计算前缀（优先 PUBLIC_BASE_URL，否则从请求推断）
            let cfg = crate::config::Config::load().unwrap_or_default();
            let mut base_prefix = cfg.public_base_url().map(|s| s.trim_end_matches('/').to_string());
            if base_prefix.is_none() {
                let ci = http_req.connection_info();
                let scheme = ci.scheme();
                let host = ci.host();
                if !host.is_empty() {
                    base_prefix = Some(format!("{}://{}", scheme, host).trim_end_matches('/').to_string());
                }
            }
            if let Some(bp) = base_prefix.as_ref() {
                for c in comments.iter_mut() {
                    if let Some(ref mut avatar) = c.author_avatar {
                        if !avatar.starts_with("http://") && !avatar.starts_with("https://") && avatar.starts_with("/uploads/") {
                            *avatar = format!("{}/{}", bp, avatar.trim_start_matches('/'));
                        }
                    }
                }
            }
            let resp = CommentListResponse { list: comments, total, page, size };
            HttpResponse::Ok().json(ApiResponse::success(resp))
        },
        Err(e) => {
            log::error!("公开获取评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &format!("获取评论失败: {}", e)))
        }
    }
}

#[get("/public/banners")]
async fn get_public_banners(admin_service: web::Data<AdminService>) -> HttpResponse {
    match admin_service.get_active_banners().await {
        Ok(banners) => HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": banners })),
        Err(_) => HttpResponse::InternalServerError().json(json!({"code":500, "message":"获取轮播图失败"})),
    }
}

// 新增：公开公告列表
#[get("/announcements")]
async fn get_public_announcements(admin_service: web::Data<AdminService>) -> HttpResponse {
    match admin_service.get_active_announcements().await {
        Ok(list) => HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": {"list": list} })),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
}

// 新增：公开公告详情
#[get("/announcements/{id}")]
async fn get_public_announcement_detail(path: web::Path<i32>, admin_service: web::Data<AdminService>) -> HttpResponse {
    let id = path.into_inner();
    match admin_service.get_announcement_by_id(id).await {
        Ok(Some(a)) => HttpResponse::Ok().json(json!({"code":0, "message":"success", "data": a })),
        Ok(None) => HttpResponse::NotFound().json(json!({"code":404, "message":"公告不存在"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()}))
    }
} 