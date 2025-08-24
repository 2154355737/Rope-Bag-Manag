use std::sync::Arc;
use tracing::instrument;
use crate::infrastructure::database::repositories::CategoryRepository;

#[derive(Debug)]
pub struct CategoryService {
    _category_repo: Arc<CategoryRepository>,
}

impl CategoryService {
    pub fn new(category_repo: Arc<CategoryRepository>) -> Self {
        Self { _category_repo: category_repo }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        tracing::debug!("Category service health check");
        true
    }
} 