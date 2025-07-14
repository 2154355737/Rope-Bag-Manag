use anyhow::Result;
use crate::models::{Stats, Category, UserAction};
use crate::repositories::system_repo::SystemRepository;

#[derive(Clone)]
pub struct AdminService {
    system_repo: SystemRepository,
}

impl AdminService {
    pub fn new(system_repo: SystemRepository) -> Self {
        Self { system_repo }
    }

    pub async fn get_stats(&self) -> Result<Stats> {
        self.system_repo.get_stats().await
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        self.system_repo.get_categories().await
    }

    pub async fn get_user_actions(&self) -> Result<Vec<UserAction>> {
        self.system_repo.get_user_actions().await
    }
} 