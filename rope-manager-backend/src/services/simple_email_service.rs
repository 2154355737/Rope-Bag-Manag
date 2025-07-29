use anyhow::Result;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use lettre::message::{header, MultiPart, SinglePart};
use log::{info, error, warn};
use std::collections::HashMap;
use chrono::Utc;

use crate::models::mail::{MailSettings, MailLog, MailType, MailStatus};
use crate::repositories::mail_repo::MailRepository;

#[derive(Clone)]
pub struct SimpleEmailService {
    mail_repo: MailRepository,
}

impl SimpleEmailService {
    pub fn new(mail_repo: MailRepository) -> Self {
        Self { mail_repo }
    }

    /// 发送邮件 - 使用同步方式，更稳定
    pub async fn send_mail(&self, to_email: &str, subject: &str, content: &str, mail_type: MailType) -> Result<i64> {
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

        // 验证配置
        if settings.username.is_empty() || settings.password.is_empty() {
            let error_msg = "邮件用户名或密码为空";
            self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.to_string())).await?;
            return Err(anyhow::anyhow!(error_msg));
        }

        info!("发送邮件: {} -> {}", subject, to_email);
        info!("使用SMTP: {}:{}", settings.smtp_server, settings.smtp_port);

        // 创建SMTP传输 - 使用同步版本
        let transport_result = self.create_transport(&settings);
        let transport = match transport_result {
            Ok(t) => t,
            Err(e) => {
                let error_msg = format!("创建SMTP连接失败: {}", e);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.clone())).await?;
                return Err(anyhow::anyhow!(error_msg));
            }
        };

        // 构建邮件
        let from_addr = format!("{} <{}>", settings.from_name, settings.username);
        
        let email_result = Message::builder()
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
            );

        let email = match email_result {
            Ok(e) => e,
            Err(e) => {
                let error_msg = format!("构建邮件失败: {}", e);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.clone())).await?;
                return Err(anyhow::anyhow!(error_msg));
            }
        };

        // 发送邮件
        match transport.send(&email) {
            Ok(response) => {
                info!("邮件发送成功: {} -> {}, 响应: {:?}", subject, to_email, response);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Sent, None).await?;
                Ok(log_id)
            },
            Err(e) => {
                let error_msg = self.format_smtp_error(&e);
                error!("邮件发送失败: {}", error_msg);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.clone())).await?;
                Err(anyhow::anyhow!("邮件发送失败: {}", error_msg))
            }
        }
    }

    /// 创建SMTP传输 - 多种配置尝试
    fn create_transport(&self, settings: &MailSettings) -> Result<SmtpTransport> {
        let creds = Credentials::new(settings.username.clone(), settings.password.clone());

        // 尝试多种配置组合
        let configs = [
            // 163邮箱推荐配置
            (465, true, "SSL"),
            (587, true, "STARTTLS"), 
            (25, false, "PLAIN"),
            (settings.smtp_port, settings.use_ssl, "CUSTOM"),
        ];

        for (port, use_ssl, desc) in &configs {
            info!("尝试SMTP配置: {}:{} ({})", settings.smtp_server, port, desc);
            
            let builder = if *use_ssl && *port == 465 {
                // SSL模式
                SmtpTransport::relay(&settings.smtp_server)?
                    .port(*port)
                    .credentials(creds.clone())
                    .timeout(Some(std::time::Duration::from_secs(30)))
                    .build()
            } else if *use_ssl {
                // STARTTLS模式
                SmtpTransport::starttls_relay(&settings.smtp_server)?
                    .port(*port)
                    .credentials(creds.clone())
                    .timeout(Some(std::time::Duration::from_secs(30)))
                    .build()
            } else {
                // 明文模式
                SmtpTransport::builder_dangerous(&settings.smtp_server)
                    .port(*port)
                    .credentials(creds.clone())
                    .timeout(Some(std::time::Duration::from_secs(30)))
                    .build()
            };

            // 测试连接
            if self.test_transport(&builder) {
                info!("SMTP连接成功: {}:{} ({})", settings.smtp_server, port, desc);
                return Ok(builder);
            }
        }

        Err(anyhow::anyhow!("所有SMTP配置都失败"))
    }

    /// 测试SMTP连接
    fn test_transport(&self, transport: &SmtpTransport) -> bool {
        match transport.test_connection() {
            Ok(true) => true,
            Ok(false) => {
                warn!("SMTP连接测试失败");
                false
            },
            Err(e) => {
                warn!("SMTP连接错误: {}", e);
                false
            }
        }
    }

    /// 简单的HTML转文本
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

    /// 格式化SMTP错误信息
    fn format_smtp_error(&self, error: &lettre::transport::smtp::Error) -> String {
        let error_msg = error.to_string().to_lowercase();
        
        if error_msg.contains("authentication") || error_msg.contains("535") {
            "SMTP认证失败。请检查：1) 邮箱地址正确 2) 使用授权码而非登录密码 3) 邮箱已开启SMTP服务".to_string()
        } else if error_msg.contains("connection") || error_msg.contains("timeout") {
            "SMTP连接失败。请检查：1) 网络连接 2) SMTP服务器地址 3) 端口设置 4) 防火墙配置".to_string()
        } else if error_msg.contains("tls") || error_msg.contains("ssl") {
            "SSL/TLS连接失败。建议：1) 尝试不同端口 2) 检查SSL设置 3) 更新邮箱服务配置".to_string()
        } else {
            format!("邮件发送错误: {}. 建议联系管理员或检查邮件服务配置", error)
        }
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

    /// 发送管理员通知邮件（新资源待审核）
    pub async fn send_admin_review_notification(&self, to_email: &str, resource_name: &str, author: &str, review_link: &str) -> Result<i64> {
        let mut variables = HashMap::new();
        variables.insert("resource_name".to_string(), resource_name.to_string());
        variables.insert("author".to_string(), author.to_string());
        variables.insert("review_link".to_string(), review_link.to_string());
        
        self.send_templated_mail(to_email, "admin_notification", variables, MailType::AdminNotification).await
    }

    /// 发送测试邮件
    pub async fn send_test_mail(&self, to_email: &str) -> Result<i64> {
        let mut variables = HashMap::new();
        variables.insert("send_time".to_string(), Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string());
        
        self.send_templated_mail(to_email, "test", variables, MailType::Test).await
    }

    /// 兼容老接口
    pub async fn send(&self, to_email: &str, subject: &str, content: &str) -> Result<()> {
        self.send_mail(to_email, subject, content, MailType::Test).await?;
        Ok(())
    }

    /// 重新加载配置
    pub async fn reload_config(&self) -> Result<()> {
        info!("简单邮件服务配置已重新加载");
        Ok(())
    }
} 