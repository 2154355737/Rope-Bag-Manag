use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("认证错误: {0}")]
    Auth(String),
    
    #[error("授权错误: {0}")]
    Forbidden(String),
    
    #[error("资源未找到: {0}")]
    NotFound(String),
    
    #[error("验证错误: {0}")]
    Validation(String),
    
    #[error("业务逻辑错误: {0}")]
    Business(String),
    
    #[error("外部服务错误: {0}")]
    External(String),
    
    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON序列化错误: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("JWT错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("BCrypt错误: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    
    #[error("缓存错误: {0}")]
    Cache(String),
    
    #[error("邮件发送错误: {0}")]
    Email(String),
    
    #[error("文件操作错误: {0}")]
    File(String),
    
    #[error("内部服务器错误")]
    Internal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
            errors: None,
        }
    }
    
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            code: 0,
            message,
            data: Some(data),
            errors: None,
        }
    }
    
    pub fn error(code: i32, message: String) -> ApiResponse<()> {
        ApiResponse {
            code,
            message,
            data: None,
            errors: None,
        }
    }
    
    pub fn validation_error(errors: Vec<ValidationError>) -> ApiResponse<()> {
        ApiResponse {
            code: 400,
            message: "validation_failed".to_string(),
            data: None,
            errors: Some(errors),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, code, message) = match self {
            AppError::Auth(_) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                401,
                self.to_string(),
            ),
            AppError::Forbidden(_) => (
                actix_web::http::StatusCode::FORBIDDEN,
                403,
                self.to_string(),
            ),
            AppError::NotFound(_) => (
                actix_web::http::StatusCode::NOT_FOUND,
                404,
                self.to_string(),
            ),
            AppError::Validation(_) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                400,
                self.to_string(),
            ),
            AppError::Business(_) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                400,
                self.to_string(),
            ),
            _ => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                500,
                "内部服务器错误".to_string(),
            ),
        };
        
        HttpResponse::build(status_code).json(ApiResponse::<()>::error(code, message))
    }
}

// 便捷宏
#[macro_export]
macro_rules! app_error {
    (Auth, $msg:expr) => {
        $crate::shared::errors::AppError::Auth($msg.to_string())
    };
    (Forbidden, $msg:expr) => {
        $crate::shared::errors::AppError::Forbidden($msg.to_string())
    };
    (NotFound, $msg:expr) => {
        $crate::shared::errors::AppError::NotFound($msg.to_string())
    };
    (Validation, $msg:expr) => {
        $crate::shared::errors::AppError::Validation($msg.to_string())
    };
    (Business, $msg:expr) => {
        $crate::shared::errors::AppError::Business($msg.to_string())
    };
} 