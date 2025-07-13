use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::data_manager::DataManager;

// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: crate::config::AppConfig,
    pub data_manager: Arc<DataManager>,
    pub limiter: Arc<Mutex<HashMap<String, (u64, u32)>>>,
    pub global: Arc<Mutex<(u64, u32)>>,
    pub stats: Arc<Mutex<StatsData>>,
}

// 用户类型
pub type GlobalLimiter = Arc<Mutex<HashMap<String, (u64, u32)>>>;
pub type GlobalCount = Arc<Mutex<(u64, u32)>>;

// 用户身份枚举
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum UserRole {
    Normal,     // 普通用户
    Developer,  // 开发者
    Elder,      // 元老
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Normal
    }
}

// 在线状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OnlineStatus {
    Online,     // 在线
    Offline,    // 离线
}

impl Default for OnlineStatus {
    fn default() -> Self {
        OnlineStatus::Offline
    }
}

// 封禁状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BanStatus {
    Normal,     // 正常
    Banned,     // 封禁
}

impl Default for BanStatus {
    fn default() -> Self {
        BanStatus::Normal
    }
}

// 用户权限结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPermissions {
    pub can_download: bool,      // 下载资源
    pub can_upload: bool,        // 上传资源
    pub can_manage_own: bool,    // 管理自己的资源
    pub can_manage_global: bool, // 管理全局资源
    pub can_manage_users: bool,  // 管理用户
    pub can_manage_system: bool, // 管理系统
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self {
            can_download: true,
            can_upload: false,
            can_manage_own: false,
            can_manage_global: false,
            can_manage_users: false,
            can_manage_system: false,
        }
    }
}

// 签到记录结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRecord {
    pub date: String,           // 签到日期 YYYY-MM-DD
    pub timestamp: u64,         // 签到时间戳
    pub continuous_days: u32,   // 连续签到天数
}

// 新的用户结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,                    // 唯一标识，整数类型
    pub username: String,           // 用户昵称
    pub password: String,           // 密码
    pub role: UserRole,             // 用户权限组
    pub star: u32,                  // 星级（1-5颗星）
    pub online_status: OnlineStatus, // 在线状态
    pub ban_status: BanStatus,      // 封禁状态
    pub ban_reason: Option<String>, // 封禁原因
    pub register_time: String,      // 注册时间
    pub login_count: u32,           // 登录次数
    pub qq_number: Option<String>,  // 绑定QQ
    pub avatar_url: Option<String>, // 头像URL
    pub sign_records: Vec<SignRecord>, // 签到记录
    pub sign_days: u32,             // 总签到天数
    pub sign_total: u32,            // 连续签到天数
    pub last_sign: String,          // 最后签到时间
    pub last_login: String,         // 最后登录时间
    pub upload_count: u32,          // 上传资源数量
    pub download_count: u32,        // 下载资源数量
    pub permissions: UserPermissions, // 用户权限
    pub is_admin: bool,             // 是否为管理员
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::new(),
            password: String::new(),
            role: UserRole::Normal,
            star: 1,
            online_status: OnlineStatus::Offline,
            ban_status: BanStatus::Normal,
            ban_reason: None,
            register_time: String::new(),
            login_count: 0,
            qq_number: None,
            avatar_url: None,
            sign_records: Vec::new(),
            sign_days: 0,
            sign_total: 0,
            last_sign: String::new(),
            last_login: String::new(),
            upload_count: 0,
            download_count: 0,
            permissions: UserPermissions::default(),
            is_admin: false,
        }
    }
}

impl User {
    // 获取用户权限
    pub fn get_permissions(&self) -> UserPermissions {
        if self.ban_status == BanStatus::Banned {
            return UserPermissions {
                can_download: false,
                can_upload: false,
                can_manage_own: false,
                can_manage_global: false,
                can_manage_users: false,
                can_manage_system: false,
            };
        }

        match self.role {
            UserRole::Normal => UserPermissions {
                can_download: true,
                can_upload: false,
                can_manage_own: false,
                can_manage_global: false,
                can_manage_users: false,
                can_manage_system: false,
            },
            UserRole::Developer => UserPermissions {
                can_download: true,
                can_upload: true,
                can_manage_own: true,
                can_manage_global: false,
                can_manage_users: false,
                can_manage_system: false,
            },
            UserRole::Elder => UserPermissions {
                can_download: true,
                can_upload: true,
                can_manage_own: true,
                can_manage_global: true,
                can_manage_users: true,
                can_manage_system: false,
            },
        }
    }

