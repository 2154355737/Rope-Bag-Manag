pub mod auth;
pub mod user;
pub mod package;
pub mod category;
pub mod comment;
pub mod admin;
pub mod cache;
pub mod community;
pub mod resource_records;
pub mod user_actions; 
pub mod forbidden_words;
pub mod subscription;
pub mod post;
pub mod feed;
pub mod tag;

use actix_web::web;

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::configure_routes)
        .configure(user::configure_routes)
        .configure(package::configure_routes)
        .configure(category::configure_routes)
        .configure(comment::configure_routes)
        .configure(admin::configure_routes)
        .configure(admin::configure_user_routes) // 公告用户端路由
        .configure(cache::configure_routes)
        .configure(community::configure_routes)
        .configure(resource_records::configure_routes)
        .configure(user_actions::configure_routes)
        .configure(forbidden_words::configure_routes)
        .configure(subscription::configure_routes)
        .configure(post::configure_routes)
        .configure(feed::configure_routes)
        .configure(tag::configure_routes);
} 