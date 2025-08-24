use std::sync::Arc;
use tracing::instrument;
use crate::infrastructure::database::repositories::{PackageRepository, UserRepository, CategoryRepository};

#[derive(Debug)]
pub struct PackageService {
    _package_repo: Arc<PackageRepository>,
    _user_repo: Arc<UserRepository>,
    _category_repo: Arc<CategoryRepository>,
}

impl PackageService {
    pub fn new(
        package_repo: Arc<PackageRepository>, 
        user_repo: Arc<UserRepository>,
        category_repo: Arc<CategoryRepository>,
    ) -> Self {
        Self { 
            _package_repo: package_repo,
            _user_repo: user_repo,
            _category_repo: category_repo,
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        tracing::debug!("Package service health check");
        true
    }
} 