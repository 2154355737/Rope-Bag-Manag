use anyhow::Result;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{Message, AsyncSmtpTransport, Tokio1Executor, transport::smtp::authentication::Credentials, AsyncTransport};
use log::{info, error, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

use crate::models::mail::{MailSettings, MailLog, MailType, MailStatus};
use crate::repositories::mail_repo::MailRepository;

#[derive(Clone)]
pub struct EmailService {
    mail_repo: MailRepository,
    mailer: Arc<RwLock<Option<AsyncSmtpTransport<Tokio1Executor>>>>,
    settings: Arc<RwLock<Option<MailSettings>>>,
}

impl EmailService {
    pub async fn new(mail_repo: MailRepository) -> Result<Self> {
        let service = Self {
            mail_repo,
            mailer: Arc::new(RwLock::new(None)),
            settings: Arc::new(RwLock::new(None)),
        };
        
        // 初始化配置
        service.reload_config().await?;
        Ok(service)
    }

    /// 从数据库重新加载邮件配置
    pub async fn reload_config(&self) -> Result<()> {
        info!("重新加载邮件配置...");
        
        let settings = match self.mail_repo.get_mail_settings().await? {
            Some(settings) => {
                info!("从数据库加载邮件配置成功");
                settings
            },
            None => {
                warn!("数据库中未找到邮件配置，使用默认配置");
                MailSettings::default()
            }
        };

        // 更新设置
        {
            let mut settings_lock = self.settings.write().await;
            *settings_lock = Some(settings.clone());
        }

        // 重新创建SMTP客户端
        self.create_mailer(&settings).await?;
        
        Ok(())
    }

    /// 根据配置创建SMTP客户端
    async fn create_mailer(&self, settings: &MailSettings) -> Result<()> {
        let mut mailer_lock = self.mailer.write().await;
        
        if !settings.enabled {
            warn!("邮件服务已禁用");
            *mailer_lock = None;
            return Ok(());
        }

        // 验证配置
        if settings.username.is_empty() || settings.password.is_empty() {
            warn!("邮件配置不完整，邮件功能将被禁用");
            *mailer_lock = None;
            return Ok(());
        }

        info!("初始化邮件服务: {}:{}", settings.smtp_server, settings.smtp_port);
        info!("发送方配置: {} <{}>", settings.from_name, settings.username);
        
        let creds = Credentials::new(settings.username.clone(), settings.password.clone());
        
        let transport = if settings.smtp_port == 465 && settings.use_ssl {
            // 465端口使用隐式SSL (SMTPS)
            info!("使用465端口SSL连接模式 (SMTPS)");
            AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.smtp_server)
                .map_err(|e| anyhow::anyhow!("创建SMTP SSL中继失败: {}", e))?
                .port(settings.smtp_port)
                .credentials(creds)
                .timeout(Some(std::time::Duration::from_secs(60)))
                .build()
        } else if settings.smtp_port == 587 || !settings.use_ssl {
            // 587端口或非SSL使用STARTTLS
            info!("使用{}端口STARTTLS连接模式", settings.smtp_port);
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.smtp_server)
                .map_err(|e| anyhow::anyhow!("创建STARTTLS中继失败: {}", e))?
                .port(settings.smtp_port)
                .credentials(creds)
                .timeout(Some(std::time::Duration::from_secs(60)))
                .build()
        } else {
            // 其他端口尝试STARTTLS
            info!("使用{}端口STARTTLS连接模式（非标准端口）", settings.smtp_port);
            warn!("非标准SMTP端口，可能存在兼容性问题");
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.smtp_server)
                .map_err(|e| anyhow::anyhow!("创建STARTTLS中继失败: {}", e))?
                .port(settings.smtp_port)
                .credentials(creds)
                .timeout(Some(std::time::Duration::from_secs(60)))
                .build()
        };
        
        *mailer_lock = Some(transport);
        info!("邮件服务初始化成功，准备就绪");
        Ok(())
    }

    /// 发送邮件（基础方法）
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

        // 检查邮件服务是否可用
        let mailer_guard = self.mailer.read().await;
        let mailer = match mailer_guard.as_ref() {
            Some(m) => m,
            None => {
                let error_msg = "邮件服务未启用或配置无效";
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.to_string())).await?;
                return Err(anyhow::anyhow!(error_msg));
        }
        };

        let settings_guard = self.settings.read().await;
        let settings = settings_guard.as_ref().unwrap();

        // 构建邮件
        let from_addr = format!("{} <{}>", settings.from_name, settings.username);
        
        let email = Message::builder()
            .from(from_addr.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                                    .singlepart(
                    SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                        .body(html2text::from_read(content.as_bytes(), 80).unwrap_or_else(|_| content.to_string()))
                )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(content.to_string())
                    )
            )?;
                    
        // 发送邮件
        match mailer.send(email).await {
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

    /// 格式化SMTP错误信息
    fn format_smtp_error(&self, error: &lettre::transport::smtp::Error) -> String {
        let error_msg = error.to_string().to_lowercase();
        
        if error_msg.contains("authentication failed") || error_msg.contains("535") {
            "SMTP认证失败，请检查用户名和密码（QQ邮箱需使用授权码，不是QQ密码）".to_string()
        } else if error_msg.contains("eof") || error_msg.contains("handshake") {
            "SSL握手失败，建议：1) 尝试使用587端口替代465端口；2) 检查网络防火墙设置；3) 确认SMTP服务器地址正确".to_string()
        } else if error_msg.contains("connection") || error_msg.contains("timeout") {
            "SMTP连接失败，请检查服务器地址、端口和网络连接".to_string()
        } else if error_msg.contains("invalid") && error_msg.contains("email") {
            "邮箱地址格式无效".to_string()
        } else {
            format!("SMTP错误: {}", error)
        }
    }

    /// 测试邮件配置连接
    pub async fn test_connection(&self) -> Result<String> {
        let settings_guard = self.settings.read().await;
        let settings = match settings_guard.as_ref() {
            Some(s) => s,
            None => return Err(anyhow::anyhow!("邮件服务未初始化"))
        };

        if !settings.enabled {
            return Err(anyhow::anyhow!("邮件服务已禁用"));
        }
        
        let mailer_guard = self.mailer.read().await;
        match mailer_guard.as_ref() {
            Some(_) => Ok("邮件服务连接正常".to_string()),
            None => Err(anyhow::anyhow!("SMTP连接未建立，请检查配置"))
        }
    }

    /// 获取邮件统计信息
    pub async fn get_mail_stats(&self) -> Result<crate::models::mail::MailStats> {
        self.mail_repo.get_mail_stats().await
    }

    /// 获取邮件发送日志
    pub async fn get_mail_logs(&self, limit: Option<i64>, mail_type: Option<MailType>) -> Result<Vec<MailLog>> {
        self.mail_repo.get_mail_logs(limit, mail_type).await
    }

    /// 检查邮件服务是否可用
    pub async fn is_enabled(&self) -> bool {
        let settings_guard = self.settings.read().await;
        match settings_guard.as_ref() {
            Some(settings) => settings.enabled,
            None => false
        }
    }

    /// 兼容老接口 - 简单发送
    pub async fn send(&self, to_email: &str, subject: &str, content: &str) -> Result<()> {
        self.send_mail(to_email, subject, content, MailType::Test).await?;
        Ok(())
    }
} 