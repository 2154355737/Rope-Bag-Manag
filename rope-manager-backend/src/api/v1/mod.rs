pub mod admin;
pub mod auth;
pub mod user;
pub mod package;
pub mod category;
pub mod cache;
pub mod comment;
pub mod resource_records;

use actix_web::web;
use crate::middleware::role_guard::RoleGuard;
use crate::models::user::UserRole;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // 这里移除多余的/api/v1作用域，正确配置路由
    cfg.configure(auth::configure_routes)
       .configure(user::configure_routes)
       .configure(package::configure_routes)
       .configure(admin::configure_routes)
       .configure(category::configure_routes)
       .configure(cache::configure_routes)
       .configure(comment::configure_routes)
       .configure(resource_records::configure_routes);
} 