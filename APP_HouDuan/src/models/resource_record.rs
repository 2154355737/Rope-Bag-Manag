use serde::{Serialize, Deserialize};

// 资源记录模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceRecord {
    pub id: i32,
    pub resource_id: i32,
    pub resource_type: String,
    pub resource_name: Option<String>,
    pub user_id: i32,
    pub user_name: Option<String>,
    pub action: String,
    pub ip_address: Option<String>,
    pub old_data: Option<String>,
    pub new_data: Option<String>,
    pub timestamp: i64,
    pub file_size: Option<u64>,
    pub download_count: Option<i32>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

// 资源记录请求模型
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourceRecordRequest {
    pub resource_id: i32,
    pub resource_type: String,
    pub action: String,
    pub old_data: Option<String>,
    pub new_data: Option<String>,
}

// 资源记录查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceRecordQuery {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i32>,
    pub user_id: Option<i32>,
    pub action: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// 资源记录批量删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteResourceRecordsRequest {
    pub ids: Vec<i32>,
}

// 资源记录清理请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ClearResourceRecordsRequest {
    pub resource_id: Option<i32>,
    pub user_id: Option<i32>,
    pub before_date: Option<String>,
}

// 资源操作统计响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceActionStats {
    pub create_count: i32,
    pub update_count: i32,
    pub delete_count: i32,
    pub download_count: i32,
    pub by_day: Vec<DailyStats>,
}

// 每日统计数据
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub count: i32,
} 