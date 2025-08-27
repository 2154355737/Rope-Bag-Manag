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
    pub subscription_locked: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub subscription_locked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub subscription_locked: Option<bool>,
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

// 社区主页配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunitySettings {
    pub site_title: String,
    pub site_subtitle: String,
    pub site_description: String,
    pub welcome_message: String,
    pub announcement: Option<String>,
    pub footer_text: String,
    pub contact_email: String,
    pub github_link: Option<String>,
    pub qq_group: Option<String>,
    pub wechat_group: Option<String>,
    // 新增主页配置字段
    pub hero_title: Option<String>,
    pub hero_subtitle: Option<String>,
    pub homepage_sections: Option<Vec<String>>,
    pub resources_per_page: Option<i32>,
    pub posts_per_page: Option<i32>,
    pub default_sort: Option<String>,
}

impl Default for CommunitySettings {
    fn default() -> Self {
        Self {
            site_title: "资源社区".to_string(),
            site_subtitle: "分享、发现、学习".to_string(),
            site_description: "一个专注于资源分享的社区平台".to_string(),
            welcome_message: "欢迎来到我们的资源社区！在这里您可以分享和发现各种有价值的资源。".to_string(),
            announcement: None,
            footer_text: "© 2024 资源社区. All rights reserved.".to_string(),
            contact_email: "contact@community.com".to_string(),
            github_link: None,
            qq_group: None,
            wechat_group: None,
            // 新增主页配置字段的默认值
            hero_title: Some("绳包管理器".to_string()),
            hero_subtitle: Some("专业的资源管理与分享平台".to_string()),
            homepage_sections: Some(vec![
                "hero_section".to_string(),
                "stats_section".to_string(),
                "popular_tags".to_string(),
                "recent_resources".to_string(),
                "community_posts".to_string(),
                "announcements".to_string()
            ]),
            resources_per_page: Some(12),
            posts_per_page: Some(10),
            default_sort: Some("latest".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommunitySettingsRequest {
    pub site_title: Option<String>,
    pub site_subtitle: Option<String>,
    pub site_description: Option<String>,
    pub welcome_message: Option<String>,
    pub announcement: Option<String>,
    pub footer_text: Option<String>,
    pub contact_email: Option<String>,
    pub github_link: Option<String>,
    pub qq_group: Option<String>,
    pub wechat_group: Option<String>,
    // 新增主页配置字段
    pub hero_title: Option<String>,
    pub hero_subtitle: Option<String>,
    pub homepage_sections: Option<Vec<String>>,
    pub resources_per_page: Option<i32>,
    pub posts_per_page: Option<i32>,
    pub default_sort: Option<String>,
} 

// 添加轮播图相关结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Banner {
    pub id: i32,
    pub title: String,
    pub image_url: String,
    pub link_type: String, // "resource", "category", "url", "none"
    pub link_target: Option<String>, // 资源ID/分类ID/外部URL
    pub priority: i32,
    pub enabled: bool,
    pub start_time: String,
    pub end_time: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBannerRequest {
    pub title: String,
    pub image_url: String,
    pub link_type: String, // "resource", "category", "url", "none"
    pub link_target: Option<String>,
    pub priority: Option<i32>,
    pub enabled: Option<bool>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBannerRequest {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub link_type: Option<String>,
    pub link_target: Option<String>,
    pub priority: Option<i32>,
    pub enabled: Option<bool>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
} 