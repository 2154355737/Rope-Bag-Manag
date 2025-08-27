// 绳包管理器后端服务
// 简化的启动入口，主要逻辑已移至bootstrap模块

mod config;
mod middleware;
mod utils;
mod models;
mod repositories;
mod services;
mod api;
mod bootstrap;

use log::error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 运行应用启动流程
    if let Err(e) = bootstrap::Bootstrap::run().await {
        error!("❌ 应用启动失败: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}