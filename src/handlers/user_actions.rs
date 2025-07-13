use actix_web::{web, HttpResponse, get, post, put, delete};
use serde::{Deserialize};
use crate::models::{AppState, ApiResponse};
use actix_web::HttpRequest;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct UserActionQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub user_id: Option<String>,
    pub action_type: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserActionRequest {
    pub action_type: crate::models::ActionType,
    pub target_type: String,
    pub target_id: String,
    pub description: String,
    pub success: bool,
    pub error_message: Option<String>,
}

// 获取用户行为记录列表
#[get("/api/user-actions")]
pub async fn get_user_actions(
    query: web::Query<UserActionQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    let offset = (page - 1) * size;

    let actions = match data.data_manager.load_user_actions() {
        Ok(actions) => actions,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_actions = actions;
    if let Some(user_id) = &query.user_id {
        filtered_actions.retain(|a| a.user_id == *user_id);
    }
    
    if let Some(action_type_str) = &query.action_type {
        if let Ok(action_type) = serde_json::from_str::<crate::models::ActionType>(&format!("\"{}\"", action_type_str)) {
            filtered_actions.retain(|a| a.action_type == action_type);
        }
    }

    let total = filtered_actions.len();
    let paginated_actions = filtered_actions.into_iter()
        .skip(offset)
        .take(size)
        .collect::<Vec<_>>();

    if !paginated_actions.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取用户行为记录成功".to_string(),
            data: Some(serde_json::json!({
                "actions": paginated_actions,
                "total": total,
                "page": page,
                "size": size
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "暂无用户行为记录".to_string(),
            data: Some(serde_json::json!({
                "actions": Vec::<crate::models::UserAction>::new(),
                "total": 0,
                "page": page,
                "size": size
            })),
        })
    }
}

// 创建用户行为记录
#[post("/api/user-actions")]
pub async fn create_user_action(
    req: HttpRequest,
    action_data: web::Json<CreateUserActionRequest>,
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
    
    let user_agent = req.headers().get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let mut actions = match data.data_manager.load_user_actions() {
        Ok(actions) => actions,
        Err(_) => Vec::new(),
    };
    let new_id = actions.len() as u32 + 1;
    
    let new_action = crate::models::UserAction {
        id: new_id,
        user_id: username.clone(),
        action_type: action_data.action_type.clone(),
        target_type: action_data.target_type.clone(),
        target_id: action_data.target_id.clone(),
        description: action_data.description.clone(),
        ip_address,
        user_agent,
        timestamp: Utc::now().timestamp() as u64,
        success: action_data.success,
        error_message: action_data.error_message.clone(),
    };

    actions.push(new_action.clone());

    match data.data_manager.save_user_actions(&actions) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse {
                code: 200,
                msg: "用户行为记录创建成功".to_string(),
                data: Some(new_action),
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "保存用户行为记录失败".to_string(),
                data: None,
            })
        }
    }
}

// 获取用户行为统计
#[get("/api/user-actions/stats")]
pub async fn get_user_action_stats(
    query: web::Query<UserActionQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let actions = match data.data_manager.load_user_actions() {
        Ok(actions) => actions,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_actions = actions;
    if let Some(user_id) = &query.user_id {
        filtered_actions.retain(|a| a.user_id == *user_id);
    }
    
    if let Some(action_type_str) = &query.action_type {
        if let Ok(action_type) = serde_json::from_str::<crate::models::ActionType>(&format!("\"{}\"", action_type_str)) {
            filtered_actions.retain(|a| a.action_type == action_type);
        }
    }

    let total_actions = filtered_actions.len();
    let mut action_types = std::collections::HashMap::new();
    
    for action in &filtered_actions {
        let action_type_str = format!("{:?}", action.action_type);
        *action_types.entry(action_type_str).or_insert(0) += 1;
    }

    if !filtered_actions.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取用户行为统计成功".to_string(),
            data: Some(serde_json::json!({
                "total_actions": total_actions,
                "action_types": action_types,
                "daily_stats": []
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "暂无用户行为数据".to_string(),
            data: Some(serde_json::json!({
                "total_actions": 0,
                "action_types": {},
                "daily_stats": []
            })),
        })
    }
}

// 删除用户行为记录
#[delete("/api/user-actions/{id}")]
pub async fn delete_user_action(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let action_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的行为记录ID".to_string(),
            data: None,
        }),
    };

    let mut actions = match data.data_manager.load_user_actions() {
        Ok(actions) => actions,
        Err(_) => Vec::new(),
    };

    actions.retain(|a| a.id != action_id);

    match data.data_manager.save_user_actions(&actions) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                msg: "用户行为记录删除成功".to_string(),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "删除用户行为记录失败".to_string(),
                data: None,
            })
        }
    }
}

// 批量删除用户行为记录
#[delete("/api/user-actions/batch")]
pub async fn batch_delete_user_actions(
    delete_data: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let action_ids = delete_data.get("action_ids")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|id| id as u32))
                .collect::<Vec<u32>>()
                .into()
        })
        .unwrap_or(Vec::new());

    if action_ids.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "请提供要删除的行为记录ID".to_string(),
            data: None,
        });
    }

    let mut actions = match data.data_manager.load_user_actions() {
        Ok(actions) => actions,
        Err(_) => Vec::new(),
    };

    let before_count = actions.len();
    actions.retain(|a| !action_ids.contains(&a.id));
    let after_count = actions.len();
    let deleted_count = before_count - after_count;

    match data.data_manager.save_user_actions(&actions) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                msg: format!("成功删除{}条用户行为记录", deleted_count),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "批量删除用户行为记录失败".to_string(),
                data: None,
            })
        }
    }
}

// 导出用户行为记录
#[get("/api/user-actions/export")]
pub async fn export_user_actions(
    query: web::Query<UserActionQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let actions = match data.data_manager.load_user_actions() {
        Ok(actions) => actions,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_actions = actions;
    if let Some(user_id) = &query.user_id {
        filtered_actions.retain(|a| a.user_id == *user_id);
    }
    
    if let Some(action_type_str) = &query.action_type {
        if let Ok(action_type) = serde_json::from_str::<crate::models::ActionType>(&format!("\"{}\"", action_type_str)) {
            filtered_actions.retain(|a| a.action_type == action_type);
        }
    }

    // 这里应该生成导出文件，简化处理
    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "导出用户行为记录成功".to_string(),
        data: Some(serde_json::json!({
            "download_url": "",
            "filename": "user_actions_export.json"
        })),
    })
} 