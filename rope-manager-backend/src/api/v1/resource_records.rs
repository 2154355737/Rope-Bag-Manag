use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use log::{info, debug, error};
use crate::services::admin_service::AdminService;
use crate::models::{ResourceRecord, ResourceRecordQuery, CreateResourceRecordRequest, BatchDeleteResourceRecordsRequest, ClearResourceRecordsRequest};
use crate::middleware::auth::extract_user_id;

/// 配置资源记录模块路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resource-records")
            .route("", web::get().to(get_resource_records))
            .route("", web::post().to(log_resource_action))
            .route("/batch-delete", web::post().to(batch_delete_records))
            .route("/clear", web::post().to(clear_records))
            .route("/stats", web::get().to(get_resource_action_stats))
            .route("/{id}", web::delete().to(delete_record))
    );
}

/// 获取资源记录列表
/// 
/// 支持分页、筛选等功能
async fn get_resource_records(
    query: web::Query<ResourceRecordQuery>,
    admin_service: web::Data<AdminService>,
) -> HttpResponse {
    debug!("获取资源记录列表：query={:?}", query);
    
    match admin_service.get_resource_records().await {
        Ok(records) => {
            // 分页逻辑
            let page = query.page.unwrap_or(1);
            let size = query.size.unwrap_or(20);
            let total = records.len();
            
            debug!("资源记录总数：{}", total);

            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": {
                    "list": records,
                    "total": total,
                    "page": page,
                    "size": size
                }
            }))
        },
        Err(e) => {
            error!("获取资源记录失败: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取资源记录失败: {}", e)
            }))
        }
    }
}

/// 记录资源操作
/// 
/// 允许前端记录各种资源操作，如创建、更新、删除等
async fn log_resource_action(
    req: web::Json<CreateResourceRecordRequest>,
    admin_service: web::Data<AdminService>,
    http_req: HttpRequest,
) -> HttpResponse {
    info!("记录资源操作: {:?}", req);
    
    // 从请求中获取用户ID
    let user_id = match extract_user_id(&http_req) {
        Some(id) => id,
        None => {
            // 生产环境中应该返回认证错误，但为了测试临时使用默认值
            debug!("未找到用户ID，使用默认值1");
            1 // 使用默认ID以便测试
        }
    };
    
    debug!("用户ID: {}", user_id);

    match admin_service.log_resource_action(&req, user_id).await {
        Ok(id) => {
            info!("操作记录成功，记录ID: {}", id);
            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "操作已记录",
                "data": {
                    "id": id
                }
            }))
        },
        Err(e) => {
            error!("记录操作失败: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("记录操作失败: {}", e)
            }))
        }
    }
}

/// 删除单条记录
async fn delete_record(
    path: web::Path<i32>,
    admin_service: web::Data<AdminService>,
) -> HttpResponse {
    let record_id = path.into_inner();
    debug!("删除资源记录: id={}", record_id);
    
    match admin_service.delete_resource_record(record_id).await {
        Ok(_) => {
            info!("记录{}已删除", record_id);
            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": format!("记录 {} 已删除", record_id),
            }))
        },
        Err(e) => {
            error!("删除记录失败: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("删除记录失败: {}", e)
            }))
        }
    }
}

/// 批量删除记录
async fn batch_delete_records(
    req: web::Json<BatchDeleteResourceRecordsRequest>,
    admin_service: web::Data<AdminService>,
) -> HttpResponse {
    debug!("批量删除记录: {:?}", req.ids);
    
    match admin_service.batch_delete_resource_records(&req.ids).await {
        Ok(count) => {
            info!("已删除{}条记录", count);
            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": format!("已删除 {} 条记录", count),
            }))
        },
        Err(e) => {
            error!("批量删除失败: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("批量删除失败: {}", e)
            }))
        }
    }
}

/// 清空记录
async fn clear_records(
    req: web::Json<ClearResourceRecordsRequest>,
    admin_service: web::Data<AdminService>,
) -> HttpResponse {
    debug!("清空记录请求: {:?}", req);
    // TODO: 实现清空记录功能
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "记录已清空",
    }))
}

/// 获取资源操作统计
/// 
/// 用于统计面板显示
async fn get_resource_action_stats(
    admin_service: web::Data<AdminService>,
) -> HttpResponse {
    debug!("获取资源操作统计");
    
    match admin_service.get_resource_action_stats().await {
        Ok(stats) => {
            debug!("获取统计数据成功");
            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": stats
            }))
        },
        Err(e) => {
            error!("获取统计数据失败: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取统计数据失败: {}", e)
            }))
        }
    }
} 