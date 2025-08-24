use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use crate::infrastructure::database::repositories::UserRepository;
use crate::shared::utils::jwt as jwt_util;
use sqlx::Row;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/me", web::get().to(get_me))
            .route("/me", web::put().to(update_me))
            .route("/me/stats", web::get().to(get_me_stats))
            .route("/me/check-in", web::post().to(check_in))
            .route("/me/weekly-report", web::get().to(get_weekly_report))
            .route("/me/achievements", web::get().to(get_achievements))
            .route("/me/activity-stats", web::get().to(get_activity_stats))
            .route("/me/resources", web::get().to(get_my_resources))
            .route("/me/posts", web::get().to(get_my_posts))
            .route("/me/comments", web::get().to(get_my_comments))
    );
}

async fn get_me(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = UserRepository::new(app.db.pool());
    if let Some(u) = repo.find_by_id(uid).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))? {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"id": u.id, "username": u.username, "email": u.email, "nickname": u.nickname, "avatar_url": u.avatar_url, "bio": u.bio, "role": u.role, "status": u.status, "created_at": u.created_at}})))
    }
    Ok(HttpResponse::NotFound().json(serde_json::json!({"code":404, "message":"用户不存在"})))
}

#[derive(Deserialize)]
struct UpdateMeReq { nickname: Option<String>, avatar_url: Option<String>, bio: Option<String>, settings: Option<serde_json::Value> }

async fn update_me(app: web::Data<crate::core::AppState>, req: HttpRequest, payload: web::Json<UpdateMeReq>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = UserRepository::new(app.db.pool());
    let _ = repo.update_profile(uid, payload.nickname.as_deref(), payload.avatar_url.as_deref(), payload.bio.as_deref(), payload.settings.clone()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    if let Some(u) = repo.find_by_id(uid).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))? {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"更新成功", "data": {"id": u.id, "username": u.username, "email": u.email, "nickname": u.nickname, "avatar_url": u.avatar_url, "bio": u.bio, "role": u.role, "status": u.status, "created_at": u.created_at}})))
    }
    Ok(HttpResponse::NotFound().json(serde_json::json!({"code":404, "message":"用户不存在"})))
}

async fn get_me_stats(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = UserRepository::new(app.db.pool());
    let (packages_count, comments_count, likes_received, downloads_total) = repo.stats_of(uid).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"packages_count": packages_count, "comments_count": comments_count, "likes_received": likes_received, "downloads_total": downloads_total}})))
}

async fn check_in(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    // 以 user_actions 记录签到，限制每日一次
    let pool = app.db.pool();
    // 检查今天是否已签到
    let exist: (i64,) = sqlx::query_as(
        "SELECT COUNT(1) FROM user_actions WHERE user_id = ? AND action_type = 'check_in' AND DATE(created_at) = DATE('now')"
    )
    .bind(uid)
    .fetch_one(pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    if exist.0 > 0 {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"今日已签到"})))
    }
    sqlx::query(
        "INSERT INTO user_actions (user_id, action_type, details) VALUES (?, 'check_in', '{}')"
    )
    .bind(uid)
    .execute(pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"签到成功"})))
}

async fn get_weekly_report(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let pool = app.db.pool();
    let (pkg7,): (i64,) = sqlx::query_as(
        "SELECT COUNT(1) FROM packages WHERE author_id = ? AND DATE(created_at) >= DATE('now','-6 days')"
    ).bind(uid).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let (post7,): (i64,) = sqlx::query_as(
        "SELECT COUNT(1) FROM posts WHERE author_id = ? AND DATE(created_at) >= DATE('now','-6 days')"
    ).bind(uid).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let (cmt7,): (i64,) = sqlx::query_as(
        "SELECT COUNT(1) FROM comments WHERE user_id = ? AND DATE(created_at) >= DATE('now','-6 days')"
    ).bind(uid).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let (dl7,): (i64,) = sqlx::query_as(
        "SELECT COUNT(1) FROM downloads WHERE user_id = ? AND DATE(created_at) >= DATE('now','-6 days')"
    ).bind(uid).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let (likes_pkg7,): (i64,) = sqlx::query_as(
        "SELECT COALESCE(COUNT(1),0) FROM likes l JOIN packages p ON l.target_type='package' AND l.target_id=p.id WHERE p.author_id = ? AND DATE(l.created_at) >= DATE('now','-6 days')"
    ).bind(uid).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let (likes_post7,): (i64,) = sqlx::query_as(
        "SELECT COALESCE(COUNT(1),0) FROM likes l JOIN posts p ON l.target_type='post' AND l.target_id=p.id WHERE p.author_id = ? AND DATE(l.created_at) >= DATE('now','-6 days')"
    ).bind(uid).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let likes7 = likes_pkg7 + likes_post7;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "data": {
            "range": "last_7_days",
            "packages_created": pkg7,
            "posts_created": post7,
            "comments_created": cmt7,
            "downloads": dl7,
            "likes_received": likes7
        }
    })))
}

