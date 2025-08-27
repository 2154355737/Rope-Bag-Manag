use std::io::{self, Write};
use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};
use std::time::Duration;

fn main() {
    println!("🌟 简化SMTP邮箱测试工具 🌟");
    println!("==================================");

    // 获取用户输入
    print!("📧 请输入邮箱地址: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim();

    print!("🔑 请输入密码/授权码: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    println!("\n🔧 开始测试SMTP配置...");

    // 检测邮箱类型并测试相应配置
    if email.contains("@163.com") {
        test_163_configs(email, password);
    } else if email.contains("@qq.com") {
        test_qq_configs(email, password);
    } else if email.contains("@gmail.com") {
        test_gmail_configs(email, password);
    } else {
        test_generic_configs(email, password);
    }
}

fn test_163_configs(email: &str, password: &str) {
    println!("📮 检测到163邮箱，使用推荐配置...");
    
    let configs = [
        ("smtp.163.com", 465, true, false, "SSL"),
        ("smtp.163.com", 587, false, true, "STARTTLS"),
        ("smtp.163.com", 994, true, false, "SSL(994)"),
        ("smtp.163.com", 25, false, false, "Plain"),
    ];

    for (server, port, use_ssl, use_starttls, desc) in configs {
        println!("\n🔌 测试配置: {} 端口:{} 模式:{}", server, port, desc);
        if test_smtp_connection(server, port, use_ssl, use_starttls, email, password) {
            println!("✅ 163邮箱配置成功: {}:{} ({})", server, port, desc);
            print_success_info(server, port, desc);
            return;
        }
    }

    println!("\n❌ 所有163邮箱配置都失败");
    print_163_troubleshooting();
}

fn test_qq_configs(email: &str, password: &str) {
    println!("📮 检测到QQ邮箱，使用推荐配置...");
    
    let configs = [
        ("smtp.qq.com", 465, true, false, "SSL"),
        ("smtp.qq.com", 587, false, true, "STARTTLS"),
    ];

    for (server, port, use_ssl, use_starttls, desc) in configs {
        println!("\n🔌 测试配置: {} 端口:{} 模式:{}", server, port, desc);
        if test_smtp_connection(server, port, use_ssl, use_starttls, email, password) {
            println!("✅ QQ邮箱配置成功: {}:{} ({})", server, port, desc);
            print_success_info(server, port, desc);
            return;
        }
    }

    println!("\n❌ 所有QQ邮箱配置都失败");
    print_qq_troubleshooting();
}

fn test_gmail_configs(email: &str, password: &str) {
    println!("📮 检测到Gmail，使用推荐配置...");
    
    let configs = [
        ("smtp.gmail.com", 587, false, true, "STARTTLS"),
        ("smtp.gmail.com", 465, true, false, "SSL"),
    ];

    for (server, port, use_ssl, use_starttls, desc) in configs {
        println!("\n🔌 测试配置: {} 端口:{} 模式:{}", server, port, desc);
        if test_smtp_connection(server, port, use_ssl, use_starttls, email, password) {
            println!("✅ Gmail配置成功: {}:{} ({})", server, port, desc);
            print_success_info(server, port, desc);
            return;
        }
    }

    println!("\n❌ 所有Gmail配置都失败");
    print_gmail_troubleshooting();
}

fn test_generic_configs(email: &str, password: &str) {
    println!("🔍 未识别邮箱类型，使用通用配置...");
    
    // 尝试猜测SMTP服务器
    let domain = email.split('@').nth(1).unwrap_or("example.com");
    let smtp_server = format!("smtp.{}", domain);
    
    let configs = [
        (smtp_server.as_str(), 465, true, false, "SSL"),
        (smtp_server.as_str(), 587, false, true, "STARTTLS"),
        (smtp_server.as_str(), 25, false, false, "Plain"),
    ];

    for (server, port, use_ssl, use_starttls, desc) in configs {
        println!("\n🔌 测试配置: {} 端口:{} 模式:{}", server, port, desc);
        if test_smtp_connection(server, port, use_ssl, use_starttls, email, password) {
            println!("✅ 通用配置成功: {}:{} ({})", server, port, desc);
            print_success_info(server, port, desc);
            return;
        }
    }

    println!("\n❌ 所有通用配置都失败");
    print_generic_troubleshooting();
}

fn test_smtp_connection(server: &str, port: u16, use_ssl: bool, use_starttls: bool, username: &str, password: &str) -> bool {
    let creds = Credentials::new(username.to_string(), password.to_string());

    let transport_result = if use_ssl {
        SmtpTransport::relay(server)
            .map(|builder| builder.port(port).credentials(creds).timeout(Some(Duration::from_secs(10))).build())
    } else if use_starttls {
        SmtpTransport::starttls_relay(server)
            .map(|builder| builder.port(port).credentials(creds).timeout(Some(Duration::from_secs(10))).build())
    } else {
        Ok(SmtpTransport::builder_dangerous(server)
            .port(port)
            .credentials(creds)
            .timeout(Some(Duration::from_secs(10)))
            .build())
    };

    match transport_result {
        Ok(transport) => {
            match transport.test_connection() {
                Ok(true) => {
                    println!("   ✅ 连接测试成功");
                    true
                },
                Ok(false) => {
                    println!("   ❌ 连接测试失败");
                    false
                },
                Err(e) => {
                    println!("   ❌ 连接错误: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            println!("   ❌ 创建连接失败: {}", e);
            false
        }
    }
}

fn print_success_info(server: &str, port: u16, mode: &str) {
    println!("\n🎉 恭喜！SMTP配置测试成功！");
    println!("📋 请在系统中使用以下配置:");
    println!("   SMTP服务器: {}", server);
    println!("   端口: {}", port);
    println!("   加密方式: {}", mode);
    println!("   用户名: [您的完整邮箱地址]");
    println!("   密码: [您的授权码]");
}

fn print_163_troubleshooting() {
    println!("\n🔧 163邮箱故障排除建议:");
    println!("1. 🔐 确保使用163邮箱授权码，不是登录密码");
    println!("2. 📱 在163邮箱设置中开启SMTP服务");
    println!("3. 📧 用户名应该是完整的邮箱地址");
    println!("4. 🔌 推荐使用465端口(SSL)");
    println!("5. 🌐 某些网络环境可能需要使用994端口");
    println!("6. 🌐 检查防火墙和网络连接");
}

fn print_qq_troubleshooting() {
    println!("\n🔧 QQ邮箱故障排除建议:");
    println!("1. 🔐 确保使用QQ邮箱授权码，不是QQ密码");
    println!("2. 📱 在QQ邮箱设置中开启SMTP服务");
    println!("3. 📧 用户名应该是完整的邮箱地址");
    println!("4. 🔌 推荐使用465端口(SSL)或587端口(STARTTLS)");
    println!("5. 🌐 检查防火墙和网络连接");
}

fn print_gmail_troubleshooting() {
    println!("\n🔧 Gmail故障排除建议:");
    println!("1. 🔐 确保使用应用专用密码，不是Google账户密码");
    println!("2. 🔒 开启两步验证并生成应用专用密码");
    println!("3. 📧 用户名应该是完整的Gmail地址");
    println!("4. 🔌 推荐使用587端口(STARTTLS)");
    println!("5. 🌐 检查防火墙和网络连接");
}

fn print_generic_troubleshooting() {
    println!("\n🔧 通用故障排除建议:");
    println!("1. 🔐 检查用户名和密码是否正确");
    println!("2. 📱 确认邮箱已开启SMTP服务");
    println!("3. 🌐 检查网络连接和防火墙设置");
    println!("4. 🔌 尝试不同的端口和加密方式");
    println!("5. 📧 确认SMTP服务器地址正确");
    println!("6. 💡 建议改用163邮箱或QQ邮箱");
} 