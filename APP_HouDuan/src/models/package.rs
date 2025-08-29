use serde::{Deserialize, Serialize, Serializer};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFile {
    pub name: String,
    pub size: i64,
    pub file_type: String,
    pub download_url: Option<String>,
}

// 自定义序列化函数：将Option<Vec<String>>中的None转换为空数组
fn serialize_option_vec<S>(option: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option {
        Some(vec) => vec.serialize(serializer),
        None => Vec::<String>::new().serialize(serializer),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub file_url: Option<String>,
    pub file_size: Option<i64>,
    pub download_count: i32,
    pub like_count: i32,
    pub favorite_count: i32,
    pub category_id: Option<i32>,
    pub status: PackageStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub reviewer_id: Option<i32>,      // 审核者ID
    pub reviewed_at: Option<DateTime<Utc>>, // 审核时间
    pub review_comment: Option<String>, // 审核备注
    // 置顶和精华字段
    pub is_pinned: bool,               // 是否置顶
    pub is_featured: bool,             // 是否精华
    // 关联标签
    #[serde(serialize_with = "serialize_option_vec")]
    pub tags: Option<Vec<String>>, // 名称列表，避免循环引用
    // 新增字段 - 支持前端发布页面
    #[serde(serialize_with = "serialize_option_vec")]
    pub screenshots: Option<Vec<String>>, // 预览截图URLs
    pub cover_image: Option<String>,      // 封面图片URL
    #[serde(serialize_with = "serialize_option_vec")]
    pub requirements: Option<Vec<String>>, // 系统要求列表
    pub included_files: Option<Vec<PackageFile>>, // 包含的文件列表
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
    pub tags: Option<Vec<String>>, // 新增
    // 置顶和精华字段
    pub is_pinned: Option<bool>,
    pub is_featured: Option<bool>,
    // 新增字段 - 支持前端发布页面
    pub screenshots: Option<Vec<String>>, // 预览截图URLs
    pub cover_image: Option<String>,      // 封面图片URL
    pub requirements: Option<Vec<String>>, // 系统要求列表
    pub included_files: Option<Vec<PackageFile>>, // 包含的文件列表
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePackageRequest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub status: Option<PackageStatus>,
    pub file_url: Option<String>,
    pub file_size: Option<i64>, // 文件大小
    pub tags: Option<Vec<String>>, // 新增
    // 置顶和精华字段
    pub is_pinned: Option<bool>,
    pub is_featured: Option<bool>,
    // 审核相关字段
    pub reviewer_id: Option<i32>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_comment: Option<String>,
    // 新增字段 - 支持前端发布页面
    pub screenshots: Option<Vec<String>>, // 预览截图URLs
    pub cover_image: Option<String>,      // 封面图片URL
    pub requirements: Option<Vec<String>>, // 系统要求列表
    pub included_files: Option<Vec<PackageFile>>, // 包含的文件列表
    // 管理员可修改作者字段
    pub author: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageListResponse {
    pub list: Vec<Package>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
} 

// 前端发布页面专用的请求结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishResourceRequest {
    pub title: String,
    pub content: String, // 对应description
    pub version: Option<String>,
    pub category: Option<String>, // 分类名称
    pub tags: Option<Vec<String>>,
    pub requirements: Option<Vec<String>>,
    // 文件相关 - 这些在发布时可能为空，后续通过上传接口填充
    pub files: Option<Vec<PublishFileInfo>>,
    pub screenshots: Option<Vec<PublishFileInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishFileInfo {
    pub name: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
}

// 前端发布帖子专用的请求结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishPostRequest {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub images: Option<Vec<PublishFileInfo>>,
    pub code_snippet: Option<String>,
} 