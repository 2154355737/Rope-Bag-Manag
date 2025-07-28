use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use lettre::message::{header, MultiPart, SinglePart};
use log::{info, error, warn};
use std::collections::HashMap;
use chrono::Utc;
use std::time::Duration;

use crate::models::mail::{MailSettings, MailLog, MailType, MailStatus};
use crate::repositories::mail_repo::MailRepository;

/// SMTP邮箱提供商配置
#[derive(Debug, Clone)]
pub struct SmtpProvider {
    pub name: &'static str,
    pub smtp_server: &'static str,
    pub ports: Vec<u16>,
    pub ssl_modes: Vec<SslMode>,
    pub requires_app_password: bool,
}

#[derive(Debug, Clone)]
pub enum SslMode {
    None,
    Ssl,      // 465端口使用
    StartTls, // 587端口使用
}

/// 优化的SMTP邮件服务
#[derive(Clone)]
pub struct OptimizedSmtpService {
    mail_repo: MailRepository,
    providers: Vec<SmtpProvider>,
}

impl OptimizedSmtpService {
    pub fn new(mail_repo: MailRepository) -> Self {
        let providers = vec![
            SmtpProvider {
                name: "QQ邮箱",
                smtp_server: "smtp.qq.com",
                ports: vec![465, 587],
                ssl_modes: vec![SslMode::Ssl, SslMode::StartTls],
                requires_app_password: true,
            },
            SmtpProvider {
                name: "163邮箱",
                smtp_server: "smtp.163.com",
                ports: vec![465, 994, 587, 25],
                ssl_modes: vec![SslMode::Ssl, SslMode::StartTls, SslMode::None],
                requires_app_password: true,
            },
            SmtpProvider {
                name: "Gmail",
                smtp_server: "smtp.gmail.com",
                ports: vec![587, 465],
                ssl_modes: vec![SslMode::StartTls, SslMode::Ssl],
                requires_app_password: true,
            },
            SmtpProvider {
                name: "Outlook/Hotmail",
                smtp_server: "smtp-mail.outlook.com",
                ports: vec![587],
                ssl_modes: vec![SslMode::StartTls],
                requires_app_password: true,
            },
            SmtpProvider {
                name: "126邮箱",
                smtp_server: "smtp.126.com",
                ports: vec![465, 587, 25],
                ssl_modes: vec![SslMode::Ssl, SslMode::StartTls, SslMode::None],
                requires_app_password: true,
            },
        ];

        Self { mail_repo, providers }
    }

