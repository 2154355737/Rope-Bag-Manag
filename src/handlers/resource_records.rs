use actix_web::{web, HttpResponse, get, post, delete};
use serde::{Deserialize};
use crate::models::{AppState, ApiResponse, ResourceRecord, ResourceType, ResourceAction};
use actix_web::HttpRequest;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct ResourceRecordQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub user_id: Option<String>,
    pub resource_type: Option<String>,
    pub action: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateResourceRecordRequest {
    pub resource_type: ResourceType,
    pub resource_id: u32,
    pub action: ResourceAction,
    pub old_data: Option<String>,
    pub new_data: Option<String>,
}

// 获取资源记录列表
#[get("/api/resource-records")]
pub async fn get_resource_records(
    query: web::Query<ResourceRecordQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    let offset = (page - 1) * size;

    let records = match data.data_manager.load_resource_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_records = records;
    if let Some(user_id) = &query.user_id {
        filtered_records.retain(|r| r.user_id == *user_id);
    }
    
    if let Some(resource_type_str) = &query.resource_type {
        if let Ok(resource_type) = serde_json::from_str::<ResourceType>(&format!("\"{}\"", resource_type_str)) {
            filtered_records.retain(|r| r.resource_type == resource_type);
        }
    }
    
    if let Some(action_str) = &query.action {
        if let Ok(action) = serde_json::from_str::<ResourceAction>(&format!("\"{}\"", action_str)) {
            filtered_records.retain(|r| r.action == action);
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
            msg: "获取资源记录成功".to_string(),
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
            msg: "暂无资源记录".to_string(),
            data: Some(serde_json::json!({
                "records": Vec::<ResourceRecord>::new(),
                "total": 0,
                "page": page,
                "size": size
            })),
        })
    }
}

// 创建资源记录
#[post("/api/resource-records")]
pub async fn create_resource_record(
    req: HttpRequest,
    record_data: web::Json<CreateResourceRecordRequest>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let username = match req.headers().get("X-Username") {
        Some(name) => name.to_str().unwrap_or("").to_string(),
        None => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 401,
            msg: "未授权访问".to_string(),
            data: None,
        }),
    };

    let ip_address = req.connection_info().peer_addr()
        .unwrap_or("unknown")
        .to_string();

    let mut records = match data.data_manager.load_resource_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    let new_id = records.len() as u32 + 1;
    
    let new_record = ResourceRecord {
        id: new_id,
        resource_type: record_data.resource_type.clone(),
        resource_id: record_data.resource_id,
        user_id: username.clone(),
        action: record_data.action.clone(),
        old_data: record_data.old_data.clone(),
        new_data: record_data.new_data.clone(),
        timestamp: Utc::now().timestamp() as u64,
        ip_address,
    };

    records.push(new_record.clone());

    match data.data_manager.save_resource_records(&records) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse {
                code: 200,
                msg: "资源记录创建成功".to_string(),
                data: Some(new_record),
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "保存资源记录失败".to_string(),
                data: None,
            })
        }
    }
}

// 获取资源记录统计
#[get("/api/resource-records/stats")]
pub async fn get_resource_record_stats(
    query: web::Query<ResourceRecordQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let records = match data.data_manager.load_resource_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_records = records;
    if let Some(user_id) = &query.user_id {
        filtered_records.retain(|r| r.user_id == *user_id);
    }
    
    if let Some(resource_type_str) = &query.resource_type {
        if let Ok(resource_type) = serde_json::from_str::<ResourceType>(&format!("\"{}\"", resource_type_str)) {
            filtered_records.retain(|r| r.resource_type == resource_type);
        }
    }
    
    if let Some(action_str) = &query.action {
        if let Ok(action) = serde_json::from_str::<ResourceAction>(&format!("\"{}\"", action_str)) {
            filtered_records.retain(|r| r.action == action);
        }
    }

    let total_records = filtered_records.len();
    let mut action_types = std::collections::HashMap::new();
    let mut resource_types = std::collections::HashMap::new();
    
    for record in &filtered_records {
        let action_str = format!("{:?}", record.action);
        *action_types.entry(action_str).or_insert(0) += 1;
        
        let resource_type_str = format!("{:?}", record.resource_type);
        *resource_types.entry(resource_type_str).or_insert(0) += 1;
    }

    if !filtered_records.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取资源记录统计成功".to_string(),
            data: Some(serde_json::json!({
                "total_records": total_records,
                "action_types": action_types,
                "resource_types": resource_types,
                "daily_stats": []
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "暂无资源记录数据".to_string(),
            data: Some(serde_json::json!({
                "total_records": 0,
                "action_types": {},
                "resource_types": {},
                "daily_stats": []
            })),
        })
    }
}

// 删除资源记录
#[delete("/api/resource-records/{id}")]
pub async fn delete_resource_record(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let record_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的资源记录ID".to_string(),
            data: None,
        }),
    };

    let mut records = match data.data_manager.load_resource_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    records.retain(|r| r.id != record_id);

    match data.data_manager.save_resource_records(&records) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                msg: "资源记录删除成功".to_string(),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "删除资源记录失败".to_string(),
                data: None,
            })
        }
    }
}

// 批量删除资源记录
#[delete("/api/resource-records/batch")]
pub async fn batch_delete_resource_records(
    delete_data: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let record_ids = delete_data.get("record_ids")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|id| id as u32))
                .collect::<Vec<u32>>()
                .into()
        })
        .unwrap_or(Vec::new());

    if record_ids.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "请提供要删除的资源记录ID".to_string(),
            data: None,
        });
    }

    let mut records = match data.data_manager.load_resource_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    let before_count = records.len();
    records.retain(|r| !record_ids.contains(&r.id));
    let after_count = records.len();
    let deleted_count = before_count - after_count;

    match data.data_manager.save_resource_records(&records) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                msg: format!("成功删除{}条资源记录", deleted_count),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "批量删除资源记录失败".to_string(),
                data: None,
            })
        }
    }
}

// 导出资源记录
#[get("/api/resource-records/export")]
pub async fn export_resource_records(
    query: web::Query<ResourceRecordQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let records = match data.data_manager.load_resource_records() {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_records = records;
    if let Some(user_id) = &query.user_id {
        filtered_records.retain(|r| r.user_id == *user_id);
    }
    
    if let Some(resource_type_str) = &query.resource_type {
        if let Ok(resource_type) = serde_json::from_str::<ResourceType>(&format!("\"{}\"", resource_type_str)) {
            filtered_records.retain(|r| r.resource_type == resource_type);
        }
    }
    
    if let Some(action_str) = &query.action {
        if let Ok(action) = serde_json::from_str::<ResourceAction>(&format!("\"{}\"", action_str)) {
            filtered_records.retain(|r| r.action == action);
        }
    }

    // 这里应该生成导出文件，简化处理
    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "导出资源记录成功".to_string(),
        data: Some(serde_json::json!({
            "download_url": "",
            "filename": "resource_records_export.json"
        })),
    })
} 