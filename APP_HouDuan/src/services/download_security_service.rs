use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use log::{info, warn, error};
use crate::models::download_security::*;
use crate::repositories::download_security_repo::DownloadSecurityRepository;
use crate::services::security_action_service::SecurityActionService;

#[derive(Clone)]
pub struct DownloadSecurityService {
    repo: DownloadSecurityRepository,
    config: DownloadSecurityConfig,
    security_action_service: Option<SecurityActionService>,
}

impl DownloadSecurityService {
    pub fn new(db_path: &str, config: DownloadSecurityConfig) -> Result<Self> {
        let repo = DownloadSecurityRepository::new(db_path)?;
        Ok(Self { 
            repo, 
            config,
            security_action_service: None
        })
    }

    pub fn with_security_action_service(mut self, security_action_service: SecurityActionService) -> Self {
        self.security_action_service = Some(security_action_service);
        self
    }

    // 持久化下载安全配置到数据库
    pub async fn persist_config(&self, new_config: DownloadSecurityConfig) -> Result<()> {
        self.repo.save_download_security_config(&new_config).await
    }

    // 读取“有效”的配置（数据库中优先，否则回退内置）
    pub async fn get_effective_config(&self) -> Result<DownloadSecurityConfig> {
        if let Some(cfg) = self.repo.load_download_security_config().await? {
            Ok(cfg)
        } else {
            Ok(self.config.clone())
        }
    }

    // 获取合并统计（下载异常 + 视图安全统计）
    pub async fn get_combined_stats(&self, hours: i32) -> Result<serde_json::Value> {
        let download_stats = self.get_anomaly_stats(hours).await?;
        let view_stats = self.repo.get_view_security_stats(hours).await?;
        Ok(serde_json::json!({
            "download": download_stats,
            "view": view_stats,
        }))
    }

    // 检查下载是否允许（动态配置）
    pub async fn check_download_allowed(
        &self,
        user_id: Option<i32>,
        package_id: i32,
        ip_address: &str,
        user_agent: Option<&str>,
    ) -> Result<DownloadCheckResult> {
        let mut result = DownloadCheckResult {
            is_allowed: true,
            reason: None,
            remaining_downloads: None,
            cooldown_seconds: None,
            anomaly_detected: false,
            anomaly_details: None,
        };

        // 读取有效配置（DB优先）
        let effective_cfg = self.get_effective_config().await.unwrap_or(self.config.clone());

        // 0. IP封禁检查（最高优先级）
        if let Some(security_service) = &self.security_action_service {
            if let Ok(is_banned) = security_service.is_ip_banned(ip_address).await {
                if is_banned {
                    result.is_allowed = false;
                    result.reason = Some("IP已被封禁，禁止访问".to_string());
                    return Ok(result);
                }
            }
        }

        // 1. 频率限制检查
        if effective_cfg.enable_rate_limiting {
            if let Err(e) = self.check_rate_limits(user_id, package_id, ip_address, &mut result).await {
                error!("频率限制检查失败: {}", e);
            }
        }

        // 2. 异常检测
        if effective_cfg.enable_anomaly_detection && result.is_allowed {
            if let Err(e) = self.detect_anomalies(user_id, package_id, ip_address, user_agent, &mut result).await {
                error!("异常检测失败: {}", e);
            }
        }

        // 3. 统计异常检测
        if effective_cfg.enable_statistical_analysis && result.is_allowed {
            if let Err(e) = self.detect_statistical_anomalies(package_id, &mut result).await {
                error!("统计异常检测失败: {}", e);
            }
        }

        Ok(result)
    }

    // 记录下载行为
    pub async fn record_download(
        &self,
        user_id: Option<i32>,
        package_id: i32,
        ip_address: &str,
        user_agent: Option<&str>,
    ) -> Result<()> {
        let record = DownloadRecord {
            id: None,
            user_id,
            package_id,
            ip_address: ip_address.to_string(),
            user_agent: user_agent.map(|s| s.to_string()),
            download_time: Utc::now(),
            created_at: Utc::now(),
        };

        self.repo.record_download(&record).await?;
        info!("记录下载行为: user_id={:?}, package_id={}, ip={}", user_id, package_id, ip_address);
        Ok(())
    }

