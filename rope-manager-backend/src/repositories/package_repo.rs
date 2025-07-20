use anyhow::Result;
use rusqlite::{Connection, params, OptionalExtension};
use crate::models::{Package, Category};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
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
                    created_at, updated_at \
             FROM packages ORDER BY created_at DESC";
        println!("[SQL] get_all_packages: {}", sql);
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR] prepare failed: {}", e);
                return Err(e.into());
            }
        };
        let packages = match stmt.query_map([], |row| {
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
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Active,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        }) {
            Ok(res) => match res.collect::<Result<Vec<_>, _>>() {
                Ok(list) => list,
                Err(e) => {
                    println!("[ERROR] collect failed: {}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                println!("[ERROR] query_map failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[SQL] get_all_packages result count: {}", packages.len());
        Ok(packages)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Package>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, author, version, description, file_url, file_size, \
                    download_count, like_count, favorite_count, category_id, status, \
                    created_at, updated_at \
             FROM packages WHERE id = ?";
        println!("[SQL] find_by_id: {} | id={}", sql, id);
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR] prepare failed: {}", e);
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
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Active,
                },
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        }) {
            Ok(val) => Some(val),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => {
                println!("[ERROR] query_row failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[SQL] find_by_id result: {:?}", package);
        Ok(package)
    }

    pub async fn create_package(&self, package: &Package) -> Result<Package> {
        let conn = self.conn.lock().await;
        let sql = "INSERT INTO packages (name, author, version, description, file_url, file_size, \
                                  download_count, like_count, favorite_count, category_id, status, \
                                  created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        println!("[SQL] create_package: {}", sql);
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
                crate::models::PackageStatus::Active => "active",
                crate::models::PackageStatus::Inactive => "inactive",
                crate::models::PackageStatus::Deleted => "deleted",
            },
            package.created_at.to_rfc3339(),
            package.updated_at.to_rfc3339(),
        ];
        match conn.execute(sql, params) {
            Ok(rows) => println!("[SQL] create_package affected rows: {}", rows),
            Err(e) => {
                println!("[ERROR] create_package failed: {}", e);
                return Err(e.into());
            }
        }
        
        // 获取最后插入的ID
        let last_id = conn.last_insert_rowid() as i32;
        
        // 创建包含ID的新包对象
        let mut created_package = package.clone();
        created_package.id = last_id;
        
        Ok(created_package)
    }

    pub async fn update_package(&self, package: &Package) -> Result<()> {
        let conn = self.conn.lock().await;
        let sql = "UPDATE packages SET name = ?, author = ?, version = ?, description = ?, \
                    file_url = ?, file_size = ?, download_count = ?, like_count = ?, \
                    favorite_count = ?, category_id = ?, status = ?, created_at = ?, \
                    updated_at = ? WHERE id = ?";
        println!("[SQL] update_package: {} | id={}", sql, package.id);
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
                crate::models::PackageStatus::Active => "active",
                crate::models::PackageStatus::Inactive => "inactive",
                crate::models::PackageStatus::Deleted => "deleted",
            },
            package.created_at.to_rfc3339(),
            package.updated_at.to_rfc3339(),
            package.id,
        ];
        match conn.execute(sql, params) {
            Ok(rows) => println!("[SQL] update_package affected rows: {}", rows),
            Err(e) => {
                println!("[ERROR] update_package failed: {}", e);
                return Err(e.into());
            }
        }
        Ok(())
    }

    pub async fn delete_package(&self, package_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        let sql = "DELETE FROM packages WHERE id = ?";
        println!("[SQL] delete_package: {} | id={}", sql, package_id);
        match conn.execute(sql, params![package_id]) {
            Ok(rows) => println!("[SQL] delete_package affected rows: {}", rows),
            Err(e) => {
                println!("[ERROR] delete_package failed: {}", e);
                return Err(e.into());
            }
        }
        Ok(())
    }

    pub async fn increment_download_count(&self, package_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        let sql = "UPDATE packages SET download_count = download_count + 1 WHERE id = ?";
        println!("[SQL] increment_download_count: {} | id={}", sql, package_id);
        match conn.execute(sql, params![package_id]) {
            Ok(rows) => println!("[SQL] increment_download_count affected rows: {}", rows),
            Err(e) => {
                println!("[ERROR] increment_download_count failed: {}", e);
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
        println!("[DEBUG] get_packages_advanced called");
        println!("[DEBUG] page: {}, page_size: {}, category: {:?}, search: {:?}, status: {:?}", page, page_size, category, search, status);
        let mut sql = String::from("SELECT id, name, author, version, description, file_url, file_size, download_count, like_count, favorite_count, category_id, status, created_at, updated_at FROM packages WHERE 1=1");
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
        println!("[DEBUG] SQL before count: {}", sql);
        println!("[DEBUG] Params before count: {:?}", params.len());
        // 统计总数（修正：直接统计，不用子查询）
        let count_sql = sql.clone();
        let count_sql = count_sql.replacen(
            "SELECT id, name, author, version, description, file_url, file_size, download_count, like_count, favorite_count, category_id, status, created_at, updated_at",
            "SELECT COUNT(*)",
            1
        );
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|v| &**v as &dyn rusqlite::ToSql).collect();
        let total: i64 = match conn.query_row(&count_sql, &*params_refs, |row| row.get(0)) {
            Ok(val) => val,
            Err(e) => {
                println!("[ERROR] count_sql failed: {}", e);
                return Err(e.into());
            }
        };
        // 分页
        sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
        params.push(Box::new(page_size as i64));
        params.push(Box::new(((page - 1) * page_size) as i64));
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|v| &**v as &dyn rusqlite::ToSql).collect();
        println!("[DEBUG] SQL final: {}", sql);
        println!("[DEBUG] Params final: {:?}", params.len());
        let mut stmt = match conn.prepare(&sql) {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR] prepare SQL failed: {}", e);
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
                    "inactive" => crate::models::PackageStatus::Inactive,
                    "deleted" => crate::models::PackageStatus::Deleted,
                    _ => crate::models::PackageStatus::Active,
                },
                created_at: row.get::<_, String>(12)?.parse().unwrap(),
                updated_at: row.get::<_, String>(13)?.parse().unwrap(),
            })
        }) {
            Ok(res) => match res.collect::<Result<Vec<_>, _>>() {
                Ok(list) => list,
                Err(e) => {
                    println!("[ERROR] collect packages failed: {}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                println!("[ERROR] query_map failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[DEBUG] packages count: {}", packages.len());
        Ok((packages, total))
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let conn = self.conn.lock().await;
        let sql = "SELECT id, name, description, enabled, created_at 
                   FROM categories ORDER BY created_at ASC";
        println!("[SQL] get_categories: {}", sql);
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR] prepare failed: {}", e);
                return Err(e.into());
            }
        };
        let categories = match stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get(3)?,
                created_at: row.get(4)?,
            })
        }) {
            Ok(res) => match res.collect::<Result<Vec<_>, _>>() {
                Ok(list) => list,
                Err(e) => {
                    println!("[ERROR] collect failed: {}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                println!("[ERROR] query_map failed: {}", e);
                return Err(e.into());
            }
        };
        println!("[SQL] get_categories result count: {}", categories.len());
        Ok(categories)
    }

    pub async fn check_package_exists(&self, package_id: i32) -> Result<bool> {
        let conn = self.conn.lock().await;
        let sql = "SELECT 1 FROM packages WHERE id = ?";
        println!("[SQL] check_package_exists: {} | id={}", sql, package_id);
        
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
        println!("[SQL] get_package_file_url: {} | id={}", sql, package_id);
        
        let file_url: String = conn.query_row(
            sql,
            params![package_id],
            |row| row.get(0)
        )?;
        
        Ok(file_url)
    }
} 