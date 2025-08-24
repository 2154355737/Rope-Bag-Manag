// 仓储实现模块
// 此模块包含所有数据访问层的实现

use sqlx::{Pool, Sqlite};
use tracing::instrument;

#[derive(Debug)]
pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PackageRepository {
    pool: Pool<Sqlite>,
}

impl PackageRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CategoryRepository {
    pool: Pool<Sqlite>,
}

impl CategoryRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CommentRepository {
    pool: Pool<Sqlite>,
}

impl CommentRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct NotificationRepository {
    pool: Pool<Sqlite>,
}

impl NotificationRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
} 