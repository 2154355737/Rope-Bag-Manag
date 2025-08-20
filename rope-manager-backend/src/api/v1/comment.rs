use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use crate::models::ApiResponse;
use crate::models::{CreateCommentRequest, CommentListResponse};
use crate::services::comment_service::CommentService;
use crate::middleware::auth::AuthenticatedUser;
use serde::{Deserialize};
use actix_web::HttpRequest;
use crate::utils::auth_helper::AuthHelper;
use crate::services::user_action_service::UserActionService;
use crate::repositories::user_action_repo::UserActionRepository;
use crate::models::user_action::CreateUserActionRequest;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .service(get_all_comments)
            .service(get_comment)
            .service(create_comment)
            .service(update_comment)
            .service(delete_comment)
            .service(get_comment_replies)
            .service(reply_comment)
            .service(batch_update_status_put)
            .service(batch_update_status_post)
            .service(batch_delete_comments_post)
            .service(batch_delete_comments_delete)
            .service(like_comment)
            .service(dislike_comment)
            .service(check_comment_like_status)
            .service(helpful_comment)
            .service(pin_comment)
    );
    
    // 特定目标的评论接口
    cfg.service(
        web::scope("/packages/{package_id}/comments")
            .service(get_package_comments)
    );
    
}

// 查询参数结构体
#[derive(Deserialize)]
pub struct CommentQueryParams {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<i32>,
    pub user_id: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub search: Option<String>,
}

// 更新评论请求
#[derive(Deserialize)]
pub struct UpdateCommentRequest {
    pub content: Option<String>,
    pub status: Option<String>,
}

// 回复评论请求
#[derive(Deserialize)]
pub struct ReplyCommentRequest {
    pub content: String,
}

// 批量更新状态请求
#[derive(Deserialize)]
pub struct BatchStatusRequest {
    pub ids: Vec<i32>,
    pub status: String,
}

// 批量操作请求
#[derive(Deserialize)]
pub struct BatchIdsRequest {
    pub ids: Vec<i32>,
}

// 点赞请求
#[derive(Deserialize)]
pub struct LikeRequest {
    pub like: bool,
}

// 点踩请求
#[derive(Deserialize)]
pub struct DislikeRequest {
    pub dislike: bool,
}

// 置顶请求
#[derive(Deserialize)]
pub struct PinRequest {
    pub pinned: bool,
}

// 获取所有评论（管理员接口 + 公开只读）
#[get("")]
async fn get_all_comments(
    http_req: HttpRequest,
    query: web::Query<CommentQueryParams>,
    comment_service: web::Data<CommentService>,
) -> impl Responder {
    // 管理员：保留原有读取全部能力（通过HttpRequest可选解析）
    let is_admin = AuthHelper::verify_user(&http_req).map(|u| matches!(u.role, crate::models::UserRole::Admin)).unwrap_or(false);
    if is_admin {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(20);
        let status = match &query.status { Some(s) if !s.is_empty() => Some(s.as_str()), _ => None };
        let target_type = match &query.target_type { Some(t) if !t.is_empty() => Some(t.as_str()), _ => None };
        let start_date = match &query.start_date { Some(d) if !d.is_empty() => Some(d.as_str()), _ => None };
        let end_date = match &query.end_date { Some(d) if !d.is_empty() => Some(d.as_str()), _ => None };
        return match comment_service.get_all_comments(
            page, size, status, target_type, query.target_id, query.user_id, start_date, end_date, query.search.as_deref(),
        ).await {
            Ok((comments, total)) => {
                let response = CommentListResponse { list: comments, total, page, size };
                HttpResponse::Ok().json(ApiResponse::success(response))
            },
            Err(e) => {
                log::error!("获取评论列表失败: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &format!("获取评论列表失败: {}", e)))
            }
        };
    }

    // 非管理员：公开读取策略（仅 Active，限定目标类型与分页大小）
    let page = query.page.unwrap_or(1);
    let mut size = query.size.unwrap_or(20);
    if size > 100 { size = 100; }

    // 仅允许特定目标类型（Post/Package）
    let ttype = match &query.target_type {
        Some(t) if !t.is_empty() => t.to_string(),
        _ => "".to_string(),
    };
    let ttype_lower = ttype.to_lowercase();
    if !(ttype_lower == "post" || ttype_lower == "package") {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(400, "必须提供合法的 target_type: Post 或 Package"));
    }

    // 公开读取强制只返回 Active
    let status = Some("Active");
    let target_type = Some(ttype_lower.as_str());
    let start_date = None;
    let end_date = None;

    match comment_service.get_all_comments(
        page, size, status, target_type, query.target_id, None, start_date, end_date, None,
    ).await {
        Ok((comments, total)) => {
            let response = CommentListResponse { list: comments, total, page, size };
            HttpResponse::Ok().json(ApiResponse::success(response))
        },
        Err(e) => {
            log::error!("公开获取评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &format!("获取评论列表失败: {}", e)))
        }
    }
}

// 获取特定评论
#[get("/{comment_id}")]
async fn get_comment(
    path: web::Path<i32>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    match comment_service.get_comment_by_id(comment_id).await {
        Ok(Some(comment)) => {
            // 检查权限：只有评论作者、管理员或长老可以查看
            if comment.user_id == auth_user.id || auth_user.is_admin() || auth_user.is_elder() {
                HttpResponse::Ok().json(ApiResponse::success(comment))
            } else {
                HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    403, "无权查看该评论"
                ))
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error(
                404, "评论不存在"
            ))
        },
        Err(e) => {
            log::error!("获取评论详情失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取评论详情失败: {}", e)
            ))
        }
    }
}

