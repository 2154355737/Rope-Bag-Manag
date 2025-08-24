pub mod models;
pub mod repositories;

use sqlx::{Pool, Sqlite, SqlitePool};
use tracing::{info, error, instrument};

use crate::config::DatabaseConfig;
use crate::shared::errors::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: Pool<Sqlite>,
}

impl DatabaseManager {
    #[instrument(skip(config))]
    pub async fn new(config: &DatabaseConfig) -> AppResult<Self> {
        info!("正在连接数据库: {}", config.url);
        
        // 确保数据库目录存在
        if let Some(db_path) = config.url.strip_prefix("sqlite:") {
            if let Some(parent) = std::path::Path::new(db_path).parent() {
                tokio::fs::create_dir_all(parent).await?;
                info!("数据库目录已创建: {:?}", parent);
            }
        }
        
        let pool = SqlitePool::connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(&config.url.strip_prefix("sqlite:").unwrap_or(&config.url))
                .create_if_missing(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                .foreign_keys(true)
        )
        .await
        .map_err(|e| {
            error!("数据库连接失败: {}", e);
            AppError::Database(e)
        })?;
        
        info!("数据库连接成功");
        Ok(Self { pool })
    }
    
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
    
    #[instrument(skip(self))]
    pub async fn run_migrations(&self) -> AppResult<()> {
        info!("开始运行数据库迁移...");
        
        // 执行迁移SQL
        let migration_sql = include_str!("../../../migrations/001_initial.sql");
        
        let mut tx = self.pool.begin().await?;
        
        // 分割SQL语句并逐个执行
        let statements: Vec<&str> = migration_sql
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
            
        info!("准备执行 {} 条SQL语句", statements.len());
        
        for (i, statement) in statements.iter().enumerate() {
            if !statement.is_empty() {
                sqlx::query(statement)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        error!("执行第{}条迁移SQL失败: {} - {}", i + 1, statement, e);
                        AppError::Database(e)
                    })?;
            }
        }
        
        tx.commit().await?;
        info!("数据库迁移完成 - 执行了 {} 条SQL语句", statements.len());
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> AppResult<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
} 