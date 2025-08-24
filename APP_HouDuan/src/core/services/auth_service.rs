use std::sync::Arc;
use tracing::instrument;
use crate::infrastructure::database::repositories::UserRepository;
use crate::config::JwtConfig;

#[derive(Debug)]
pub struct AuthService {
    _user_repo: Arc<UserRepository>,
    _jwt_config: JwtConfig,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>, jwt_config: &JwtConfig) -> Self {
        Self { 
            _user_repo: user_repo,
            _jwt_config: jwt_config.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        tracing::debug!("Auth service health check");
        true
    }
} 