// 创建评论
#[post("")]
async fn create_comment(
    req: web::Json<CreateCommentRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    // 创建评论
    match comment_service.create_comment(
        auth_user.id,
        req.target_type.clone(),
        req.target_id,
        req.content.clone(),
        req.parent_id,
    ).await {
        Ok(comment) => {
            HttpResponse::Created().json(ApiResponse::success(comment))
        },
        Err(e) => {
            let msg = e.to_string();
            let (http_status, code) = if msg.contains("违禁词") {
                (actix_web::http::StatusCode::OK, 400)
            } else {
                (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, 500)
            };
            log::warn!("创建评论失败: {}", msg);
            HttpResponse::build(http_status).json(ApiResponse::<()>::error(code, &msg))
        }
    }
}

// 更新评论
#[put("/{comment_id}")]
async fn update_comment(
    path: web::Path<i32>,
    req: web::Json<UpdateCommentRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    // 检查评论是否存在
    match comment_service.get_comment_by_id(comment_id).await {
        Ok(Some(comment)) => {
            // 检查权限：只有评论作者或管理员可以修改
            let is_admin = auth_user.is_admin();
            if comment.user_id == auth_user.id || is_admin {
                // 更新评论
                match comment_service.update_comment(
                    comment_id,
                    req.content.clone(),
                    req.status.clone(),
                    is_admin,
                ).await {
                    Ok(updated_comment) => {
                        HttpResponse::Ok().json(ApiResponse::success(updated_comment))
                    },
                    Err(e) => {
                        log::error!("更新评论失败: {}", e);
                        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                            500, &format!("更新评论失败: {}", e)
                        ))
                    }
                }
            } else {
                HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    403, "无权修改该评论"
                ))
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error(
                404, "评论不存在"
            ))
        },
        Err(e) => {
            log::error!("获取评论详情失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取评论详情失败: {}", e)
            ))
        }
    }
}

// 删除评论
#[delete("/{comment_id}")]
async fn delete_comment(
    path: web::Path<i32>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    // 检查评论是否存在
    match comment_service.get_comment_by_id(comment_id).await {
        Ok(Some(comment)) => {
            // 检查权限：只有评论作者、管理员或长老可以删除
            if comment.user_id == auth_user.id || auth_user.is_admin() || auth_user.is_elder() {
                // 删除评论
                match comment_service.delete_comment(comment_id, auth_user.is_admin()).await {
                    Ok(_) => {
                        HttpResponse::Ok().json(ApiResponse::<()>::success_msg("评论删除成功"))
                    },
                    Err(e) => {
                        log::error!("删除评论失败: {}", e);
                        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                            500, &format!("删除评论失败: {}", e)
                        ))
                    }
                }
            } else {
                HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    403, "无权删除该评论"
                ))
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error(
                404, "评论不存在"
            ))
        },
        Err(e) => {
            log::error!("获取评论详情失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取评论详情失败: {}", e)
            ))
        }
    }
}

