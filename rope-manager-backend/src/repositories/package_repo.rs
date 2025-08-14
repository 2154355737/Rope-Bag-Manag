use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};
use crate::models::{Package, Category};
use crate::models::Tag; // 需要Tag模型
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
#[derive(Debug)]
pub struct PackageRepository {
    conn: Arc<Mutex<Connection>>,
}

impl PackageRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn get_all_packages(&self) -> Result<Vec<Package>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, author, version, description, file_url, file_size, \
                    download_count, like_count, favorite_count, category_id, status, \
                    created_at, updated_at, reviewer_id, reviewed_at, review_comment, \
                    is_pinned, is_featured \
             FROM packages ORDER BY is_pinned DESC, is_featured DESC, created_at DESC";
        log::debug!("🗄️ SQL: get_all_packages: {}", sql);
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => {
                log::error!("❌ prepare failed: {}", e);
                return Err(e.into());
            }
        };
        let mut packages = match stmt.query_map([], |row| {
            Ok(Package {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                version: row.get(3)?,
                description: row.get(4)?,
                file_url: row.get(5)?,
                file_size: row.get(6)?,
                download_count: row.get(7)?,
                like_count: row.get(8)?,
                favorite_count: row.get(9)?,
                category_id: row.get(10)?,
                status: match row.get::<_, String>(11)?.as_str() {
                    "pending" => crate::models::PackageStatus::Pending,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
                reviewer_id: row.get(14)?,
                reviewed_at: row.get(15)?,
                review_comment: row.get(16)?,
                is_pinned: row.get(17).unwrap_or(false),
                is_featured: row.get(18).unwrap_or(false),
                tags: None, // 这里会在后面填充
            })
        }) {
            Ok(res) => match res.collect::<Result<Vec<_>, _>>() {
                Ok(list) => list,
                Err(e) => {
                    log::error!("❌ collect failed: {}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                log::error!("❌ query_map failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[SQL] get_all_packages result count: {}", packages.len());
        // 为每个package填充标签
        for pkg in &mut packages {
            pkg.tags = Some(Self::get_tags_for_package_internal(&conn, pkg.id)?);
        }
        Ok(packages)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Package>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, author, version, description, file_url, file_size, \
                    download_count, like_count, favorite_count, category_id, status, \
                    created_at, updated_at, reviewer_id, reviewed_at, review_comment, \
                    is_pinned, is_featured \
             FROM packages WHERE id = ?";
        log::debug!("🗄️ SQL: find_by_id: {} | id={}", sql, id);
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => {
                log::error!("❌ prepare failed: {}", e);
                return Err(e.into());
            }
        };
        let package = match stmt.query_row(params![id], |row| {
            Ok(Package {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                version: row.get(3)?,
                description: row.get(4)?,
                file_url: row.get(5)?,
                file_size: row.get(6)?,
                download_count: row.get(7)?,
                like_count: row.get(8)?,
                favorite_count: row.get(9)?,
                category_id: row.get(10)?,
                status: match row.get::<_, String>(11)?.as_str() {
                    "pending" => crate::models::PackageStatus::Pending,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
                reviewer_id: row.get(14)?,
                reviewed_at: row.get(15)?,
                review_comment: row.get(16)?,
                is_pinned: row.get(17).unwrap_or(false),
                is_featured: row.get(18).unwrap_or(false),
                tags: None, // 这里会在后面填充
            })
        }) {
            Ok(val) => Some(val),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => {
                log::error!("❌ query_row failed: {}", e);
                return Err(e.into());
            }
        };
        log::debug!("🗄️ SQL: find_by_id result: {:?}", package);
        let package = if let Some(mut pkg) = package {
            pkg.tags = Some(Self::get_tags_for_package_internal(&conn, pkg.id)?);
            Some(pkg)
        } else { None };
        log::debug!("🗄️ SQL: find_by_id tags fetched");
        Ok(package)
    }

    pub async fn create_package(&self, package: &Package) -> Result<Package> {
        let conn = self.conn.lock().await;
        let sql = "INSERT INTO packages (name, author, version, description, file_url, file_size, \
                                  download_count, like_count, favorite_count, category_id, status, \
                                  created_at, updated_at, is_pinned, is_featured) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        log::debug!("🗄️ SQL: create_package: {}", sql);
        let params = params![
            package.name,
            package.author,
            package.version.as_deref(),
            package.description.as_deref(),
            package.file_url,
            package.file_size,
            package.download_count,
            package.like_count,
            package.favorite_count,
            package.category_id,
            match package.status {
                crate::models::PackageStatus::Pending => "pending",
                crate::models::PackageStatus::Active => "active",
                crate::models::PackageStatus::Rejected => "rejected",
                crate::models::PackageStatus::Inactive => "inactive",
                crate::models::PackageStatus::Deleted => "deleted",
            },
            package.created_at.to_rfc3339(),
            package.updated_at.to_rfc3339(),
            package.is_pinned,
            package.is_featured,
        ];
        match conn.execute(sql, params) {
            Ok(rows) => println!("[SQL] create_package affected rows: {}", rows),
            Err(e) => {
                log::error!("❌ create_package failed: {}", e);
                return Err(e.into());
            }
        }
        
        // 获取最后插入的ID
        let last_id = conn.last_insert_rowid() as i32;
        
        // 创建包含ID的新包对象
        let mut created_package = package.clone();
        created_package.id = last_id;
        // 插入标签关联
        if let Some(ref tags) = created_package.tags {
            Self::replace_tags_for_package_internal(&conn, created_package.id, tags)?;
        }
        Ok(created_package)
    }

    pub async fn update_package(&self, package: &Package) -> Result<()> {
        let conn = self.conn.lock().await;
        let sql = "UPDATE packages SET name = ?, author = ?, version = ?, description = ?, \
                    file_url = ?, file_size = ?, download_count = ?, like_count = ?, \
                    favorite_count = ?, category_id = ?, status = ?, created_at = ?, \
                    updated_at = ?, is_pinned = ?, is_featured = ? WHERE id = ?";
        log::debug!("🗄️ SQL: update_package: {} | id={}", sql, package.id);
        let params = params![
            package.name,
            package.author,
            package.version.as_deref(),
            package.description.as_deref(),
            package.file_url,
            package.file_size,
            package.download_count,
            package.like_count,
            package.favorite_count,
            package.category_id,
            match package.status {
                crate::models::PackageStatus::Pending => "pending",
                crate::models::PackageStatus::Active => "active",
                crate::models::PackageStatus::Rejected => "rejected",
                crate::models::PackageStatus::Inactive => "inactive",
                crate::models::PackageStatus::Deleted => "deleted",
            },
            package.created_at.to_rfc3339(),
            package.updated_at.to_rfc3339(),
            package.is_pinned,
            package.is_featured,
            package.id,
        ];
        match conn.execute(sql, params) {
            Ok(rows) => println!("[SQL] update_package affected rows: {}", rows),
            Err(e) => {
                log::error!("❌ update_package failed: {}", e);
                return Err(e.into());
            }
        }
        // 更新标签关联
        if let Some(ref tags) = package.tags {
            Self::replace_tags_for_package_internal(&conn, package.id, tags)?;
        }
        Ok(())
    }

    pub async fn delete_package(&self, package_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 开始事务，确保所有删除操作要么全部成功，要么全部回滚
        conn.execute("BEGIN TRANSACTION", [])?;
        
        let result = (|| -> Result<()> {
            // 1. 删除package_likes表中的相关记录
            log::debug!("🗄️ 删除package_likes中的相关记录: package_id={}", package_id);
            conn.execute("DELETE FROM package_likes WHERE package_id = ?", params![package_id])?;
            
            // 2. 删除comments表中的相关记录
            log::debug!("🗄️ 删除comments中的相关记录: package_id={}", package_id);
            conn.execute("DELETE FROM comments WHERE package_id = ?", params![package_id])?;
            
            // 3. 删除package_tags表中的相关记录（如果存在）
            log::debug!("🗄️ 删除package_tags中的相关记录: package_id={}", package_id);
            conn.execute("DELETE FROM package_tags WHERE package_id = ?", params![package_id])?;
            
            // 4. 删除package_views表中的相关记录（如果存在）
            log::debug!("🗄️ 删除package_views中的相关记录: package_id={}", package_id);
            conn.execute("DELETE FROM package_views WHERE package_id = ?", params![package_id])?;
            
            // 5. 删除download_records表中的相关记录（如果存在）
            log::debug!("🗄️ 删除download_records中的相关记录: package_id={}", package_id);
            conn.execute("DELETE FROM download_records WHERE package_id = ?", params![package_id])?;
            
            // 6. 最后删除packages表中的记录
            let sql = "DELETE FROM packages WHERE id = ?";
            log::debug!("🗄️ SQL: delete_package: {} | id={}", sql, package_id);
            match conn.execute(sql, params![package_id]) {
                Ok(rows) => {
                    log::info!("[SQL] delete_package affected rows: {}", rows);
                    if rows == 0 {
                        return Err(anyhow::anyhow!("Package with ID {} not found", package_id));
                    }
                },
                Err(e) => {
                    log::error!("❌ delete_package failed: {}", e);
                    return Err(e.into());
                }
            }
            
            Ok(())
        })();
        
        // 根据执行结果提交或回滚事务
        match result {
            Ok(()) => {
                conn.execute("COMMIT", [])?;
                log::info!("✅ 成功删除绳包及所有相关记录: package_id={}", package_id);
                Ok(())
            },
            Err(e) => {
                conn.execute("ROLLBACK", [])?;
                log::error!("❌ 删除绳包失败，已回滚所有操作: {}", e);
                Err(e)
            }
        }
    }

    pub async fn increment_download_count(&self, package_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        let sql = "UPDATE packages SET download_count = download_count + 1 WHERE id = ?";
        log::debug!("🗄️ SQL: increment_download_count: {} | id={}", sql, package_id);
        match conn.execute(sql, params![package_id]) {
            Ok(rows) => println!("[SQL] increment_download_count affected rows: {}", rows),
            Err(e) => {
                log::error!("❌ increment_download_count failed: {}", e);
                return Err(e.into());
            }
        }
        Ok(())
    }

    pub async fn get_packages_advanced(
        &self,
        page: u32,
        page_size: u32,
        category: Option<i32>,
        search: Option<String>,
        status: Option<String>,
    ) -> anyhow::Result<(Vec<Package>, i64)> {
        let conn = self.conn.lock().await;
        log::debug!("🔍 get_packages_advanced called");
        log::debug!("🔍 page: {}, page_size: {}, category: {:?}, search: {:?}, status: {:?}", page, page_size, category, search, status);
        let mut sql = String::from("SELECT id, name, author, version, description, file_url, file_size, download_count, like_count, favorite_count, category_id, status, created_at, updated_at, reviewer_id, reviewed_at, review_comment, is_pinned, is_featured FROM packages WHERE 1=1");
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        if let Some(category_id) = category {
            sql.push_str(" AND category_id = ?");
            params.push(Box::new(category_id));
        }
        if let Some(ref search) = search {
            sql.push_str(" AND (name LIKE ? OR description LIKE ? OR author LIKE ?)");
            let pattern = format!("%{}%", search);
            params.push(Box::new(pattern.clone()));
            params.push(Box::new(pattern.clone()));
            params.push(Box::new(pattern));
        }
        if let Some(ref status) = status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status.clone()));
        }
        log::debug!("🔍 SQL before count: {}", sql);
        println!("[DEBUG] Params before count: {:?}", params.len());
        // 统计总数（修正：直接统计，不用子查询）
        let count_sql = sql.clone();
        let count_sql = count_sql.replacen(
            "SELECT id, name, author, version, description, file_url, file_size, download_count, like_count, favorite_count, category_id, status, created_at, updated_at, reviewer_id, reviewed_at, review_comment, is_pinned, is_featured",
            "SELECT COUNT(*)",
            1
        );
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|v| &**v as &dyn rusqlite::ToSql).collect();
        let total: i64 = match conn.query_row(&count_sql, &*params_refs, |row| row.get(0)) {
            Ok(val) => val,
            Err(e) => {
                log::error!("❌ count_sql failed: {}", e);
                return Err(e.into());
            }
        };
        // 分页
        sql.push_str(" ORDER BY is_pinned DESC, is_featured DESC, created_at DESC LIMIT ? OFFSET ?");
        params.push(Box::new(page_size as i64));
        params.push(Box::new(((page - 1) * page_size) as i64));
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|v| &**v as &dyn rusqlite::ToSql).collect();
        log::debug!("🔍 SQL final: {}", sql);
        println!("[DEBUG] Params final: {:?}", params.len());
        let mut stmt = match conn.prepare(&sql) {
            Ok(s) => s,
            Err(e) => {
                log::error!("❌ prepare SQL failed: {}", e);
                return Err(e.into());
            }
        };
        let packages = match stmt.query_map(&*params_refs, |row| {
            Ok(Package {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                version: row.get(3)?,
                description: row.get(4)?,
                file_url: row.get(5)?,
                file_size: row.get(6)?,
                download_count: row.get(7)?,
                like_count: row.get(8)?,
                favorite_count: row.get(9)?,
                category_id: row.get(10)?,
                status: match row.get::<_, String>(11)?.as_str() {
                    "pending" => crate::models::PackageStatus::Pending,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: row.get::<_, String>(12)?.parse().unwrap(),
                updated_at: row.get::<_, String>(13)?.parse().unwrap(),
                reviewer_id: row.get(14)?,
                reviewed_at: row.get(15)?,
                review_comment: row.get(16)?,
                is_pinned: row.get(17).unwrap_or(false),
                is_featured: row.get(18).unwrap_or(false),
                tags: None, // 这里会在后面填充
            })
        }) {
            Ok(res) => match res.collect::<Result<Vec<_>, _>>() {
                Ok(list) => list,
                Err(e) => {
                    log::error!("❌ collect packages failed: {}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                log::error!("❌ query_map failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[DEBUG] packages count: {}", packages.len());
        Ok((packages, total))
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let conn = self.conn.lock().await;
        
        let mut stmt = conn.prepare("SELECT id, name, description, enabled, subscription_locked, created_at, updated_at FROM categories")?;
        
        let categories = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3)?,
                subscription_locked: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(categories)
    }

    pub async fn check_package_exists(&self, package_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        let sql = "SELECT 1 FROM packages WHERE id = ?";
        log::debug!("🗄️ SQL: check_package_exists: {} | id={}", sql, package_id);
        
        let exists = conn.query_row(
            sql,
            params![package_id],
            |_| Ok(true)
        ).optional()?;
        
        Ok(exists.is_some())
    }
    
    pub async fn get_package_file_url(&self, package_id: i32) -> Result<String> {
        let conn = self.conn.lock().await;
        let sql = "SELECT file_url FROM packages WHERE id = ?";
        log::debug!("🗄️ SQL: get_package_file_url: {} | id={}", sql, package_id);
        
        let file_url: String = conn.query_row(
            sql,
            params![package_id],
            |row| row.get(0)
        )?;
        
        Ok(file_url)
    }

    // 新增：按下载量TOP榜
    pub async fn top_by_downloads(&self, limit: i32) -> Result<Vec<Package>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, author, version, description, file_url, file_size, \
                    download_count, like_count, favorite_count, category_id, status, \
                    created_at, updated_at, reviewer_id, reviewed_at, review_comment, \
                    is_pinned, is_featured \
             FROM packages WHERE status = 'active' ORDER BY download_count DESC LIMIT ?";
        let mut stmt = conn.prepare(sql)?;
        let list = stmt.query_map(params![limit], |row| {
            Ok(Package {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                version: row.get(3)?,
                description: row.get(4)?,
                file_url: row.get(5)?,
                file_size: row.get(6)?,
                download_count: row.get(7)?,
                like_count: row.get(8)?,
                favorite_count: row.get(9)?,
                category_id: row.get(10)?,
                status: match row.get::<_, String>(11)?.as_str() {
                    "pending" => crate::models::PackageStatus::Pending,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
                reviewer_id: row.get(14)?,
                reviewed_at: row.get(15)?,
                review_comment: row.get(16)?,
                is_pinned: row.get(17).unwrap_or(false),
                is_featured: row.get(18).unwrap_or(false),
                tags: None,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(list)
    }

    // 新增：按点赞数TOP热门榜
    pub async fn top_by_likes(&self, limit: i32) -> Result<Vec<Package>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, author, version, description, file_url, file_size, \
                    download_count, like_count, favorite_count, category_id, status, \
                    created_at, updated_at, reviewer_id, reviewed_at, review_comment, \
                    is_pinned, is_featured \
             FROM packages WHERE status = 'active' ORDER BY like_count DESC, download_count DESC LIMIT ?";
        let mut stmt = conn.prepare(sql)?;
        let list = stmt.query_map(params![limit], |row| {
            Ok(Package {
                id: row.get(0)?,
                name: row.get(1)?,
                author: row.get(2)?,
                version: row.get(3)?,
                description: row.get(4)?,
                file_url: row.get(5)?,
                file_size: row.get(6)?,
                download_count: row.get(7)?,
                like_count: row.get(8)?,
                favorite_count: row.get(9)?,
                category_id: row.get(10)?,
                status: match row.get::<_, String>(11)?.as_str() {
                    "pending" => crate::models::PackageStatus::Pending,
                    "active" => crate::models::PackageStatus::Active,
                    "rejected" => crate::models::PackageStatus::Rejected,
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Pending,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
                reviewer_id: row.get(14)?,
                reviewed_at: row.get(15)?,
                review_comment: row.get(16)?,
                is_pinned: row.get(17).unwrap_or(false),
                is_featured: row.get(18).unwrap_or(false),
                tags: None,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(list)
    }

    // 统计指定分类的资源数量（只统计active状态的资源）
    pub async fn count_packages_by_category(&self, category_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        let sql = "SELECT COUNT(*) FROM packages WHERE category_id = ? AND status = 'active'";
        log::debug!("🗄️ SQL: count_packages_by_category: {} | category_id={}", sql, category_id);
        
        let count: i32 = conn.query_row(
            sql,
            params![category_id],
            |row| row.get(0)
        )?;
        
        Ok(count)
    }

    pub async fn like_package(&self, user_id: i32, package_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        
        // 检查是否已经点赞
        let already_liked: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM package_likes WHERE user_id = ? AND package_id = ?)",
            params![user_id, package_id],
            |row| row.get(0),
        )?;
        
        if already_liked {
            return Err(anyhow::anyhow!("Already liked this package"));
        }
        
        // 插入点赞记录
        conn.execute(
            "INSERT INTO package_likes (user_id, package_id) VALUES (?, ?)",
            params![user_id, package_id],
        )?;
        
        // 记录用户行为（已由触发器自动更新like_count）
        conn.execute(
            "INSERT INTO user_actions (user_id, action_type, target_type, target_id) VALUES (?, 'Like', 'Package', ?)",
            params![user_id, package_id],
        )?;
        
        // 返回当前点赞总数
        let cnt: i32 = conn.query_row(
            "SELECT like_count FROM packages WHERE id = ?", 
            params![package_id], 
            |r| r.get(0)
        ).unwrap_or(0);
        
        Ok(cnt)
    }

    pub async fn unlike_package(&self, user_id: i32, package_id: i32) -> Result<i32> {
        let conn = self.conn.lock().await;
        
        // 检查是否已点赞
        let is_liked: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM package_likes WHERE user_id = ? AND package_id = ?)",
            params![user_id, package_id],
            |row| row.get(0),
        )?;
        
        if !is_liked {
            return Err(anyhow::anyhow!("Package not liked yet"));
        }
        
        // 删除点赞记录（触发器会自动更新like_count）
        conn.execute(
            "DELETE FROM package_likes WHERE user_id = ? AND package_id = ?",
            params![user_id, package_id],
        )?;
        
        // 记录用户行为
        conn.execute(
            "INSERT INTO user_actions (user_id, action_type, target_type, target_id) VALUES (?, 'Unlike', 'Package', ?)",
            params![user_id, package_id],
        )?;
        
        // 返回当前点赞总数
        let cnt: i32 = conn.query_row(
            "SELECT like_count FROM packages WHERE id = ?", 
            params![package_id], 
            |r| r.get(0)
        ).unwrap_or(0);
        
        Ok(cnt)
    }

    // 检查用户是否已点赞某个资源
    pub async fn check_like_status(&self, user_id: i32, package_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        let is_liked: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM package_likes WHERE user_id = ? AND package_id = ?)",
            params![user_id, package_id],
            |row| row.get(0),
        )?;
        Ok(is_liked)
    }

    // 记录资源访问
    pub async fn record_view(&self, package_id: i32, user_id: Option<i32>, ip_address: Option<String>, user_agent: Option<String>) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // 防重复访问检查（同一用户/IP在24小时内只记录一次访问）
        let mut should_record = true;
        
        if let Some(uid) = user_id {
            // 已登录用户：检查24小时内是否已访问
            let recent_view: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM package_views WHERE package_id = ? AND user_id = ? AND created_at > datetime('now', '-24 hours'))",
                params![package_id, uid],
                |row| row.get(0),
            ).unwrap_or(false);
            should_record = !recent_view;
        } else if let Some(ref ip) = ip_address {
            // 访客用户：检查IP在24小时内是否已访问
            let recent_view: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM package_views WHERE package_id = ? AND ip_address = ? AND created_at > datetime('now', '-24 hours'))",
                params![package_id, ip],
                |row| row.get(0),
            ).unwrap_or(false);
            should_record = !recent_view;
        }
        
        if should_record {
            // 插入访问记录（触发器会自动更新view_count）
            conn.execute(
                "INSERT INTO package_views (package_id, user_id, ip_address, user_agent) VALUES (?, ?, ?, ?)",
                params![package_id, user_id, ip_address, user_agent],
            )?;
            
            // 记录用户行为
            if let Some(uid) = user_id {
                conn.execute(
                    "INSERT INTO user_actions (user_id, action_type, target_type, target_id, ip_address, user_agent) VALUES (?, 'View', 'Package', ?, ?, ?)",
                    params![uid, package_id, ip_address, user_agent],
                )?;
            } else {
                // 访客用户的访问记录
                conn.execute(
                    "INSERT INTO user_actions (user_id, action_type, target_type, target_id, ip_address, user_agent) VALUES (NULL, 'View', 'Package', ?, ?, ?)",
                    params![package_id, ip_address, user_agent],
                )?;
            }
        }
        
        Ok(())
    }

    /* ---------------- 标签关联辅助 ---------------- */
    // 获取指定包的标签名称列表
    fn get_tags_for_package_internal(conn: &Connection, package_id: i32) -> anyhow::Result<Vec<String>> {
        let mut stmt = conn.prepare("SELECT t.name FROM tags t JOIN package_tags pt ON t.id = pt.tag_id WHERE pt.package_id = ?")?;
        let tags = stmt.query_map(params![package_id], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tags)
    }

    // 替换一个包的标签（先删后插）
    fn replace_tags_for_package_internal(conn: &Connection, package_id: i32, tags: &[String]) -> anyhow::Result<()> {
        // 删除旧关联
        conn.execute("DELETE FROM package_tags WHERE package_id = ?", params![package_id])?;

        if tags.is_empty() {
            return Ok(());
        }

        // 为每个标签名称找到ID，如果不存在则跳过
        for tag_name in tags {
            let tag_id: Option<i32> = conn.query_row(
                "SELECT id FROM tags WHERE name = ?",
                params![tag_name],
                |row| row.get(0),
            ).optional()?;

            if let Some(id) = tag_id {
                conn.execute(
                    "INSERT OR IGNORE INTO package_tags(package_id, tag_id) VALUES(?, ?)",
                    params![package_id, id],
                )?;
            }
        }
        Ok(())
    }
} 