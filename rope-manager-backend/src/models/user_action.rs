use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAction {
    pub id: i32,
    pub user_id: i32,
    pub action_type: String,
    pub target_type: Option<String>,
    pub target_id: Option<i32>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserActionRequest {
    pub user_id: i32,
    pub action_type: String,
    pub target_type: Option<String>,
    pub target_id: Option<i32>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActionQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: Option<i32>,
    pub action_type: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserActionStats {
    pub total_actions: i64,
    pub active_users: i64,
    pub by_day: Vec<DailyActionStat>,
    pub by_type: Vec<ActionTypeStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyActionStat {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionTypeStat {
    pub action_type: String,
    pub count: i64,
} 