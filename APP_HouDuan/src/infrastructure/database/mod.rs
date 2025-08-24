pub mod models;
pub mod repositories;

use sqlx::{Pool, Sqlite, SqlitePool};
use tracing::{info, error, instrument, warn};

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
        
        // 使用配置中的 sqlite 相对路径（相对于进程工作目录），与目录创建保持一致
        let raw_path = config.url.strip_prefix("sqlite:").unwrap_or(&config.url);
        let db_path = std::path::Path::new(raw_path);
        info!("数据库文件（工作目录相对）: {}", db_path.display());
        
        // 确保数据库目录存在（基于工作目录）
        if let Some(parent) = db_path.parent() {
            if let Err(e) = tokio::fs::create_dir_all(parent).await {
                warn!("创建数据库目录失败: {} - {}", parent.display(), e);
            } else {
                info!("数据库目录已创建: {}", parent.display());
            }
        }
        
        let pool = SqlitePool::connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(&db_path)
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

    /// 计时执行器：执行一个异步闭包，并记录耗时，超过阈值触发慢查询告警
    pub async fn timed<F, Fut, T>(&self, label: &str, threshold_ms: u128, f: F) -> AppResult<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, sqlx::Error>>,
    {
        let start = std::time::Instant::now();
        let result = f().await;
        let elapsed = start.elapsed().as_millis();
        if elapsed >= threshold_ms {
            warn!(operation = %label, duration_ms = elapsed, "慢查询/慢操作检测到");
        } else {
            info!(operation = %label, duration_ms = elapsed, "数据库操作完成");
        }
        result.map_err(AppError::Database)
    }
    
    #[instrument(skip(self))]
    pub async fn run_migrations(&self) -> AppResult<()> {
        info!("开始运行数据库迁移...");

        // 建立迁移表（幂等）
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS schema_migrations (version TEXT PRIMARY KEY, applied_at DATETIME DEFAULT CURRENT_TIMESTAMP)"
        )
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        // 运行 001 初始迁移（幂等）
        let migration_001 = include_str!("../../../migrations/001_initial.sql");
        let count_001 = Self::execute_sql_script(self.pool(), migration_001).await?;
        info!("迁移 001 执行完成（语句数: {}）", count_001);
        sqlx::query("INSERT OR IGNORE INTO schema_migrations (version) VALUES ('001')")
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        // 运行 002 迁移（仅未执行时）
        let applied_002: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM schema_migrations WHERE version = '002' LIMIT 1"
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)?;

        if applied_002.is_none() {
            info!("检测到未应用的迁移 002，开始执行...");
            let migration_002 = include_str!("../../../migrations/002_refactor_adjustments.sql");
            let count_002 = Self::execute_sql_script(self.pool(), migration_002).await?;
            info!("迁移 002 执行完成（语句数: {}）", count_002);
            sqlx::query("INSERT OR IGNORE INTO schema_migrations (version) VALUES ('002')")
                .execute(&self.pool)
                .await
                .map_err(AppError::Database)?;
        } else {
            info!("迁移 002 已应用，跳过。");
        }

        // 运行 003 迁移（仅未执行时）
        let applied_003: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM schema_migrations WHERE version = '003' LIMIT 1"
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)?;
        if applied_003.is_none() {
            info!("检测到未应用的迁移 003，开始执行...");
            let migration_003 = include_str!("../../../migrations/003_storage_files.sql");
            let count_003 = Self::execute_sql_script(self.pool(), migration_003).await?;
            info!("迁移 003 执行完成（语句数: {}）", count_003);
            sqlx::query("INSERT OR IGNORE INTO schema_migrations (version) VALUES ('003')")
                .execute(&self.pool)
                .await
                .map_err(AppError::Database)?;
        } else {
            info!("迁移 003 已应用，跳过。");
        }

        info!("数据库迁移全部完成");
        Ok(())
    }

    async fn execute_sql_script(pool: &Pool<Sqlite>, script: &str) -> AppResult<usize> {
        let mut tx = pool.begin().await.map_err(AppError::Database)?;
        let statements = Self::split_sql_statements(script);
        let mut executed: usize = 0;

        for statement in statements {
            let trimmed = statement.trim();
            if trimmed.is_empty() { continue; }
            sqlx::query(trimmed)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    error!("执行迁移SQL失败: {} - {}", trimmed, e);
                    AppError::Database(e)
                })?;
            executed += 1;
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(executed)
    }

    fn split_sql_statements(script: &str) -> Vec<String> {
        let mut statements: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut begin_end_depth: i32 = 0;
        let mut word = String::new();

        for ch in script.chars() {
            match ch {
                '\'' => {
                    if !in_double_quote { in_single_quote = !in_single_quote; }
                    current.push(ch);
                    word.clear();
                }
                '"' => {
                    if !in_single_quote { in_double_quote = !in_double_quote; }
                    current.push(ch);
                    word.clear();
                }
                ';' => {
                    // 在遇到分号前，先根据累计的单词判断是否是 END，从而正确关闭触发器块
                    if !in_single_quote && !in_double_quote {
                        if word.to_uppercase() == "END" && begin_end_depth > 0 {
                            begin_end_depth -= 1;
                        }
                        current.push(ch);
                        if begin_end_depth == 0 {
                            let stmt = current.trim().to_string();
                            if !stmt.is_empty() { statements.push(stmt); }
                            current.clear();
                        }
                        word.clear();
                    } else {
                        current.push(ch);
                    }
                }
                c => {
                    // 维护 BEGIN/END 计数（仅在非引号内生效）
                    if !in_single_quote && !in_double_quote {
                        if c.is_alphabetic() {
                            word.push(c);
                        } else {
                            if !word.is_empty() {
                                let upper = word.to_uppercase();
                                if upper == "BEGIN" {
                                    begin_end_depth += 1;
                                } else if upper == "END" && begin_end_depth > 0 {
                                    begin_end_depth -= 1;
                                }
                                word.clear();
                            }
                        }
                    }
                    current.push(c);
                }
            }
        }

        let tail = current.trim();
        if !tail.is_empty() {
            statements.push(tail.to_string());
        }

        statements
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