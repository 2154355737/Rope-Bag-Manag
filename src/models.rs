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

// 用户结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub star: u8,
    pub banned: bool,
    pub sign_days: u32,
    pub sign_total: u32,
    pub last_sign: String,
    pub is_admin: bool,
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
    pub username: String,
    pub nickname: String,
    pub star: u8,
    pub banned: bool,
    pub sign_days: u32,
    pub sign_total: u32,
    pub is_admin: bool,
}

// 用户信息响应结构
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub nickname: String,
    pub star: u8,
    pub banned: bool,
    pub sign_days: u32,
    pub sign_total: u32,
}

// 昵称信息结构
#[derive(Debug, Serialize)]
pub struct NicknameInfo {
    pub nickname: String,
    pub star: u8,
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