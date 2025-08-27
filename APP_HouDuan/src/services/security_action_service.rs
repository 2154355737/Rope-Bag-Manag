use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use log::{info, warn, error};
use serde_json::json;
use crate::models::download_security::*;
use crate::repositories::download_security_repo::DownloadSecurityRepository;

#[derive(Clone)]
pub struct SecurityActionService {
    repo: DownloadSecurityRepository,
    config: SecurityConfig,
}

impl SecurityActionService {
    pub fn new(db_path: &str, config: SecurityConfig) -> Result<Self> {
        let repo = DownloadSecurityRepository::new(db_path)?;
        Ok(Self { repo, config })
    }

    // 检查IP是否被封禁
    pub async fn is_ip_banned(&self, ip_address: &str) -> Result<bool> {
        // 检查白名单
        if self.config.enable_ip_whitelist {
            if self.repo.is_ip_whitelisted(ip_address).await? {
                return Ok(false);
            }
        }

        // 检查封禁列表
        let ban_info = self.repo.get_ip_ban_info(ip_address).await?;
        if let Some(ban) = ban_info {
            if ban.is_active {
                // 检查是否过期
                if let Some(expires_at) = ban.expires_at {
                    if expires_at < Utc::now() {
                        // 封禁已过期，自动解除
                        self.repo.deactivate_ip_ban(ip_address).await?;
                        info!("IP封禁已过期，自动解除: {}", ip_address);
                        return Ok(false);
                    }
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    // 自动处理异常
    pub async fn handle_anomaly(&self, anomaly: &DownloadAnomaly) -> Result<()> {
        let ip_address = if let Some(ip) = &anomaly.ip_address {
            ip.clone()
        } else {
            return Ok(());
        };

        // 根据异常严重程度决定处理方式
        match anomaly.severity.as_str() {
            "critical" => {
                if self.config.critical_anomaly_auto_ban {
                    self.ban_ip(&ip_address, &anomaly.anomaly_type, "critical", None).await?;
                }
            }
            "high" => {
                // 检查异常次数
                let anomaly_count = self.repo.get_ip_anomaly_count(&ip_address, 24).await?;
                if anomaly_count >= self.config.auto_ban_threshold {
                    self.ban_ip(&ip_address, &anomaly.anomaly_type, "high", Some(self.config.ban_duration_hours)).await?;
                }
            }
            "medium" => {
                // 记录警告
                self.record_security_action(
                    "user_warning",
                    "ip",
                    &ip_address,
                    &format!("检测到异常行为: {}", anomaly.anomaly_type),
                    "medium",
                    None,
                    "system"
                ).await?;
            }
            _ => {
                // 仅记录日志
                self.record_security_action(
                    "log_only",
                    "ip",
                    &ip_address,
                    &format!("检测到异常行为: {}", anomaly.anomaly_type),
                    "low",
                    None,
                    "system"
                ).await?;
            }
        }

        // 发送管理员通知
        if self.config.enable_admin_notification && anomaly.severity == "critical" {
            self.send_admin_notification(anomaly).await?;
        }

        Ok(())
    }

    // 封禁IP
    pub async fn ban_ip(&self, ip_address: &str, reason: &str, severity: &str, duration_hours: Option<i32>) -> Result<()> {
        let ban_type = if severity == "critical" { "permanent" } else { "temporary" };
        let expires_at = if let Some(hours) = duration_hours {
            Some(Utc::now() + Duration::hours(hours as i64))
        } else {
            None
        };

        let ban = IpBan {
            id: None,
            ip_address: ip_address.to_string(),
            reason: reason.to_string(),
            ban_type: ban_type.to_string(),
            duration_hours,
            created_at: Utc::now(),
            expires_at,
            is_active: true,
            created_by: Some("system".to_string()),
            notes: Some(format!("自动封禁，严重程度: {}", severity)),
        };

        self.repo.create_ip_ban(&ban).await?;
        info!("IP已封禁: {} (原因: {}, 时长: {:?}小时)", ip_address, reason, duration_hours);

        // 记录安全操作
        self.record_security_action(
            "ip_ban",
            "ip",
            ip_address,
            reason,
            severity,
            duration_hours,
            "system"
        ).await?;

        Ok(())
    }

    // 解除IP封禁
    pub async fn unban_ip(&self, ip_address: &str, admin_name: &str) -> Result<()> {
        self.repo.deactivate_ip_ban(ip_address).await?;
        info!("IP封禁已解除: {} (管理员: {})", ip_address, admin_name);

        // 记录安全操作
        self.record_security_action(
            "ip_unban",
            "ip",
            ip_address,
            "管理员手动解除封禁",
            "low",
            None,
            admin_name
        ).await?;

        Ok(())
    }

    // 添加IP到白名单
    pub async fn add_ip_to_whitelist(&self, ip_address: &str, description: &str, admin_name: &str) -> Result<()> {
        self.repo.add_ip_to_whitelist(ip_address, description, admin_name).await?;
        info!("IP已添加到白名单: {} (管理员: {})", ip_address, admin_name);
        Ok(())
    }

    // 从白名单移除IP
    pub async fn remove_ip_from_whitelist(&self, ip_address: &str, admin_name: &str) -> Result<()> {
        self.repo.remove_ip_from_whitelist(ip_address).await?;
        info!("IP已从白名单移除: {} (管理员: {})", ip_address, admin_name);
        Ok(())
    }

    // 记录安全操作
    async fn record_security_action(
        &self,
        action_type: &str,
        target_type: &str,
        target_id: &str,
        reason: &str,
        severity: &str,
        duration_hours: Option<i32>,
        created_by: &str,
    ) -> Result<()> {
        let action = SecurityAction {
            id: None,
            action_type: action_type.to_string(),
            target_type: target_type.to_string(),
            target_id: target_id.to_string(),
            reason: reason.to_string(),
            severity: severity.to_string(),
            duration_hours,
            is_active: true,
            created_at: Utc::now(),
            expires_at: duration_hours.map(|h| Utc::now() + Duration::hours(h as i64)),
            created_by: created_by.to_string(),
            notes: None,
        };

        self.repo.record_security_action(&action).await?;
        Ok(())
    }

    // 发送管理员通知
    async fn send_admin_notification(&self, anomaly: &DownloadAnomaly) -> Result<()> {
        // 这里可以集成邮件服务或其他通知方式
        warn!("严重异常需要管理员关注: {:?}", anomaly);
        Ok(())
    }

    // 获取封禁统计
    pub async fn get_ban_stats(&self) -> Result<serde_json::Value> {
        let total_bans = self.repo.get_total_ip_bans().await?;
        let active_bans = self.repo.get_active_ip_bans().await?;
        let recent_bans = self.repo.get_recent_ip_bans(24).await?;

        Ok(json!({
            "total_bans": total_bans,
            "active_bans": active_bans,
            "recent_bans_24h": recent_bans,
            "auto_ban_enabled": self.config.enable_auto_ban,
            "whitelist_enabled": self.config.enable_ip_whitelist
        }))
    }

    // 更新安全配置
    pub fn update_config(&mut self, new_config: SecurityConfig) {
        self.config = new_config;
    }

    // 获取当前配置
    pub fn get_config(&self) -> &SecurityConfig {
        &self.config
    }

    // 获取IP封禁列表
    pub async fn get_ip_bans(&self) -> Result<Vec<serde_json::Value>> {
        self.repo.get_ip_bans().await
    }

    // 获取IP白名单
    pub async fn get_ip_whitelist(&self) -> Result<Vec<serde_json::Value>> {
        self.repo.get_ip_whitelist().await
    }
} 