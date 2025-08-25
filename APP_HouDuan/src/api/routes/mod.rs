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
            .configure(handlers::posts::configure)
            .configure(handlers::resources::configure)
            .configure(handlers::categories::configure)
            .configure(handlers::search::configure)
            .configure(handlers::notifications::configure)
            .configure(handlers::auth::configure)
            .configure(handlers::announcements::configure)
            .configure(handlers::me::configure)
            .configure(handlers::users::configure)
            .configure(handlers::admin::configure)
            .configure(handlers::comments::configure)
            .configure(handlers::debug::configure)
    );
} 