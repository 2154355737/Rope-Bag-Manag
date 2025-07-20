pub mod v1;

use actix_web::{web, HttpResponse, Result};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(v1::configure_api)
    )
    .service(
        web::resource("/health")
            .route(web::get().to(health_check))
    );
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "绳包管理器后端服务运行正常",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
} 