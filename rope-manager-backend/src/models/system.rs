use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_users: i64,
    pub total_packages: i64,
    pub total_comments: i64,
    pub active_users: i64,
    pub new_users_today: i64,
    pub new_packages_today: i64,
    pub system_status: String,
    pub uptime: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
} 