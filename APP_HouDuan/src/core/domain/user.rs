use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub settings: serde_json::Value,
    pub stats: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[repr(u8)]
pub enum UserRole {
    #[serde(rename = "user")]
    User = 0,
    #[serde(rename = "elder")]
    Elder = 1,
    #[serde(rename = "moderator")]
    Moderator = 2,
    #[serde(rename = "admin")]
    Admin = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[repr(u8)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active = 0,
    #[serde(rename = "suspended")]
    Suspended = 1,
    #[serde(rename = "banned")]
    Banned = 2,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 6, max = 128))]
    pub password: String,
    
    #[validate(length(max = 100))]
    pub nickname: Option<String>,
    
    pub verification_code: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUserRequest {
    #[validate(length(max = 100))]
    pub nickname: Option<String>,
    
    #[validate(url)]
    pub avatar_url: Option<String>,
    
    #[validate(length(max = 500))]
    pub bio: Option<String>,
    
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub stats: UserStats,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub packages_count: i64,
    pub comments_count: i64,
    pub likes_received: i64,
    pub downloads_total: i64,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserProfile,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Elder => write!(f, "elder"),
            UserRole::Moderator => write!(f, "moderator"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::Banned => write!(f, "banned"),
        }
    }
}

impl User {
    pub fn can_access_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Moderator)
    }
    
    pub fn can_moderate(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Moderator | UserRole::Elder)
    }
    
    pub fn can_review_packages(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Moderator | UserRole::Elder)
    }
    
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
    
    pub fn to_profile(&self, stats: UserStats) -> UserProfile {
        UserProfile {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            nickname: self.nickname.clone(),
            avatar_url: self.avatar_url.clone(),
            bio: self.bio.clone(),
            role: self.role.clone(),
            status: self.status.clone(),
            stats,
            created_at: self.created_at,
        }
    }
} 