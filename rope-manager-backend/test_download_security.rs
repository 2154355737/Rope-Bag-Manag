use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://127.0.0.1:15201";
    
    println!("🧪 开始测试防刷量系统...");
    
    // 1. 测试正常下载
    println!("\n📥 测试1: 正常下载");
    let response = client
        .get(&format!("{}/api/v1/packages/1/download", base_url))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;
    
    println!("状态码: {}", response.status());
    if response.status().is_success() {
        println!("✅ 正常下载成功");
    } else {
        println!("❌ 正常下载失败: {}", response.text().await?);
    }
    
    // 2. 测试快速连续下载（模拟刷量）
    println!("\n🚀 测试2: 快速连续下载（模拟刷量）");
    for i in 1..=15 {
        let response = client
            .get(&format!("{}/api/v1/packages/1/download", base_url))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;
        
        println!("第{}次下载 - 状态码: {}", i, response.status());
        
        if response.status().as_u16() == 403 {
            println!("✅ 防刷量系统生效，第{}次下载被阻止", i);
            break;
        }
        
        // 短暂延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    // 3. 测试可疑User-Agent
    println!("\n🤖 测试3: 可疑User-Agent");
    let response = client
        .get(&format!("{}/api/v1/packages/1/download", base_url))
        .header("User-Agent", "python-requests/2.28.1")
        .send()
        .await?;
    
    println!("可疑User-Agent下载 - 状态码: {}", response.status());
    if response.status().as_u16() == 403 {
        println!("✅ 可疑User-Agent被正确识别和阻止");
    } else {
        println!("⚠️ 可疑User-Agent未被阻止");
    }
    
    // 4. 测试管理员API
    println!("\n👨‍💼 测试4: 管理员API");
    let response = client
        .get(&format!("{}/api/v1/download-security/stats", base_url))
        .send()
        .await?;
    
    println!("获取安全统计 - 状态码: {}", response.status());
    if response.status().as_u16() == 403 {
        println!("✅ 管理员API权限控制正常");
    } else {
        println!("⚠️ 管理员API权限控制可能有问题");
    }
    
    // 5. 测试不同IP的下载
    println!("\n🌐 测试5: 不同IP下载");
    let response = client
        .get(&format!("{}/api/v1/packages/1/download", base_url))
        .header("X-Forwarded-For", "192.168.1.100")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;
    
    println!("不同IP下载 - 状态码: {}", response.status());
    if response.status().is_success() {
        println!("✅ 不同IP下载成功");
    } else {
        println!("❌ 不同IP下载失败: {}", response.text().await?);
    }
    
    println!("\n🎉 防刷量系统测试完成！");
    println!("📊 请查看服务器日志了解详细的检测情况");
    
    Ok(())
} 