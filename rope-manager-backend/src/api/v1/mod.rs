pub mod admin;
pub mod auth;
pub mod cache; // 添加缓存管理模块
pub mod category;
pub mod community;
pub mod package;
pub mod user;

use actix_web::web;
use crate::middleware::role_guard::RoleGuard;
use crate::models::user::UserRole;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .configure(auth::configure_routes))
        .service(
            web::scope("/users")
                .wrap(RoleGuard { required_roles: vec![UserRole::User, UserRole::Elder, UserRole::Admin] })
                .configure(user::configure_routes))
        .service(
            web::scope("/packages")
                .wrap(RoleGuard { required_roles: vec![UserRole::Admin, UserRole::Elder] })
                .configure(package::configure_routes))
        .service(
            web::scope("/admin")
                .wrap(RoleGuard { required_roles: vec![UserRole::Admin] })
                .configure(admin::configure_routes))
        .service(
            web::scope("/community")
                .wrap(RoleGuard { required_roles: vec![UserRole::User, UserRole::Elder, UserRole::Admin] })
                .configure(community::configure_routes))
        .service(
            web::scope("/categories")
                .configure(category::configure_routes))
        .service(
            web::scope("/cache")
                .configure(cache::configure_routes));
} 