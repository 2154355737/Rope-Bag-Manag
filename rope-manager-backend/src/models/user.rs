use serde::{Deserialize, Serialize, Serializer};
use chrono::{DateTime, Utc};
use std::fmt;
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub nickname: Option<String>,
    pub role: UserRole,
    pub star: i32,
    pub ban_status: BanStatus,
    pub ban_reason: Option<String>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
    pub login_count: i32,
    pub upload_count: i32,
    pub download_count: i32,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Moderator,
    Elder,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::Elder => "elder",
            UserRole::User => "user",
        };
        write!(f, "{}", s)
    }
}

impl serde::Serialize for UserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::Elder => "elder",
            UserRole::User => "user",
        };
        serializer.serialize_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
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