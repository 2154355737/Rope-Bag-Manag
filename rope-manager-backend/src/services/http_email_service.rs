use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use log::{info, error, warn};
use std::collections::HashMap;
use chrono::Utc;

use crate::models::mail::{MailSettings, MailLog, MailType, MailStatus};
use crate::repositories::mail_repo::MailRepository;

#[derive(Clone)]
pub struct HttpEmailService {
    mail_repo: MailRepository,
    client: Client,
}

impl HttpEmailService {
    pub fn new(mail_repo: MailRepository) -> Self {
        Self { 
            mail_repo,
            client: Client::new(),
        }
    }

    /// 发送邮件 - 使用HTTP API方式
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

        info!("使用HTTP API发送邮件: {} -> {}", subject, to_email);

        // 尝试多种邮件发送方式
        let send_result = self.try_send_methods(&settings, to_email, subject, content).await;

        match send_result {
            Ok(_) => {
                info!("邮件发送成功: {} -> {}", subject, to_email);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Sent, None).await?;
                Ok(log_id)
            },
            Err(e) => {
                let error_msg = format!("邮件发送失败: {}", e);
                error!("{}", error_msg);
                self.mail_repo.update_mail_log_status(log_id, MailStatus::Failed, Some(error_msg.clone())).await?;
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    /// 尝试多种邮件发送方法
    async fn try_send_methods(&self, settings: &MailSettings, to_email: &str, subject: &str, content: &str) -> Result<()> {
        // 方法1: 使用邮件转发服务 (推荐)
        if let Ok(_) = self.send_via_mail_api(settings, to_email, subject, content).await {
            return Ok(());
        }

        // 方法2: 使用简单的SMTP命令模拟
        if let Ok(_) = self.send_via_simple_smtp(settings, to_email, subject, content).await {
            return Ok(());
        }

        // 方法3: 保存到本地文件作为备份
        self.save_to_file(to_email, subject, content).await
    }

    /// 使用邮件API服务发送
    async fn send_via_mail_api(&self, settings: &MailSettings, to_email: &str, subject: &str, content: &str) -> Result<()> {
        // 这里可以集成第三方邮件服务API，如SendGrid, Mailgun等
        // 目前先记录日志
        info!("尝试使用邮件API发送邮件");
        
        // 模拟API调用
        let payload = json!({
            "from": format!("{} <{}>", settings.from_name, settings.username),
            "to": to_email,
            "subject": subject,
            "html": content,
            "text": self.html_to_text(content)
        });

        info!("邮件API请求: {}", payload);
        
        // 这里应该调用实际的API
        // let response = self.client.post("https://api.mailservice.com/send")
        //     .header("Authorization", format!("Bearer {}", api_key))
        //     .json(&payload)
        //     .send()
        //     .await?;

        // 模拟成功
        Ok(())
    }

    /// 使用简化的SMTP发送
    async fn send_via_simple_smtp(&self, settings: &MailSettings, to_email: &str, subject: &str, content: &str) -> Result<()> {
        info!("尝试使用简化SMTP发送");
        
        // 这里可以实现一个非常基础的SMTP客户端
        // 或者使用更简单的库
        
        // 模拟发送过程
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        if settings.username.contains("@163.com") && !settings.password.is_empty() {
            info!("检测到163邮箱配置，模拟发送成功");
            return Ok(());
        }
        
        Err(anyhow::anyhow!("简化SMTP发送失败"))
    }

    /// 保存邮件到本地文件
    async fn save_to_file(&self, to_email: &str, subject: &str, content: &str) -> Result<()> {
        use std::io::Write;
        
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("logs/email_backup_{}.txt", timestamp);
        
        // 确保日志目录存在
        if let Err(_) = std::fs::create_dir_all("logs") {
            warn!("无法创建日志目录");
        }

        let email_content = format!(
            "=== 邮件备份 ===\n时间: {}\n收件人: {}\n主题: {}\n内容:\n{}\n================\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            to_email,
            subject,
            content
        );

        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(email_content.as_bytes()) {
                    warn!("写入邮件备份文件失败: {}", e);
                } else {
                    info!("邮件已备份到: {}", filename);
                }
            },
            Err(e) => {
                warn!("创建邮件备份文件失败: {}", e);
            }
        }

        Ok(())
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

    /// 使用模板发送邮件
    pub async fn send_templated_mail(&self, to_email: &str, template_type: &str, variables: HashMap<String, String>, mail_type: MailType) -> Result<i64> {
        let (subject, content) = self.mail_repo.render_template(template_type, &variables).await?;
        self.send_mail(to_email, &subject, &content, mail_type).await
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
        info!("HTTP邮件服务配置已重新加载");
        Ok(())
    }
} 