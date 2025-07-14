use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub file_url: String,
    pub file_size: Option<i64>,
    pub download_count: i32,
    pub like_count: i32,
    pub favorite_count: i32,
    pub category_id: Option<i32>,
    pub status: PackageStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PackageStatus {
    Active,
    Inactive,
    Deleted,
}

impl Default for PackageStatus {
    fn default() -> Self {
        PackageStatus::Active
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePackageRequest {
    pub name: String,
    pub author: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePackageRequest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub status: Option<PackageStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageListResponse {
    pub list: Vec<Package>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
} 