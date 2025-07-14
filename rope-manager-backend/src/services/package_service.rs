use anyhow::Result;
use crate::models::{Package, CreatePackageRequest, UpdatePackageRequest};
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

    pub async fn create_package(&self, req: &CreatePackageRequest) -> Result<Package> {
        // 创建绳包记录
        let package = Package {
            id: 0, // 数据库会自动生成
            name: req.name.clone(),
            author: req.author.clone(),
            version: req.version.clone(),
            description: req.description.clone(),
            file_url: String::new(), // 暂时为空，上传文件后会更新
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

    pub async fn update_package(&self, package_id: i32, req: &UpdatePackageRequest) -> Result<()> {
        let mut package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        // 更新绳包信息
        let mut updated_package = package.clone();
        if let Some(name) = &req.name {
            updated_package.name = name.clone();
        }
        if let Some(version) = &req.version {
            updated_package.version = version.clone();
        }
        if let Some(description) = &req.description {
            updated_package.description = description.clone();
        }
        if let Some(category_id) = req.category_id {
            updated_package.category_id = Some(category_id);
        }
        if let Some(status) = &req.status {
            updated_package.status = status.clone();
        }
        updated_package.updated_at = Utc::now();

        self.package_repo.update_package(&updated_package).await
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
} 