use actix_web::{web, HttpResponse, get, post, delete, Responder};
use serde::Deserialize;
use crate::models::{AppState, ApiResponse, BackupRecord, BackupType, BackupStatus};
use chrono::Utc;
use std::fs;
use std::path::Path;
use std::io::Write;
use serde::Serialize;

// 备份数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct BackupData {
    pub timestamp: String,
    pub version: String,
    pub users: Vec<crate::models::User>,
    pub packages: Vec<crate::models::RopePackage>,
    pub settings: crate::models::SystemSettings,
    pub stats: crate::models::StatsData,
}

// 执行实际备份操作
async fn perform_backup_operation(backup_path: &str, data_manager: &crate::data_manager::DataManager) -> Result<u64, Box<dyn std::error::Error>> {
    // 确保备份目录存在
    let backup_dir = Path::new(backup_path).parent().unwrap_or(Path::new("./backups"));
    fs::create_dir_all(backup_dir)?;
    
    // 收集所有数据
    let users = data_manager.load_users()?;
    let raw_data = data_manager.load_raw_data()?;
    let settings = data_manager.load_settings()?;
    
    // 创建默认的统计数据结构
    let stats = crate::models::StatsData::default();
    
    let backup_data = BackupData {
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        users: users.values().cloned().collect(),
        packages: raw_data.绳包列表.into_iter().map(|p| crate::models::RopePackage {
            id: p.id,
            name: p.绳包名称,
            author: p.作者,
            version: p.版本,
            desc: p.简介,
            url: p.项目直链,
            downloads: p.下载次数,
            upload_time: p.上架时间,
            category: p.分类,
            status: p.状态,
            is_starred: p.是否标星,
            star_time: p.标星时间,
            star_by: p.标星人,
        }).collect(),
        settings,
        stats,
    };
    
    // 序列化数据
    let json_data = serde_json::to_string_pretty(&backup_data)?;
    
    // 写入文件
    let mut file = fs::File::create(backup_path)?;
    file.write_all(json_data.as_bytes())?;
    file.sync_all()?;
    
    // 返回文件大小
    Ok(json_data.len() as u64)
}

#[derive(Debug, Deserialize)]
pub struct BackupRecordQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub backup_type: Option<String>,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBackupRecordRequest {
    pub backup_type: BackupType,
    pub description: String,
    pub filename: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AutoBackupConfig {
    pub enable_auto_backup: bool,
    pub backup_interval_hours: u32,
    pub backup_location: String,
    pub max_backup_files: u32,
}

#[derive(Debug, Deserialize)]
pub struct BatchDeleteRequest {
    pub ids: Vec<u32>,
}

// 获取备份记录列表
#[get("/api/backup-records")]
pub async fn get_backup_records(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let params = crate::utils::parse_query_params(req.query_string());
    
    let page = params.get("page")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);
    let size = params.get("size")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    let offset = (page - 1) * size;

    let backup_type = params.get("backup_type").cloned();
    let status = params.get("status").cloned();
    let start_time = params.get("start_time").cloned();
    let end_time = params.get("end_time").cloned();

    let records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_records = records;
    if let Some(backup_type_str) = &backup_type {
        if let Ok(backup_type_enum) = serde_json::from_str::<BackupType>(&format!("\"{}\"", backup_type_str)) {
            filtered_records.retain(|r| r.backup_type == backup_type_enum);
        }
    }
    
    if let Some(status_str) = &status {
        if let Ok(status_enum) = serde_json::from_str::<BackupStatus>(&format!("\"{}\"", status_str)) {
            filtered_records.retain(|r| r.status == status_enum);
        }
    }

    let total = filtered_records.len();
    let paginated_records = filtered_records.into_iter()
        .skip(offset)
        .take(size)
        .collect::<Vec<_>>();

    if !paginated_records.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "获取备份记录成功".to_string(),
            data: Some(serde_json::json!({
                "list": paginated_records,
                "total": total,
                "page": page,
                "size": size
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 0,
            msg: "暂无备份记录".to_string(),
            data: Some(serde_json::json!({
                "list": Vec::<BackupRecord>::new(),
                "total": 0,
                "page": page,
                "size": size
            })),
        })
    }
}

