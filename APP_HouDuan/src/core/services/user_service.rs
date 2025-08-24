use std::sync::Arc;
use tracing::instrument;
use crate::infrastructure::database::repositories::UserRepository;

#[derive(Debug)]
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        // 简单的健康检查示例
        tracing::debug!("User service health check");
        true
    }
} 