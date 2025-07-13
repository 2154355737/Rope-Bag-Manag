use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::{RawDataJson, RawRopePackage, RopePackage, User};
use crate::utils::{save_json, load_json};

/// 统一的数据管理服务
/// 确保内存数据和文件数据的一致性
pub struct DataManager {
    raw_data: Arc<Mutex<RawDataJson>>,
    ropes: Arc<Mutex<HashMap<u32, RopePackage>>>,
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl DataManager {
    /// 创建新的数据管理器
    pub fn new() -> Self {
        let raw_data = Arc::new(Mutex::new(load_json::<RawDataJson>("data/data.json")));
        let users = Arc::new(Mutex::new(load_json::<HashMap<String, User>>("data/users.json")));
        
        // 从原始数据构建ropes映射
        let raw_guard = raw_data.lock().unwrap();
        let mut ropes_map = HashMap::new();
        for raw_rope in &raw_guard.绳包列表 {
            ropes_map.insert(raw_rope.id, RopePackage {
                id: raw_rope.id,
                name: raw_rope.绳包名称.clone(),
                author: raw_rope.作者.clone(),
                version: raw_rope.版本.clone(),
                desc: raw_rope.简介.clone(),
                url: raw_rope.项目直链.clone(),
                downloads: raw_rope.下载次数,
                upload_time: raw_rope.上架时间.clone(),
                category: raw_rope.分类.clone(),
                status: raw_rope.状态.clone(),
                is_starred: raw_rope.是否标星,
                star_time: raw_rope.标星时间.clone(),
                star_by: raw_rope.标星人.clone(),
            });
        }
        drop(raw_guard);
        
        Self {
            raw_data,
            ropes: Arc::new(Mutex::new(ropes_map)),
            users,
        }
    }

    /// 添加绳包资源（原子性操作）
    #[allow(dead_code)]
    pub fn add_package(&self, package: RawRopePackage) -> Result<(), Box<dyn std::error::Error>> {
        // 同时更新内存和文件数据
        let mut raw_guard = self.raw_data.lock().unwrap();
        let mut ropes_guard = self.ropes.lock().unwrap();
        
        // 添加到原始数据
        raw_guard.绳包列表.push(package.clone());
        
        // 添加到内存映射
        ropes_guard.insert(package.id, RopePackage {
            id: package.id,
            name: package.绳包名称.clone(),
            author: package.作者.clone(),
            version: package.版本.clone(),
            desc: package.简介.clone(),
            url: package.项目直链.clone(),
            downloads: package.下载次数,
            upload_time: package.上架时间.clone(),
            category: package.分类.clone(),
            status: package.状态.clone(),
            is_starred: package.是否标星,
            star_time: package.标星时间.clone(),
            star_by: package.标星人.clone(),
        });
        
        // 更新数据库配置
        raw_guard.数据库配置.数据库名称 = "结绳绳包数据库".to_string();
        raw_guard.数据库配置.数据库项目 = raw_guard.绳包列表.len() as u32;
        raw_guard.数据库配置.数据库版本 = self::bump_version(&raw_guard.数据库配置.数据库版本);
        raw_guard.数据库配置.数据库更新时间 = chrono::Local::now().format("%Y%m%d").to_string();
        
        // 保存到文件
        if let Err(e) = save_json("data/data.json", &*raw_guard) {
            eprintln!("保存data.json失败: {}", e);
        }
        
        Ok(())
    }

    /// 更新绳包资源（原子性操作）
    #[allow(dead_code)]
    pub fn update_package(&self, id: u32, updates: RawRopePackage) -> Result<(), Box<dyn std::error::Error>> {
        let mut raw_guard = self.raw_data.lock().unwrap();
        let mut ropes_guard = self.ropes.lock().unwrap();
        
        // 更新原始数据
        let mut found = false;
        for raw_rope in &mut raw_guard.绳包列表 {
            if raw_rope.id == id {
                raw_rope.绳包名称 = updates.绳包名称.clone();
                raw_rope.作者 = updates.作者.clone();
                raw_rope.版本 = updates.版本.clone();
                raw_rope.简介 = updates.简介.clone();
                raw_rope.项目直链 = updates.项目直链.clone();
                found = true;
                break;
            }
        }
        
        if !found {
            return Err("绳包不存在".into());
        }
        
        // 更新内存映射
        if let Some(rope) = ropes_guard.get_mut(&id) {
            rope.name = updates.绳包名称.clone();
            rope.author = updates.作者.clone();
            rope.version = updates.版本.clone();
            rope.desc = updates.简介.clone();
            rope.url = updates.项目直链.clone();
        }
        
        // 更新数据库配置
        raw_guard.数据库配置.数据库版本 = self::bump_version(&raw_guard.数据库配置.数据库版本);
        raw_guard.数据库配置.数据库更新时间 = chrono::Local::now().format("%Y%m%d").to_string();
        
        // 保存到文件
        if let Err(e) = save_json("data/data.json", &*raw_guard) {
            eprintln!("保存data.json失败: {}", e);
        }
        
        Ok(())
    }

    /// 删除绳包资源（原子性操作）
    #[allow(dead_code)]
    pub fn delete_package(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut raw_guard = self.raw_data.lock().unwrap();
        let mut ropes_guard = self.ropes.lock().unwrap();
        
        // 从原始数据中删除
        let before = raw_guard.绳包列表.len();
        raw_guard.绳包列表.retain(|p| p.id != id);
        let after = raw_guard.绳包列表.len();
        
        if before == after {
            return Err("绳包不存在".into());
        }
        
        // 从内存映射中删除
        ropes_guard.remove(&id);
        
        // 更新数据库配置
        raw_guard.数据库配置.数据库项目 = raw_guard.绳包列表.len() as u32;
        raw_guard.数据库配置.数据库版本 = self::bump_version(&raw_guard.数据库配置.数据库版本);
        raw_guard.数据库配置.数据库更新时间 = chrono::Local::now().format("%Y%m%d").to_string();
        
        // 保存到文件
        if let Err(e) = save_json("data/data.json", &*raw_guard) {
            eprintln!("保存data.json失败: {}", e);
        }
        
        Ok(())
    }

    /// 更新下载次数（原子性操作）
    #[allow(dead_code)]
    pub fn update_download_count(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut raw_guard = self.raw_data.lock().unwrap();
        let mut ropes_guard = self.ropes.lock().unwrap();
        
        // 更新原始数据
        for raw_rope in &mut raw_guard.绳包列表 {
            if raw_rope.id == id {
                raw_rope.下载次数 += 1;
                break;
            }
        }
        
        // 更新内存映射
        if let Some(rope) = ropes_guard.get_mut(&id) {
            rope.downloads += 1;
        }
        
        // 保存到文件
        if let Err(e) = save_json("data/data.json", &*raw_guard) {
            eprintln!("保存data.json失败: {}", e);
        }
        
        Ok(())
    }

    /// 获取所有绳包数据
    #[allow(dead_code)]
    pub fn get_all_packages(&self) -> Vec<RopePackage> {
        let ropes_guard = self.ropes.lock().unwrap();
        ropes_guard.values().cloned().collect()
    }

    /// 根据ID获取绳包
    #[allow(dead_code)]
    pub fn get_package(&self, id: u32) -> Option<RopePackage> {
        let ropes_guard = self.ropes.lock().unwrap();
        ropes_guard.get(&id).cloned()
    }

    /// 获取原始数据
    #[allow(dead_code)]
    pub fn get_raw_data(&self) -> RawDataJson {
        let raw_guard = self.raw_data.lock().unwrap();
        raw_guard.clone()
    }

    /// 获取用户数据
    #[allow(dead_code)]
    pub fn get_users(&self) -> HashMap<String, User> {
        let users_guard = self.users.lock().unwrap();
        users_guard.clone()
    }

    /// 更新用户数据
    #[allow(dead_code)]
    pub fn update_user(&self, username: String, user: User) -> Result<(), Box<dyn std::error::Error>> {
        let mut users_guard = self.users.lock().unwrap();
        users_guard.insert(username, user);
        if let Err(e) = save_json("data/users.json", &*users_guard) {
            eprintln!("保存users.json失败: {}", e);
        }
        Ok(())
    }

    /// 获取用户
    #[allow(dead_code)]
    pub fn get_user(&self, username: &str) -> Option<User> {
        let users_guard = self.users.lock().unwrap();
        users_guard.get(username).cloned()
    }

    /// 加载用户数据
    pub fn load_users(&self) -> Result<HashMap<String, User>, Box<dyn std::error::Error>> {
        let users_guard = self.users.lock().unwrap();
        Ok(users_guard.clone())
    }

    /// 保存用户数据
    pub fn save_users(&self, users: &HashMap<String, User>) -> Result<(), Box<dyn std::error::Error>> {
        let mut users_guard = self.users.lock().unwrap();
        *users_guard = users.clone();
        if let Err(e) = save_json("data/users.json", users) {
            eprintln!("保存users.json失败: {}", e);
            return Err(e.into());
        }
        Ok(())
    }

    // ====== 系统设置管理 ======
    pub fn load_settings(&self) -> Result<crate::models::SystemSettings, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/settings.json")
    }

