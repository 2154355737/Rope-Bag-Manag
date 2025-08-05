use std::process::Command;

fn main() {
    println!("🧪 测试防刷量系统...");
    
    // 测试1: 正常下载
    println!("\n📥 测试1: 正常下载");
    let output = Command::new("curl")
        .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", "http://localhost:15201/api/v1/packages/1/download"])
        .output();
    
    match output {
        Ok(output) => {
            let status = String::from_utf8_lossy(&output.stdout);
            println!("状态码: {}", status);
            if status.trim() == "200" {
                println!("✅ 正常下载成功");
            } else {
                println!("❌ 正常下载失败");
            }
        }
        Err(e) => println!("❌ 请求失败: {}", e),
    }
    
    // 测试2: 快速连续下载（模拟刷量）
    println!("\n🚀 测试2: 快速连续下载（模拟刷量）");
    for i in 1..=5 {
        let output = Command::new("curl")
            .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", "http://localhost:15201/api/v1/packages/1/download"])
            .output();
        
        match output {
            Ok(output) => {
                let status = String::from_utf8_lossy(&output.stdout);
                println!("第{}次下载 - 状态码: {}", i, status);
                
                if status.trim() == "403" {
                    println!("✅ 防刷量系统生效，第{}次下载被阻止", i);
                    break;
                }
            }
            Err(e) => println!("❌ 请求失败: {}", e),
        }
        
        // 短暂延迟
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    println!("\n🎉 防刷量系统测试完成！");
    println!("📊 请查看服务器日志了解详细的检测情况");
} 