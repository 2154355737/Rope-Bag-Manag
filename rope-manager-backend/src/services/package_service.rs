use anyhow::Result;
use crate::models::{Package, CreatePackageRequest, UpdatePackageRequest, Category, CreateResourceRecordRequest};
use crate::repositories::package_repo::PackageRepository;
use crate::repositories::system_repo::SystemRepository;
use crate::utils::file::FileUtils;
use chrono::Utc;

#[derive(Clone)]
pub struct PackageService {
    package_repo: PackageRepository,
    system_repo: Option<SystemRepository>,
    file_utils: FileUtils,
}

impl PackageService {
    pub fn new(package_repo: PackageRepository, upload_path: String) -> Self {
        Self {
            package_repo,
            system_repo: None,
            file_utils: FileUtils::new(upload_path),
        }
    }

    // 设置系统仓库，用于记录资源操作
    pub fn with_system_repo(mut self, system_repo: SystemRepository) -> Self {
        self.system_repo = Some(system_repo);
        self
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

        // 创建包
        let created_package = self.package_repo.create_package(&package).await?;
        
        // 记录资源操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: created_package.id,
                resource_type: "Package".to_string(),
                action: "Create".to_string(),
                old_data: None,
                new_data: Some(serde_json::to_string(&created_package).unwrap_or_default()),
            };
            
            // 使用系统仓库记录操作，默认用户ID为1（可以根据实际情况修改）
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                // 仅记录错误，不影响主要功能
                log::error!("记录资源创建操作失败: {}", e);
            } else {
                log::info!("成功记录资源创建操作: Package ID={}", created_package.id);
            }
        }
        
        Ok(created_package)
    }

    pub async fn update_package(&self, package_id: i32, req: &UpdatePackageRequest) -> Result<Package> {
        let package = self.package_repo.find_by_id(package_id).await?;
        let package = package.ok_or_else(|| anyhow::anyhow!("绳包不存在"))?;

        // 克隆package用于记录旧数据
        let old_package = package.clone();
        
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
        
        // 记录资源更新操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: package_id,
                resource_type: "Package".to_string(),
                action: "Update".to_string(),
                old_data: Some(serde_json::to_string(&old_package).unwrap_or_default()),
                new_data: Some(serde_json::to_string(&updated_package).unwrap_or_default()),
            };
            
            // 使用系统仓库记录操作
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                log::error!("记录资源更新操作失败: {}", e);
            } else {
                log::info!("成功记录资源更新操作: Package ID={}", package_id);
            }
        }
        
        Ok(updated_package)
    }

    pub async fn delete_package(&self, package_id: i32) -> Result<()> {
        // 先获取包信息，用于记录
        let package = self.package_repo.find_by_id(package_id).await?;
        
        // 删除包
        self.package_repo.delete_package(package_id).await?;
        
        // 记录资源删除操作
        if let Some(system_repo) = &self.system_repo {
            if let Some(package) = package {
                // 创建资源记录请求
                let record = CreateResourceRecordRequest {
                    resource_id: package_id,
                    resource_type: "Package".to_string(),
                    action: "Delete".to_string(),
                    old_data: Some(serde_json::to_string(&package).unwrap_or_default()),
                    new_data: None,
                };
                
                // 使用系统仓库记录操作
                if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                    log::error!("记录资源删除操作失败: {}", e);
                } else {
                    log::info!("成功记录资源删除操作: Package ID={}", package_id);
                }
            }
        }
        
        Ok(())
    }

    pub async fn download_package(&self, package_id: i32) -> Result<String> {
        // 首先检查包是否存在
        let exists = self.package_repo.check_package_exists(package_id).await?;
        if !exists {
            return Err(anyhow::anyhow!("绳包不存在"));
        }
        
        // 获取文件URL
        let file_url = self.package_repo.get_package_file_url(package_id).await?;
        
        // 增加下载次数
        self.package_repo.increment_download_count(package_id).await?;
        
        // 记录资源下载操作
        if let Some(system_repo) = &self.system_repo {
            // 创建资源记录请求
            let record = CreateResourceRecordRequest {
                resource_id: package_id,
                resource_type: "Package".to_string(),
                action: "Download".to_string(),
                old_data: None,
                new_data: None,
            };
            
            // 使用系统仓库记录操作
            if let Err(e) = system_repo.log_resource_action(&record, 1).await {
                log::error!("记录资源下载操作失败: {}", e);
            } else {
                log::info!("成功记录资源下载操作: Package ID={}", package_id);
            }
        }

        Ok(file_url)
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