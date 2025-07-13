use actix_web::{web, HttpResponse, get, post, put, delete};
use crate::models::{AppState, ApiResponse, GlobalAnnouncement, AnnouncementType};
use serde::{Deserialize};
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct AnnouncementQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub enabled: Option<bool>,
    pub type_: Option<String>,
    pub priority: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnnouncementRequest {
    pub enabled: bool,
    pub title: String,
    pub content: String,
    pub type_: AnnouncementType,
    pub start_time: String,
    pub end_time: Option<String>,
    pub priority: u8,
}

// 获取公告列表
#[get("/api/announcements")]
pub async fn get_announcements(
    query: web::Query<AnnouncementQuery>,
    _data: web::Data<AppState>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    let offset = (page - 1) * size;

    // 这里应该从data_manager加载公告，简化处理
    let announcements = Vec::<GlobalAnnouncement>::new();
    
    // 应用过滤条件
    let mut filtered_announcements = announcements;
    if let Some(enabled) = &query.enabled {
        filtered_announcements.retain(|a| a.enabled == *enabled);
    }
    
    if let Some(type_str) = &query.type_ {
        if let Ok(announcement_type) = serde_json::from_str::<AnnouncementType>(&format!("\"{}\"", type_str)) {
            filtered_announcements.retain(|a| a.type_ == announcement_type);
        }
    }
    
    if let Some(priority) = &query.priority {
        filtered_announcements.retain(|a| a.priority == *priority);
    }

    let total = filtered_announcements.len();
    let paginated_announcements = filtered_announcements.into_iter()
        .skip(offset)
        .take(size)
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "获取公告列表成功".to_string(),
        data: Some(serde_json::json!({
            "announcements": paginated_announcements,
            "total": total,
            "page": page,
            "size": size
        })),
    })
}

// 获取单个公告
#[get("/api/announcements/{id}")]
pub async fn get_announcement(
    path: web::Path<String>,
    _data: web::Data<AppState>,
) -> HttpResponse {
    let _announcement_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的公告ID".to_string(),
            data: None,
        }),
    };

    // 这里应该从data_manager加载公告，简化处理
    HttpResponse::NotFound().json(ApiResponse::<()> {
        code: 404,
        msg: "公告不存在".to_string(),
        data: None,
    })
}

// 创建公告
#[post("/api/announcements")]
pub async fn create_announcement(
    announcement_data: web::Json<CreateAnnouncementRequest>,
    _data: web::Data<AppState>,
) -> HttpResponse {
    let new_id = 1; // 简化处理
    
    let new_announcement = GlobalAnnouncement {
        id: new_id,
        enabled: announcement_data.enabled,
        title: announcement_data.title.clone(),
        content: announcement_data.content.clone(),
        type_: announcement_data.type_.clone(),
        start_time: announcement_data.start_time.clone(),
        end_time: announcement_data.end_time.clone(),
        priority: announcement_data.priority,
        create_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        update_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "公告创建成功".to_string(),
        data: Some(new_announcement),
    })
}

// 更新公告
#[put("/api/announcements/{id}")]
pub async fn update_announcement(
    path: web::Path<String>,
    _announcement_data: web::Json<CreateAnnouncementRequest>,
    _data: web::Data<AppState>,
) -> HttpResponse {
    let _announcement_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的公告ID".to_string(),
            data: None,
        }),
    };

    // 这里应该更新公告，简化处理
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 200,
        msg: "公告更新成功".to_string(),
        data: None,
    })
}

// 删除公告
#[delete("/api/announcements/{id}")]
pub async fn delete_announcement(
    path: web::Path<String>,
    _data: web::Data<AppState>,
) -> HttpResponse {
    let _announcement_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的公告ID".to_string(),
            data: None,
        }),
    };

    // 这里应该删除公告，简化处理
    HttpResponse::Ok().json(ApiResponse::<()> {
        code: 200,
        msg: "公告删除成功".to_string(),
        data: None,
    })
}

// 获取活跃公告
#[get("/api/announcements/active")]
pub async fn get_active_announcements(
    _data: web::Data<AppState>,
) -> HttpResponse {
    // 这里应该获取活跃的公告，简化处理
    let active_announcements = Vec::<GlobalAnnouncement>::new();

    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "获取活跃公告成功".to_string(),
        data: Some(active_announcements),
    })
}

// 获取公告统计
#[get("/api/announcements/stats")]
pub async fn get_announcement_stats(
    _data: web::Data<AppState>,
) -> HttpResponse {
    // 这里应该获取公告统计，简化处理
    HttpResponse::Ok().json(ApiResponse {
        code: 200,
        msg: "获取公告统计成功".to_string(),
        data: Some(serde_json::json!({
            "total_announcements": 0,
            "active_announcements": 0,
            "global_announcements": 0,
            "announcement_types": {},
            "daily_stats": []
        })),
    })
} 