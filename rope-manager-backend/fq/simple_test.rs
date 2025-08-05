use std::process::Command;

fn main() {
    println!("🧪 测试用户行为记录API...");
    
    // 使用Windows的curl命令
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Invoke-RestMethod -Uri 'http://127.0.0.1:15201/api/v1/user-actions?page=1&page_size=5' -Method Get -ContentType 'application/json'")
        .output();
    
    match output {
        Ok(result) => {
            println!("✅ HTTP请求成功");
            println!("状态: {}", result.status);
            println!("输出: {}", String::from_utf8_lossy(&result.stdout));
            if !result.stderr.is_empty() {
                println!("错误: {}", String::from_utf8_lossy(&result.stderr));
            }
        },
        Err(e) => {
            println!("❌ HTTP请求失败: {}", e);
        }
    }
} 