// 创建备份记录
#[post("/api/backup-records")]
pub async fn create_backup_record(
    record_data: web::Json<CreateBackupRecordRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    let new_id = records.len() as u32 + 1;
    
    // 自动生成文件名和路径
    let filename = record_data.filename.clone().unwrap_or_else(|| {
        format!("backup_{}_{}.json", 
            Utc::now().format("%Y%m%d_%H%M%S"),
            new_id
        )
    });
    
    let file_path = record_data.file_path.clone().unwrap_or_else(|| {
        format!("./backups/{}", filename)
    });
    
    let mut new_record = BackupRecord {
        id: new_id,
        filename: filename.clone(),
        file_size: 0,
        backup_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        backup_type: record_data.backup_type.clone(),
        status: BackupStatus::InProgress,
        description: record_data.description.clone(),
        file_path: file_path.clone(),
        restore_time: None,
        restore_by: None,
    };

    // 先保存记录，状态为进行中
    records.push(new_record.clone());
    if let Err(_) = data.data_manager.save_backup_records(&records) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 500,
            msg: "保存备份记录失败".to_string(),
            data: None,
        });
    }

    // 执行实际备份操作
    match perform_backup_operation(&file_path, &data.data_manager).await {
        Ok(file_size) => {
            // 检查文件是否真的创建成功
            if !std::path::Path::new(&file_path).exists() {
                // 备份失败，更新状态
                new_record.status = BackupStatus::Failed;
                if let Some(record) = records.iter_mut().find(|r| r.id == new_id) {
                    record.status = BackupStatus::Failed;
                }
                let _ = data.data_manager.save_backup_records(&records);
                
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "备份文件创建失败".to_string(),
                    data: None,
                });
            }
            
            // 更新记录状态和文件大小
            new_record.status = BackupStatus::Success;
            new_record.file_size = file_size;
            
            // 更新记录列表
            if let Some(record) = records.iter_mut().find(|r| r.id == new_id) {
                record.status = BackupStatus::Success;
                record.file_size = file_size;
            }
            
            if let Err(_) = data.data_manager.save_backup_records(&records) {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "更新备份记录失败".to_string(),
                    data: None,
                });
            }
            
            HttpResponse::Ok().json(ApiResponse {
                code: 0,
                msg: "备份创建成功".to_string(),
                data: Some(new_record),
            })
        }
        Err(e) => {
            // 备份失败，更新状态
            new_record.status = BackupStatus::Failed;
            if let Some(record) = records.iter_mut().find(|r| r.id == new_id) {
                record.status = BackupStatus::Failed;
            }
            let _ = data.data_manager.save_backup_records(&records);
            
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: format!("备份操作失败: {}", e),
                data: None,
            })
        }
    }
}

// 下载备份文件
pub async fn download_backup(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let backup_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的备份ID".to_string(),
            data: None,
        }),
    };

    let records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    if let Some(record) = records.iter().find(|r| r.id == backup_id) {
        // 检查文件是否存在
        if !std::path::Path::new(&record.file_path).exists() {
            return HttpResponse::NotFound().json(ApiResponse::<()> {
                code: 404,
                msg: "备份文件不存在".to_string(),
                data: None,
            });
        }
        
        // 读取文件内容并返回
        match std::fs::read_to_string(&record.file_path) {
            Ok(content) => {
                HttpResponse::Ok()
                    .content_type("application/json")
                    .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", record.filename)))
                    .body(content)
            }
            Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "读取备份文件失败".to_string(),
                data: None,
            })
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "备份记录不存在".to_string(),
            data: None,
        })
    }
}

