use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use crate::services::user_action_service::UserActionService;
use crate::models::user_action::{CreateUserActionRequest, UserActionQueryParams};
use crate::middleware::auth::extract_user_id;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user-actions")
            .service(
                web::resource("")
                    .route(web::get().to(get_user_actions))
                    .route(web::post().to(log_user_action))
            )
            .service(
                web::resource("/stats")
                    .route(web::get().to(get_action_stats))
            )
            .service(
                web::resource("/community")
                    .route(web::get().to(get_community_activities))
            )
            .service(
                web::resource("/batch")
                    .route(web::delete().to(batch_delete_actions))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_user_action))
                    .route(web::delete().to(delete_user_action))
            )
    );
}

// 获取用户行为记录列表
async fn get_user_actions(
    req: HttpRequest,
    query: web::Query<UserActionQueryParams>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    match user_action_service.get_user_actions(&query).await {
        Ok((actions, total)) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "actions": actions,
                "total": total,
                "page": query.page.unwrap_or(1),
                "pageSize": query.page_size.unwrap_or(20),
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 获取单个用户行为记录
async fn get_user_action(
    path: web::Path<i32>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    let action_id = path.into_inner();
    
    // 这里简化处理，直接返回404
    Ok(HttpResponse::NotFound().json(json!({
        "code": 404,
        "message": "记录不存在"
    })))
}

// 记录用户行为
async fn log_user_action(
    req: HttpRequest,
    action_req: web::Json<serde_json::Value>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 打印详细日志
    println!("开始记录用户行为");
    
    // 从JWT中提取用户ID，访客用户使用None
    let user_id = extract_user_id(&req);
    println!("使用的用户ID: {:?}", user_id);
    
    // 打印请求内容
    println!("请求内容: {:?}", action_req);
    
    // 从请求中提取数据
    let action_type = action_req.get("action_type").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
    println!("行为类型: {}", action_type);
    
    let target_type = action_req.get("target_type").and_then(|v| v.as_str()).map(|s| s.to_string());
    let target_id = action_req.get("target_id").and_then(|v| v.as_i64()).map(|id| id as i32);
    let details = action_req.get("details").and_then(|v| v.as_str()).map(|s| s.to_string());
    
    // 获取IP地址和User-Agent
    let ip_address = req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let user_agent = req.headers().get("User-Agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    
    println!("IP地址: {:?}, User-Agent: {:?}", ip_address, user_agent);
    
    // 创建用户行为记录请求
    let create_req = CreateUserActionRequest {
        user_id,
        action_type,
        target_type,
        target_id,
        details,
        ip_address,
        user_agent,
    };
    
    println!("准备创建用户行为记录: user_id={:?}, action_type={}", create_req.user_id, create_req.action_type);
    
    // 记录用户行为
    match user_action_service.log_user_action(&create_req).await {
        Ok(action) => {
            println!("记录成功: ID={}", action.id);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "用户行为记录成功",
                "data": action
            })))
        },
        Err(e) => {
            println!("记录失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("记录失败: {}", e)
            })))
        }
    }
}

// 删除用户行为记录
async fn delete_user_action(
    path: web::Path<i32>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    let action_id = path.into_inner();
    
    match user_action_service.delete_user_action(action_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 批量删除用户行为记录
async fn batch_delete_actions(
    req: web::Json<serde_json::Value>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    // 从请求中提取ID列表
    let action_ids = req.get("action_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_i64().map(|id| id as i32))
                .collect::<Vec<i32>>()
        })
        .unwrap_or_default();
    
    if action_ids.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": "未提供有效的ID列表"
        })));
    }
    
    match user_action_service.batch_delete_user_actions(&action_ids).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "批量删除成功"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 获取用户行为统计数据
async fn get_action_stats(
    query: web::Query<UserActionQueryParams>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    match user_action_service.get_action_stats(&query).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": stats
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

// 获取社区动态（包含用户信息）
async fn get_community_activities(
    query: web::Query<UserActionQueryParams>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    log::debug!("🔍 get_community_activities called with params: {:?}", query);
    
    match user_action_service.get_user_actions_with_user(&query).await {
        Ok((actions, total)) => {
            println!("[DEBUG] get_community_activities success: {} actions, total: {}", actions.len(), total);
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": {
                    "actions": actions,
                    "total": total,
                    "page": query.page.unwrap_or(1),
                    "pageSize": query.page_size.unwrap_or(20),
                }
            })))
        },
        Err(e) => {
            log::error!("❌ get_community_activities failed: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": e.to_string()
            })))
        }
    }
} 