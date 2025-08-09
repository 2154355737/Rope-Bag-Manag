pub mod user;
pub mod package;
pub mod system;
pub mod comment;
pub mod download_security;
pub mod notification;
pub mod resource_record;
pub mod post;
pub mod tag;
pub mod user_action;
pub mod mail;

use serde::{Serialize, Deserialize};

pub use user::*;
pub use package::*;
pub use comment::*;
pub use system::*;
pub use resource_record::*;
pub use post::*;
pub use tag::*;
pub use download_security::*;
pub use notification::*;
pub use user_action::*;
pub use mail::*;

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