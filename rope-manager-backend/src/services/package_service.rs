use anyhow::Result;
use crate::models::{Package, CreatePackageRequest, UpdatePackageRequest, Category};
use crate::repositories::package_repo::PackageRepository;
use crate::utils::file::FileUtils;
use chrono::Utc;

#[derive(Clone)]
pub struct PackageService {
    package_repo: PackageRepository,
    file_utils: FileUtils,
}

impl PackageService {
    pub fn new(package_repo: PackageRepository, upload_path: String) -> Self {
        Self {
            package_repo,
            file_utils: FileUtils::new(upload_path),
        }
    }

    pub async fn get_packages(&self) -> Result<Vec<Package>> {
        self.package_repo.get_all_packages().await
    }

    pub async fn get_package(&self, package_id: i32) -> Result<Option<Package>> {
        self.package_repo.find_by_id(package_id).await
    }

    // 新增方法：根据ID获取包（别名方法）
    pub async fn get_package_by_id(&self, package_id: i32) -> Result<Option<Package>> {
        self.package_repo.find_by_id(package_id).await
    }

    pub async fn create_package(&self, req: &CreatePackageRequest) -> Result<Package> {
        // 创建绳包记录
        let package = Package {
            id: 0, // 数据库会自动生成
            name: req.name.clone(),
            author: req.author.clone(),
            version: req.version.clone(),
            description: req.description.clone(),
            file_url: req.file_url.clone().unwrap_or_else(String::new), // 使用请求中的file_url
            file_size: None,
            download_count: 0,
            like_count: 0,
            favorite_count: 0,
            category_id: req.category_id,
            status: crate::models::PackageStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.package_repo.create_package(&package).await?;
        Ok(package)
    }

    pub async fn update_package(&self, package_id: i32, req: &UpdatePackageRequest) -> Result<Package> {
        let package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        let updated_package = Package {
            id: package_id,
            name: req.name.clone().unwrap_or(package.name),
            author: package.author,
            version: req.version.clone().or(package.version),
            description: req.description.clone().or(package.description),
            category_id: req.category_id.or(package.category_id),
            status: req.status.clone().unwrap_or(package.status),
            file_url: req.file_url.clone().unwrap_or(package.file_url), // 使用请求中的file_url，如果没有则保持原值
            file_size: package.file_size,
            download_count: package.download_count,
            like_count: package.like_count,
            favorite_count: package.favorite_count,
            created_at: package.created_at,
            updated_at: chrono::Utc::now(),
        };

        self.package_repo.update_package(&updated_package).await?;
        Ok(updated_package)
    }

    pub async fn delete_package(&self, package_id: i32) -> Result<()> {
        self.package_repo.delete_package(package_id).await
    }

    pub async fn download_package(&self, package_id: i32) -> Result<String> {
        let package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        // 增加下载次数
        self.package_repo.increment_download_count(package_id).await?;

        Ok(package.file_url)
    }

    // 新增方法：更新包文件
    pub async fn update_package_file(&self, package_id: i32) -> Result<Package> {
        let package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        // TODO: 实现文件上传逻辑
        // 这里应该处理文件上传并更新包信息

        Ok(package)
    }

    // 新增方法：获取分类
    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        self.package_repo.get_categories().await
    }

    pub async fn get_packages_advanced(
        &self,
        page: u32,
        page_size: u32,
        category: Option<i32>,
        search: Option<String>,
        status: Option<String>,
    ) -> anyhow::Result<(Vec<Package>, i64)> {
        self.package_repo.get_packages_advanced(page, page_size, category, search, status).await
    }
} 