use reqwest;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ 专门测试 /resources 路径安全防护...");
    
    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:15201";
    
    // 先获取一个有效的资源ID
    let resources_response = client
        .get(&format!("{}/api/v1/resources", base_url))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await?;
    
    let resources_data: serde_json::Value = resources_response.json().await?;
    let resource_id = if let Some(list) = resources_data["data"]["list"].as_array() {
        if let Some(first_resource) = list.first() {
            first_resource["id"].as_i64().unwrap_or(1) as i32
        } else {
            println!("❌ 没有找到可用的资源，使用默认ID 1");
            1
        }
    } else {
        println!("❌ 无法解析资源列表，使用默认ID 1");
        1
    };
    
    println!("📦 使用资源ID: {}", resource_id);
    
    // 测试 /resources 路径的高频访问
    println!("\n⚡ 测试 /resources 路径高频访问防护:");
    let mut success_count = 0;
    let mut blocked_count = 0;
    
    for i in 1..=10 {
        let response = client
            .get(&format!("{}/api/v1/resources/{}", base_url, resource_id))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .send()
            .await?;
            
        let status = response.status();
        println!("   请求 {}: {} {}", i, status.as_u16(), status.canonical_reason().unwrap_or(""));
        
        if status.as_u16() == 429 {
            blocked_count += 1;
            let body = response.text().await?;
            println!("   🛡️ 被安全系统拦截: {}", body);
        } else if status.is_success() {
            success_count += 1;
        }
        
        // 短暂延迟
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    println!("\n📊 /resources 路径测试结果:");
    println!("   ✅ 成功请求: {}", success_count);
    println!("   🛡️ 被拦截: {}", blocked_count);
    
    // 测试不同User-Agent
    println!("\n🤖 测试 /resources 路径机器人检测:");
    let bot_user_agents = vec![
        "curl/7.68.0",
        "python-requests/2.25.1",
        "Wget/1.20.3",
        "bot-test-agent/1.0",
    ];
    
    for user_agent in bot_user_agents {
        let response = client
            .get(&format!("{}/api/v1/resources/{}", base_url, resource_id))
            .header("User-Agent", user_agent)
            .send()
            .await?;
            
        let status = response.status();
        println!("   User-Agent: {} -> {} {}", 
                user_agent, 
                status.as_u16(), 
                status.canonical_reason().unwrap_or(""));
                
        if status.as_u16() == 429 {
            let body = response.text().await?;
            println!("      🛡️ 被拦截: {}", body);
        }
    }
    
    // 测试快速连续访问 (模拟攻击者行为)
    println!("\n💥 模拟攻击者快速连续访问:");
    let mut attack_blocked = 0;
    let start_time = std::time::Instant::now();
    
    for i in 1..=20 {
        let response = client
            .get(&format!("{}/api/v1/resources/{}", base_url, resource_id))
            .header("User-Agent", "AttackBot/1.0")
            .send()
            .await?;
            
        let status = response.status();
        if status.as_u16() == 429 {
            attack_blocked += 1;
            if i <= 5 {  // 只显示前5次拦截信息
                let body = response.text().await?;
                println!("   攻击请求 {} 被拦截: {}", i, body);
            }
        }
        
        // 非常短的延迟，模拟真实攻击
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    
    let elapsed = start_time.elapsed();
    println!("   ⏱️  20次攻击请求耗时: {:?}", elapsed);
    println!("   🛡️ 拦截次数: {}/20", attack_blocked);
    
    // 检查安全日志
    println!("\n📋 检查安全日志记录:");
    
    // 这里可以添加检查安全日志的逻辑
    // 由于我们在内存中，无法直接查询数据库，但可以通过API间接验证
    
    println!("\n✅ /resources 路径安全测试完成!");
    
    if blocked_count > 0 || attack_blocked > 0 {
        println!("🎉 安全系统正常工作，成功拦截了 {} + {} = {} 次可疑请求", 
                blocked_count, attack_blocked, blocked_count + attack_blocked);
    } else {
        println!("⚠️  警告: 安全系统没有拦截任何请求，可能存在问题!");
    }
    
    Ok(())
} 