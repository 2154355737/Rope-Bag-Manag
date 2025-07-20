pub mod auth_service;
pub mod user_service;
pub mod package_service;
pub mod comment_service;
pub mod community_service;
pub mod admin_service;
pub mod user_action_service; // 添加用户行为记录服务

pub use auth_service::*;
pub use user_service::*;
pub use package_service::*;
pub use admin_service::*;
pub use community_service::*;
pub use comment_service::*; 