use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::services::community_service::CommunityService;
use crate::services::user_action_service::UserActionService;
use crate::models::{CreateCommentRequest, user_action::UserActionQueryParams};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/community")
            .service(
                web::resource("/comments/{package_id}")
                    .route(web::get().to(get_comments))
                    .route(web::post().to(create_comment))
            )
            .service(
                web::resource("/activities")
                    .route(web::get().to(get_public_activities))
            )
    );
}

async fn get_comments(
    path: web::Path<i32>,
    community_service: web::Data<CommunityService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match community_service.get_comments(package_id).await {
        Ok(comments) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                "list": comments,
                "total": comments.len(),
                "page": 1,
                "size": comments.len()
            }
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string()
        })))
    }
}

async fn create_comment(
    path: web::Path<i32>,
    req: web::Json<CreateCommentRequest>,
    community_service: web::Data<CommunityService>,
) -> Result<HttpResponse, actix_web::Error> {
    let package_id = path.into_inner();
    match community_service.create_comment(package_id, &req.content).await {
        Ok(comment) => Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "评论成功",
            "data": comment
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": e.to_string()
        })))
    }
}

// 获取公开的社区动态（无需认证）
async fn get_public_activities(
    query: web::Query<UserActionQueryParams>,
    user_action_service: web::Data<UserActionService>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("[DEBUG] get_public_activities called with params: {:?}", query);
    
    // 限制只返回有意义的活动类型
    let meaningful_actions = vec![
        "Upload", "Create", "Update", "Comment", "Like", 
        "Register", "Post", "Share", "Download"
    ];
    
    match user_action_service.get_user_actions_with_user(&query).await {
        Ok((actions, total)) => {
            // 过滤出有意义的活动并格式化显示文本
            let filtered_actions: Vec<_> = actions.into_iter()
                .filter(|action| {
                    meaningful_actions.iter().any(|&meaningful| 
                        action.action_type.contains(meaningful)
                    )
                })
                .map(|mut action| {
                    // 根据是否有用户信息来格式化显示文本
                    if let Some(details) = &action.details {
                        if action.username.is_some() {
                            // 登录用户：保持原有格式，如"某某用户下载了资源"
                            action.details = Some(details.clone());
                        } else {
                            // 未登录用户：修改为被动语态，如"某某资源被下载"
                            let new_details = match action.action_type.as_str() {
                                "Download" => details.replace("下载了", "被下载").replace("访客", ""),
                                "Upload" => details.replace("上传了", "被上传").replace("访客", ""),
                                "PageView" => details.replace("查看了", "被查看").replace("访客", ""),
                                _ => details.clone()
                            };
                            action.details = Some(new_details);
                        }
                    }
                    action
                })
                .collect();
                
            println!("[DEBUG] get_public_activities filtered: {} actions from {} total", 
                     filtered_actions.len(), total);
            
            Ok(HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "success",
                "data": {
                    "actions": filtered_actions,
                    "total": filtered_actions.len(),
                    "page": query.page.unwrap_or(1),
                    "pageSize": query.page_size.unwrap_or(20),
                }
            })))
        },
        Err(e) => {
            println!("[ERROR] get_public_activities failed: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": e.to_string()
            })))
        }
    }
} 