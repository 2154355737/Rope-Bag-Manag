use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRecord {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub package_id: i32,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub download_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRateLimit {
    pub id: Option<i32>,
    pub rule_type: String, // 'user', 'ip', 'resource', 'global'
    pub target_id: Option<i32>,
    pub time_window: i32, // 时间窗口(秒)
    pub max_downloads: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadAnomaly {
    pub id: Option<i32>,
    pub anomaly_type: String, // 'rate_limit_exceeded', 'suspicious_pattern', 'statistical_anomaly'
    pub user_id: Option<i32>,
    pub package_id: Option<i32>,
    pub ip_address: Option<String>,
    pub details: Option<String>, // JSON格式的详细信息
    pub severity: String, // 'low', 'medium', 'high', 'critical'
    pub is_resolved: bool,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAccessStats {
    pub id: Option<i32>,
    pub package_id: i32,
    pub date: String, // YYYY-MM-DD格式
    pub view_count: i32,
    pub download_count: i32,
    pub unique_visitors: i32,
    pub unique_downloaders: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSecurityConfig {
    pub enable_rate_limiting: bool,
    pub enable_anomaly_detection: bool,
    pub enable_statistical_analysis: bool,
    pub max_downloads_per_user_per_hour: i32,
    pub max_downloads_per_ip_per_hour: i32,
    pub max_downloads_per_resource_per_day: i32,
    pub suspicious_pattern_threshold: f64,
    pub statistical_anomaly_threshold: f64,
}

impl Default for DownloadSecurityConfig {
    fn default() -> Self {
        Self {
            enable_rate_limiting: true,
            enable_anomaly_detection: true,
            enable_statistical_analysis: true,
            max_downloads_per_user_per_hour: 10,
            max_downloads_per_ip_per_hour: 20,
            max_downloads_per_resource_per_day: 50,
            suspicious_pattern_threshold: 0.8,
            statistical_anomaly_threshold: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadCheckResult {
    pub is_allowed: bool,
    pub reason: Option<String>,
    pub remaining_downloads: Option<i32>,
    pub cooldown_seconds: Option<i32>,
    pub anomaly_detected: bool,
    pub anomaly_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    pub is_anomaly: bool,
    pub anomaly_type: Option<String>,
    pub severity: Option<String>,
    pub details: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBan {
    pub id: Option<i32>,
    pub ip_address: String,
    pub reason: String, // 封禁原因
    pub ban_type: String, // 'temporary', 'permanent', 'download_only'
    pub duration_hours: Option<i32>, // 临时封禁时长（小时），永久封禁为None
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>, // 过期时间，永久封禁为None
    pub is_active: bool,
    pub created_by: Option<String>, // 'system', 'admin'
    pub notes: Option<String>, // 备注信息
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAction {
    pub id: Option<i32>,
    pub action_type: String, // 'ip_ban', 'user_warning', 'rate_limit', 'log_only'
    pub target_type: String, // 'ip', 'user', 'resource'
    pub target_id: String, // IP地址、用户ID或资源ID
    pub reason: String,
    pub severity: String, // 'low', 'medium', 'high', 'critical'
    pub duration_hours: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: String, // 'system', 'admin'
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_auto_ban: bool,
    pub auto_ban_threshold: i32, // 自动封禁阈值（异常次数）
    pub ban_duration_hours: i32, // 默认封禁时长（小时）
    pub critical_anomaly_auto_ban: bool, // 严重异常是否自动封禁
    pub suspicious_ip_auto_ban: bool, // 可疑IP是否自动封禁
    pub enable_ip_whitelist: bool, // 是否启用IP白名单
    pub enable_admin_notification: bool, // 是否启用管理员通知
    pub notification_email: Option<String>, // 通知邮箱
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_auto_ban: true,
            auto_ban_threshold: 3,
            ban_duration_hours: 24,
            critical_anomaly_auto_ban: true,
            suspicious_ip_auto_ban: true,
            enable_ip_whitelist: false,
            enable_admin_notification: true,
            notification_email: None,
        }
    }
} 