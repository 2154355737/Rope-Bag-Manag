use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub total_users: i32,
    pub total_packages: i32,
    pub total_comments: i32,
    pub active_users: i32,
    pub new_users_today: i32,
    pub new_packages_today: i32,
    pub system_status: String,
    pub uptime: i32,
}

// 添加分类相关结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemLog {
    pub id: i32,
    pub level: String,
    pub message: String,
    pub timestamp: String,
    pub details: Option<String>,
}

// 添加更完善的备份模型
#[derive(Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub file_path: String,
    pub file_size: u64,
    pub backup_type: String,
    pub status: String,
    pub description: Option<String>,
    pub backup_time: String, // ISO 8601 格式时间字符串
    pub created_by: Option<i32>, // 用户ID
    pub created_by_name: Option<String>, // 用户名
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBackupRequest {
    pub backup_type: String, // "Manual", "Auto", "Scheduled"
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupScheduleConfig {
    pub enabled: bool,
    pub frequency: String, // "Daily", "Weekly", "Monthly"
    pub time: String,      // "HH:MM" 格式
    pub day: Option<i32>,  // 针对Weekly(1-7)和Monthly(1-31)
    pub retain_count: i32, // 保留备份数量
    pub auto_clean: bool,  // 是否自动清理旧备份
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreBackupRequest {
    pub backup_id: String,
    pub confirm: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupStats {
    pub total_backups: i32,
    pub success_backups: i32,
    pub failed_backups: i32,
    pub total_size: u64,
    pub last_backup_time: Option<String>,
    pub next_scheduled_backup: Option<String>,
} 