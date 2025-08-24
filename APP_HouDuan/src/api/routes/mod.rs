use actix_web::web;

use crate::api::handlers;

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(handlers::health::health_check))
            .route("/version", web::get().to(handlers::health::version))
    );
}

pub fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // 在这里添加更多API路由
            // .service(web::scope("/users"))
            // .service(web::scope("/packages"))
    );
} 