    // 检查是否可以执行操作
    pub fn can_perform(&self, action: &str) -> bool {
        if self.ban_status == BanStatus::Banned {
            return false;
        }

        match action {
            "download" => self.permissions.can_download,
            "upload" => self.permissions.can_upload,
            "manage_own" => self.permissions.can_manage_own,
            "manage_global" => self.permissions.can_manage_global,
            "manage_users" => self.permissions.can_manage_users,
            "manage_system" => self.permissions.can_manage_system,
            _ => false,
        }
    }

    // 更新用户身份（基于星级）
    pub fn update_role(&mut self) {
        if self.star >= 3 && self.role == UserRole::Normal {
            self.role = UserRole::Developer;
            self.permissions = self.get_permissions();
        }
        if self.star >= 5 && self.role == UserRole::Developer {
            self.role = UserRole::Elder;
            self.permissions = self.get_permissions();
        }
    }

    // 签到
    pub fn sign_in(&mut self) -> (bool, String) {
        if self.ban_status == BanStatus::Banned {
            return (false, "用户已被封禁，无法签到".to_string());
        }

        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let now = chrono::Utc::now().timestamp() as u64;

        // 检查今天是否已经签到
        if self.sign_records.iter().any(|record| record.date == today) {
            return (false, "今天已经签到过了".to_string());
        }

        // 检查昨天是否签到（计算连续天数）
        let yesterday = chrono::Utc::now() - chrono::Duration::days(1);
        let yesterday_str = yesterday.format("%Y-%m-%d").to_string();
        let continuous_days = if self.sign_records.iter().any(|record| record.date == yesterday_str) {
            self.sign_total + 1
        } else {
            1
        };

        // 添加签到记录
        let sign_record = SignRecord {
            date: today.clone(),
            timestamp: now,
            continuous_days,
        };
        self.sign_records.push(sign_record);

        // 更新签到统计
        self.sign_days += 1;
        self.sign_total = continuous_days;
        self.last_sign = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 签到奖励：增加星级
        if continuous_days % 7 == 0 {
            self.add_star(1);
        }

        (true, format!("签到成功！连续签到{}天", continuous_days))
    }

    // 设置QQ号码
    pub fn set_qq(&mut self, qq_number: String) {
        self.qq_number = Some(qq_number);
    }

    // 封禁用户
    pub fn ban(&mut self, reason: String) {
        self.ban_status = BanStatus::Banned;
        self.ban_reason = Some(reason);
    }

    // 解封用户
    pub fn unban(&mut self) {
        self.ban_status = BanStatus::Normal;
        self.ban_reason = None;
    }

    // 增加星级
    pub fn add_star(&mut self, amount: u32) {
        self.star = (self.star + amount).min(5); // 最大5颗星
        self.update_role();
    }

    // 设置在线状态
    pub fn set_online_status(&mut self, status: OnlineStatus) {
        self.online_status = status;
    }

    // 增加登录次数
    pub fn increment_login_count(&mut self) {
        self.login_count += 1;
        self.last_login = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
}

// 绳包结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RopePackage {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub version: String,
    pub desc: String,
    pub url: String,
    pub downloads: u32,
    pub upload_time: String,
    pub category: String,
    pub status: String,
    pub is_starred: bool,        // 是否被标星
    pub star_time: Option<String>, // 标星时间
    pub star_by: Option<String>,   // 标星人
}

// 原始数据JSON结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawDataJson {
    pub 数据库配置: DatabaseConfig,
    pub 绳包列表: Vec<RawRopePackage>,
}

