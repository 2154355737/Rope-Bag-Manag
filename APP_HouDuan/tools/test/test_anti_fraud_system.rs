use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ 开始测试反欺诈系统...");
    
    let base_url = "http://127.0.0.1:15201";
    let client = Client::new();
    
    // 测试包浏览量防护
    println!("\n📦 测试包浏览量安全防护:");
    test_package_view_security(&client, base_url).await?;
    
    // 测试帖子浏览量防护
    println!("\n📄 测试帖子浏览量安全防护:");
    test_post_view_security(&client, base_url).await?;
    
    // 测试机器人检测
    println!("\n🤖 测试机器人检测:");
    test_bot_detection(&client, base_url).await?;
    
    // 测试高频访问检测
    println!("\n⚡ 测试高频访问检测:");
    test_high_frequency_detection(&client, base_url).await?;
    
    print_test_summary();
    println!("\n✅ 所有安全测试完成!");
    Ok(())
}

async fn test_package_view_security(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("   正在获取第一个可用的包...");
    
    // 获取包列表
    let packages_response = client
        .get(&format!("{}/api/v1/packages", base_url))
        .send()
        .await?;
    
    if !packages_response.status().is_success() {
        println!("   ⚠️ 无法获取包列表，跳过包测试");
        return Ok(());
    }
    
    let packages_data: Value = packages_response.json().await?;
    let packages = packages_data["data"]["list"].as_array();
    
    if let Some(packages) = packages {
        if let Some(first_package) = packages.first() {
            let package_id = first_package["id"].as_i64().unwrap_or(1);
            println!("   使用包ID: {}", package_id);
            
            // 正常访问测试
            println!("   测试正常访问...");
            let response = client
                .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .send()
                .await?;
            
            if response.status().is_success() {
                println!("   ✅ 正常访问成功");
            } else {
                println!("   ❌ 正常访问失败: {}", response.status());
            }
            
            // 模拟频繁访问
            println!("   测试频繁访问检测...");
            let mut success_count = 0;
            let mut blocked_count = 0;
            
            for _i in 1..=15 {
                let response = client
                    .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                    .send()
                    .await?;
                
                if response.status().is_success() {
                    success_count += 1;
                } else {
                    blocked_count += 1;
                }
                
                // 快速连续访问
                sleep(Duration::from_millis(100)).await;
            }
            
            println!("   📊 频繁访问结果: 成功 {}, 被阻止 {}", success_count, blocked_count);
        }
    } else {
        println!("   ⚠️ 没有可用的包进行测试");
    }
    
    Ok(())
}

async fn test_post_view_security(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("   正在获取第一个可用的帖子...");
    
    // 获取帖子列表
    let posts_response = client
        .get(&format!("{}/api/v1/posts", base_url))
        .send()
        .await?;
    
    if !posts_response.status().is_success() {
        println!("   ⚠️ 无法获取帖子列表，跳过帖子测试");
        return Ok(());
    }
    
    let posts_data: Value = posts_response.json().await?;
    let posts = posts_data["data"]["list"].as_array();
    
    if let Some(posts) = posts {
        if let Some(first_post) = posts.first() {
            let post_id = first_post["id"].as_i64().unwrap_or(1);
            println!("   使用帖子ID: {}", post_id);
            
            // 正常访问测试
            println!("   测试正常访问...");
            let response = client
                .get(&format!("{}/api/v1/posts/{}", base_url, post_id))
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .send()
                .await?;
            
            if response.status().is_success() {
                println!("   ✅ 正常访问成功");
            } else {
                println!("   ❌ 正常访问失败: {}", response.status());
            }
            
            // 测试专用的浏览量增加API
            println!("   测试浏览量增加API...");
            let mut success_count = 0;
            let mut blocked_count = 0;
            
            for i in 1..=10 {
                let response = client
                    .post(&format!("{}/api/v1/posts/{}/view", base_url, post_id))
                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                    .send()
                    .await?;
                
                if response.status().is_success() {
                    success_count += 1;
                } else if response.status() == 429 {
                    blocked_count += 1;
                    println!("   🛡️ 第{}次请求被安全系统阻止", i);
                } else {
                    println!("   ⚠️ 第{}次请求失败: {}", i, response.status());
                }
                
                // 快速连续访问
                sleep(Duration::from_millis(50)).await;
            }
            
            println!("   📊 浏览量API测试结果: 成功 {}, 被阻止 {}", success_count, blocked_count);
        }
    } else {
        println!("   ⚠️ 没有可用的帖子进行测试");
    }
    
    Ok(())
}

