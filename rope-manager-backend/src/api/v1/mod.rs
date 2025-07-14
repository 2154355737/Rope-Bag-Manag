pub mod auth;
pub mod user;
pub mod package;
pub mod admin;
pub mod community;

use actix_web::web;
use crate::middleware::role_guard::RoleGuard;
use crate::models::UserRole;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .configure(auth::configure_routes)
    )
    .service(
        web::scope("/users")
            .wrap(RoleGuard { required_roles: vec![UserRole::User, UserRole::Elder] })
            .configure(user::configure_routes)
    )
    .service(
        web::scope("/packages")
            .wrap(RoleGuard { required_roles: vec![UserRole::Admin, UserRole::Elder] })
            .configure(package::configure_routes)
    )
    .service(
        web::scope("/admin")
            .wrap(RoleGuard { required_roles: vec![UserRole::Admin] })
            .configure(admin::configure_routes)
    )
    .service(
        web::scope("/community")
            .wrap(RoleGuard { required_roles: vec![UserRole::User, UserRole::Elder, UserRole::Admin] })
            .configure(community::configure_routes)
    );
} 