use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheck {
    pub is_allowed: bool,
    pub reason: String,
    pub risk_score: i32, // 0-100，风险评分
    pub action_type: String,
}

#[derive(Debug, Clone)]
pub struct UserBehaviorPattern {
    pub user_id: Option<i32>,
    pub ip_address: String,
    pub user_agent: String,
    pub action_count_1h: i32,
    pub action_count_24h: i32,
    pub last_actions: Vec<DateTime<Utc>>,
    pub suspicious_patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResourceStats {
    pub resource_id: i32,
    pub total_views: i32,
    pub total_downloads: i32,
    pub view_download_ratio: f32,
    pub recent_activity_spike: bool,
}

pub struct AntiFraudService {
    conn: Arc<Mutex<Connection>>,
}

impl AntiFraudService {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// 检查下载行为是否合法
    pub async fn check_download_security(
        &self,
        user_id: Option<i32>,
        resource_id: i32,
        ip_address: &str,
        user_agent: &str,
    ) -> Result<SecurityCheck> {
        let mut risk_score = 0;
        let mut reasons = Vec::new();

        // 1. 检查资源下载量与访问量比例
        let resource_stats = self.get_resource_stats(resource_id).await?;
        if self.check_download_ratio_limit(&resource_stats, &mut risk_score, &mut reasons) {
            return Ok(SecurityCheck {
                is_allowed: false,
                reason: reasons.join("; "),
                risk_score,
                action_type: "download".to_string(),
            });
        }

        // 2. 检查用户/IP行为模式
        let behavior_pattern = self.analyze_user_behavior(user_id, ip_address, user_agent, "Download").await?;
        self.check_behavior_patterns(&behavior_pattern, &mut risk_score, &mut reasons);

        // 3. 检查频率限制
        self.check_frequency_limits(&behavior_pattern, &mut risk_score, &mut reasons);

        // 4. 检查异常模式
        self.check_suspicious_patterns(&behavior_pattern, &mut risk_score, &mut reasons);

        let is_allowed = risk_score < 70; // 风险评分70以下允许

        Ok(SecurityCheck {
            is_allowed,
            reason: if reasons.is_empty() { "正常".to_string() } else { reasons.join("; ") },
            risk_score,
            action_type: "download".to_string(),
        })
    }

    /// 检查访问行为是否合法
    pub async fn check_view_security(
        &self,
        user_id: Option<i32>,
        resource_id: i32,
        ip_address: &str,
        user_agent: &str,
    ) -> Result<SecurityCheck> {
        let mut risk_score = 0;
        let mut reasons = Vec::new();

        // 检查用户行为模式
        let behavior_pattern = self.analyze_user_behavior(user_id, ip_address, user_agent, "PageView").await?;
        
        // 检查访问频率
        self.check_view_frequency(&behavior_pattern, &mut risk_score, &mut reasons);
        
        // 检查机器人行为
        self.check_bot_behavior(&behavior_pattern, &mut risk_score, &mut reasons);

        let is_allowed = risk_score < 80; // 访问的容忍度稍高

        Ok(SecurityCheck {
            is_allowed,
            reason: if reasons.is_empty() { "正常".to_string() } else { reasons.join("; ") },
            risk_score,
            action_type: "view".to_string(),
        })
    }

    /// 获取资源统计信息
    async fn get_resource_stats(&self, resource_id: i32) -> Result<ResourceStats> {
        let conn = self.conn.lock().await;
        
        // 获取总访问量
        let total_views = conn.query_row(
            "SELECT COUNT(*) FROM user_actions WHERE target_id = ? AND action_type = 'PageView'",
            params![resource_id],
            |row| Ok(row.get::<_, i32>(0)?)
        ).unwrap_or(0);

        // 获取总下载量
        let total_downloads = conn.query_row(
            "SELECT COUNT(*) FROM user_actions WHERE target_id = ? AND action_type = 'Download'",
            params![resource_id],
            |row| Ok(row.get::<_, i32>(0)?)
        ).unwrap_or(0);

        // 检查最近1小时是否有异常活动激增
        let recent_activity_count = conn.query_row(
            "SELECT COUNT(*) FROM user_actions 
             WHERE target_id = ? AND created_at > datetime('now', '-1 hour')",
            params![resource_id],
            |row| Ok(row.get::<_, i32>(0)?)
        ).unwrap_or(0);

        let recent_activity_spike = recent_activity_count > 50; // 1小时超过50次活动视为异常

        let view_download_ratio = if total_views > 0 {
            total_downloads as f32 / total_views as f32
        } else {
            0.0
        };

        Ok(ResourceStats {
            resource_id,
            total_views,
            total_downloads,
            view_download_ratio,
            recent_activity_spike,
        })
    }

