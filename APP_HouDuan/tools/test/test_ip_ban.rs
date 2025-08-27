use std::process::Command;

fn main() {
    println!("🧪 测试IP封禁功能...");

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

    // 测试2: 快速连续下载（模拟刷量，触发IP封禁）
    println!("\n🚀 测试2: 快速连续下载（模拟刷量）");
    for i in 1..=10 {
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

    // 测试3: 尝试访问其他API（验证IP封禁是否生效）
    println!("\n🔒 测试3: 验证IP封禁是否生效");
    let apis = vec![
        "http://localhost:15201/api/v1/packages",
        "http://localhost:15201/api/v1/categories",
        "http://localhost:15201/api/v1/posts"
    ];

    for api in apis {
        let output = Command::new("curl")
            .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", api])
            .output();

        match output {
            Ok(output) => {
                let status = String::from_utf8_lossy(&output.stdout);
                println!("访问 {} - 状态码: {}", api, status);
                
                if status.trim() == "403" {
                    println!("✅ IP封禁生效，访问被阻止");
                } else {
                    println!("⚠️ IP封禁可能未生效或已解除");
                }
            }
            Err(e) => println!("❌ 请求失败: {}", e),
        }
    }

    println!("\n🎉 IP封禁功能测试完成！");
    println!("📊 请查看服务器日志了解详细的封禁情况");
    println!("💡 提示：如果IP被封禁，需要等待自动解除或管理员手动解除");
} 