impl Default for RawDataJson {
    fn default() -> Self {
        Self {
            数据库配置: DatabaseConfig {
                数据库名称: "结绳绳包数据库".to_string(),
                数据库项目: 0,
                数据库版本: "0.0.1".to_string(),
                数据库更新时间: "".to_string(),
            },
            绳包列表: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub 数据库名称: String,
    pub 数据库项目: u32,
    pub 数据库版本: String,
    pub 数据库更新时间: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawRopePackage {
    pub id: u32,
    pub 绳包名称: String,
    pub 作者: String,
    pub 版本: String,
    pub 简介: String,
    pub 项目直链: String,
    pub 下载次数: u32,
    pub 上架时间: String,
    pub 分类: String,
    pub 状态: String,
    pub 是否标星: bool,
    pub 标星时间: Option<String>,
    pub 标星人: Option<String>,
}

// API响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

// 登录响应结构
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: u64,
    pub username: String,
    pub star: u32,
    pub role: UserRole,
    pub online_status: OnlineStatus,
    pub ban_status: BanStatus,
    pub ban_reason: Option<String>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
    pub sign_days: u32,
    pub sign_total: u32,
    pub last_sign: String,
    pub permissions: UserPermissions,
    pub is_admin: bool,
}

// 用户信息响应结构
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub id: u64,
    pub username: String,
    pub star: u32,
    pub role: UserRole,
    pub online_status: OnlineStatus,
    pub ban_status: BanStatus,
    pub ban_reason: Option<String>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
    pub sign_days: u32,
    pub sign_total: u32,
    pub last_sign: String,
    pub register_time: String,
    pub last_login: String,
    pub login_count: u32,
    pub upload_count: u32,
    pub download_count: u32,
    pub permissions: UserPermissions,
    pub is_admin: bool,
}

// 昵称信息结构
#[derive(Debug, Serialize)]
pub struct NicknameInfo {
    pub id: u64,
    pub username: String,
    pub star: u32,
    pub role: UserRole,
    pub online_status: OnlineStatus,
    pub avatar_url: Option<String>,
}

// 统计数据结构
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StatsData {
    pub api_counts: HashMap<String, u32>,
    pub downloads: HashMap<String, u32>,
    pub api_calls: Vec<ApiCallRecord>,
    pub api_last_used: HashMap<String, u64>,
    pub api_performance: HashMap<String, ApiPerformance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiCallRecord {
    pub timestamp: u64,
    pub api_name: String,
    pub username: String,
    pub ip_address: String,
    pub user_agent: String,
    pub response_time_ms: u64,
    pub status_code: u16,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiPerformance {
    pub total_calls: u32,
    pub success_calls: u32,
    pub failed_calls: u32,
    pub avg_response_time_ms: u64,
    pub min_response_time_ms: u64,
    pub max_response_time_ms: u64,
    pub last_called: u64,
    pub unique_users: std::collections::HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub count: u32,
}

// 系统设置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemSettings {
    pub theme: ThemeSettings,
    pub system_mode: SystemMode,
    pub feature_flags: FeatureFlags,
    pub backend_config: BackendConfig,
    pub backup_settings: BackupSettings,
    pub global_announcement: GlobalAnnouncement,
}

// 主题设置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeSettings {
    pub community_theme: String,    // 资源社区主题
    pub admin_theme: String,        // 后台管理主题
}

// 系统模式
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SystemMode {
    Normal,     // 正常运行
    Maintenance, // 维护中
}

impl Default for SystemMode {
    fn default() -> Self {
        SystemMode::Normal
    }
}

// 功能开关
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureFlags {
    pub enable_registration: bool,      // 是否开启用户注册
    pub enable_community: bool,         // 是否开启资源社区
    pub enable_upload: bool,            // 是否开启用户上传
    pub enable_comments: bool,          // 是否开启用户评论
    pub enable_qq_binding: bool,        // 是否开启QQ绑定
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enable_registration: true,
            enable_community: true,
            enable_upload: true,
            enable_comments: true,
            enable_qq_binding: true,
        }
    }
}

// 后端配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendConfig {
    pub proxy_address: String,          // 后台代理地址
    pub api_timeout: u64,               // API超时时间（秒）
    pub max_upload_size: u64,           // 最大上传大小（MB）
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            proxy_address: "http://127.0.0.1:15202".to_string(),
            api_timeout: 30,
            max_upload_size: 100,
        }
    }
}

// 备份设置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupSettings {
    pub enable_auto_backup: bool,       // 是否开启自动备份
    pub backup_interval_hours: u32,     // 备份间隔（小时）
    pub backup_location: String,        // 备份位置
    pub max_backup_files: u32,          // 最大备份文件数
}

impl Default for BackupSettings {
    fn default() -> Self {
        Self {
            enable_auto_backup: true,
            backup_interval_hours: 24,
            backup_location: "./backups".to_string(),
            max_backup_files: 30,
        }
    }
}

