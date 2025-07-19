use anyhow::Result;
use crate::repositories::system_repo::SystemRepository;
use crate::services::user_service::UserService;
use crate::models::{Stats, Category, UserAction};
use serde::Serialize;
use serde_json::Value;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AdminService {
    system_repo: SystemRepository,
    user_service: UserService,
}

impl AdminService {
    pub fn new(system_repo: SystemRepository, user_service: UserService) -> Self {
        Self { system_repo, user_service }
    }

    pub async fn get_stats(&self) -> Result<Stats> {
        self.system_repo.get_stats().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        self.system_repo.get_categories().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub async fn get_user_actions(&self) -> Result<Vec<UserAction>> {
        self.system_repo.get_user_actions().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取系统日志
    pub async fn get_logs(&self) -> Result<Vec<SystemLog>> {
        self.system_repo.get_logs().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：创建备份
    pub async fn create_backup(&self) -> Result<BackupInfo> {
        let backup_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let file_path = format!("backups/backup_{}.db", backup_id);
        
        // TODO: 实现实际的数据库备份逻辑
        self.system_repo.create_backup(&file_path).await?;
        
        Ok(BackupInfo {
            backup_id: backup_id.clone(),
            file_path,
            size: 1024 * 1024, // 示例大小
            created_at: timestamp.to_rfc3339(),
        })
    }

    // 新增方法：获取备份列表
    pub async fn get_backups(&self) -> Result<Vec<BackupInfo>> {
        self.system_repo.get_backups().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：下载备份
    pub async fn download_backup(&self, backup_id: &str) -> Result<String> {
        self.system_repo.get_backup_path(backup_id).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：删除备份
    pub async fn delete_backup(&self, backup_id: &str) -> Result<()> {
        self.system_repo.delete_backup(backup_id).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取公告
    pub async fn get_announcements(&self) -> Result<Vec<Announcement>> {
        self.system_repo.get_announcements().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：创建公告
    pub async fn create_announcement(&self, data: &Value) -> Result<Announcement> {
        let title = data["title"].as_str().unwrap_or("").to_string();
        let content = data["content"].as_str().unwrap_or("").to_string();
        let priority = data["priority"].as_i64().unwrap_or(1);
        
        self.system_repo.create_announcement(&title, &content, priority as i32).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：更新公告
    pub async fn update_announcement(&self, id: i32, data: &Value) -> Result<()> {
        let title = data["title"].as_str().unwrap_or("").to_string();
        let content = data["content"].as_str().unwrap_or("").to_string();
        let priority = data["priority"].as_i64().unwrap_or(1);
        
        self.system_repo.update_announcement(id, &title, &content, priority as i32).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：删除公告
    pub async fn delete_announcement(&self, id: i32) -> Result<()> {
        self.system_repo.delete_announcement(id).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取主题设置
    pub async fn get_theme_settings(&self) -> Result<ThemeSettings> {
        self.system_repo.get_theme_settings().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：更新主题设置
    pub async fn update_theme_settings(&self, data: &Value) -> Result<()> {
        let primary_color = data["primary_color"].as_str().unwrap_or("#409EFF").to_string();
        let secondary_color = data["secondary_color"].as_str().unwrap_or("#67C23A").to_string();
        let dark_mode = data["dark_mode"].as_bool().unwrap_or(false);
        
        self.system_repo.update_theme_settings(&primary_color, &secondary_color, dark_mode).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取资源记录
    pub async fn get_resource_records(&self) -> Result<Vec<ResourceRecord>> {
        self.system_repo.get_resource_records().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取用户统计
    pub async fn get_user_stats(&self) -> Result<(i32, i32, i32)> {
        self.user_service.get_user_stats().await
    }
}

#[derive(Serialize)]
pub struct DashboardStats {
    pub system_status: String,
    pub uptime: String,
    pub log_today: i64,
    pub package_total: i64,
    pub package_available: i64,
    pub user_total: i64,
    pub user_active: i64,
}

#[derive(Serialize)]
pub struct SystemLog {
    pub id: i32,
    pub level: String,
    pub message: String,
    pub timestamp: String,
    pub details: Option<String>,
}

#[derive(Serialize)]
pub struct BackupInfo {
    pub backup_id: String,
    pub file_path: String,
    pub size: u64,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct Announcement {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub priority: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct ThemeSettings {
    pub primary_color: String,
    pub secondary_color: String,
    pub dark_mode: bool,
    pub font_size: String,
    pub language: String,
}

#[derive(Serialize)]
pub struct ResourceRecord {
    pub id: i32,
    pub resource_name: String,
    pub resource_type: String,
    pub file_size: u64,
    pub download_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_dashboard_stats() -> DashboardStats {
    // TODO: 替换为真实数据库查询
    DashboardStats {
        system_status: "正常".to_string(),
        uptime: "15天 8小时 32分钟".to_string(),
        log_today: 12,
        package_total: 123,
        package_available: 120,
        user_total: 456,
        user_active: 78,
    }
} 