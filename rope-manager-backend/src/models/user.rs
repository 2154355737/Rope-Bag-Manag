use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub nickname: Option<String>,
    pub role: UserRole,
    pub star: i32,
    pub ban_status: BanStatus,
    pub ban_reason: Option<String>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BanStatus {
    Normal,
    Suspended,
    Banned,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

impl Default for BanStatus {
    fn default() -> Self {
        BanStatus::Normal
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub qq_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub star: Option<i32>,
    pub ban_status: Option<BanStatus>,
    pub role: Option<UserRole>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
} 