async fn get_achievements(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let repo = UserRepository::new(app.db.pool());
    let (packages_count, comments_count, likes_received, downloads_total) = repo.stats_of(uid).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let mut achievements: Vec<serde_json::Value> = Vec::new();
    if packages_count >= 1 {
        achievements.push(serde_json::json!({"key":"first_contribution","title":"首次贡献","level":1}));
    }
    if packages_count >= 10 {
        achievements.push(serde_json::json!({"key":"prolific_author","title":"多产作者","level":2}));
    }
    if likes_received >= 50 {
        achievements.push(serde_json::json!({"key":"well_liked","title":"备受喜爱","level":1}));
    }
    if downloads_total >= 1000 {
        achievements.push(serde_json::json!({"key":"popular_author","title":"人气作者","level":1}));
    }
    if comments_count >= 20 {
        achievements.push(serde_json::json!({"key":"community_voice","title":"积极发言","level":1}));
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": achievements})))
}

async fn get_activity_stats(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let pool = app.db.pool();
    // 最近7天
    let pkg_rows = sqlx::query(
        "SELECT DATE(created_at) as d, COUNT(1) as c FROM packages WHERE author_id = ? AND DATE(created_at) >= DATE('now','-6 days') GROUP BY DATE(created_at)"
    ).bind(uid).fetch_all(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let post_rows = sqlx::query(
        "SELECT DATE(created_at) as d, COUNT(1) as c FROM posts WHERE author_id = ? AND DATE(created_at) >= DATE('now','-6 days') GROUP BY DATE(created_at)"
    ).bind(uid).fetch_all(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let cmt_rows = sqlx::query(
        "SELECT DATE(created_at) as d, COUNT(1) as c FROM comments WHERE user_id = ? AND DATE(created_at) >= DATE('now','-6 days') GROUP BY DATE(created_at)"
    ).bind(uid).fetch_all(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let dl_rows = sqlx::query(
        "SELECT DATE(created_at) as d, COUNT(1) as c FROM downloads WHERE user_id = ? AND DATE(created_at) >= DATE('now','-6 days') GROUP BY DATE(created_at)"
    ).bind(uid).fetch_all(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let likes_pkg_rows = sqlx::query(
        "SELECT DATE(l.created_at) as d, COUNT(1) as c FROM likes l JOIN packages p ON l.target_type='package' AND l.target_id=p.id WHERE p.author_id = ? AND DATE(l.created_at) >= DATE('now','-6 days') GROUP BY DATE(l.created_at)"
    ).bind(uid).fetch_all(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let likes_post_rows = sqlx::query(
        "SELECT DATE(l.created_at) as d, COUNT(1) as c FROM likes l JOIN posts p ON l.target_type='post' AND l.target_id=p.id WHERE p.author_id = ? AND DATE(l.created_at) >= DATE('now','-6 days') GROUP BY DATE(l.created_at)"
    ).bind(uid).fetch_all(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    use std::collections::HashMap;
    let mut pkg_map: HashMap<String,i64> = HashMap::new();
    for r in pkg_rows { pkg_map.insert(r.try_get::<String, _>("d").unwrap_or_default(), r.try_get::<i64, _>("c").unwrap_or(0)); }
    let mut post_map: HashMap<String,i64> = HashMap::new();
    for r in post_rows { post_map.insert(r.try_get::<String, _>("d").unwrap_or_default(), r.try_get::<i64, _>("c").unwrap_or(0)); }
    let mut cmt_map: HashMap<String,i64> = HashMap::new();
    for r in cmt_rows { cmt_map.insert(r.try_get::<String, _>("d").unwrap_or_default(), r.try_get::<i64, _>("c").unwrap_or(0)); }
    let mut dl_map: HashMap<String,i64> = HashMap::new();
    for r in dl_rows { dl_map.insert(r.try_get::<String, _>("d").unwrap_or_default(), r.try_get::<i64, _>("c").unwrap_or(0)); }
    let mut likes_map: HashMap<String,i64> = HashMap::new();
    for r in likes_pkg_rows { let d = r.try_get::<String, _>("d").unwrap_or_default(); let c = r.try_get::<i64, _>("c").unwrap_or(0); *likes_map.entry(d).or_insert(0) += c; }
    for r in likes_post_rows { let d = r.try_get::<String, _>("d").unwrap_or_default(); let c = r.try_get::<i64, _>("c").unwrap_or(0); *likes_map.entry(d).or_insert(0) += c; }

    // 组装最近7日数组，缺省为0
    let mut days: Vec<String> = Vec::new();
    for i in (0..7).rev() {
        let row = sqlx::query("SELECT DATE('now', ?)").bind(format!("-{} days", i)).fetch_one(pool).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        let d: String = row.try_get::<String, _>(0).unwrap_or_default();
        days.push(d);
    }
    let mut series: Vec<serde_json::Value> = Vec::new();
    for d in days {
        series.push(serde_json::json!({
            "date": d,
            "packages": pkg_map.get(&d).cloned().unwrap_or(0),
            "posts": post_map.get(&d).cloned().unwrap_or(0),
            "comments": cmt_map.get(&d).cloned().unwrap_or(0),
            "downloads": dl_map.get(&d).cloned().unwrap_or(0),
            "likes_received": likes_map.get(&d).cloned().unwrap_or(0)
        }));
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": series})))
}

#[derive(Deserialize)]
struct PaginationQuery { page: Option<i64>, pageSize: Option<i64> }

async fn get_my_resources(app: web::Data<crate::core::AppState>, req: HttpRequest, query: web::Query<PaginationQuery>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.pageSize.unwrap_or(20).clamp(1, 100);
    
    let result = app.services.resource_service.get_user_resources(uid, page, page_size).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": result})))
}

async fn get_my_posts(app: web::Data<crate::core::AppState>, req: HttpRequest, query: web::Query<PaginationQuery>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.pageSize.unwrap_or(20).clamp(1, 100);
    
    let result = app.services.post_service.get_user_posts(uid, page, page_size).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": result})))
}

async fn get_my_comments(app: web::Data<crate::core::AppState>, req: HttpRequest, query: web::Query<PaginationQuery>) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.pageSize.unwrap_or(20).clamp(1, 100);
    
    let pool = app.db.pool();
    let offset = (page - 1) * page_size;
    
    // 获取总数
    let (total,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM comments WHERE user_id = ? AND status != 'deleted'")
        .bind(uid)
        .fetch_one(pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    // 获取评论列表
    let comments = sqlx::query_as::<_, (i64, i64, String, i64, String, Option<i64>, i64, i64, String)>(
        "SELECT id, user_id, target_type, target_id, content, parent_id, likes_count, helpful_count, created_at FROM comments WHERE user_id = ? AND status != 'deleted' ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(uid)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    let list: Vec<serde_json::Value> = comments.into_iter().map(|(id, user_id, target_type, target_id, content, parent_id, likes_count, helpful_count, created_at)| {
        serde_json::json!({
            "id": id,
            "user_id": user_id,
            "target_type": target_type,
            "target_id": target_id,
            "content": content,
            "parent_id": parent_id,
            "likes_count": likes_count,
            "helpful_count": helpful_count,
            "created_at": created_at
        })
    }).collect();
    
    let result = serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
        "totalPages": (total + page_size - 1) / page_size
    });
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": result})))
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