// 全局公告
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalAnnouncement {
    pub id: u32,
    pub enabled: bool,                  // 是否启用
    pub title: String,                  // 公告标题
    pub content: String,                // 公告内容
    pub type_: AnnouncementType,        // 公告类型
    pub start_time: String,             // 开始时间
    pub end_time: Option<String>,       // 结束时间（可选）
    pub priority: u8,                   // 优先级（1-10）
    pub create_time: String,            // 创建时间
    pub update_time: String,            // 更新时间
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AnnouncementType {
    Info,       // 信息
    Warning,    // 警告
    Error,      // 错误
    Success,    // 成功
}

impl Default for AnnouncementType {
    fn default() -> Self {
        AnnouncementType::Info
    }
}

// 用户评论
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: u32,
    pub user_id: String,                // 评论用户
    pub target_type: CommentTargetType, // 评论目标类型
    pub target_id: u32,                 // 评论目标ID
    pub content: String,                // 评论内容
    pub create_time: String,            // 创建时间
    pub update_time: String,            // 更新时间
    pub likes: u32,                     // 点赞数
    pub dislikes: u32,                  // 点踩数
    pub parent_id: Option<u32>,         // 父评论ID（回复功能）
    pub status: CommentStatus,          // 评论状态
    pub replies: Vec<Comment>,          // 回复列表
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CommentTargetType {
    Package,    // 绳包
    User,       // 用户
    System,     // 系统
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CommentStatus {
    Active,     // 正常
    Hidden,     // 隐藏
    Deleted,    // 已删除
}

impl Default for CommentStatus {
    fn default() -> Self {
        CommentStatus::Active
    }
}

// 用户行为记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAction {
    pub id: u32,
    pub user_id: String,                // 用户ID
    pub action_type: ActionType,        // 行为类型
    pub target_type: String,            // 目标类型
    pub target_id: String,              // 目标ID
    pub description: String,            // 行为描述
    pub ip_address: String,             // IP地址
    pub user_agent: String,             // 用户代理
    pub timestamp: u64,                 // 时间戳
    pub success: bool,                  // 是否成功
    pub error_message: Option<String>,  // 错误信息
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ActionType {
    Login,          // 登录
    Logout,         // 登出
    Register,       // 注册
    Upload,         // 上传
    Download,       // 下载
    Comment,        // 评论
    Like,           // 点赞
    Share,          // 分享
    Settings,       // 设置
    Admin,          // 管理操作
}

// 资源记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceRecord {
    pub id: u32,
    pub resource_type: ResourceType,    // 资源类型
    pub resource_id: u32,              // 资源ID
    pub user_id: String,               // 操作用户
    pub action: ResourceAction,        // 操作类型
    pub old_data: Option<String>,      // 旧数据（JSON）
    pub new_data: Option<String>,      // 新数据（JSON）
    pub timestamp: u64,                // 时间戳
    pub ip_address: String,            // IP地址
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ResourceType {
    Package,        // 绳包
    User,           // 用户
    Category,       // 分类
    Comment,        // 评论
    Settings,       // 设置
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ResourceAction {
    Create,         // 创建
    Update,         // 更新
    Delete,         // 删除
    Download,       // 下载
    Upload,         // 上传
    Star,           // 标星
    Ban,            // 封禁
}

// 数据库备份记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupRecord {
    pub id: u32,
    pub filename: String,               // 备份文件名
    pub file_size: u64,                // 文件大小（字节）
    pub backup_time: String,            // 备份时间
    pub backup_type: BackupType,       // 备份类型
    pub status: BackupStatus,          // 备份状态
    pub description: String,            // 备份描述
    pub file_path: String,             // 文件路径
    pub restore_time: Option<String>,  // 恢复时间
    pub restore_by: Option<String>,    // 恢复人
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupType {
    Auto,           // 自动备份
    Manual,         // 手动备份
    Scheduled,      // 定时备份
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupStatus {
    Success,        // 成功
    Failed,         // 失败
    InProgress,     // 进行中
}

impl Default for BackupStatus {
    fn default() -> Self {
        BackupStatus::InProgress
    }
}

// 普通用户后台数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDashboard {
    pub user_id: String,
    pub upload_count: u32,             // 上传数量
    pub download_count: u32,            // 下载数量
    pub comment_count: u32,             // 评论数量
    pub like_count: u32,                // 获赞数量
    pub star_count: u32,                // 获星数量
    pub last_activity: String,          // 最后活动时间
    pub favorite_packages: Vec<u32>,    // 收藏的绳包
    pub recent_activities: Vec<String>, // 最近活动
}

// QQ用户信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QQUserInfo {
    pub qq_number: String,
    pub nickname: String,
    pub avatar_url: String,
    pub level: u32,
    pub vip_level: u32,
}