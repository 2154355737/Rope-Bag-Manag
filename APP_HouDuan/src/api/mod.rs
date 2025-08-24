pub mod handlers;
pub mod routes;
pub mod middleware;

use actix_web::web;

/// 配置所有API路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    routes::configure_health_routes(cfg);
    routes::configure_api_routes(cfg);
} 