use crate::config::EmailConfig;
use crate::infrastructure::database::repositories::MailSettingsRepository;
use lettre::message::{header, Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::transport::smtp::client::{Tls, TlsParameters};

pub struct EmailSender;

impl EmailSender {
    pub async fn send(
        db: &crate::infrastructure::database::DatabaseManager,
        fallback: &EmailConfig,
        to_email: &str,
        to_name: Option<&str>,
        subject: &str,
        html_body: &str,
    ) -> anyhow::Result<()> {
        // 优先从 DB mail_settings 读取有效配置
        let repo = MailSettingsRepository::new(db.pool());
        let settings_opt = repo.get().await.ok().flatten();
        let enabled = settings_opt.as_ref().map(|s| s.enabled != 0).unwrap_or(false);
        let (host, port, username, password, from_address, from_name) = if enabled {
            let s = settings_opt.unwrap();
            (
                s.smtp_host.unwrap_or_else(|| fallback.smtp_host.clone()),
                s.smtp_port.unwrap_or(fallback.smtp_port as i64) as u16,
                s.username.unwrap_or_else(|| fallback.smtp_username.clone()),
                s.password.unwrap_or_else(|| fallback.smtp_password.clone()),
                s.from_address.unwrap_or_else(|| fallback.from_address.clone()),
                s.from_name.unwrap_or_else(|| fallback.from_name.clone()),
            )
        } else {
            (
                fallback.smtp_host.clone(),
                fallback.smtp_port,
                fallback.smtp_username.clone(),
                fallback.smtp_password.clone(),
                fallback.from_address.clone(),
                fallback.from_name.clone(),
            )
        };

        let creds = Credentials::new(username.clone(), password.clone());
        // 465 端口（SMTPS）需要隐式TLS包裹
        let mut builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)?
            .port(port)
            .credentials(creds);
        if port == 465 {
            let tls = TlsParameters::new(host.clone())?;
            builder = builder.tls(Tls::Wrapper(tls));
        }
        let mailer: AsyncSmtpTransport<Tokio1Executor> = builder.build();

        // 修正发件人配置：地址必须是邮箱，名称为空则使用默认
        let effective_from_address = if from_address.contains('@') { from_address.clone() } else { username.clone() };
        let effective_from_name = if from_name.trim().is_empty() { "绳包管理器".to_string() } else { from_name.clone() };

        let from: Mailbox = Mailbox::new(Some(effective_from_name), effective_from_address.parse()?);
        let to: Mailbox = Mailbox::new(to_name.map(|s| s.to_string()), to_email.parse()?);

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .header(header::ContentType::TEXT_HTML)
            .body(html_body.to_string())?;

        mailer.send(email).await?;
        Ok(())
    }
} 