pub mod auth;
pub mod user;
pub mod package;
pub mod admin;
pub mod community;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .configure(auth::configure_routes)
    )
    .service(
        web::scope("/users")
            .configure(user::configure_routes)
    )
    .service(
        web::scope("/packages")
            .configure(package::configure_routes)
    )
    .service(
        web::scope("/admin")
            .configure(admin::configure_routes)
    )
    .service(
        web::scope("/community")
            .configure(community::configure_routes)
    );
} 