    /// 发送邮件 - 智能SMTP配置
    pub async fn send_mail(&self, to_email: &str, subject: &str, content: &str, mail_type: MailType) -> Result<i64> {
        info!("🚀 启动智能SMTP邮件发送: {} -> {}", subject, to_email);

        // 创建发送记录
        let log = MailLog {
            id: None,
            to_email: to_email.to_string(),
            subject: subject.to_string(),
            mail_type: mail_type.clone(),
            status: MailStatus::Pending,
            error_message: None,
            retry_count: 0,
            sent_at: None,
            created_at: Some(Utc::now()),
        };

        let log_id = self.mail_repo.log_mail(&log).await?;

        // 获取邮件配置
        let settings = match self.mail_repo.get_mail_settings().await? {
            Some(settings) if settings.enabled => settings,
            _ => {
                let error_msg = "邮件服务未启用或配置无效";
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.to_string())).await?;
                return Err(anyhow::anyhow!(error_msg));
            }
        };

        // 验证配置完整性
        if let Err(e) = self.validate_settings(&settings) {
            self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(e.to_string())).await?;
            return Err(e);
        }

        // 智能发送邮件
        match self.smart_send(&settings, to_email, subject, content).await {
            Ok(response) => {
                info!("✅ 邮件发送成功: {}", response);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Sent, None).await?;
                Ok(log_id)
            },
            Err(e) => {
                let error_msg = self.format_error_with_suggestions(&e, &settings);
                error!("❌ 邮件发送失败: {}", error_msg);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.clone())).await?;
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    /// 验证邮件设置
    fn validate_settings(&self, settings: &MailSettings) -> Result<()> {
        if settings.username.is_empty() {
            return Err(anyhow::anyhow!("📧 邮箱地址不能为空"));
        }

        if settings.password.is_empty() {
            return Err(anyhow::anyhow!("🔑 邮箱密码/授权码不能为空"));
        }

        if !settings.username.contains("@") {
            return Err(anyhow::anyhow!("📧 邮箱地址格式不正确，必须包含@符号"));
        }

        if settings.smtp_server.is_empty() {
            return Err(anyhow::anyhow!("🌐 SMTP服务器地址不能为空"));
        }

        Ok(())
    }

    /// 智能发送邮件 - 自动尝试最佳配置
    async fn smart_send(&self, settings: &MailSettings, to_email: &str, subject: &str, content: &str) -> Result<String> {
        info!("🧠 智能SMTP配置检测开始...");

        // 检测邮箱提供商
        let provider = self.detect_provider(&settings.username);
        
        match provider {
            Some(provider) => {
                info!("📮 检测到邮箱提供商: {}", provider.name);
                self.try_provider_configs(settings, provider, to_email, subject, content).await
            },
            None => {
                info!("🔍 未识别邮箱提供商，使用通用配置");
                self.try_generic_configs(settings, to_email, subject, content).await
            }
        }
    }

    /// 检测邮箱提供商
    fn detect_provider(&self, email: &str) -> Option<&SmtpProvider> {
        let domain = email.split('@').nth(1)?;
        
        self.providers.iter().find(|provider| {
            match domain {
                "qq.com" => provider.name == "QQ邮箱",
                "163.com" => provider.name == "163邮箱",
                "gmail.com" => provider.name == "Gmail",
                "outlook.com" | "hotmail.com" | "live.com" => provider.name == "Outlook/Hotmail",
                "126.com" => provider.name == "126邮箱",
                _ => false,
            }
        })
    }

    /// 尝试特定提供商的配置
    async fn try_provider_configs(&self, settings: &MailSettings, provider: &SmtpProvider, to_email: &str, subject: &str, content: &str) -> Result<String> {
        info!("🔧 为 {} 尝试推荐配置", provider.name);

        // 为该提供商尝试所有可能的配置组合
        for &port in &provider.ports {
            for ssl_mode in &provider.ssl_modes {
                info!("🔌 尝试配置: {}:{} 模式:{:?}", provider.smtp_server, port, ssl_mode);
                
                match self.try_send_with_config(settings, provider.smtp_server, port, ssl_mode, to_email, subject, content).await {
                    Ok(response) => {
                        info!("✅ 成功使用配置: {}:{} {:?}", provider.smtp_server, port, ssl_mode);
                        return Ok(response);
                    },
                    Err(e) => {
                        warn!("⚠️ 配置失败 {}:{} {:?}: {}", provider.smtp_server, port, ssl_mode, e);
                        continue;
                    }
                }
            }
        }

        Err(anyhow::anyhow!("所有 {} 推荐配置都失败", provider.name))
    }

    /// 尝试通用配置
    async fn try_generic_configs(&self, settings: &MailSettings, to_email: &str, subject: &str, content: &str) -> Result<String> {
        let configs = [
            (settings.smtp_port, if settings.use_ssl { SslMode::Ssl } else { SslMode::StartTls }),
            (465, SslMode::Ssl),
            (587, SslMode::StartTls),
            (25, SslMode::None),
        ];

        for (port, ssl_mode) in &configs {
            info!("🔌 尝试通用配置: {}:{} {:?}", settings.smtp_server, port, ssl_mode);
            
            match self.try_send_with_config(settings, &settings.smtp_server, *port, ssl_mode, to_email, subject, content).await {
                Ok(response) => {
                    info!("✅ 成功使用通用配置: {}:{} {:?}", settings.smtp_server, port, ssl_mode);
                    return Ok(response);
                },
                Err(e) => {
                    warn!("⚠️ 通用配置失败 {}:{} {:?}: {}", settings.smtp_server, port, ssl_mode, e);
                    continue;
                }
            }
        }

        Err(anyhow::anyhow!("所有通用配置都失败"))
    }

    /// 使用特定配置尝试发送
    async fn try_send_with_config(
        &self,
        settings: &MailSettings,
        smtp_server: &str,
        port: u16,
        ssl_mode: &SslMode,
        to_email: &str,
        subject: &str,
        content: &str,
    ) -> Result<String> {
        // 创建SMTP传输
        let transport = self.create_transport(smtp_server, port, ssl_mode, &settings.username, &settings.password)?;

        // 构建邮件
        let email = self.build_email(&settings.from_name, &settings.username, to_email, subject, content)?;

        // 发送邮件
        match transport.send(&email) {
            Ok(response) => Ok(format!("邮件发送成功: {:?}", response)),
            Err(e) => Err(anyhow::anyhow!("SMTP发送失败: {}", e)),
        }
    }

    /// 创建SMTP传输
    fn create_transport(&self, smtp_server: &str, port: u16, ssl_mode: &SslMode, username: &str, password: &str) -> Result<SmtpTransport> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let transport = match ssl_mode {
            SslMode::Ssl => {
                // 使用SSL (通常465端口)
                SmtpTransport::relay(smtp_server)?
                    .port(port)
                    .credentials(creds)
                    .timeout(Some(Duration::from_secs(30)))
                    .build()
            },
            SslMode::StartTls => {
                // 使用STARTTLS (通常587端口)
                SmtpTransport::starttls_relay(smtp_server)?
                    .port(port)
                    .credentials(creds)
                    .timeout(Some(Duration::from_secs(30)))
                    .build()
            },
            SslMode::None => {
                // 不使用加密 (通常25端口)
                SmtpTransport::builder_dangerous(smtp_server)
                    .port(port)
                    .credentials(creds)
                    .timeout(Some(Duration::from_secs(30)))
                    .build()
            }
        };

        Ok(transport)
    }

    /// 构建邮件消息
    fn build_email(&self, from_name: &str, from_email: &str, to_email: &str, subject: &str, content: &str) -> Result<Message> {
        let from_addr = format!("{} <{}>", from_name, from_email);
        
        Message::builder()
            .from(from_addr.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(self.html_to_text(content))
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(content.to_string())
                    )
            )
            .map_err(|e| anyhow::anyhow!("构建邮件失败: {}", e))
    }

    /// HTML转纯文本
    fn html_to_text(&self, html: &str) -> String {
        html.replace("<br/>", "\n")
            .replace("<br>", "\n")
            .replace("</p>", "\n")
            .replace("</div>", "\n")
            .replace("</h1>", "\n")
            .replace("</h2>", "\n")
            .replace("</h3>", "\n")
            .chars()
            .filter(|c| !c.is_control() || c == &'\n')
            .collect::<String>()
            .replace("\n\n\n", "\n\n")
    }

    /// 格式化错误并提供建议
    fn format_error_with_suggestions(&self, error: &anyhow::Error, settings: &MailSettings) -> String {
        let error_msg = error.to_string().to_lowercase();
        let provider = self.detect_provider(&settings.username);

        let mut suggestions = Vec::new();

        // 根据错误类型提供建议
        if error_msg.contains("authentication") || error_msg.contains("535") {
            suggestions.push("🔑 检查邮箱用户名和密码是否正确".to_string());
            if let Some(provider) = provider {
                if provider.requires_app_password {
                    suggestions.push(format!("📱 {} 需要使用应用专用密码/授权码，不是登录密码", provider.name));
                }
            }
            suggestions.push("🌐 确认已开启SMTP服务".to_string());
        }

        if error_msg.contains("connection") || error_msg.contains("timeout") || error_msg.contains("eof") {
            suggestions.push("🔌 检查网络连接".to_string());
            suggestions.push("🚪 检查防火墙是否阻止SMTP端口".to_string());
            suggestions.push("🌐 确认SMTP服务器地址正确".to_string());
            if let Some(provider) = provider {
                suggestions.push(format!("📮 建议使用 {} 的推荐端口配置", provider.name));
            }
        }

        if error_msg.contains("tls") || error_msg.contains("ssl") {
            suggestions.push("🔐 尝试不同的SSL/TLS设置".to_string());
            suggestions.push("🔌 尝试其他端口 (465/587/25)".to_string());
        }

        let suggestion_text = if suggestions.is_empty() {
            "请检查邮箱配置".to_string()
        } else {
            format!("\n解决建议:\n{}", suggestions.join("\n"))
        };

        format!("邮件发送失败: {}{}", error, suggestion_text)
    }

    /// 使用模板发送邮件
    pub async fn send_templated_mail(&self, to_email: &str, template_type: &str, variables: HashMap<String, String>, mail_type: MailType) -> Result<i64> {
        let (subject, content) = self.mail_repo.render_template(template_type, &variables).await?;
        self.send_mail(to_email, &subject, &content, mail_type).await
    }

    /// 发送验证码邮件
    pub async fn send_verification_code(&self, to_email: &str, code: &str) -> Result<i64> {
        let mut variables = HashMap::new();
        variables.insert("code".to_string(), code.to_string());
        
        self.send_templated_mail(to_email, "verification", variables, MailType::Verification).await
    }

    /// 发送密码重置邮件
    pub async fn send_password_reset(&self, to_email: &str, reset_link: &str) -> Result<i64> {
        let mut variables = HashMap::new();
        variables.insert("reset_link".to_string(), reset_link.to_string());
        
        self.send_templated_mail(to_email, "reset_password", variables, MailType::ResetPassword).await
    }

    /// 发送资源通知邮件
    pub async fn send_resource_notification(&self, to_email: &str, resource_name: &str, resource_description: &str, resource_link: &str) -> Result<i64> {
        let mut variables = HashMap::new();
        variables.insert("resource_name".to_string(), resource_name.to_string());
        variables.insert("resource_description".to_string(), resource_description.to_string());
        variables.insert("resource_link".to_string(), resource_link.to_string());
        
        self.send_templated_mail(to_email, "notification", variables, MailType::Notification).await
    }

    /// 发送测试邮件
    pub async fn send_test_mail(&self, to_email: &str) -> Result<i64> {
        let mut variables = HashMap::new();
        variables.insert("send_time".to_string(), Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string());
        
        self.send_templated_mail(to_email, "test", variables, MailType::Test).await
    }

    /// 测试SMTP连接
    pub async fn test_connection(&self) -> Result<String> {
        let settings = match self.mail_repo.get_mail_settings().await? {
            Some(settings) => settings,
            None => return Err(anyhow::anyhow!("未找到邮件配置")),
        };

        self.validate_settings(&settings)?;

        // 尝试建立连接但不发送邮件
        let provider = self.detect_provider(&settings.username);
        
        if let Some(provider) = provider {
            info!("🔍 测试 {} 连接", provider.name);
            for &port in &provider.ports {
                for ssl_mode in &provider.ssl_modes {
                    match self.create_transport(provider.smtp_server, port, ssl_mode, &settings.username, &settings.password) {
                        Ok(transport) => {
                            if transport.test_connection().unwrap_or(false) {
                                return Ok(format!("✅ SMTP连接测试成功: {}:{} {:?}", provider.smtp_server, port, ssl_mode));
                            }
                        },
                        Err(_) => continue,
                    }
                }
            }
        }

        Err(anyhow::anyhow!("❌ SMTP连接测试失败"))
    }

    /// 兼容性方法
    pub async fn send(&self, to_email: &str, subject: &str, content: &str) -> Result<()> {
        self.send_mail(to_email, subject, content, MailType::Test).await?;
        Ok(())
    }

    /// 重新加载配置
    pub async fn reload_config(&self) -> Result<()> {
        info!("🔄 优化SMTP服务配置已重新加载");
        Ok(())
    }
} 