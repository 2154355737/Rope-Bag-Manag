use anyhow::Result;
use rusqlite::{Connection, params};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::models::mail::{MailSettings, MailLog, MailTemplate, MailType, MailStatus, MailStats};

#[derive(Clone)]
pub struct MailRepository {
    db_path: String,
}

impl MailRepository {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    fn get_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        Ok(conn)
    }

    // 邮件配置管理
    pub async fn get_mail_settings(&self) -> Result<Option<MailSettings>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, smtp_server, smtp_port, username, password, from_name, enabled, use_ssl, auth_required, created_at, updated_at 
             FROM mail_settings WHERE id = 1"
        )?;

        let mut rows = stmt.query_map([], |row| {
            let created_at_str: Option<String> = row.get(9)?;
            let updated_at_str: Option<String> = row.get(10)?;
            
            Ok(MailSettings {
                id: Some(row.get(0)?),
                smtp_server: row.get(1)?,
                smtp_port: row.get::<_, u16>(2)?,
                username: row.get(3)?,
                password: row.get(4)?,
                from_name: row.get(5)?,
                enabled: row.get::<_, i32>(6)? == 1,
                use_ssl: row.get::<_, i32>(7)? == 1,
                auth_required: row.get::<_, i32>(8)? == 1,
                created_at: created_at_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
                updated_at: updated_at_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    pub async fn save_mail_settings(&self, settings: &MailSettings) -> Result<()> {
        let conn = self.get_connection()?;
        
        // 使用 UPSERT 语句
        conn.execute(
            "INSERT INTO mail_settings (id, smtp_server, smtp_port, username, password, from_name, enabled, use_ssl, auth_required, updated_at)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET
                smtp_server = excluded.smtp_server,
                smtp_port = excluded.smtp_port,
                username = excluded.username,
                password = excluded.password,
                from_name = excluded.from_name,
                enabled = excluded.enabled,
                use_ssl = excluded.use_ssl,
                auth_required = excluded.auth_required,
                updated_at = datetime('now')",
            params![
                settings.smtp_server,
                settings.smtp_port,
                settings.username,
                settings.password,
                settings.from_name,
                if settings.enabled { 1 } else { 0 },
                if settings.use_ssl { 1 } else { 0 },
                if settings.auth_required { 1 } else { 0 }
            ],
        )?;

        Ok(())
    }

    // 邮件发送记录管理
    pub async fn log_mail(&self, log: &MailLog) -> Result<i64> {
        let conn = self.get_connection()?;
        
        let _result = conn.execute(
            "INSERT INTO mail_logs (to_email, subject, mail_type, status, error_message, retry_count, sent_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                log.to_email,
                log.subject,
                log.mail_type.to_string(),
                log.status.to_string(),
                log.error_message,
                log.retry_count,
                log.sent_at.map(|dt| dt.to_rfc3339())
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub async fn update_mail_log_status(&self, log_id: i64, status: MailStatus, error_message: Option<String>) -> Result<()> {
        let conn = self.get_connection()?;
        
        let sent_at = if matches!(status, MailStatus::Sent) {
            Some(Utc::now().to_rfc3339())
        } else {
            None
        };

        conn.execute(
            "UPDATE mail_logs SET status = ?1, error_message = ?2, sent_at = ?3 WHERE id = ?4",
            params![status.to_string(), error_message, sent_at, log_id],
        )?;

        Ok(())
    }

    pub async fn get_mail_logs(&self, limit: Option<i64>, mail_type: Option<MailType>) -> Result<Vec<MailLog>> {
        let conn = self.get_connection()?;
        
        let mut query = "SELECT id, to_email, subject, mail_type, status, error_message, retry_count, sent_at, created_at FROM mail_logs".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(mt) = mail_type {
            query.push_str(" WHERE mail_type = ?");
            params.push(Box::new(mt.to_string()));
        }
        
        query.push_str(" ORDER BY created_at DESC");
        
        if let Some(l) = limit {
            query.push_str(" LIMIT ?");
            params.push(Box::new(l));
        }

        let mut stmt = conn.prepare(&query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        
        let rows = stmt.query_map(&param_refs[..], |row| {
            let sent_at_str: Option<String> = row.get(7)?;
            let created_at_str: String = row.get(8)?;
            
            Ok(MailLog {
                id: Some(row.get(0)?),
                to_email: row.get(1)?,
                subject: row.get(2)?,
                mail_type: MailType::from(row.get::<_, String>(3)?.as_str()),
                status: MailStatus::from(row.get::<_, String>(4)?.as_str()),
                error_message: row.get(5)?,
                retry_count: row.get(6)?,
                sent_at: sent_at_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
                created_at: DateTime::parse_from_rfc3339(&created_at_str).ok().map(|dt| dt.with_timezone(&Utc)),
            })
        })?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(row?);
        }

        Ok(logs)
    }

    // 邮件模板管理
    pub async fn get_mail_template(&self, template_type: &str) -> Result<Option<MailTemplate>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, template_type, subject, content, variables, enabled, created_at, updated_at 
             FROM mail_templates WHERE template_type = ?1 AND enabled = 1"
        )?;

        let mut rows = stmt.query_map(params![template_type], |row| {
            let created_at_str: String = row.get(6)?;
            let updated_at_str: String = row.get(7)?;
            
            Ok(MailTemplate {
                id: Some(row.get(0)?),
                template_type: row.get(1)?,
                subject: row.get(2)?,
                content: row.get(3)?,
                variables: row.get(4)?,
                enabled: row.get::<_, i32>(5)? == 1,
                created_at: DateTime::parse_from_rfc3339(&created_at_str).ok().map(|dt| dt.with_timezone(&Utc)),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str).ok().map(|dt| dt.with_timezone(&Utc)),
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    pub async fn render_template(&self, template_type: &str, variables: &HashMap<String, String>) -> Result<(String, String)> {
        if let Some(template) = self.get_mail_template(template_type).await? {
            let mut subject = template.subject;
            let mut content = template.content;
            
            // 简单的模板变量替换
            for (key, value) in variables {
                let placeholder = format!("{{{{{}}}}}", key);
                subject = subject.replace(&placeholder, value);
                content = content.replace(&placeholder, value);
            }
            
            Ok((subject, content))
        } else {
            Err(anyhow::anyhow!("邮件模板 {} 不存在或未启用", template_type))
        }
    }

    // 邮件统计
    pub async fn get_mail_stats(&self) -> Result<MailStats> {
        let conn = self.get_connection()?;
        
        // 获取总体统计
        let mut stmt = conn.prepare("SELECT status, COUNT(*) FROM mail_logs GROUP BY status")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        let mut total_sent = 0i64;
        let mut total_failed = 0i64;
        for row in rows {
            let (status, count) = row?;
            match status.as_str() {
                "sent" => total_sent = count,
                "failed" => total_failed = count,
                _ => {}
            }
        }

        // 获取今日统计
        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) FROM mail_logs 
             WHERE date(created_at) = date('now') 
             GROUP BY status"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        let mut today_sent = 0i64;
        let mut today_failed = 0i64;
        for row in rows {
            let (status, count) = row?;
            match status.as_str() {
                "sent" => today_sent = count,
                "failed" => today_failed = count,
                _ => {}
            }
        }

        // 获取最后发送时间
        let mut stmt = conn.prepare("SELECT MAX(sent_at) FROM mail_logs WHERE status = 'sent'")?;
        let last_sent_at: Option<String> = stmt.query_row([], |row| row.get(0)).ok();

        Ok(MailStats {
            total_sent,
            total_failed,
            today_sent,
            today_failed,
            last_sent_at: last_sent_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
        })
    }
} 