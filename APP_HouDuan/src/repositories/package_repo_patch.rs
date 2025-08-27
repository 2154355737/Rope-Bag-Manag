// 补丁函数，用于解析JSON字段
pub fn parse_json_array(json_str: Option<String>) -> Option<Vec<String>> {
    json_str.and_then(|s| serde_json::from_str(&s).ok())
}

// 补丁函数，用于序列化Vec<String>为JSON字符串  
pub fn serialize_json_array(arr: &Option<Vec<String>>) -> Option<String> {
    arr.as_ref().and_then(|v| serde_json::to_string(v).ok())
}

// 用于Package结构体创建的标准字段列表
pub const PACKAGE_SELECT_FIELDS: &str = "id, name, author, version, description, file_url, file_size, \
                                         download_count, like_count, favorite_count, category_id, status, \
                                         created_at, updated_at, reviewer_id, reviewed_at, review_comment, \
                                         is_pinned, is_featured, screenshots, cover_image, requirements";

// 标准Package结构体创建宏
#[macro_export]
macro_rules! create_package_from_row {
    ($row:expr) => {
        {
            use crate::repositories::package_repo_patch::parse_json_array;
            Package {
                id: $row.get(0)?,
                name: $row.get(1)?,
                author: $row.get(2)?,
                version: $row.get(3)?,
                description: $row.get(4)?,
                file_url: $row.get(5)?,
                file_size: $row.get(6)?,
                download_count: $row.get(7)?,
                like_count: $row.get(8)?,
                favorite_count: $row.get(9)?,
                category_id: $row.get(10)?,
                status: match $row.get::<_, String>(11)?.as_str() {
                    "pending" => crate::models::PackageStatus::Pending,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: $row.get(12)?,
                updated_at: $row.get(13)?,
                reviewer_id: $row.get(14)?,
                reviewed_at: $row.get(15)?,
                review_comment: $row.get(16)?,
                is_pinned: $row.get(17).unwrap_or(false),
                is_featured: $row.get(18).unwrap_or(false),
                screenshots: parse_json_array($row.get::<_, Option<String>>(19).ok().flatten()),
                cover_image: $row.get(20).ok(),
                requirements: parse_json_array($row.get::<_, Option<String>>(21).ok().flatten()),
                tags: None, // 这里会在后面填充
            }
        }
    };
} 