    /// 检查下载量与访问量比例限制
    fn check_download_ratio_limit(
        &self,
        stats: &ResourceStats,
        risk_score: &mut i32,
        reasons: &mut Vec<String>,
    ) -> bool {
        const MAX_DOWNLOAD_RATIO: f32 = 0.25; // 25%的下载率上限
        const MIN_VIEWS_FOR_CHECK: i32 = 10; // 至少10次访问才开始检查比例

        if stats.total_views < MIN_VIEWS_FOR_CHECK {
            return false; // 访问量太少，暂不检查
        }

        let max_allowed_downloads = (stats.total_views as f32 * MAX_DOWNLOAD_RATIO) as i32;
        
        if stats.total_downloads >= max_allowed_downloads {
            *risk_score += 80; // 高风险
            reasons.push(format!(
                "下载量超限：当前{}/{}（{}%），限制{}%",
                stats.total_downloads,
                stats.total_views,
                (stats.view_download_ratio * 100.0) as i32,
                (MAX_DOWNLOAD_RATIO * 100.0) as i32
            ));
            return true; // 直接阻止
        }

        // 接近限制时增加风险评分
        if stats.view_download_ratio > MAX_DOWNLOAD_RATIO * 0.8 {
            *risk_score += 30;
            reasons.push("下载率接近上限".to_string());
        }

        false
    }

    /// 分析用户行为模式
    async fn analyze_user_behavior(
        &self,
        user_id: Option<i32>,
        ip_address: &str,
        user_agent: &str,
        action_type: &str,
    ) -> Result<UserBehaviorPattern> {
        let conn = self.conn.lock().await;
        
        // 构建查询条件
        let (where_clause, params) = if let Some(uid) = user_id {
            ("WHERE user_id = ?", vec![uid.to_string()])
        } else {
            ("WHERE ip_address = ?", vec![ip_address.to_string()])
        };

        // 获取1小时内的行为次数
        let action_count_1h = conn.query_row(
            &format!(
                "SELECT COUNT(*) FROM user_actions {} AND action_type = ? AND created_at > datetime('now', '-1 hour')",
                where_clause
            ),
            params![params[0], action_type],
            |row| Ok(row.get::<_, i32>(0)?)
        ).unwrap_or(0);

        // 获取24小时内的行为次数
        let action_count_24h = conn.query_row(
            &format!(
                "SELECT COUNT(*) FROM user_actions {} AND action_type = ? AND created_at > datetime('now', '-24 hours')",
                where_clause
            ),
            params![params[0], action_type],
            |row| Ok(row.get::<_, i32>(0)?)
        ).unwrap_or(0);

        // 获取最近的行为时间
        let mut stmt = conn.prepare(&format!(
            "SELECT created_at FROM user_actions {} AND action_type = ? ORDER BY created_at DESC LIMIT 10",
            where_clause
        ))?;
        
        let last_actions = stmt.query_map(params![params[0], action_type], |row| {
            let time_str: String = row.get(0)?;
            Ok(chrono::DateTime::parse_from_rfc3339(&time_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc))
        })?.collect::<Result<Vec<_>, _>>().unwrap_or_default();

        let suspicious_patterns = self.detect_suspicious_patterns(&last_actions, action_count_1h, action_count_24h);

        Ok(UserBehaviorPattern {
            user_id,
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            action_count_1h,
            action_count_24h,
            last_actions,
            suspicious_patterns,
        })
    }