    pub fn save_settings(&self, settings: &crate::models::SystemSettings) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/settings.json", settings)
    }

    // ====== 评论管理 ======
    pub fn load_comments(&self) -> Result<Vec<crate::models::Comment>, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/comments.json")
    }

    pub fn save_comments(&self, comments: &Vec<crate::models::Comment>) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/comments.json", comments)
    }

    // ====== 用户行为记录管理 ======
    pub fn load_user_actions(&self) -> Result<Vec<crate::models::UserAction>, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/user_actions.json")
    }

    pub fn save_user_actions(&self, actions: &Vec<crate::models::UserAction>) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/user_actions.json", actions)
    }

    // ====== 资源记录管理 ======
    pub fn load_resource_records(&self) -> Result<Vec<crate::models::ResourceRecord>, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/resource_records.json")
    }

    pub fn save_resource_records(&self, records: &Vec<crate::models::ResourceRecord>) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/resource_records.json", records)
    }

    // ====== 备份记录管理 ======
    pub fn load_backup_records(&self) -> Result<Vec<crate::models::BackupRecord>, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/backup_records.json")
    }

    pub fn save_backup_records(&self, records: &Vec<crate::models::BackupRecord>) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/backup_records.json", records)
    }

    // ====== 原始数据管理 ======
    pub fn load_raw_data(&self) -> Result<RawDataJson, Box<dyn std::error::Error>> {
        let raw_guard = self.raw_data.lock().unwrap();
        Ok(raw_guard.clone())
    }

    pub fn save_raw_data(&self, data: &RawDataJson) -> Result<(), Box<dyn std::error::Error>> {
        let mut raw_guard = self.raw_data.lock().unwrap();
        *raw_guard = data.clone();
        if let Err(e) = save_json("data/data.json", data) {
            eprintln!("保存data.json失败: {}", e);
            return Err(e.into());
        }
        Ok(())
    }

    // ====== 分类管理 ======
    pub fn load_categories(&self) -> Result<Vec<crate::models::Category>, Box<dyn std::error::Error>> {
        crate::utils::load_json_result("data/categories.json")
    }

    pub fn save_categories(&self, categories: &Vec<crate::models::Category>) -> Result<(), Box<dyn std::error::Error>> {
        crate::utils::save_json("data/categories.json", categories)
    }
}

/// 版本号递增函数
#[allow(dead_code)]
fn bump_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 3 {
        let patch = parts[2].parse::<u32>().unwrap_or(0) + 1;
        format!("{}.{}.{}", parts[0], parts[1], patch)
    } else {
        format!("{}.0.1", version)
    }
} 