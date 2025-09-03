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
pub mod download_security;
pub mod security_management;

// 添加public模块
pub mod public;
// 新增通知模块
pub mod notification;
pub mod storage;
pub mod search;
// 新增发布模块
pub mod publish;
// 新增排行榜模块
pub mod ranking;
// 新增关注模块
pub mod follow;

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
        .configure(tag::configure_routes)
        .configure(download_security::configure_routes)
        .configure(security_management::configure_routes)
        .configure(notification::configure_routes)
        .configure(storage::configure_routes)
        .configure(search::configure_routes)
        .configure(publish::configure_routes) // 添加发布路由
        .configure(ranking::configure_routes) // 添加排行榜路由
        .configure(follow::configure_routes); // 添加关注路由

    // 添加公共API路由
    public::public_routes(cfg);
} 