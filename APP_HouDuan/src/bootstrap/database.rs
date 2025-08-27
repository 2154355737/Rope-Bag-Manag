// 数据库初始化和管理模块

use anyhow::Result;
use log::{info, warn, error};
use rusqlite::Connection;
use std::fs;
use crate::config::Config;
use super::BootstrapError;

/// 数据库管理器
pub struct DatabaseManager {
    config: Config,
    db_path: String,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new(config: &Config) -> Result<Self, BootstrapError> {
        let db_path = config.database_url().to_string();
        
        // 创建必要的目录
        Self::create_directories(config)?;
        
        Ok(Self {
            config: config.clone(),
            db_path,
        })
    }
    
    /// 初始化数据库
    pub async fn initialize(&self) -> Result<(), BootstrapError> {
        info!("🗃️ 开始数据库初始化...");
        
        let conn = self.get_connection()?;
        
        // 执行初始化步骤
        self.run_init_sql(&conn).await?;
        self.run_migrations(&conn).await?;
        self.run_maintenance(&conn).await?;
        self.check_mail_config(&conn).await?;
        
        info!("✅ 数据库初始化完成");
        Ok(())
    }
    
    /// 获取数据库连接
    fn get_connection(&self) -> Result<Connection, BootstrapError> {
        Connection::open(&self.db_path)
            .map_err(|e| BootstrapError::Database(e))
    }
    
    /// 执行初始化SQL
    async fn run_init_sql(&self, conn: &Connection) -> Result<(), BootstrapError> {
        info!("📋 执行数据库初始化脚本...");
        
        match conn.execute_batch(include_str!("../../sql/init.sql")) {
            Ok(_) => {
                info!("✅ 数据库初始化脚本执行成功");
            }
            Err(e) => {
                if e.to_string().contains("already exists") {
                    info!("ℹ️ 数据库表已存在，跳过初始化");
                } else {
                    error!("❌ 数据库初始化失败: {}", e);
                    return Err(BootstrapError::Database(e));
                }
            }
        }
        Ok(())
    }
    
    /// 执行迁移脚本
    async fn run_migrations(&self, conn: &Connection) -> Result<(), BootstrapError> {
        info!("🔄 检查并执行数据库迁移...");
        
        match conn.execute_batch(include_str!("../../sql/migrations/001_add_missing_columns.sql")) {
            Ok(_) => {
                info!("✅ 数据库迁移执行成功");
            }
            Err(e) => {
                if e.to_string().contains("already exists") || e.to_string().contains("duplicate column") {
                    info!("ℹ️ 数据库结构已是最新版本");
                } else {
                    warn!("⚠️ 迁移执行警告: {}", e);
                    // 迁移失败不阻止启动
                }
            }
        }
        Ok(())
    }
    
    /// 执行数据库维护
    async fn run_maintenance(&self, conn: &Connection) -> Result<(), BootstrapError> {
        info!("🧹 执行数据库维护和优化...");
        
        match conn.execute_batch(include_str!("../../sql/database_manager.sql")) {
            Ok(_) => {
                info!("✅ 数据库维护完成");
            }
            Err(e) => {
                // 维护脚本可能包含查询操作，错误可以忽略
                info!("ℹ️ 数据库维护操作完成: {}", e);
            }
        }
        Ok(())
    }
    
    /// 检查和配置邮件服务
    async fn check_mail_config(&self, conn: &Connection) -> Result<(), BootstrapError> {
        info!("📧 检查邮件服务配置...");
        
        match conn.prepare("SELECT username, password, enabled FROM mail_settings WHERE id = 1") {
            Ok(mut stmt) => {
                if let Ok(row) = stmt.query_row([], |row| {
                    Ok((
                        row.get::<_, String>(0)?, // username
                        row.get::<_, String>(1)?, // password
                        row.get::<_, i32>(2)?     // enabled
                    ))
                }) {
                    let (username, password, enabled) = row;
                    if !username.is_empty() && !password.is_empty() && enabled == 0 {
                        match conn.execute("UPDATE mail_settings SET enabled = 1 WHERE id = 1", []) {
                            Ok(_) => info!("✅ 检测到有效的邮件配置，已自动启用邮件服务"),
                            Err(e) => warn!("⚠️ 自动启用邮件服务失败: {}", e),
                        }
                    }
                }
            }
            Err(_) => {
                info!("ℹ️ 邮件配置表不存在，将使用默认配置");
            }
        }
        Ok(())
    }
    
    /// 创建必要的目录
    fn create_directories(config: &Config) -> Result<(), BootstrapError> {
        // 创建上传目录
        fs::create_dir_all(&config.file.upload_path)?;
        
        // 创建临时目录
        fs::create_dir_all(&config.file.temp_path)?;
        
        // 创建日志目录
        if let Some(log_path) = &config.logging.file_path {
            if let Some(log_dir) = std::path::Path::new(log_path).parent() {
                fs::create_dir_all(log_dir)?;
            }
        }
        
        info!("✅ 必要目录创建完成");
        Ok(())
    }
    
    /// 获取数据库连接（供其他模块使用）
    pub fn create_connection(&self) -> Result<Connection, BootstrapError> {
        self.get_connection()
    }
}