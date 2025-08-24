use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 资源实体 - 核心业务对象
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Resource {
    pub id: i64,
    pub title: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author_id: i64,
    pub category_id: i64,
    pub file_path: String,
    pub file_size: i64,
    pub download_count: i64,
    pub like_count: i64,
    pub view_count: i64,
    pub comment_count: i64,
    pub rating: f64,
    pub status: ResourceStatus,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub requirements: Option<String>, // JSON array as string
    pub tags: Option<String>, // JSON array as string
    pub screenshots: Option<String>, // JSON array as string
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 资源状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "resource_status", rename_all = "lowercase")]
pub enum ResourceStatus {
    Draft,     // 草稿
    Published, // 已发布
    Archived,  // 已归档
    Banned,    // 已封禁
}

/// 创建资源请求DTO
#[derive(Debug, Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub category_id: i64,
    pub file_size: i64,
    pub requirements: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<String>>,
}

/// 更新资源请求DTO
#[derive(Debug, Deserialize)]
pub struct UpdateResourceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub category_id: Option<i64>,
    pub requirements: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<String>>,
}

/// 资源搜索参数
#[derive(Debug, Clone)]
pub struct ResourceSearchParams {
    pub query: Option<String>,
    pub category_id: Option<i64>,
    pub tag: Option<String>,
    pub status: Option<ResourceStatus>,
    pub author_id: Option<i64>,
    pub is_featured: Option<bool>,
    pub page: i64,
    pub page_size: i64,
}

/// 资源详情响应DTO
#[derive(Debug, Serialize)]
pub struct ResourceDetail {
    #[serde(flatten)]
    pub resource: Resource,
    pub author: Option<UserProfile>,
    pub category: Option<CategoryInfo>,
    pub is_liked: bool,
    pub is_bookmarked: bool,
    pub download_url: Option<String>,
}

/// 用户资料信息
#[derive(Debug, Serialize, Clone)]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
}

/// 分类信息
#[derive(Debug, Serialize, Clone)]
pub struct CategoryInfo {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

/// 创建资源数据
#[derive(Debug, Clone)]
pub struct CreateResourceData {
    pub title: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author_id: i64,
    pub category_id: i64,
    pub file_path: String,
    pub file_size: i64,
    pub status: ResourceStatus,
    pub requirements: Option<String>,
    pub tags: Option<String>,
    pub screenshots: Option<String>,
}

impl Resource {
    /// 检查资源是否可以被下载
    pub fn is_downloadable(&self) -> bool {
        matches!(self.status, ResourceStatus::Published)
    }
    
    /// 检查用户是否可以编辑此资源
    pub fn can_edit(&self, user_id: i64, user_role: &str) -> bool {
        self.author_id == user_id || user_role == "admin" || user_role == "moderator"
    }
    
    /// 增加下载次数
    pub fn increment_download_count(&mut self) {
        self.download_count += 1;
    }
    
    /// 增加查看次数
    pub fn increment_view_count(&mut self) {
        self.view_count += 1;
    }
    
    /// 转换为详情响应
    pub fn to_detail(
        self,
        author: Option<UserProfile>,
        category: Option<CategoryInfo>,
        is_liked: bool,
        is_bookmarked: bool,
        download_url: Option<String>,
    ) -> ResourceDetail {
        ResourceDetail {
            resource: self,
            author,
            category,
            is_liked,
            is_bookmarked,
            download_url,
        }
    }
}

impl CreateResourceRequest {
    /// 转换为创建数据
    pub fn to_create_data(
        self,
        author_id: i64,
        file_path: String,
    ) -> CreateResourceData {
        CreateResourceData {
            title: self.title,
            name: self.name,
            description: self.description,
            version: self.version,
            author_id,
            category_id: self.category_id,
            file_path,
            file_size: self.file_size,
            status: ResourceStatus::Draft,
            requirements: self.requirements.map(|r| serde_json::to_string(&r).unwrap_or_default()),
            tags: self.tags.map(|t| serde_json::to_string(&t).unwrap_or_default()),
            screenshots: self.screenshots.map(|s| serde_json::to_string(&s).unwrap_or_default()),
        }
    }
}

impl Default for ResourceStatus {
    fn default() -> Self {
        ResourceStatus::Draft
    }
} 