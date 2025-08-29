use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 开始激进安全测试...");
    
    let base_url = "http://127.0.0.1:15201";
    let client = Client::new();
    
    // 获取有效的包ID
    let packages_response = client.get(&format!("{}/api/v1/packages", base_url)).send().await?;
    let package_id = if packages_response.status().is_success() {
        let packages_data: Value = packages_response.json().await?;
        packages_data["data"]["list"].as_array()
            .and_then(|arr| arr.first())
            .and_then(|pkg| pkg["id"].as_i64())
            .unwrap_or(67)
    } else {
        67
    };

    println!("使用包ID: {}", package_id);

    // 测试1: 快速连续访问包详情 - 应该触发频率检测
    println!("\n🚀 测试1: 快速连续访问包详情 (50次)");
    for i in 1..=50 {
        let response = client
            .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;
        
        if i % 10 == 0 {
            println!("   已发送 {} 个请求, 状态: {}", i, response.status());
        }
        
        if !response.status().is_success() {
            println!("   🛡️ 第{}次请求被阻止: {}", i, response.status());
            break;
        }
        
        // 非常快的连续访问
        sleep(Duration::from_millis(10)).await;
    }

    // 等待一会儿
    sleep(Duration::from_secs(2)).await;

    // 测试2: 机器人User-Agent快速访问
    println!("\n🤖 测试2: 机器人User-Agent快速访问");
    for i in 1..=20 {
        let response = client
            .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
            .header("User-Agent", "curl/7.68.0")
            .send()
            .await?;
        
        if i % 5 == 0 {
            println!("   已发送 {} 个bot请求, 状态: {}", i, response.status());
        }
        
        if !response.status().is_success() {
            println!("   🛡️ 机器人第{}次请求被阻止: {}", i, response.status());
            break;
        }
        
        sleep(Duration::from_millis(50)).await;
    }

    // 获取帖子ID
    let posts_response = client.get(&format!("{}/api/v1/posts", base_url)).send().await?;
    let post_id = if posts_response.status().is_success() {
        let posts_data: Value = posts_response.json().await?;
        posts_data["data"]["list"].as_array()
            .and_then(|arr| arr.first())
            .and_then(|post| post["id"].as_i64())
            .unwrap_or(34)
    } else {
        34
    };

    println!("使用帖子ID: {}", post_id);

    // 测试3: 快速调用帖子浏览量API
    println!("\n📄 测试3: 快速调用帖子浏览量API (30次)");
    for i in 1..=30 {
        let response = client
            .post(&format!("{}/api/v1/posts/{}/view", base_url, post_id))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;
        
        if i % 5 == 0 || !response.status().is_success() {
            println!("   第{}次浏览量API请求, 状态: {}", i, response.status());
        }
        
        if response.status() == 429 {
            println!("   🛡️ 浏览量API在第{}次请求时被阻止", i);
            break;
        }
        
        sleep(Duration::from_millis(20)).await;
    }

    // 等待并检查结果
    sleep(Duration::from_secs(1)).await;
    
    println!("\n📊 检查安全日志...");
    
    // 这里我们无法直接查询数据库，但可以通过观察响应状态来判断
    println!("✅ 激进安全测试完成!");
    println!("🔍 请检查后端日志和security_logs表查看安全检测结果");
    
    Ok(())
} 