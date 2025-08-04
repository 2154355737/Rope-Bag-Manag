use anyhow::Result;
use rusqlite::{params, Connection};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, Duration};
use serde_json;
use crate::models::download_security::*;

#[derive(Clone)]
pub struct DownloadSecurityRepository {
    conn: Arc<Mutex<Connection>>,
}

impl DownloadSecurityRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // 初始化表结构
        conn.execute_batch(include_str!("../../sql/create_download_security_tables.sql"))?;
        conn.execute_batch(include_str!("../../sql/create_ip_ban_tables.sql"))?;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    // 记录下载行为
    pub async fn record_download(&self, record: &DownloadRecord) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO download_records (user_id, package_id, ip_address, user_agent, download_time, created_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                record.user_id,
                record.package_id,
                record.ip_address,
                record.user_agent,
                record.download_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                record.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
        )?;
        Ok(())
    }

    // 获取用户在指定时间窗口内的下载次数
    pub async fn get_user_downloads_in_window(&self, user_id: i32, package_id: i32, hours: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let window_start = Utc::now() - Duration::hours(hours as i64);
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM download_records 
             WHERE user_id = ? AND package_id = ? AND download_time >= ?",
            params![
                user_id,
                package_id,
                window_start.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
            |row| row.get(0),
        )?;
        
        Ok(count)
    }

    // 获取IP在指定时间窗口内的下载次数
    pub async fn get_ip_downloads_in_window(&self, ip_address: &str, hours: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let window_start = Utc::now() - Duration::hours(hours as i64);
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM download_records 
             WHERE ip_address = ? AND download_time >= ?",
            params![
                ip_address,
                window_start.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
            |row| row.get(0),
        )?;
        
        Ok(count)
    }

    // 获取资源在指定时间窗口内的下载次数
    pub async fn get_resource_downloads_in_window(&self, package_id: i32, hours: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let window_start = Utc::now() - Duration::hours(hours as i64);
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM download_records 
             WHERE package_id = ? AND download_time >= ?",
            params![
                package_id,
                window_start.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
            |row| row.get(0),
        )?;
        
        Ok(count)
    }

    // 获取下载限制规则
    pub async fn get_rate_limits(&self) -> Result<Vec<DownloadRateLimit>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, rule_type, target_id, time_window, max_downloads, is_active, created_at, updated_at 
             FROM download_rate_limits WHERE is_active = 1"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(DownloadRateLimit {
                id: row.get(0)?,
                rule_type: row.get(1)?,
                target_id: row.get(2)?,
                time_window: row.get(3)?,
                max_downloads: row.get(4)?,
                is_active: row.get(5)?,
                created_at: DateTime::parse_from_str(&row.get::<_, String>(6)?, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_default().with_timezone(&Utc),
                updated_at: DateTime::parse_from_str(&row.get::<_, String>(7)?, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_default().with_timezone(&Utc),
            })
        })?;
        
        let mut limits = Vec::new();
        for row in rows {
            limits.push(row?);
        }
        
        Ok(limits)
    }

    // 记录异常
    pub async fn record_anomaly(&self, anomaly: &DownloadAnomaly) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO download_anomalies (anomaly_type, user_id, package_id, ip_address, details, severity, is_resolved, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                anomaly.anomaly_type,
                anomaly.user_id,
                anomaly.package_id,
                anomaly.ip_address,
                anomaly.details,
                anomaly.severity,
                anomaly.is_resolved,
                anomaly.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
        )?;
        Ok(())
    }

    // 获取资源访问统计
    pub async fn get_resource_access_stats(&self, package_id: i32, days: i32) -> Result<Vec<ResourceAccessStats>> {
        let conn = self.conn.lock().await;
        let start_date = Utc::now() - Duration::days(days as i64);
        
        let mut stmt = conn.prepare(
            "SELECT id, package_id, date, view_count, download_count, unique_visitors, unique_downloaders, created_at, updated_at 
             FROM resource_access_stats 
             WHERE package_id = ? AND date >= ? 
             ORDER BY date DESC"
        )?;
        
        let rows = stmt.query_map(params![package_id, start_date.format("%Y-%m-%d").to_string()], |row| {
            Ok(ResourceAccessStats {
                id: row.get(0)?,
                package_id: row.get(1)?,
                date: row.get(2)?,
                view_count: row.get(3)?,
                download_count: row.get(4)?,
                unique_visitors: row.get(5)?,
                unique_downloaders: row.get(6)?,
                created_at: DateTime::parse_from_str(&row.get::<_, String>(7)?, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_default().with_timezone(&Utc),
                updated_at: DateTime::parse_from_str(&row.get::<_, String>(8)?, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_default().with_timezone(&Utc),
            })
        })?;
        
        let mut stats = Vec::new();
        for row in rows {
            stats.push(row?);
        }
        
        Ok(stats)
    }

    // 更新资源访问统计
    pub async fn update_resource_access_stats(&self, package_id: i32, date: &str, view_count: i32, download_count: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO resource_access_stats (package_id, date, view_count, download_count, updated_at) 
             VALUES (?, ?, ?, ?, ?)",
            params![
                package_id,
                date,
                view_count,
                download_count,
                Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
        )?;
        Ok(())
    }

    // 获取最近的异常记录
    pub async fn get_recent_anomalies(&self, hours: i32) -> Result<Vec<DownloadAnomaly>> {
        let conn = self.conn.lock().await;
        let start_time = Utc::now() - Duration::hours(hours as i64);
        
        let mut stmt = conn.prepare(
            "SELECT id, anomaly_type, user_id, package_id, ip_address, details, severity, is_resolved, created_at, resolved_at 
             FROM download_anomalies 
             WHERE created_at >= ? 
             ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map(params![start_time.format("%Y-%m-%d %H:%M:%S").to_string()], |row| {
            Ok(DownloadAnomaly {
                id: row.get(0)?,
                anomaly_type: row.get(1)?,
                user_id: row.get(2)?,
                package_id: row.get(3)?,
                ip_address: row.get(4)?,
                details: row.get(5)?,
                severity: row.get(6)?,
                is_resolved: row.get(7)?,
                created_at: DateTime::parse_from_str(&row.get::<_, String>(8)?, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_default().with_timezone(&Utc),
                resolved_at: row.get::<_, Option<String>>(9)?.map(|s| {
                    DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                        .unwrap_or_default().with_timezone(&Utc)
                }),
            })
        })?;
        
        let mut anomalies = Vec::new();
        for row in rows {
            anomalies.push(row?);
        }
        
        Ok(anomalies)
    }

    // IP封禁相关方法
    pub async fn create_ip_ban(&self, ban: &IpBan) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO ip_bans (ip_address, reason, ban_type, duration_hours, created_at, expires_at, is_active, created_by, notes) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                ban.ip_address,
                ban.reason,
                ban.ban_type,
                ban.duration_hours,
                ban.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                ban.expires_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                ban.is_active,
                ban.created_by,
                ban.notes,
            ],
        )?;
        Ok(())
    }

    pub async fn get_ip_ban_info(&self, ip_address: &str) -> Result<Option<IpBan>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, ip_address, reason, ban_type, duration_hours, created_at, expires_at, is_active, created_by, notes 
             FROM ip_bans WHERE ip_address = ?"
        )?;
        
        let rows = stmt.query_map(params![ip_address], |row| {
            Ok(IpBan {
                id: row.get(0)?,
                ip_address: row.get(1)?,
                reason: row.get(2)?,
                ban_type: row.get(3)?,
                duration_hours: row.get(4)?,
                created_at: DateTime::parse_from_str(&row.get::<_, String>(5)?, "%Y-%m-%d %H:%M:%S")
                    .unwrap_or_default().with_timezone(&Utc),
                expires_at: row.get::<_, Option<String>>(6)?.map(|s| {
                    DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                        .unwrap_or_default().with_timezone(&Utc)
                }),
                is_active: row.get(7)?,
                created_by: row.get(8)?,
                notes: row.get(9)?,
            })
        })?;
        
        for row in rows {
            return Ok(Some(row?));
        }
        Ok(None)
    }

    pub async fn deactivate_ip_ban(&self, ip_address: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE ip_bans SET is_active = 0 WHERE ip_address = ?",
            params![ip_address],
        )?;
        Ok(())
    }

    pub async fn is_ip_whitelisted(&self, ip_address: &str) -> Result<bool> {
        let conn = self.conn.lock().await;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM ip_whitelist WHERE ip_address = ?",
            params![ip_address],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub async fn add_ip_to_whitelist(&self, ip_address: &str, description: &str, created_by: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO ip_whitelist (ip_address, description, created_by) VALUES (?, ?, ?)",
            params![ip_address, description, created_by],
        )?;
        Ok(())
    }

    pub async fn remove_ip_from_whitelist(&self, ip_address: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "DELETE FROM ip_whitelist WHERE ip_address = ?",
            params![ip_address],
        )?;
        Ok(())
    }

    pub async fn record_security_action(&self, action: &SecurityAction) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO security_actions (action_type, target_type, target_id, reason, severity, duration_hours, is_active, created_at, expires_at, created_by, notes) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                action.action_type,
                action.target_type,
                action.target_id,
                action.reason,
                action.severity,
                action.duration_hours,
                action.is_active,
                action.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                action.expires_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                action.created_by,
                action.notes,
            ],
        )?;
        Ok(())
    }

    pub async fn get_ip_anomaly_count(&self, ip_address: &str, hours: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let start_time = Utc::now() - Duration::hours(hours as i64);
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM download_anomalies 
             WHERE ip_address = ? AND created_at >= ? AND severity IN ('high', 'critical')",
            params![
                ip_address,
                start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            ],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub async fn get_total_ip_bans(&self) -> Result<i32> {
        let conn = self.conn.lock().await;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM ip_bans",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub async fn get_active_ip_bans(&self) -> Result<i32> {
        let conn = self.conn.lock().await;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM ip_bans WHERE is_active = 1",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub async fn get_recent_ip_bans(&self, hours: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let start_time = Utc::now() - Duration::hours(hours as i64);
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM ip_bans WHERE created_at >= ?",
            params![start_time.format("%Y-%m-%d %H:%M:%S").to_string()],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    // 获取IP封禁列表
    pub async fn get_ip_bans(&self) -> Result<Vec<serde_json::Value>> {
        let conn = self.conn.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT id, ip_address, reason, ban_type, duration_hours, created_at, expires_at, is_active, created_by, notes 
             FROM ip_bans 
             ORDER BY created_at DESC"
        )?;
        
        let bans = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i32>(0)?,
                "ip_address": row.get::<_, String>(1)?,
                "reason": row.get::<_, String>(2)?,
                "ban_type": row.get::<_, String>(3)?,
                "duration_hours": row.get::<_, Option<i32>>(4)?,
                "created_at": row.get::<_, String>(5)?,
                "expires_at": row.get::<_, Option<String>>(6)?,
                "is_active": row.get::<_, bool>(7)?,
                "created_by": row.get::<_, String>(8)?,
                "notes": row.get::<_, Option<String>>(9)?
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(bans)
    }

    // 获取IP白名单
    pub async fn get_ip_whitelist(&self) -> Result<Vec<serde_json::Value>> {
        let conn = self.conn.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT id, ip_address, description, created_at, created_by 
             FROM ip_whitelist 
             ORDER BY created_at DESC"
        )?;
        
        let whitelist = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i32>(0)?,
                "ip_address": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "created_at": row.get::<_, String>(3)?,
                "created_by": row.get::<_, String>(4)?
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(whitelist)
    }
} 