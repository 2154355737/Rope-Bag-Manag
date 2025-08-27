use reqwest;
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 测试 AList 服务连接...");
    
    let base_url = "http://alist.tiecode.org.cn";
    let username = "2154355737@qq.com";
    let password = "ahk12378dx";
    
    let client = reqwest::Client::new();
    
    // 1. 测试基础连接
    println!("\n📡 1. 测试基础连接...");
    match test_basic_connection(&client, base_url).await {
        Ok(_) => println!("✅ 基础连接成功"),
        Err(e) => println!("❌ 基础连接失败: {}", e),
    }
    
    // 2. 尝试登录获取 token
    println!("\n🔐 2. 尝试登录获取 token...");
    match login_and_get_token(&client, base_url, username, password).await {
        Ok(token) => {
            println!("✅ 登录成功，Token: {}", &token[..20]); // 只显示前20个字符
            
            // 3. 测试文件系统操作
            println!("\n📁 3. 测试文件系统操作...");
            test_file_operations(&client, base_url, &token).await?;
        },
        Err(e) => println!("❌ 登录失败: {}", e),
    }
    
    Ok(())
}

async fn test_basic_connection(client: &reqwest::Client, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ping_url = format!("{}/api/public/ping", base_url);
    let response = client.get(&ping_url).send().await?;
    
    println!("状态码: {}", response.status());
    let text = response.text().await?;
    println!("响应: {}", text);
    
    Ok(())
}

async fn login_and_get_token(
    client: &reqwest::Client, 
    base_url: &str, 
    username: &str, 
    password: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let login_url = format!("{}/api/auth/login", base_url);
    
    let login_data = json!({
        "username": username,
        "password": password
    });
    
    let response = client
        .post(&login_url)
        .header("Content-Type", "application/json")
        .json(&login_data)
        .send()
        .await?;
    
    let status = response.status();
    println!("登录响应状态码: {}", status);
    
    let response_text = response.text().await?;
    println!("登录响应: {}", response_text);
    
    let response_json: Value = serde_json::from_str(&response_text)?;
    
    if let Some(data) = response_json.get("data") {
        if let Some(token) = data.get("token").and_then(|t| t.as_str()) {
            return Ok(token.to_string());
        }
    }
    
    Err("无法从响应中提取 token".into())
}

async fn test_file_operations(
    client: &reqwest::Client,
    base_url: &str,
    token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // 测试获取根目录列表
    let fs_url = format!("{}/api/fs/list", base_url);
    
    let list_data = json!({
        "path": "/",
        "password": "",
        "page": 1,
        "per_page": 10
    });
    
    let response = client
        .post(&fs_url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&list_data)
        .send()
        .await?;
    
    println!("文件列表状态码: {}", response.status());
    let text = response.text().await?;
    println!("文件列表响应: {}", text);
    
    Ok(())
} 