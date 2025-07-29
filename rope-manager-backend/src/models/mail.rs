use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailSettings {
    pub id: Option<i32>,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_name: String,
    pub enabled: bool,
    pub use_ssl: bool,
    pub auth_required: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Default for MailSettings {
    fn default() -> Self {
        Self {
            id: None,
            smtp_server: "smtp.qq.com".to_string(),
            smtp_port: 465,
            username: String::new(),
            password: String::new(),
            from_name: "绳包管理器".to_string(),
            enabled: false,
            use_ssl: true,
            auth_required: true,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailLog {
    pub id: Option<i32>,
    pub to_email: String,
    pub subject: String,
    pub mail_type: MailType,
    pub status: MailStatus,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MailType {
    Verification,
    ResetPassword,
    Notification,
    AdminNotification, // 管理员通知（新资源待审核等）
    Test,
}

impl ToString for MailType {
    fn to_string(&self) -> String {
        match self {
            MailType::Verification => "verification".to_string(),
            MailType::ResetPassword => "reset_password".to_string(),
            MailType::Notification => "notification".to_string(),
            MailType::AdminNotification => "admin_notification".to_string(),
            MailType::Test => "test".to_string(),
        }
    }
}

impl From<&str> for MailType {
    fn from(s: &str) -> Self {
        match s {
            "verification" => MailType::Verification,
            "reset_password" => MailType::ResetPassword,
            "notification" => MailType::Notification,
            "admin_notification" => MailType::AdminNotification,
            "test" => MailType::Test,
            _ => MailType::Test,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MailStatus {
    Pending,
    Sent,
    Failed,
}

impl ToString for MailStatus {
    fn to_string(&self) -> String {
        match self {
            MailStatus::Pending => "pending".to_string(),
            MailStatus::Sent => "sent".to_string(),
            MailStatus::Failed => "failed".to_string(),
        }
    }
}

impl From<&str> for MailStatus {
    fn from(s: &str) -> Self {
        match s {
            "pending" => MailStatus::Pending,
            "sent" => MailStatus::Sent,
            "failed" => MailStatus::Failed,
            _ => MailStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailTemplate {
    pub id: Option<i32>,
    pub template_type: String,
    pub subject: String,
    pub content: String,
    pub variables: Option<String>,
    pub enabled: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMailRequest {
    pub to_email: String,
    pub subject: String,
    pub content: String,
    pub mail_type: MailType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailStats {
    pub total_sent: i64,
    pub total_failed: i64,
    pub today_sent: i64,
    pub today_failed: i64,
    pub last_sent_at: Option<DateTime<Utc>>,
} 