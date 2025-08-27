use log::error;

mod config;
mod middleware;
mod utils;
mod models;
mod repositories;
mod services;
mod api;
mod bootstrap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 使用 bootstrap 模块进行应用启动
    if let Err(e) = bootstrap::Bootstrap::run().await {
        error!("应用启动失败: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