async fn test_bot_detection(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("   测试机器人User-Agent检测...");
    
    // 先获取一个有效的包ID
    let packages_response = client.get(&format!("{}/api/v1/packages", base_url)).send().await?;
    let package_id = if packages_response.status().is_success() {
        let packages_data: Value = packages_response.json().await?;
        packages_data["data"]["list"].as_array()
            .and_then(|arr| arr.first())
            .and_then(|pkg| pkg["id"].as_i64())
            .unwrap_or(67) // 使用测试中发现的有效ID
    } else {
        67 // fallback到已知有效的ID
    };
    
    let bot_user_agents = vec![
        "curl/7.68.0",
        "Wget/1.20.3",
        "python-requests/2.25.1",
        "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
        "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)",
    ];
    
    for user_agent in bot_user_agents.iter() {
        println!("   测试User-Agent: {}", user_agent);
        
        let response = client
            .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
            .header("User-Agent", *user_agent)
            .send()
            .await?;
        
        println!("   响应状态: {}", response.status());
        
        // 快速连续请求以触发检测
        for _ in 0..5 {
            let response = client
                .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
                .header("User-Agent", *user_agent)
                .send()
                .await?;
            
            if !response.status().is_success() {
                println!("   🛡️ 机器人请求被阻止: {}", response.status());
                break;
            }
            
            sleep(Duration::from_millis(100)).await;
        }
    }
    
    Ok(())
}

async fn test_high_frequency_detection(client: &Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("   测试高频访问检测 (短时间内大量请求)...");
    
    // 先获取一个有效的包ID
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
    
    let mut request_results = Vec::new();
    let start_time = std::time::Instant::now();
    
    // 在1分钟内发送120个请求（每0.5秒2个请求）
    for i in 1..=120 {
        let response = client
            .get(&format!("{}/api/v1/packages/{}", base_url, package_id))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;
        
        request_results.push((i, response.status().as_u16()));
        
        if i % 10 == 0 {
            println!("   已发送 {} 个请求...", i);
        }
        
        // 控制请求频率
        sleep(Duration::from_millis(500)).await;
        
        // 如果检测到被阻止，提前结束
        if !response.status().is_success() && request_results.len() >= 10 {
            println!("   🛡️ 高频访问被检测并阻止，提前结束测试");
            break;
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("   测试耗时: {:?}", elapsed);
    
    // 统计结果
    let success_count = request_results.iter().filter(|(_, status)| *status == 200).count();
    let blocked_count = request_results.iter().filter(|(_, status)| *status == 429).count();
    let error_count = request_results.iter().filter(|(_, status)| *status != 200 && *status != 429).count();
    
    println!("   📊 高频访问测试结果:");
    println!("      成功: {}", success_count);
    println!("      被阻止: {}", blocked_count);
    println!("      错误: {}", error_count);
    println!("      总请求数: {}", request_results.len());
    
    if blocked_count > 0 {
        println!("   ✅ 高频访问检测正常工作");
    } else {
        println!("   ⚠️ 高频访问检测可能需要调整");
    }
    
    Ok(())
}

fn print_test_summary() {
    println!("\n📋 测试总结:");
    println!("   1. 包浏览量安全防护 - 检测频繁访问模式");
    println!("   2. 帖子浏览量安全防护 - 检测API滥用");
    println!("   3. 机器人检测 - 识别可疑User-Agent");
    println!("   4. 高频访问检测 - 防止短时间内大量请求");
    println!("\n🔍 检查后端日志以查看详细的安全检测信息");
    println!("🗂️ 查看security_logs表以分析安全事件");
} 