use std::collections::HashMap;
use crate::models::{User, RopePackage, StatsData, RawDataJson};
use crate::config::AppConfig;
use crate::utils::{load_json_result, save_json};
use chrono::Utc;

pub struct DataManager;

impl DataManager {
    pub fn new() -> Self {
        DataManager
    }

    // 生成用户ID
    pub fn generate_user_id(&self) -> u64 {
        let now = Utc::now();
        let timestamp = now.timestamp() as u64;
        let random_part = (timestamp % 10000) as u64;
        timestamp * 10000 + random_part
    }

    // 用户数据管理 - 使用ID作为键
    pub fn load_users(&self) -> Result<HashMap<u64, User>, Box<dyn std::error::Error>> {
        load_json_result("data/users.json")
    }

    pub fn save_users(&self, users: &HashMap<u64, User>) -> Result<(), Box<dyn std::error::Error>> {
        save_json("data/users.json", users)
    }

    // 根据用户名查找用户
    pub fn find_user_by_username(&self, username: &str) -> Result<Option<(u64, User)>, Box<dyn std::error::Error>> {
        let users = self.load_users()?;
        for (id, user) in users {
            if user.username == username {
                return Ok(Some((id, user)));
            }
        }
        Ok(None)
    }

    // 创建新用户
    pub fn create_user(&self, username: String, password: String) -> Result<u64, Box<dyn std::error::Error>> {
        let mut users = self.load_users().unwrap_or_default();
        
        // 检查用户名是否已存在
        for (_, user) in &users {
            if user.username == username {
                return Err("用户名已存在".into());
            }
        }

        let user_id = self.generate_user_id();
        let new_user = User {
            id: user_id,
            username,
            password,
            role: crate::models::UserRole::Normal,
            star: 1,
            online_status: crate::models::OnlineStatus::Offline,
            ban_status: crate::models::BanStatus::Normal,
            ban_reason: None,
            register_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            login_count: 0,
            qq_number: None,
            avatar_url: None,
            sign_records: Vec::new(),
            sign_days: 0,
            sign_total: 0,
            last_sign: String::new(),
            last_login: String::new(),
            upload_count: 0,
            download_count: 0,
            permissions: crate::models::UserPermissions::default(),
            is_admin: false,
        };

        users.insert(user_id, new_user);
        self.save_users(&users)?;
        
        Ok(user_id)
    }

    // 更新用户
    pub fn update_user(&self, user_id: u64, user: User) -> Result<(), Box<dyn std::error::Error>> {
        let mut users = self.load_users()?;
        users.insert(user_id, user);
        self.save_users(&users)
    }

    // 删除用户
    pub fn delete_user(&self, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut users = self.load_users()?;
        users.remove(&user_id);
        self.save_users(&users)
    }

    // 获取所有用户
    pub fn get_all_users(&self) -> Result<Vec<(u64, User)>, Box<dyn std::error::Error>> {
        let users = self.load_users()?;
        Ok(users.into_iter().collect())
    }

    // 绳包数据管理
    #[allow(dead_code)]
    pub fn load_packages(&self) -> Result<HashMap<u32, RopePackage>, Box<dyn std::error::Error>> {
        let raw_data: RawDataJson = load_json_result("data/data.json")?;
        let mut packages = HashMap::new();
        
        for raw_package in raw_data.绳包列表 {
            let package = RopePackage {
                id: raw_package.id,
                name: raw_package.绳包名称,
                author: raw_package.作者,
                version: raw_package.版本,
                desc: raw_package.简介,
                url: raw_package.项目直链,
                downloads: raw_package.下载次数,
                upload_time: raw_package.上架时间,
                category: raw_package.分类,
                status: raw_package.状态,
                is_starred: raw_package.是否标星,
                star_time: raw_package.标星时间,
                star_by: raw_package.标星人,
            };
            packages.insert(raw_package.id, package);
        }
        
        Ok(packages)
    }

    #[allow(dead_code)]
    pub fn save_packages(&self, packages: &HashMap<u32, RopePackage>) -> Result<(), Box<dyn std::error::Error>> {
        save_json("data/data.json", packages)
    }

    // 分类数据管理
    pub fn load_categories(&self) -> Result<Vec<crate::models::Category>, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/categories.json")
    }
    pub fn save_categories(&self, categories: &Vec<crate::models::Category>) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/categories.json", categories)
    }

    // 配置数据管理
    #[allow(dead_code)]
    pub fn load_config(&self) -> Result<AppConfig, Box<dyn std::error::Error>> {
        load_json_result("data/config.json")
    }

    #[allow(dead_code)]
    pub fn save_config(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        save_json("data/config.json", config)
    }

    // 统计数据管理
    #[allow(dead_code)]
    pub fn load_stats(&self) -> Result<StatsData, Box<dyn std::error::Error>> {
        load_json_result("data/stats.json")
    }

    #[allow(dead_code)]
    pub fn save_stats(&self, stats: &StatsData) -> Result<(), Box<dyn std::error::Error>> {
        save_json("data/stats.json", stats)
    }

    // 原始数据管理
    pub fn load_raw_data(&self) -> Result<RawDataJson, Box<dyn std::error::Error>> {
        load_json_result("data/data.json")
    }

    pub fn save_raw_data(&self, data: &RawDataJson) -> Result<(), Box<dyn std::error::Error>> {
        save_json("data/data.json", data)
    }

    // API性能数据管理
    pub fn load_api_performance(&self) -> Result<HashMap<String, crate::models::ApiPerformance>, Box<dyn std::error::Error>> {
        let stats = self.load_stats()?;
        Ok(stats.api_performance)
    }

    pub fn load_api_calls(&self) -> Result<Vec<crate::models::ApiCallRecord>, Box<dyn std::error::Error>> {
        let stats = self.load_stats()?;
        Ok(stats.api_calls)
    }

    pub fn load_api_last_used(&self) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
        let stats = self.load_stats()?;
        Ok(stats.api_last_used)
    }

    pub fn load_downloads(&self) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
        let stats = self.load_stats()?;
        Ok(stats.downloads)
    }

    pub fn load_api_counts(&self) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
        let stats = self.load_stats()?;
        Ok(stats.api_counts)
    }
}