// 获取评论回复
#[get("/{comment_id}/replies")]
async fn get_comment_replies(
    path: web::Path<i32>,
    comment_service: web::Data<CommentService>,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    match comment_service.get_comment_replies(comment_id).await {
        Ok(replies) => {
            HttpResponse::Ok().json(ApiResponse::success(replies))
        },
        Err(e) => {
            log::error!("获取评论回复失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取评论回复失败: {}", e)
            ))
        }
    }
}

// 回复评论
#[post("/{comment_id}/reply")]
async fn reply_comment(
    path: web::Path<i32>,
    req: web::Json<ReplyCommentRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let parent_id = path.into_inner();
    
    // 检查父评论是否存在
    match comment_service.get_comment_by_id(parent_id).await {
        Ok(Some(parent_comment)) => {
            // 创建回复评论
            match comment_service.create_comment(
                auth_user.id,
                parent_comment.target_type,
                parent_comment.target_id,
                req.content.clone(),
                Some(parent_id),
            ).await {
                Ok(comment) => {
                    HttpResponse::Created().json(ApiResponse::success(comment))
                },
                Err(e) => {
                    log::error!("回复评论失败: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        500, &format!("回复评论失败: {}", e)
                    ))
                }
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error(
                404, "父评论不存在"
            ))
        },
        Err(e) => {
            log::error!("获取父评论详情失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取父评论详情失败: {}", e)
            ))
        }
    }
}

// 批量更新评论状态 - 内部处理函数
async fn do_batch_update_status(
    payload: BatchStatusRequest,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> HttpResponse {
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(403, "只有管理员可以批量更新评论状态"));
    }
    match comment_service.batch_update_status(payload.ids, payload.status).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success_msg("批量更新评论状态成功")),
        Err(e) => {
            log::error!("批量更新评论状态失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &format!("批量更新评论状态失败: {}", e)))
        }
    }
}

// 批量更新评论状态
#[put("/batch/status")]
async fn batch_update_status_put(
    req: web::Json<BatchStatusRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    do_batch_update_status(req.into_inner(), comment_service, auth_user).await
}

#[post("/batch-status")]
async fn batch_update_status_post(
    req: web::Json<BatchStatusRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    do_batch_update_status(req.into_inner(), comment_service, auth_user).await
}

// 批量删除评论 - 内部处理函数
async fn do_batch_delete_comments(
    payload: BatchIdsRequest,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> HttpResponse {
    if !auth_user.is_admin() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(403, "只有管理员可以批量删除评论"));
    }
    match comment_service.batch_delete_comments(payload.ids).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success_msg("批量删除评论成功")),
        Err(e) => {
            log::error!("批量删除评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &format!("批量删除评论失败: {}", e)))
        }
    }
}

#[post("/batch-delete")]
async fn batch_delete_comments_post(
    req: web::Json<BatchIdsRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    do_batch_delete_comments(req.into_inner(), comment_service, auth_user).await
}

#[delete("/batch-delete")]
async fn batch_delete_comments_delete(
    req: web::Json<BatchIdsRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    do_batch_delete_comments(req.into_inner(), comment_service, auth_user).await
}

// 评论“有用”投票（幂等切换），记录到 user_actions（action_type=Helpful）
#[post("/{comment_id}/helpful")]
async fn helpful_comment(
    path: web::Path<i32>,
    auth_user: AuthenticatedUser,
    http_req: HttpRequest,
) -> impl Responder {
    let comment_id = path.into_inner();
    let conn = match crate::repositories::get_connection() { Ok(c) => c, Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())) };
    let repo = UserActionRepository::new(std::sync::Arc::new(tokio::sync::Mutex::new(conn)));
    let service = UserActionService::new(repo);

    // 查询是否已有有用投票
    let mut params = crate::models::user_action::UserActionQueryParams { page: Some(1), page_size: Some(1), user_id: Some(auth_user.id), action_type: Some("Helpful".to_string()), target_type: Some("Comment".to_string()), target_id: Some(comment_id), start_time: None, end_time: None };
    let (existing, _) = match service.get_user_actions(&params).await { Ok(x) => x, Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string())) };

    if existing.is_empty() {
        let ip = http_req.connection_info().realip_remote_addr().map(|s| s.to_string());
        let ua = http_req.headers().get("User-Agent").and_then(|h| h.to_str().ok()).map(|s| s.to_string());
        let req = CreateUserActionRequest { user_id: Some(auth_user.id), action_type: "Helpful".to_string(), target_type: Some("Comment".to_string()), target_id: Some(comment_id), details: None, ip_address: ip, user_agent: ua };
        match service.log_user_action(&req).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"active": true}))),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string()))
        }
    } else {
        // 删除该记录（toggle off）
        match service.delete_user_action(existing[0].id).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"active": false}))),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(500, &e.to_string()))
        }
    }
}

