use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/announcements")
            .route("", web::get().to(list_announcements))
    );
    cfg.service(
        web::scope("/public")
            .route("/announcements", web::get().to(list_public_announcements))
    );
}

async fn list_announcements(q: web::Query<serde_json::Value>) -> Result<HttpResponse, actix_web::Error> {
    let page = q.get("page").and_then(|v| v.as_i64()).unwrap_or(1);
    let page_size = q.get("pageSize").and_then(|v| v.as_i64()).unwrap_or(20);
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "success",
        "data": {
            "list": [],
            "total": 0,
            "page": page,
            "pageSize": page_size
        }
    })))
}

async fn list_public_announcements(q: web::Query<serde_json::Value>) -> Result<HttpResponse, actix_web::Error> {
    list_announcements(q).await
} 