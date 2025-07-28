use std::io::{self, Write};
use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};
use std::time::Duration;

#[derive(Debug)]
struct SmtpConfig {
    email: String,
    password: String,
    smtp_server: String,
    port: u16,
    use_ssl: bool,
}

#[derive(Debug)]
struct ProviderConfig {
    name: &'static str,
    smtp_server: &'static str,
    ports_ssl: Vec<u16>,
    ports_starttls: Vec<u16>,
    ports_plain: Vec<u16>,
}

fn main() {
    println!("🌟 SMTP邮箱配置测试工具 🌟");
    println!("=================================");

    // 获取用户输入
    let config = get_user_input();
    
    // 检测邮箱提供商
    let provider = detect_provider(&config.email);
    
    if let Some(provider) = provider {
        println!("\n📮 检测到邮箱提供商: {}", provider.name);
        test_provider_configs(&config, provider);
    } else {
        println!("\n🔍 未识别的邮箱提供商，使用通用配置测试");
        test_generic_configs(&config);
    }
}

fn get_user_input() -> SmtpConfig {
    print!("📧 请输入邮箱地址: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim().to_string();

    print!("🔑 请输入密码/授权码: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim().to_string();

    print!("🌐 SMTP服务器 (留空自动检测): ");
    io::stdout().flush().unwrap();
    let mut smtp_server = String::new();
    io::stdin().read_line(&mut smtp_server).unwrap();
    let smtp_server = smtp_server.trim().to_string();

    print!("🔌 端口 (留空自动尝试): ");
    io::stdout().flush().unwrap();
    let mut port_str = String::new();
    io::stdin().read_line(&mut port_str).unwrap();
    let port = port_str.trim().parse().unwrap_or(0);

    print!("🔐 使用SSL? (y/n): ");
    io::stdout().flush().unwrap();
    let mut ssl_str = String::new();
    io::stdin().read_line(&mut ssl_str).unwrap();
    let use_ssl = ssl_str.trim().to_lowercase().starts_with('y');

    let smtp_server = if smtp_server.is_empty() { 
        auto_detect_smtp_server(&email) 
    } else { 
        smtp_server 
    };

    SmtpConfig {
        email,
        password,
        smtp_server,
        port,
        use_ssl,
    }
}

fn auto_detect_smtp_server(email: &str) -> String {
    if let Some(domain) = email.split('@').nth(1) {
        match domain {
            "qq.com" => "smtp.qq.com".to_string(),
            "163.com" => "smtp.163.com".to_string(),
            "gmail.com" => "smtp.gmail.com".to_string(),
            "126.com" => "smtp.126.com".to_string(),
            "outlook.com" | "hotmail.com" | "live.com" => "smtp-mail.outlook.com".to_string(),
            _ => format!("smtp.{}", domain),
        }
    } else {
        "smtp.example.com".to_string()
    }
}

fn detect_provider(email: &str) -> Option<ProviderConfig> {
    let providers = [
        ProviderConfig {
            name: "QQ邮箱",
            smtp_server: "smtp.qq.com",
            ports_ssl: vec![465],
            ports_starttls: vec![587],
            ports_plain: vec![],
        },
        ProviderConfig {
            name: "163邮箱", 
            smtp_server: "smtp.163.com",
            ports_ssl: vec![465, 994],
            ports_starttls: vec![587],
            ports_plain: vec![25],
        },
        ProviderConfig {
            name: "Gmail",
            smtp_server: "smtp.gmail.com",
            ports_ssl: vec![465],
            ports_starttls: vec![587],
            ports_plain: vec![],
        },
        ProviderConfig {
            name: "126邮箱",
            smtp_server: "smtp.126.com", 
            ports_ssl: vec![465],
            ports_starttls: vec![587],
            ports_plain: vec![25],
        },
        ProviderConfig {
            name: "Outlook",
            smtp_server: "smtp-mail.outlook.com",
            ports_ssl: vec![],
            ports_starttls: vec![587],
            ports_plain: vec![],
        },
    ];

    if let Some(domain) = email.split('@').nth(1) {
        providers.iter().find(|provider| {
            match domain {
                "qq.com" => provider.name == "QQ邮箱",
                "163.com" => provider.name == "163邮箱",
                "gmail.com" => provider.name == "Gmail",
                "126.com" => provider.name == "126邮箱",
                "outlook.com" | "hotmail.com" | "live.com" => provider.name == "Outlook",
                _ => false,
            }
        }).cloned()
    } else {
        None
    }
}

fn test_provider_configs(config: &SmtpConfig, provider: &ProviderConfig) {
    println!("\n🔧 测试 {} 推荐配置:", provider.name);
    
    let mut success = false;

    // 测试SSL端口
    for &port in &provider.ports_ssl {
        println!("\n🔌 测试SSL配置: {}:{}", provider.smtp_server, port);
        if test_smtp_connection(provider.smtp_server, port, true, false, &config.email, &config.password) {
            println!("✅ SSL配置成功: {}:{}", provider.smtp_server, port);
            success = true;
            break;
        }
    }

    if !success {
        // 测试STARTTLS端口
        for &port in &provider.ports_starttls {
            println!("\n🔌 测试STARTTLS配置: {}:{}", provider.smtp_server, port);
            if test_smtp_connection(provider.smtp_server, port, false, true, &config.email, &config.password) {
                println!("✅ STARTTLS配置成功: {}:{}", provider.smtp_server, port);
                success = true;
                break;
            }
        }
    }

    if !success {
        // 测试明文端口
        for &port in &provider.ports_plain {
            println!("\n🔌 测试明文配置: {}:{}", provider.smtp_server, port);
            if test_smtp_connection(provider.smtp_server, port, false, false, &config.email, &config.password) {
                println!("✅ 明文配置成功: {}:{}", provider.smtp_server, port);
                success = true;
                break;
            }
        }
    }

    if !success {
        println!("\n❌ 所有 {} 推荐配置都失败", provider.name);
        print_troubleshooting_tips(provider.name);
    }
}

fn test_generic_configs(config: &SmtpConfig) {
    let test_configs = if config.port != 0 {
        vec![(config.port, config.use_ssl, !config.use_ssl)]
    } else {
        vec![
            (465, true, false),   // SSL
            (587, false, true),   // STARTTLS
            (25, false, false),   // Plain
        ]
    };

    for (port, use_ssl, use_starttls) in test_configs {
        let mode = if use_ssl { "SSL" } else if use_starttls { "STARTTLS" } else { "Plain" };
        println!("\n🔌 测试{}配置: {}:{}", mode, config.smtp_server, port);
        
        if test_smtp_connection(&config.smtp_server, port, use_ssl, use_starttls, &config.email, &config.password) {
            println!("✅ {}配置成功: {}:{}", mode, config.smtp_server, port);
            return;
        }
    }

    println!("\n❌ 所有配置都失败");
    print_generic_troubleshooting_tips();
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

fn print_troubleshooting_tips(provider_name: &str) {
    println!("\n🔧 {} 故障排除建议:", provider_name);
    
    match provider_name {
        "QQ邮箱" => {
            println!("1. 🔐 确保使用QQ邮箱授权码，不是QQ密码");
            println!("2. 📱 在QQ邮箱设置中开启SMTP服务");
            println!("3. 📧 用户名应该是完整的邮箱地址");
            println!("4. 🔌 推荐使用465端口(SSL)或587端口(STARTTLS)");
        },
        "163邮箱" => {
            println!("1. 🔐 确保使用163邮箱授权码，不是登录密码");
            println!("2. 📱 在163邮箱设置中开启SMTP服务");
            println!("3. 📧 用户名应该是完整的邮箱地址");
            println!("4. 🔌 推荐使用465端口(SSL)或587端口(STARTTLS)");
            println!("5. 🌐 某些网络可能需要使用994端口");
        },
        "Gmail" => {
            println!("1. 🔐 确保使用应用专用密码，不是Google账户密码");
            println!("2. 🔒 开启两步验证并生成应用专用密码");
            println!("3. 📧 用户名应该是完整的Gmail地址");
            println!("4. 🔌 推荐使用587端口(STARTTLS)");
        },
        _ => {
            print_generic_troubleshooting_tips();
        }
    }
}

fn print_generic_troubleshooting_tips() {
    println!("\n🔧 通用故障排除建议:");
    println!("1. 🔐 检查用户名和密码是否正确");
    println!("2. 📱 确认邮箱已开启SMTP服务");
    println!("3. 🌐 检查网络连接和防火墙设置");
    println!("4. 🔌 尝试不同的端口和加密方式");
    println!("5. 📧 确认SMTP服务器地址正确");
    println!("6. ⏰ 检查系统时间是否正确");
} 