// 点赞评论
#[post("/{comment_id}/like")]
async fn like_comment(
    path: web::Path<i32>,
    req: web::Json<LikeRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    match comment_service.update_comment_likes(comment_id, auth_user.id, req.like).await {
        Ok(likes) => {
            HttpResponse::Ok().json(ApiResponse::success(likes))
        },
        Err(e) => {
            log::error!("点赞评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("点赞评论失败: {}", e)
            ))
        }
    }
}

// 点踩评论
#[post("/{comment_id}/dislike")]
async fn dislike_comment(
    path: web::Path<i32>,
    req: web::Json<DislikeRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    match comment_service.update_comment_dislikes(comment_id, auth_user.id, req.dislike).await {
        Ok(dislikes) => {
            HttpResponse::Ok().json(ApiResponse::success(dislikes))
        },
        Err(e) => {
            log::error!("点踩评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("点踩评论失败: {}", e)
            ))
        }
    }
}

// 置顶评论
#[put("/{comment_id}/pin")]
async fn pin_comment(
    path: web::Path<i32>,
    req: web::Json<PinRequest>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    // 检查评论是否存在
    match comment_service.get_comment_by_id(comment_id).await {
        Ok(Some(comment)) => {
            // 检查权限：只有评论作者或管理员可以置顶
            if comment.user_id == auth_user.id || auth_user.is_admin() {
                                 match comment_service.pin_comment(comment_id, auth_user.id, req.pinned).await {
                    Ok(pinned_comment) => {
                        HttpResponse::Ok().json(ApiResponse::success(pinned_comment))
                    },
                    Err(e) => {
                        log::error!("置顶评论失败: {}", e);
                        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                            500, &format!("置顶评论失败: {}", e)
                        ))
                    }
                }
            } else {
                HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                    403, "无权置顶该评论"
                ))
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error(
                404, "评论不存在"
            ))
        },
        Err(e) => {
            log::error!("获取评论详情失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取评论详情失败: {}", e)
            ))
        }
    }
}

// 获取资源评论
#[get("")]
async fn get_package_comments(
    path: web::Path<i32>,
    query: web::Query<CommentQueryParams>,
    comment_service: web::Data<CommentService>,
) -> impl Responder {
    let package_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);
    
    match comment_service.get_package_comments(package_id, page, size).await {
        Ok((comments, total)) => {
            let response = CommentListResponse {
                list: comments,
                total,
                page,
                size,
            };
            HttpResponse::Ok().json(ApiResponse::success(response))
        },
        Err(e) => {
            log::error!("获取资源评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取资源评论失败: {}", e)
            ))
        }
    }
}

// 获取用户评论
pub async fn get_user_comments(
    path: web::Path<i32>,
    query: web::Query<CommentQueryParams>,
    auth_user: AuthenticatedUser,
    comment_service: web::Data<CommentService>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    // 检查权限：只有本人、管理员或长老可以查看用户评论
    if user_id != auth_user.id && !auth_user.is_admin() && !auth_user.is_elder() {
        return HttpResponse::Forbidden().json(ApiResponse::<()>::error(
            403, "无权查看该用户评论"
        ));
    }
    
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(20);
    
    match comment_service.get_user_comments(user_id, page, size).await {
        Ok((comments, total)) => {
            let response = CommentListResponse {
                list: comments,
                total,
                page,
                size,
            };
            HttpResponse::Ok().json(ApiResponse::success(response))
        },
        Err(e) => {
            log::error!("获取用户评论失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("获取用户评论失败: {}", e)
            ))
        }
    }
} 

// 检查评论点赞状态
#[get("/{comment_id}/like-status")]
async fn check_comment_like_status(
    path: web::Path<i32>,
    comment_service: web::Data<CommentService>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let comment_id = path.into_inner();
    
    match comment_service.check_comment_like_status(comment_id, auth_user.id).await {
        Ok(is_liked) => {
            HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
                "liked": is_liked
            })))
        },
        Err(e) => {
            log::error!("检查评论点赞状态失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                500, &format!("检查评论点赞状态失败: {}", e)
            ))
        }
    }
} 