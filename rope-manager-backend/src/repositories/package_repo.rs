use anyhow::Result;
use rusqlite::{Connection, params};
use crate::models::Package;
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
        let mut stmt = conn.prepare(
            "SELECT id, name, author, version, description, file_url, file_size, 
                    download_count, like_count, favorite_count, category_id, status, 
                    created_at, updated_at 
             FROM packages ORDER BY created_at DESC"
        )?;

        let packages = stmt.query_map([], |row| {
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
                status: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(packages)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Package>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, name, author, version, description, file_url, file_size, 
                    download_count, like_count, favorite_count, category_id, status, 
                    created_at, updated_at 
             FROM packages WHERE id = ?"
        )?;

        let package = stmt.query_row(params![id], |row| {
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
                status: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        }).optional()?;

        Ok(package)
    }

    pub async fn create_package(&self, package: &Package) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO packages (name, author, version, description, file_url, file_size, 
                                  download_count, like_count, favorite_count, category_id, status, 
                                  created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                package.name,
                package.author,
                package.version,
                package.description,
                package.file_url,
                package.file_size,
                package.download_count,
                package.like_count,
                package.favorite_count,
                package.category_id,
                package.status,
                package.created_at,
                package.updated_at,
            ]
        )?;
        Ok(())
    }

    pub async fn update_package(&self, package: &Package) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE packages SET name = ?, author = ?, version = ?, description = ?, 
                    file_url = ?, file_size = ?, download_count = ?, like_count = ?, 
                    favorite_count = ?, category_id = ?, status = ?, created_at = ?, 
                    updated_at = ? WHERE id = ?",
            params![
                package.name,
                package.author,
                package.version,
                package.description,
                package.file_url,
                package.file_size,
                package.download_count,
                package.like_count,
                package.favorite_count,
                package.category_id,
                package.status,
                package.created_at,
                package.updated_at,
                package.id,
            ]
        )?;
        Ok(())
    }

    pub async fn delete_package(&self, package_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM packages WHERE id = ?", params![package_id])?;
        Ok(())
    }

    pub async fn increment_download_count(&self, package_id: i32) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE packages SET download_count = download_count + 1 WHERE id = ?",
            params![package_id]
        )?;
        Ok(())
    }
} 