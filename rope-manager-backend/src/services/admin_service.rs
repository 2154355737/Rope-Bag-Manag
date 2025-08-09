use anyhow::Result;
use crate::repositories::system_repo::SystemRepository;
use crate::repositories::mail_repo::MailRepository;
use crate::services::user_service::UserService;
use crate::models::{Stats, ResourceRecord, ResourceActionStats, CreateResourceRecordRequest};
use crate::models::user_action::UserAction;
use crate::models::system::{Category, BackupInfo, BackupStats};
use crate::models::mail::MailSettings;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AdminService {
    system_repo: SystemRepository,
    user_service: UserService,
    mail_repo: MailRepository,
}

impl AdminService {
    pub fn new(db_url: &str) -> Self {
        Self {
            system_repo: SystemRepository::new(db_url).expect("创建系统仓库失败"),
            user_service: UserService::new(crate::repositories::UserRepository::new(db_url).expect("创建用户仓库失败")),
            mail_repo: MailRepository::new(db_url),
        }
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
    pub async fn get_logs(&self, page: Option<u32>, page_size: Option<u32>, level: Option<&str>, search: Option<&str>) -> Result<(Vec<SystemLog>, i64)> {
        self.system_repo.get_logs(page, page_size, level, search).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 添加日志
    pub async fn add_log(&self, level: &str, message: &str, details: Option<&str>) -> Result<i64> {
        self.system_repo.add_log(level, message, details).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 更新创建备份方法
    pub async fn create_backup(&self, backup_type: &str, description: Option<&str>, user_id: Option<i32>) -> Result<BackupInfo> {
        self.system_repo.create_backup(backup_type, description, user_id).await
    }

    // 更新获取备份列表方法
    pub async fn get_backups(&self, page: Option<u32>, page_size: Option<u32>) -> Result<(Vec<BackupInfo>, i64)> {
        let limit = page_size;
        let offset = if let (Some(p), Some(ps)) = (page, page_size) {
            Some((p - 1) * ps)
        } else {
            None
        };
        
        self.system_repo.get_backups(limit, offset).await
    }

    // 获取备份详情方法
    pub async fn get_backup_details(&self, backup_id: &str) -> Result<BackupInfo> {
        self.system_repo.get_backup_details(backup_id).await
    }

    // 更新删除备份方法
    pub async fn delete_backup(&self, backup_id: &str) -> Result<()> {
        self.system_repo.delete_backup(backup_id).await
    }

    // 批量删除备份方法
    pub async fn batch_delete_backups(&self, backup_ids: &[String]) -> Result<usize> {
        self.system_repo.batch_delete_backups(backup_ids).await
    }

    // 恢复备份方法
    pub async fn restore_backup(&self, backup_id: &str) -> Result<()> {
        self.system_repo.restore_backup(backup_id).await
    }

    // 获取备份统计方法
    pub async fn get_backup_stats(&self) -> Result<BackupStats> {
        self.system_repo.get_backup_stats().await
    }

    // 获取备份下载路径方法
    pub async fn get_backup_download_path(&self, backup_id: &str) -> Result<String> {
        self.system_repo.get_backup_path(backup_id).await
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
        let start_time = data["start_time"].as_str().unwrap_or("").to_string();
        let end_time = data["end_time"].as_str().map(|s| s.to_string());
        self.system_repo.create_announcement(&title, &content, priority as i32, &start_time, end_time.as_deref()).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：更新公告
    pub async fn update_announcement(&self, id: i32, data: &Value) -> Result<()> {
        let title = data["title"].as_str().unwrap_or("").to_string();
        let content = data["content"].as_str().unwrap_or("").to_string();
        let priority = data["priority"].as_i64().unwrap_or(1);
        let start_time = data["start_time"].as_str().unwrap_or("").to_string();
        let end_time = data["end_time"].as_str().map(|s| s.to_string());
        self.system_repo.update_announcement(id, &title, &content, priority as i32, &start_time, end_time.as_deref()).await
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
        let font_size = data["font_size"].as_str().unwrap_or("14px").to_string();
        let language = data["language"].as_str().unwrap_or("zh-CN").to_string();
        
        self.system_repo.update_theme_settings(&primary_color, &secondary_color, dark_mode, &font_size, &language).await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 获取所有设置
    pub async fn get_all_settings(&self) -> Result<HashMap<String, String>> {
        self.system_repo.get_all_settings().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 获取单个设置值
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        self.system_repo.get_setting(key).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 更新单个设置值
    pub async fn update_setting(&self, key: &str, value: &str) -> Result<()> {
        self.system_repo.update_setting(key, value).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取资源记录
    pub async fn get_resource_records(&self) -> Result<Vec<ResourceRecord>> {
        self.system_repo.get_resource_records().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：记录资源操作
    pub async fn log_resource_action(&self, record: &CreateResourceRecordRequest, user_id: i32) -> Result<i64> {
        self.system_repo.log_resource_action(record, user_id).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：删除资源记录
    pub async fn delete_resource_record(&self, record_id: i32) -> Result<()> {
        self.system_repo.delete_resource_record(record_id).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：批量删除资源记录
    pub async fn batch_delete_resource_records(&self, record_ids: &[i32]) -> Result<usize> {
        self.system_repo.batch_delete_resource_records(record_ids).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取资源操作统计
    pub async fn get_resource_action_stats(&self) -> Result<ResourceActionStats> {
        self.system_repo.get_resource_action_stats().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取用户统计
    pub async fn get_user_stats(&self) -> Result<(i32, i32, i32)> {
        self.user_service.get_user_stats().await
    }

    // 新增方法：根据ID获取公告
    pub async fn get_announcement_by_id(&self, id: i32) -> Result<Option<Announcement>> {
        self.system_repo.get_announcement_by_id(id).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：批量更新公告状态
    pub async fn batch_update_announcement_status(&self, ids: &[i32], enabled: bool) -> Result<usize> {
        self.system_repo.batch_update_announcement_status(ids, enabled).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：批量删除公告
    pub async fn batch_delete_announcements(&self, ids: &[i32]) -> Result<usize> {
        self.system_repo.batch_delete_announcements(ids).await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 新增方法：获取当前有效公告
    pub async fn get_active_announcements(&self) -> Result<Vec<Announcement>> {
        self.system_repo.get_active_announcements().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 备份计划
    pub async fn get_backup_schedule(&self) -> Result<Value> {
        let v = self.get_setting("backup_schedule").await?.unwrap_or("{}".to_string());
        Ok(serde_json::from_str(&v).unwrap_or(Value::Object(Default::default())))
    }

    pub async fn update_backup_schedule(&self, config: &Value) -> Result<()> {
        self.update_setting("backup_schedule", &config.to_string()).await
    }

    // 邮件设置管理 - 使用专门的邮件表
    pub async fn get_mail_settings(&self) -> Result<Value> {
        match self.mail_repo.get_mail_settings().await? {
            Some(settings) => Ok(serde_json::to_value(settings)?),
            None => {
                // 如果数据库中没有配置，返回默认配置
                let default_settings = MailSettings::default();
                Ok(serde_json::to_value(default_settings)?)
            }
        }
    }

    pub async fn update_mail_settings(&self, config: &Value) -> Result<()> {
        let settings: MailSettings = serde_json::from_value(config.clone())?;
        self.mail_repo.save_mail_settings(&settings).await?;
        Ok(())
    }

    // 邮件统计和日志
    pub async fn get_mail_stats(&self) -> Result<Value> {
        let stats = self.mail_repo.get_mail_stats().await?;
        Ok(serde_json::to_value(stats)?)
    }

    pub async fn get_mail_logs(&self, limit: Option<i64>) -> Result<Value> {
        let logs = self.mail_repo.get_mail_logs(limit, None).await?;
        Ok(serde_json::to_value(logs)?)
    }

    // 获取所有轮播图
    pub async fn get_banners(&self) -> Result<Vec<crate::models::system::Banner>> {
        self.system_repo.get_banners(false).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 获取活动轮播图
    pub async fn get_active_banners(&self) -> Result<Vec<crate::models::system::Banner>> {
        self.system_repo.get_banners(true).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 获取轮播图详情
    pub async fn get_banner_by_id(&self, id: i32) -> Result<Option<crate::models::system::Banner>> {
        self.system_repo.get_banner_by_id(id).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 创建轮播图
    pub async fn create_banner(&self, req: &crate::models::system::CreateBannerRequest) -> Result<crate::models::system::Banner> {
        self.system_repo.create_banner(req).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 更新轮播图
    pub async fn update_banner(&self, id: i32, req: &crate::models::system::UpdateBannerRequest) -> Result<()> {
        self.system_repo.update_banner(id, req).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 删除轮播图
    pub async fn delete_banner(&self, id: i32) -> Result<()> {
        self.system_repo.delete_banner(id).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 批量更新轮播图状态
    pub async fn batch_update_banner_status(&self, ids: &[i32], enabled: bool) -> Result<usize> {
        self.system_repo.batch_update_banner_status(ids, enabled).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
    
    // 批量删除轮播图
    pub async fn batch_delete_banners(&self, ids: &[i32]) -> Result<usize> {
        self.system_repo.batch_delete_banners(ids).await.map_err(|e| anyhow::anyhow!("{}", e))
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
pub struct Announcement {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub type_: String, // 'Info', 'Warning', 'Error', 'Success'
    pub priority: i32,
    pub enabled: bool,
    pub start_time: String,
    pub end_time: Option<String>,
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

impl AdminService {
    // 获取社区设置
    pub async fn get_community_settings(&self) -> Result<crate::models::system::CommunitySettings> {
        self.system_repo.get_community_settings().await.map_err(|e| anyhow::anyhow!("{}", e))
    }

    // 更新社区设置
    pub async fn update_community_settings(&self, request: &crate::models::system::UpdateCommunitySettingsRequest) -> Result<()> {
        self.system_repo.update_community_settings(request).await.map_err(|e| anyhow::anyhow!("{}", e))
    }
} 