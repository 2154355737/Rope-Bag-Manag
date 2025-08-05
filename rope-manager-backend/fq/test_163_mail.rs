use rusqlite::{Connection, Result as SqlResult};
use std::io::{self, Write};

#[derive(Debug)]
struct MailConfig {
    smtp_server: String,
    smtp_port: i32,
    username: String,
    password: String,
    from_name: String,
    enabled: bool,
    use_ssl: bool,
    auth_required: bool,
}

fn load_mail_config() -> SqlResult<Option<MailConfig>> {
    let conn = Connection::open("rope-manager-backend/data.db")?;
    
    let mut stmt = conn.prepare(
        "SELECT smtp_server, smtp_port, username, password, from_name, enabled, use_ssl, auth_required 
         FROM mail_settings WHERE id = 1"
    )?;
    
    let config = stmt.query_row([], |row| {
        Ok(MailConfig {
            smtp_server: row.get(0)?,
            smtp_port: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
            from_name: row.get(4)?,
            enabled: row.get::<_, i32>(5)? == 1,
            use_ssl: row.get::<_, i32>(6)? == 1,
            auth_required: row.get::<_, i32>(7)? == 1,
        })
    }).optional()?;
    
    Ok(config)
}

fn apply_163_fix() -> SqlResult<()> {
    let conn = Connection::open("rope-manager-backend/data.db")?;
    
    println!("🔧 应用163邮箱专用配置...");
    
    // 更新为推荐的163邮箱配置
    conn.execute(
        "UPDATE mail_settings 
         SET smtp_server = 'smtp.163.com',
             smtp_port = 465,
             use_ssl = 1,
             auth_required = 1,
             enabled = 1
         WHERE id = 1",
        [],
    )?;
    
    println!("✅ 163邮箱配置已更新");
    Ok(())
}

fn suggest_alternatives() {
    println!("\n🔄 如果465端口仍然失败，请尝试以下配置：");
    println!("\n方案A: 587端口 + STARTTLS");
    println!("  SMTP端口: 587");
    println!("  SSL/TLS: 启用");
    
    println!("\n方案B: 25端口 + 明文/STARTTLS");
    println!("  SMTP端口: 25");
    println!("  SSL/TLS: 禁用或启用");
    
    println!("\n方案C: 检查授权码");
    println!("  1. 登录163邮箱网页版");
    println!("  2. 设置 -> POP3/SMTP/IMAP");
    println!("  3. 开启SMTP服务并获取新的授权码");
    println!("  4. 确保使用授权码而不是登录密码");
}

fn main() -> SqlResult<()> {
    println!("🔍 163邮箱SMTP故障诊断工具");
    println!("=================================");
    
    // 加载当前配置
    match load_mail_config()? {
        Some(config) => {
            println!("\n📋 当前邮件配置:");
            println!("  SMTP服务器: {}", config.smtp_server);
            println!("  端口: {}", config.smtp_port);
            println!("  用户名: {}", config.username);
            println!("  密码状态: {}", if config.password.len() > 8 { "已设置" } else { "可能过短" });
            println!("  发送方: {}", config.from_name);
            println!("  已启用: {}", if config.enabled { "是" } else { "否" });
            println!("  SSL/TLS: {}", if config.use_ssl { "启用" } else { "禁用" });
            println!("  需要认证: {}", if config.auth_required { "是" } else { "否" });
            
            // 诊断问题
            println!("\n🔍 问题诊断:");
            
            if !config.enabled {
                println!("  ❌ 邮件服务未启用");
            }
            
            if config.username.is_empty() || config.password.is_empty() {
                println!("  ❌ 用户名或密码为空");
            }
            
            if config.smtp_port == 25 {
                println!("  ⚠️  使用25端口可能被ISP阻止");
            }
            
            if config.smtp_server != "smtp.163.com" {
                println!("  ⚠️  SMTP服务器不是163邮箱标准配置");
            }
            
            // 应用修复
            print!("\n❓ 是否应用163邮箱推荐配置? (y/N): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            if input.trim().to_lowercase() == "y" {
                apply_163_fix()?;
                
                // 显示更新后的配置
                if let Some(updated_config) = load_mail_config()? {
                    println!("\n✅ 更新后的配置:");
                    println!("  SMTP服务器: {}", updated_config.smtp_server);
                    println!("  端口: {}", updated_config.smtp_port);
                    println!("  SSL/TLS: {}", if updated_config.use_ssl { "启用" } else { "禁用" });
                    println!("  已启用: {}", if updated_config.enabled { "是" } else { "否" });
                }
                
                println!("\n🚀 请重启后端服务并测试邮件发送");
            }
            
            suggest_alternatives();
        },
        None => {
            println!("❌ 未找到邮件配置，请先通过前端界面配置邮件服务");
        }
    }
    
    println!("\n💡 如果问题仍然存在:");
    println!("1. 检查163邮箱是否开启了SMTP服务");
    println!("2. 确认使用的是授权码而不是登录密码");
    println!("3. 尝试不同的端口配置");
    println!("4. 检查防火墙和网络连接");
    
    Ok(())
} 