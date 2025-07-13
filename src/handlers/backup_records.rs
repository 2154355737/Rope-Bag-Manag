use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize};
use crate::models::{AppState, BackupRecord, BackupType, BackupStatus, ApiResponse};
use crate::data_manager::DataManager;
use chrono::Utc;

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
    pub filename: String,
    pub backup_type: BackupType,
    pub description: String,
    pub file_path: String,
}

// 获取备份记录列表
pub async fn get_backup_records(
    query: web::Query<BackupRecordQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    let offset = (page - 1) * size;

    let records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_records = records;
    if let Some(backup_type_str) = &query.backup_type {
        if let Ok(backup_type) = serde_json::from_str::<BackupType>(&format!("\"{}\"", backup_type_str)) {
            filtered_records.retain(|r| r.backup_type == backup_type);
        }
    }
    
    if let Some(status_str) = &query.status {
        if let Ok(status) = serde_json::from_str::<BackupStatus>(&format!("\"{}\"", status_str)) {
            filtered_records.retain(|r| r.status == status);
        }
    }

    let total = filtered_records.len();
    let paginated_records = filtered_records.into_iter()
        .skip(offset)
        .take(size)
        .collect::<Vec<_>>();

    if !paginated_records.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取备份记录成功".to_string(),
            data: Some(serde_json::json!({
                "records": paginated_records,
                "total": total,
                "page": page,
                "size": size
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "暂无备份记录".to_string(),
            data: Some(serde_json::json!({
                "records": Vec::<BackupRecord>::new(),
                "total": 0,
                "page": page,
                "size": size
            })),
        })
    }
}

// 创建备份记录
pub async fn create_backup_record(
    record_data: web::Json<CreateBackupRecordRequest>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    let new_id = records.len() as u32 + 1;
    
    let new_record = BackupRecord {
        id: new_id,
        filename: record_data.filename.clone(),
        file_size: 0, // 需要实际计算文件大小
        backup_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        backup_type: record_data.backup_type.clone(),
        status: BackupStatus::InProgress,
        description: record_data.description.clone(),
        file_path: record_data.file_path.clone(),
    };

    records.push(new_record.clone());

    match data.data_manager.save_backup_records(&records) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse {
                code: 200,
                msg: "备份记录创建成功".to_string(),
                data: Some(new_record),
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "保存备份记录失败".to_string(),
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
    
    if let Some(_record) = records.iter().find(|r| r.id == backup_id) {
        // 这里应该返回实际的文件下载，简化处理
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取下载链接成功".to_string(),
            data: Some(serde_json::json!({
                "download_url": format!("/api/backup-records/{}/download", backup_id),
                "filename": format!("backup_{}.json", backup_id)
            })),
        })
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "备份记录不存在".to_string(),
            data: None,
        })
    }
}

// 删除备份记录
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

    records.retain(|r| r.id != backup_id);

    match data.data_manager.save_backup_records(&records) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                msg: "备份记录删除成功".to_string(),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "删除备份记录失败".to_string(),
                data: None,
            })
        }
    }
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
    // 这里应该执行实际的备份操作，简化处理
    let backup_record = CreateBackupRecordRequest {
        filename: format!("manual_backup_{}.json", Utc::now().timestamp()),
        backup_type: BackupType::Manual,
        description: "手动备份".to_string(),
        file_path: format!("./backups/manual_backup_{}.json", Utc::now().timestamp()),
    };

    let mut records = match data.data_manager.load_backup_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    let new_id = records.len() as u32 + 1;
    
    let new_record = BackupRecord {
        id: new_id,
        filename: backup_record.filename.clone(),
        file_size: 0,
        backup_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        backup_type: backup_record.backup_type.clone(),
        status: BackupStatus::InProgress,
        description: backup_record.description.clone(),
        file_path: backup_record.file_path.clone(),
    };

    records.push(new_record.clone());

    match data.data_manager.save_backup_records(&records) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse {
                code: 200,
                msg: "手动备份已启动".to_string(),
                data: Some(new_record),
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "启动手动备份失败".to_string(),
                data: None,
            })
        }
    }
} 