    // 检查频率限制
    async fn check_rate_limits(
        &self,
        user_id: Option<i32>,
        package_id: i32,
        ip_address: &str,
        result: &mut DownloadCheckResult,
    ) -> Result<()> {
        let limits = self.repo.get_rate_limits().await?;

        for limit in limits {
            match limit.rule_type.as_str() {
                "user" => {
                    if let Some(uid) = user_id {
                        let downloads = self.repo.get_user_downloads_in_window(uid, package_id, 1).await?;
                        if downloads >= limit.max_downloads {
                            result.is_allowed = false;
                            result.reason = Some(format!("用户下载频率超限: {}次/小时", limit.max_downloads));
                            result.remaining_downloads = Some(0);
                            result.cooldown_seconds = Some(3600);
                            break;
                        } else {
                            result.remaining_downloads = Some(limit.max_downloads - downloads);
                        }
                    }
                }
                "ip" => {
                    let downloads = self.repo.get_ip_downloads_in_window(ip_address, 1).await?;
                    if downloads >= limit.max_downloads {
                        result.is_allowed = false;
                        result.reason = Some(format!("IP下载频率超限: {}次/小时", limit.max_downloads));
                        result.remaining_downloads = Some(0);
                        result.cooldown_seconds = Some(3600);
                        break;
                    } else {
                        result.remaining_downloads = Some(limit.max_downloads - downloads);
                    }
                }
                "resource" => {
                    let downloads = self.repo.get_resource_downloads_in_window(package_id, 24).await?;
                    if downloads >= limit.max_downloads {
                        result.is_allowed = false;
                        result.reason = Some(format!("资源下载频率超限: {}次/天", limit.max_downloads));
                        result.remaining_downloads = Some(0);
                        result.cooldown_seconds = Some(86400);
                        break;
                    } else {
                        result.remaining_downloads = Some(limit.max_downloads - downloads);
                    }
                }
                "global" => {
                    let downloads = self.repo.get_ip_downloads_in_window(ip_address, 1).await?;
                    if downloads >= limit.max_downloads {
                        result.is_allowed = false;
                        result.reason = Some(format!("全局下载频率超限: {}次/小时", limit.max_downloads));
                        result.remaining_downloads = Some(0);
                        result.cooldown_seconds = Some(3600);
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    // 检测异常行为
    async fn detect_anomalies(
        &self,
        user_id: Option<i32>,
        package_id: i32,
        ip_address: &str,
        user_agent: Option<&str>,
        result: &mut DownloadCheckResult,
    ) -> Result<()> {
        // 检测可疑模式
        let anomaly_result = self.detect_suspicious_patterns(user_id, package_id, ip_address, user_agent).await?;
        
        if anomaly_result.is_anomaly {
            result.anomaly_detected = true;
            result.anomaly_details = anomaly_result.details.clone();
            
            // 根据严重程度决定是否阻止下载
            if let Some(severity) = &anomaly_result.severity {
                match severity.as_str() {
                    "critical" | "high" => {
                        result.is_allowed = false;
                        result.reason = Some("检测到严重异常行为，下载被阻止".to_string());
                    }
                    "medium" => {
                        warn!("检测到中等异常行为: {}", anomaly_result.details.as_deref().unwrap_or(""));
                    }
                    _ => {}
                }
            }

                                    // 记录异常
                        let anomaly = DownloadAnomaly {
                            id: None,
                            anomaly_type: anomaly_result.anomaly_type.unwrap_or_else(|| "suspicious_pattern".to_string()),
                            user_id,
                            package_id: Some(package_id),
                            ip_address: Some(ip_address.to_string()),
                            details: anomaly_result.details.clone(),
                            severity: anomaly_result.severity.unwrap_or_else(|| "medium".to_string()),
                            is_resolved: false,
                            created_at: Utc::now(),
                            resolved_at: None,
                        };

                        self.repo.record_anomaly(&anomaly).await?;

                        // 自动处理异常（如IP封禁）
                        if let Some(security_service) = &self.security_action_service {
                            if let Err(e) = security_service.handle_anomaly(&anomaly).await {
                                error!("自动处理异常失败: {}", e);
                            }
                        }
        }

        Ok(())
    }

    // 检测可疑模式（阈值动态读取）
    async fn detect_suspicious_patterns(
        &self,
        user_id: Option<i32>,
        package_id: i32,
        ip_address: &str,
        user_agent: Option<&str>,
    ) -> Result<AnomalyDetectionResult> {
        let mut confidence = 0.0;
        let mut suspicious_factors = Vec::new();

        // 1. 检测User-Agent异常
        if let Some(ua) = user_agent {
            if self.is_suspicious_user_agent(ua) {
                confidence += 0.3;
                suspicious_factors.push("可疑的User-Agent".to_string());
            }
        }

        // 2. 检测短时间内大量下载
        if let Some(uid) = user_id {
            let user_downloads = self.repo.get_user_downloads_in_window(uid, package_id, 1).await?;
            if user_downloads > 5 {
                confidence += 0.4;
                suspicious_factors.push(format!("用户1小时内下载{}次", user_downloads));
            }
        }

        // 3. 检测IP异常
        let ip_downloads = self.repo.get_ip_downloads_in_window(ip_address, 1).await?;
        if ip_downloads > 10 {
            confidence += 0.5;
            suspicious_factors.push(format!("IP 1小时内下载{}次", ip_downloads));
        }

        // 4. 检测资源异常
        let resource_downloads = self.repo.get_resource_downloads_in_window(package_id, 24).await?;
        if resource_downloads > 100 {
            confidence += 0.3;
            suspicious_factors.push(format!("资源24小时内下载{}次", resource_downloads));
        }

        // 使用动态配置阈值
        let threshold = self
            .repo
            .load_download_security_config()
            .await?
            .unwrap_or(self.config.clone())
            .suspicious_pattern_threshold;

        let is_anomaly = confidence >= threshold;
        let severity = if confidence >= 0.8 { "high" } else if confidence >= 0.6 { "medium" } else { "low" };

        Ok(AnomalyDetectionResult {
            is_anomaly,
            anomaly_type: if is_anomaly { Some("suspicious_pattern".to_string()) } else { None },
            severity: if is_anomaly { Some(severity.to_string()) } else { None },
            details: if is_anomaly { 
                Some(serde_json::json!({
                    "confidence": confidence,
                    "factors": suspicious_factors
                }).to_string())
            } else { None },
            confidence,
        })
    }

    // 检测统计异常
    async fn detect_statistical_anomalies(
        &self,
        package_id: i32,
        result: &mut DownloadCheckResult,
    ) -> Result<()> {
        // 获取最近7天的访问统计
        let stats = self.repo.get_resource_access_stats(package_id, 7).await?;
        
        if stats.len() < 3 {
            return Ok(()); // 数据不足，跳过检测
        }

        // 计算平均下载率
        let total_views: i32 = stats.iter().map(|s| s.view_count).sum();
        let total_downloads: i32 = stats.iter().map(|s| s.download_count).sum();
        
        if total_views == 0 {
            return Ok(());
        }

        let avg_download_rate = total_downloads as f64 / total_views as f64;
        
        // 如果下载率超过50%，可能是异常
        if avg_download_rate > 0.5 {
            result.anomaly_detected = true;
            result.anomaly_details = Some(format!("下载率异常: {:.2}%", avg_download_rate * 100.0));
            
            // 记录统计异常
            let anomaly = DownloadAnomaly {
                id: None,
                anomaly_type: "statistical_anomaly".to_string(),
                user_id: None,
                package_id: Some(package_id),
                ip_address: None,
                details: Some(json!({
                    "avg_download_rate": avg_download_rate,
                    "total_views": total_views,
                    "total_downloads": total_downloads
                }).to_string()),
                severity: "medium".to_string(),
                is_resolved: false,
                created_at: Utc::now(),
                resolved_at: None,
            };

            self.repo.record_anomaly(&anomaly).await?;
        }

        Ok(())
    }

    // 判断是否为可疑的User-Agent
    fn is_suspicious_user_agent(&self, user_agent: &str) -> bool {
        let suspicious_patterns = [
            "bot", "crawler", "spider", "scraper", "curl", "wget", "python", "java",
            "headless", "phantom", "selenium", "automation"
        ];

        let ua_lower = user_agent.to_lowercase();
        suspicious_patterns.iter().any(|pattern| ua_lower.contains(pattern))
    }

    // 获取异常统计
    pub async fn get_anomaly_stats(&self, hours: i32) -> Result<serde_json::Value> {
        let anomalies = self.repo.get_recent_anomalies(hours).await?;
        
        let mut stats = json!({
            "total_anomalies": anomalies.len(),
            "by_severity": {
                "critical": 0,
                "high": 0,
                "medium": 0,
                "low": 0
            },
            "by_type": {
                "rate_limit_exceeded": 0,
                "suspicious_pattern": 0,
                "statistical_anomaly": 0
            }
        });

        for anomaly in anomalies {
            // 按严重程度统计
            if let Some(severity) = stats["by_severity"].as_object_mut() {
                let current_count = severity.get(&anomaly.severity).unwrap_or(&json!(0)).as_i64().unwrap_or(0);
                *severity.get_mut(&anomaly.severity).unwrap_or(&mut json!(0)) = json!(current_count + 1);
            }

            // 按类型统计
            if let Some(types) = stats["by_type"].as_object_mut() {
                let current_count = types.get(&anomaly.anomaly_type).unwrap_or(&json!(0)).as_i64().unwrap_or(0);
                *types.get_mut(&anomaly.anomaly_type).unwrap_or(&mut json!(0)) = json!(current_count + 1);
            }
        }

        Ok(stats)
    }

    // 更新配置
    pub fn update_config(&mut self, new_config: DownloadSecurityConfig) {
        self.config = new_config;
    }

    // 获取当前配置
    pub fn get_config(&self) -> &DownloadSecurityConfig {
        &self.config
    }
} 