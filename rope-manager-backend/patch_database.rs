use std::fs;
use std::path::Path;
use rusqlite::{Connection, Result};

/// 数据库修补工具
/// 用于安全地执行数据库结构更新和配置项添加
pub struct DatabasePatcher {
    db_path: String,
}

impl DatabasePatcher {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    /// 执行数据库修补
    pub fn patch(&self) -> Result<()> {
        println!("🔧 开始数据库修补...");
        
        // 检查数据库文件是否存在
        if !Path::new(&self.db_path).exists() {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some("数据库文件不存在".to_string())
            ));
        }

        // 备份原数据库
        self.backup_database()?;

        // 连接数据库
        let conn = Connection::open(&self.db_path)?;
        
        // 启用外键约束
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        // 读取修补脚本
        let patch_sql = fs::read_to_string("sql/patch_database_settings.sql")
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("无法读取修补脚本: {}", e))
            ))?;

        println!("📄 执行修补脚本...");
        
        // 执行修补脚本
        conn.execute_batch(&patch_sql)?;
        
        println!("✅ 数据库修补完成！");
        
        // 验证修补结果
        self.verify_patch(&conn)?;
        
        Ok(())
    }

    /// 备份数据库
    fn backup_database(&self) -> Result<()> {
        let backup_path = format!("{}.backup_{}", 
            self.db_path, 
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        
        println!("💾 备份数据库到: {}", backup_path);
        
        fs::copy(&self.db_path, &backup_path)
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("备份失败: {}", e))
            ))?;
        
        Ok(())
    }

    /// 验证修补结果
    fn verify_patch(&self, conn: &Connection) -> Result<()> {
        println!("🔍 验证修补结果...");
        
        // 检查system_settings表结构
        let mut stmt = conn.prepare("PRAGMA table_info(system_settings)")?;
        let column_info: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(1)?) // 获取列名
        })?.collect::<Result<Vec<_>>>()?;
        
        let expected_columns = vec!["key", "value", "description", "created_at", "updated_at"];
        for col in &expected_columns {
            if !column_info.contains(&col.to_string()) {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_SCHEMA),
                    Some(format!("缺少列: {}", col))
                ));
            }
        }
        
        // 检查关键配置项是否存在
        let key_settings = vec![
            "hero_title", "hero_subtitle", "homepage_sections",
            "resources_per_page", "posts_per_page", "default_sort",
            "copyright_text", "seo_keywords", "seo_description"
        ];
        
        for key in &key_settings {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM system_settings WHERE key = ?",
                [key],
                |row| row.get(0)
            )?;
            
            if count == 0 {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                    Some(format!("缺少配置项: {}", key))
                ));
            }
        }
        
        println!("✅ 验证通过！所有配置项已正确添加。");
        Ok(())
    }

    /// 显示修补状态
    pub fn show_status(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        println!("📊 数据库修补状态:");
        println!("================");
        
        // 统计配置项数量
        let settings_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM system_settings",
            [],
            |row| row.get(0)
        )?;
        
        println!("配置项总数: {}", settings_count);
        
        // 显示最近的修补日志
        let mut stmt = conn.prepare(
            "SELECT level, message, timestamp FROM system_logs 
             WHERE message LIKE '%修补%' OR message LIKE '%patch%' 
             ORDER BY timestamp DESC LIMIT 5"
        )?;
        
        let logs: Vec<(String, String, String)> = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?
            ))
        })?.collect::<Result<Vec<_>>>()?;
        
        if !logs.is_empty() {
            println!("\n最近的修补记录:");
            for (level, message, timestamp) in logs {
                println!("[{}] {} - {}", level, timestamp, message);
            }
        }
        
        Ok(())
    }

    /// 回滚到备份
    pub fn rollback(&self, backup_path: &str) -> Result<()> {
        println!("🔄 回滚数据库...");
        
        if !Path::new(backup_path).exists() {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some("备份文件不存在".to_string())
            ));
        }
        
        fs::copy(backup_path, &self.db_path)
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("回滚失败: {}", e))
            ))?;
        
        println!("✅ 数据库已回滚到备份版本");
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("🚀 绳包管理器 - 数据库修补工具");
    println!("================================");
    
    let args: Vec<String> = std::env::args().collect();
    let db_path = args.get(1).unwrap_or(&"data.db".to_string()).clone();
    
    let patcher = DatabasePatcher::new(&db_path);
    
    match args.get(2).map(|s| s.as_str()) {
        Some("status") => {
            patcher.show_status()?;
        },
        Some("rollback") => {
            if let Some(backup_path) = args.get(3) {
                patcher.rollback(backup_path)?;
            } else {
                eprintln!("❌ 请指定备份文件路径");
                std::process::exit(1);
            }
        },
        _ => {
            // 默认执行修补
            match patcher.patch() {
                Ok(()) => {
                    println!("\n🎉 数据库修补成功完成！");
                    println!("现在您可以使用新的配置管理功能了。");
                },
                Err(e) => {
                    eprintln!("❌ 修补失败: {}", e);
                    eprintln!("请检查错误信息并尝试回滚到备份版本。");
                    std::process::exit(1);
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_database_patcher() {
        // 创建临时数据库文件
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        // 创建基本的数据库结构
        let conn = Connection::open(db_path).unwrap();
        conn.execute_batch(r#"
            CREATE TABLE system_settings (
                id INTEGER PRIMARY KEY,
                key TEXT,
                value TEXT
            );
        "#).unwrap();
        
        // 测试修补器（注意：实际测试需要修补脚本文件）
        let patcher = DatabasePatcher::new(db_path);
        
        // 这里只测试基本功能，实际修补需要SQL文件
        assert!(patcher.show_status().is_ok());
    }
} 