// 恢复备份
pub async fn restore_backup(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let backup_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的备份ID".to_string(),
            data: None,
        }),
    };

    let records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    if let Some(record) = records.iter().find(|r| r.id == backup_id) {
        // 检查文件是否存在
        if !std::path::Path::new(&record.file_path).exists() {
            return HttpResponse::NotFound().json(ApiResponse::<()> {
                code: 404,
                msg: "备份文件不存在".to_string(),
                data: None,
            });
        }
        
        // 检查备份状态
        if record.status != BackupStatus::Success {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                code: 400,
                msg: "只能恢复成功的备份".to_string(),
                data: None,
            });
        }
        
        // 读取备份文件
        match std::fs::read_to_string(&record.file_path) {
            Ok(content) => {
                // 解析备份数据
                match serde_json::from_str::<BackupData>(&content) {
                    Ok(backup_data) => {
                        // 恢复用户数据
                        if let Err(e) = data.data_manager.save_users(&backup_data.users.into_iter().map(|u| (u.id.to_string(), u)).collect()) {
                            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                                code: 500,
                                msg: format!("恢复用户数据失败: {}", e),
                                data: None,
                            });
                        }
                        
                        // 恢复系统设置
                        if let Err(e) = data.data_manager.save_settings(&backup_data.settings) {
                            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                                code: 500,
                                msg: format!("恢复系统设置失败: {}", e),
                                data: None,
                            });
                        }
                        
                        // 恢复绳包数据
                        let packages_count = backup_data.packages.len();
                        let raw_data = crate::models::RawDataJson {
                            绳包列表: backup_data.packages.into_iter().map(|p| crate::models::RawRopePackage {
                                id: p.id,
                                绳包名称: p.name,
                                作者: p.author,
                                版本: p.version,
                                简介: p.desc,
                                项目直链: p.url,
                                下载次数: p.downloads,
                                上架时间: p.upload_time,
                                分类: p.category,
                                状态: p.status,
                                是否标星: p.is_starred,
                                标星时间: p.star_time,
                                标星人: p.star_by,
                            }).collect(),
                            数据库配置: crate::models::DatabaseConfig {
                                数据库名称: "结绳绳包数据库".to_string(),
                                数据库项目: packages_count as u32,
                                数据库版本: "1.0.0".to_string(),
                                数据库更新时间: chrono::Local::now().format("%Y%m%d").to_string(),
                            },
                        };
                        
                        if let Err(e) = data.data_manager.save_raw_data(&raw_data) {
                            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                                code: 500,
                                msg: format!("恢复绳包数据失败: {}", e),
                                data: None,
                            });
                        }
                        
                        // 更新恢复时间和操作人
                        let mut updated_records = records;
                        if let Some(record) = updated_records.iter_mut().find(|r| r.id == backup_id) {
                            record.restore_time = Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
                            record.restore_by = Some("admin".to_string());
                        }
                        
                        if let Err(_) = data.data_manager.save_backup_records(&updated_records) {
                            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                                code: 500,
                                msg: "更新恢复记录失败".to_string(),
                                data: None,
                            });
                        }
                        
                        HttpResponse::Ok().json(ApiResponse::<()> {
                            code: 0,
                            msg: "备份恢复成功".to_string(),
                            data: None,
                        })
                    }
                    Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()> {
                        code: 400,
                        msg: format!("备份文件格式错误: {}", e),
                        data: None,
                    })
                }
            }
            Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "读取备份文件失败".to_string(),
                data: None,
            })
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "备份记录不存在".to_string(),
            data: None,
        })
    }
}

#[post("/api/backup-records/configure")]
pub async fn configure_auto_backup(
    config: web::Json<AutoBackupConfig>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut settings = match data.data_manager.load_settings() {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "读取系统设置失败".to_string(),
            data: None,
        }),
    };
    settings.backup_settings.enable_auto_backup = config.enable_auto_backup;
    settings.backup_settings.backup_interval_hours = config.backup_interval_hours;
    settings.backup_settings.backup_location = config.backup_location.clone();
    settings.backup_settings.max_backup_files = config.max_backup_files;
    if let Err(_) = data.data_manager.save_settings(&settings) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "保存系统设置失败".to_string(),
            data: None,
        });
    }
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: "自动备份配置已保存".to_string(),
        data: None,
    })
}

// 删除备份记录
#[delete("/api/backup-records/{id}")]
pub async fn delete_backup_record(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let backup_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的备份ID".to_string(),
            data: None,
        }),
    };

    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    // 查找要删除的备份文件路径
    let file_path = records.iter().find(|r| r.id == backup_id).map(|r| r.file_path.clone());

    records.retain(|r| r.id != backup_id);

    if let Err(_) = data.data_manager.save_backup_records(&records) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "删除备份记录失败".to_string(),
            data: None,
        });
    }

    // 删除实际文件
    if let Some(path) = file_path {
        let _ = std::fs::remove_file(path);
    }

    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: "备份记录删除成功".to_string(),
        data: None,
    })
}

