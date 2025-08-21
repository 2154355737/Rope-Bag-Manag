use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub skills: Option<String>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Moderator,
    Elder,
    User,
}

impl<'de> serde::Deserialize<'de> for UserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "admin" => Ok(UserRole::Admin),
            "moderator" => Ok(UserRole::Moderator),
            "elder" => Ok(UserRole::Elder),
            "user" => Ok(UserRole::User),
            _ => Err(serde::de::Error::custom(format!("Invalid user role: {}", s))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BanStatus {
    Normal,
    Suspended,
    Banned,
}

impl<'de> serde::Deserialize<'de> for BanStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "normal" => Ok(BanStatus::Normal),
            "suspended" => Ok(BanStatus::Suspended),
            "banned" => Ok(BanStatus::Banned),
            _ => Err(serde::de::Error::custom(format!("Invalid ban status: {}", s))),
        }
    }
}

impl serde::Serialize for BanStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            BanStatus::Normal => "normal",
            BanStatus::Suspended => "suspended", 
            BanStatus::Banned => "banned",
        };
        serializer.serialize_str(s)
    }
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

impl fmt::Display for BanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BanStatus::Normal => write!(f, "normal"),
            BanStatus::Suspended => write!(f, "suspended"),
            BanStatus::Banned => write!(f, "banned"),
        }
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
    pub verification_code: String, // 邮箱验证码
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailLoginRequest {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendCodeRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub star: Option<i32>,
    pub ban_status: Option<BanStatus>,
    pub ban_reason: Option<String>,
    pub role: Option<UserRole>,
    pub qq_number: Option<String>,
    pub avatar_url: Option<String>,
} 