    /// 检测可疑模式
    fn detect_suspicious_patterns(
        &self,
        last_actions: &[DateTime<Utc>],
        count_1h: i32,
        count_24h: i32,
    ) -> Vec<String> {
        let mut patterns = Vec::new();

        // 检查频率过高
        if count_1h > 30 {
            patterns.push("1小时内操作过于频繁".to_string());
        }
        
        if count_24h > 200 {
            patterns.push("24小时内操作异常频繁".to_string());
        }

        // 检查时间间隔模式（机器人行为）
        if last_actions.len() >= 5 {
            let intervals: Vec<i64> = last_actions
                .windows(2)
                .map(|w| (w[0] - w[1]).num_seconds().abs())
                .collect();
            
            // 检查是否有太多相同的时间间隔（机器人特征）
            let mut interval_map = HashMap::new();
            for interval in intervals {
                *interval_map.entry(interval).or_insert(0) += 1;
            }
            
            for (interval, count) in interval_map {
                if count >= 3 && interval < 60 { // 3次以上相同间隔且小于60秒
                    patterns.push(format!("疑似机器人行为：{}秒固定间隔", interval));
                }
            }
        }

        patterns
    }

    /// 检查行为模式
    fn check_behavior_patterns(
        &self,
        pattern: &UserBehaviorPattern,
        risk_score: &mut i32,
        reasons: &mut Vec<String>,
    ) {
        // 可疑模式检查
        for suspicious in &pattern.suspicious_patterns {
            *risk_score += 25;
            reasons.push(suspicious.clone());
        }

        // User-Agent检查
        if self.is_suspicious_user_agent(&pattern.user_agent) {
            *risk_score += 20;
            reasons.push("可疑的User-Agent".to_string());
        }
    }

    /// 检查频率限制
    fn check_frequency_limits(
        &self,
        pattern: &UserBehaviorPattern,
        risk_score: &mut i32,
        reasons: &mut Vec<String>,
    ) {
        // 1小时下载次数限制
        if pattern.action_count_1h > 20 {
            *risk_score += 40;
            reasons.push(format!("1小时内下载{}次，超过限制", pattern.action_count_1h));
        }

        // 24小时下载次数限制  
        if pattern.action_count_24h > 100 {
            *risk_score += 30;
            reasons.push(format!("24小时内下载{}次，超过限制", pattern.action_count_24h));
        }
    }

    /// 检查可疑模式
    fn check_suspicious_patterns(
        &self,
        pattern: &UserBehaviorPattern,
        risk_score: &mut i32,
        reasons: &mut Vec<String>,
    ) {
        // 已在check_behavior_patterns处理
    }

    /// 检查访问频率
    fn check_view_frequency(
        &self,
        pattern: &UserBehaviorPattern,
        risk_score: &mut i32,
        reasons: &mut Vec<String>,
    ) {
        // 访问频率检查，标准比下载宽松
        if pattern.action_count_1h > 100 {
            *risk_score += 30;
            reasons.push(format!("1小时访问{}次，频率过高", pattern.action_count_1h));
        }

        if pattern.action_count_24h > 1000 {
            *risk_score += 25;
            reasons.push(format!("24小时访问{}次，频率异常", pattern.action_count_24h));
        }
    }

    /// 检查机器人行为
    fn check_bot_behavior(
        &self,
        pattern: &UserBehaviorPattern,
        risk_score: &mut i32,
        reasons: &mut Vec<String>,
    ) {
        if self.is_suspicious_user_agent(&pattern.user_agent) {
            *risk_score += 15;
            reasons.push("疑似爬虫或机器人".to_string());
        }
    }

    /// 检查是否为可疑的User-Agent
    fn is_suspicious_user_agent(&self, user_agent: &str) -> bool {
        let suspicious_patterns = [
            "bot", "crawler", "spider", "scraper", "curl", "wget", "python", "java", "go-http",
            "scrapy", "requests", "urllib", "httpx", "axios", "postman", "insomnia"
        ];
        
        let user_agent_lower = user_agent.to_lowercase();
        suspicious_patterns.iter().any(|pattern| user_agent_lower.contains(pattern))
    }

    /// 记录安全检查日志
    pub async fn log_security_check(
        &self,
        user_id: Option<i32>,
        ip_address: &str,
        action_type: &str,
        resource_id: i32,
        check_result: &SecurityCheck,
    ) -> Result<()> {
        let conn = self.conn.lock().await;
        
        conn.execute(
            "INSERT INTO security_logs (user_id, ip_address, action_type, resource_id, is_allowed, risk_score, reason, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))",
            params![
                user_id,
                ip_address,
                action_type,
                resource_id,
                check_result.is_allowed,
                check_result.risk_score,
                check_result.reason
            ]
        )?;

        Ok(())
    }
}