// 批量删除备份记录
#[post("/api/backup-records/batch")]
pub async fn batch_delete_backup_records(
    delete_data: web::Json<BatchDeleteRequest>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    let ids_to_delete: std::collections::HashSet<u32> = delete_data.ids.iter().cloned().collect();
    
    // 收集要删除的文件路径
    let files_to_delete: Vec<String> = records
        .iter()
        .filter(|r| ids_to_delete.contains(&r.id))
        .map(|r| r.file_path.clone())
        .collect();

    // 删除记录
    records.retain(|r| !ids_to_delete.contains(&r.id));

    if let Err(_) = data.data_manager.save_backup_records(&records) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 1,
            msg: "批量删除备份记录失败".to_string(),
            data: None,
        });
    }

    // 删除实际文件
    for file_path in files_to_delete {
        let _ = std::fs::remove_file(file_path);
    }

    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 0,
        msg: format!("成功删除 {} 个备份记录", delete_data.ids.len()),
        data: None,
    })
}

// 更新备份状态
pub async fn update_backup_status(
    path: web::Path<String>,
    status_data: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let backup_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的备份ID".to_string(),
            data: None,
        }),
    };

    let status_str = status_data.get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let status = match status_str {
        "success" => BackupStatus::Success,
        "failed" => BackupStatus::Failed,
        "in_progress" => BackupStatus::InProgress,
        _ => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的状态值".to_string(),
            data: None,
        }),
    };

    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    if let Some(record) = records.iter_mut().find(|r| r.id == backup_id) {
        record.status = status;

        match data.data_manager.save_backup_records(&records) {
            Ok(_) => {
                HttpResponse::Ok().json(ApiResponse::<()> {
                    code: 200,
                    msg: "备份状态更新成功".to_string(),
                    data: None,
                })
            }
            Err(_) => {
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "更新备份状态失败".to_string(),
                    data: None,
                })
            }
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "备份记录不存在".to_string(),
            data: None,
        })
    }
}

// 获取备份统计
pub async fn get_backup_stats(
    data: web::Data<AppState>,
) -> HttpResponse {
    let records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    let total_backups = records.len();
    let success_backups = records.iter().filter(|r| r.status == BackupStatus::Success).count();
    let failed_backups = records.iter().filter(|r| r.status == BackupStatus::Failed).count();
    let in_progress_backups = records.iter().filter(|r| r.status == BackupStatus::InProgress).count();

    if !records.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取备份统计成功".to_string(),
            data: Some(serde_json::json!({
                "total_backups": total_backups,
                "success_backups": success_backups,
                "failed_backups": failed_backups,
                "in_progress_backups": in_progress_backups,
                "daily_stats": []
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "暂无备份数据".to_string(),
            data: Some(serde_json::json!({
                "total_backups": 0,
                "success_backups": 0,
                "failed_backups": 0,
                "in_progress_backups": 0,
                "daily_stats": []
            })),
        })
    }
}

// 执行手动备份
pub async fn perform_manual_backup(
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    let new_id = records.len() as u32 + 1;
    
    let filename = format!("manual_backup_{}.json", Utc::now().timestamp());
    let file_path = format!("./backups/{}", filename);
    
    let mut new_record = BackupRecord {
        id: new_id,
        filename: filename.clone(),
        file_size: 0,
        backup_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        backup_type: BackupType::Manual,
        status: BackupStatus::InProgress,
        description: "手动备份".to_string(),
        file_path: file_path.clone(),
        restore_time: None,
        restore_by: None,
    };

    // 先保存记录，状态为进行中
    records.push(new_record.clone());
    if let Err(_) = data.data_manager.save_backup_records(&records) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 500,
            msg: "保存备份记录失败".to_string(),
            data: None,
        });
    }

    // 执行实际备份操作
    match perform_backup_operation(&file_path, &data.data_manager).await {
        Ok(file_size) => {
            // 更新记录状态和文件大小
            new_record.status = BackupStatus::Success;
            new_record.file_size = file_size;
            
            // 更新记录列表
            if let Some(record) = records.iter_mut().find(|r| r.id == new_id) {
                record.status = BackupStatus::Success;
                record.file_size = file_size;
            }
            
            if let Err(_) = data.data_manager.save_backup_records(&records) {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "更新备份记录失败".to_string(),
                    data: None,
                });
            }
            
            HttpResponse::Ok().json(ApiResponse {
                code: 200,
                msg: "手动备份完成".to_string(),
                data: Some(new_record),
            })
        }
        Err(e) => {
            // 备份失败，更新状态
            new_record.status = BackupStatus::Failed;
            if let Some(record) = records.iter_mut().find(|r| r.id == new_id) {
                record.status = BackupStatus::Failed;
            }
            let _ = data.data_manager.save_backup_records(&records);
            
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: format!("手动备份失败: {}", e),
                data: None,
            })
        }
    }
} 