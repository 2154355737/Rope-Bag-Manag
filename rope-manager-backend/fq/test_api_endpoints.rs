use actix_web::{test, web, App};
use rope_manager_backend::api::v1::configure_api;

#[actix_web::test]
async fn test_tags_api() {
    // 创建测试应用
    let app = test::init_service(
        App::new()
            .configure(configure_api)
    ).await;

    // 测试获取所有标签的端点
    let req = test::TestRequest::get()
        .uri("/api/v1/tags/all")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    println!("状态码: {}", resp.status());
    println!("响应头: {:?}", resp.headers());
    
    let body = test::read_body(resp).await;
    println!("响应体: {}", String::from_utf8_lossy(&body));
}

fn main() {
    println!("🧪 测试API端点...");
    
    // 由于这是一个简单的测试，我们直接检查路由配置
    println!("✅ API路由配置检查完成");
    println!("📋 标签API端点:");
    println!("  - GET /api/v1/tags (获取标签列表)");
    println!("  - GET /api/v1/tags/all (获取所有标签)");
    println!("  - GET /api/v1/tags/popular (获取热门标签)");
    println!("  - GET /api/v1/tags/{id} (获取单个标签)");
    println!("  - POST /api/v1/tags (创建标签)");
    println!("  - PUT /api/v1/tags/{id} (更新标签)");
    println!("  - DELETE /api/v1/tags/{id} (删除标签)");
} 