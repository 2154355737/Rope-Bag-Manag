pub mod user;
pub mod package;
pub mod comment;
pub mod resource_record;
pub mod system;
pub mod user_action; // 添加用户行为记录模型
pub mod mail; // 添加邮件模块
pub mod post; // 添加帖子模型
pub mod tag; // 添加标签模型
pub mod download_security; // 添加下载安全模型

use serde::{Serialize, Deserialize};

pub use user::*;
pub use package::*;
pub use comment::*;
pub use system::*;
pub use resource_record::*;
pub use post::*;
pub use tag::*;
pub use download_security::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }
    
    pub fn success_msg(message: &str) -> Self {
        Self {
            code: 0,
            message: message.to_string(),
            data: None,
        }
    }
    
    pub fn error(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

impl ApiError {
    pub fn new(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
} 