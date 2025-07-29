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
    // 审核相关字段
    pub reviewer_id: Option<i32>,      // 审核者ID
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
    pub review_comment: Option<String>, // 审核备注
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PackageStatus {
    Pending,    // 待审核
    Active,     // 已上架
    Rejected,   // 审核拒绝
    Inactive,   // 已下架
    Deleted,    // 已删除
}

impl Default for PackageStatus {
    fn default() -> Self {
        PackageStatus::Pending  // 新上传的资源默认为待审核状态
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePackageRequest {
    pub name: String,
    pub author: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub file_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePackageRequest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub status: Option<PackageStatus>,
    pub file_url: Option<String>,
    // 审核相关字段
    pub reviewer_id: Option<i32>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageListResponse {
    pub list: Vec<Package>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
} 