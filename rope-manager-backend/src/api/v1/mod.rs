pub mod auth;
pub mod user;
pub mod package;
pub mod category;
pub mod comment;
pub mod admin;
pub mod cache;
pub mod community;
pub mod resource_records;
pub mod user_actions; // 添加用户行为记录API模块

use actix_web::web;
use crate::middleware::role_guard::RoleGuard;
use crate::models::user::UserRole;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    auth::configure_routes(cfg);
    user::configure_routes(cfg);
    package::configure_routes(cfg);
    category::configure_routes(cfg);
    comment::configure_routes(cfg);
    admin::configure_routes(cfg);
    cache::configure_routes(cfg);
    community::configure_routes(cfg);
    resource_records::configure_routes(cfg);
    user_actions::configure_routes(cfg); // 配置用户行为记录路由
} 