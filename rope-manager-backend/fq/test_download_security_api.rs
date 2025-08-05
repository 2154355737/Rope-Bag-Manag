use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://127.0.0.1:15201";

    println!("🧪 测试下载安全API...");

    // 1. 测试获取安全统计（需要管理员权限）
    println!("\n📊 测试1: 获取安全统计");
    let response = client
        .get(&format!("{}/api/v1/download-security/stats", base_url))
        .send()
        .await?;

    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应: {}", body);

    // 2. 测试获取安全配置
    println!("\n⚙️ 测试2: 获取安全配置");
    let response = client
        .get(&format!("{}/api/v1/download-security/config", base_url))
        .send()
        .await?;

    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应: {}", body);

    // 3. 测试获取异常记录
    println!("\n🚨 测试3: 获取异常记录");
    let response = client
        .get(&format!("{}/api/v1/download-security/anomalies?hours=24", base_url))
        .send()
        .await?;

    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应: {}", body);

    println!("\n✅ API测试完成！");
    println!("📝 注意: 403错误是正常的，因为需要管理员权限");

    Ok(())
} 