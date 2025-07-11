use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ====== 用户相关数据结构 ======
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub star: u8,
    pub banned: bool,
    pub sign_days: u32,
    pub sign_total: u32,
    pub last_sign: String,
    pub is_admin: bool,
}

// ====== 绳包相关数据结构 ======
#[derive(Serialize, Deserialize, Clone)]
pub struct RopePackage {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub version: String,
    pub desc: String,
    pub url: String,
    pub downloads: u32,
    pub upload_time: String,
}

// ====== 统计数据结构 ======
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Stats {
    pub api_counts: HashMap<String, u32>,
    pub downloads: HashMap<String, u32>,
    pub api_calls: Vec<ApiCallRecord>,
    pub api_performance: HashMap<String, ApiPerformance>,
    pub api_last_used: HashMap<String, u64>, // 新增字段
}

// ====== API调用记录结构 ======
#[derive(Serialize, Deserialize, Clone)]
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

// ====== API性能统计结构 ======
#[derive(Serialize, Deserialize, Clone, Default)]
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

// ====== 配置数据结构 ======
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub admin_username: String,
    pub admin_password: String,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_api: u32,
    #[serde(default = "default_rate_window")] 
    pub rate_limit_window_sec: u32,
    #[serde(default = "default_global_rate_limit")]
    pub global_rate_limit: u32,
    #[serde(default = "default_global_rate_window")]
    pub global_rate_window_sec: u32,
    pub sign_reward_days: u32,
    pub sign_reward_star: u8,
    pub min_star_add_package: u8,
    #[serde(default = "default_banned_duration")]
    pub banned_duration: u32,
}

fn default_rate_limit() -> u32 { 100 }
fn default_rate_window() -> u32 { 60 }
fn default_global_rate_limit() -> u32 { 1000 }
fn default_global_rate_window() -> u32 { 60 }
fn default_banned_duration() -> u32 { 86400 }

// ====== 数据库配置结构 ======
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DatabaseConfig {
    pub 数据库名称: String,
    pub 数据库版本: String,
    pub 数据库项目: u32,
    pub 数据库更新时间: String,
}

// ====== 原始绳包数据结构 ======
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RawRopePackage {
    pub id: u32,
    pub 绳包名称: String,
    pub 作者: String,
    pub 版本: String,
    pub 简介: String,
    pub 项目直链: String,
    #[serde(default)]
    pub 下载次数: u32,
    #[serde(default)]
    pub 上架时间: String,
}

// ====== 原始数据JSON结构 ======
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RawDataJson {
    pub 数据库配置: DatabaseConfig,
    pub 绳包列表: Vec<RawRopePackage>,
}

// ====== 类型别名 ======
pub type Users = Arc<Mutex<HashMap<String, User>>>;
pub type RopePackages = Arc<Mutex<HashMap<u32, RopePackage>>>;
pub type StatsData = Arc<Mutex<Stats>>;
pub type GlobalLimiter = Arc<Mutex<HashMap<String, (u64, u32)>>>;
pub type GlobalCount = Arc<Mutex<(u64, u32)>>;

// ====== 应用状态结构 ======
#[allow(dead_code)]
pub struct AppState {
    pub users: Users,
    pub ropes: RopePackages,
    pub stats: StatsData,
    pub config: crate::config::AppConfig,
    pub limiter: GlobalLimiter,
    pub global: GlobalCount,
}

// ====== API响应结构 ======
#[derive(Serialize)]
pub struct LoginResponse {
    pub username: String,
    pub nickname: String,
    pub star: u8,
    pub banned: bool,
    pub sign_days: u32,
    pub sign_total: u32,
    pub is_admin: bool,
}

#[derive(Serialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub nickname: String,
    pub star: u8,
    pub banned: bool,
    pub sign_days: u32,
    pub sign_total: u32,
}

#[derive(Serialize)]
pub struct NicknameInfo {
    pub nickname: String,
    pub star: u8,
}