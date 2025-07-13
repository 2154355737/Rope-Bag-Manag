use actix_web::{web, HttpResponse, get, post, put, delete, Responder};
use crate::models::{AppState, ApiResponse};
use serde::{Deserialize};
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub target_type: crate::models::CommentTargetType,
    pub target_id: u32,
    pub content: String,
    pub parent_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentQuery {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub target_type: Option<crate::models::CommentTargetType>,
    pub target_id: Option<u32>,
    pub status: Option<String>,
    pub user_id: Option<String>,
}

// 获取评论列表
#[get("/api/comments")]
pub async fn get_comments(
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

    let target_type = params.get("target_type").cloned();
    let target_id = params.get("target_id")
        .and_then(|s| s.parse::<u32>().ok());
    let status = params.get("status").cloned();
    let user_id = params.get("user_id").cloned();
    let comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    
    // 应用过滤条件
    let mut filtered_comments = comments;
    if let Some(target_type_str) = &target_type {
        if let Ok(target_type_enum) = serde_json::from_str::<crate::models::CommentTargetType>(&format!("\"{}\"", target_type_str)) {
            filtered_comments.retain(|c| c.target_type == target_type_enum);
        }
    }
    
    if let Some(target_id_val) = &target_id {
        filtered_comments.retain(|c| c.target_id == *target_id_val);
    }
    
    if let Some(status_str) = &status {
        if let Ok(status_enum) = serde_json::from_str::<crate::models::CommentStatus>(&format!("\"{}\"", status_str)) {
            filtered_comments.retain(|c| c.status == status_enum);
        }
    }
    
    if let Some(user_id_val) = &user_id {
        filtered_comments.retain(|c| c.user_id == *user_id_val);
    }

    let total = filtered_comments.len();
    let paginated_comments = filtered_comments.into_iter()
        .skip(offset)
        .take(size)
        .collect::<Vec<_>>();

    if !paginated_comments.is_empty() {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取评论列表成功".to_string(),
            data: Some(serde_json::json!({
                "comments": paginated_comments,
                "total": total,
                "page": page,
                "size": size
            })),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "暂无评论数据".to_string(),
            data: Some(serde_json::json!({
                "comments": Vec::<crate::models::Comment>::new(),
                "total": 0,
                "page": page,
                "size": size
            })),
        })
    }
}

// 创建评论
#[post("/api/comments")]
pub async fn create_comment(
    req: actix_web::HttpRequest,
    comment_data: web::Json<CreateCommentRequest>,
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

    let mut comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    let new_id = comments.len() as u32 + 1;
    
    let new_comment = crate::models::Comment {
        id: new_id,
        user_id: username.clone(),
        target_type: comment_data.target_type.clone(),
        target_id: comment_data.target_id,
        content: comment_data.content.clone(),
        create_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        update_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        likes: 0,
        dislikes: 0,
        parent_id: comment_data.parent_id,
        status: crate::models::CommentStatus::Active,
        replies: Vec::new(),
    };

    // 如果有父评论，添加到父评论的回复中
    if let Some(parent_id) = comment_data.parent_id {
        if let Some(parent_comment) = comments.iter_mut().find(|c| c.id == parent_id) {
            parent_comment.replies.push(new_comment.clone());
        }
    } else {
        comments.push(new_comment.clone());
    }

    match data.data_manager.save_comments(&comments) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse {
                code: 200,
                msg: "评论创建成功".to_string(),
                data: Some(new_comment),
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "保存评论失败".to_string(),
                data: None,
            })
        }
    }
}

// 获取单个评论
#[get("/api/comments/{id}")]
pub async fn get_comment(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let comment_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的评论ID".to_string(),
            data: None,
        }),
    };

    let comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    
    if let Some(comment) = comments.iter().find(|c| c.id == comment_id) {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "获取评论成功".to_string(),
            data: Some(comment.clone()),
        })
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "评论不存在".to_string(),
            data: None,
        })
    }
}

// 删除评论
#[delete("/api/comments/{id}")]
pub async fn delete_comment(
    path: web::Path<String>,
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let comment_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的评论ID".to_string(),
            data: None,
        }),
    };

    let username = match req.headers().get("X-Username") {
        Some(name) => name.to_str().unwrap_or("").to_string(),
        None => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 401,
            msg: "未授权访问".to_string(),
            data: None,
        }),
    };

    let mut comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    
    // 检查权限（只能删除自己的评论或管理员）
    if let Some(comment) = comments.iter().find(|c| c.id == comment_id) {
        if comment.user_id != username {
            return HttpResponse::Forbidden().json(ApiResponse::<()> {
                code: 403,
                msg: "无权限删除此评论".to_string(),
                data: None,
            });
        }
    }

    comments.retain(|c| c.id != comment_id);

    match data.data_manager.save_comments(&comments) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                msg: "评论删除成功".to_string(),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                msg: "删除评论失败".to_string(),
                data: None,
            })
        }
    }
}

// 更新评论状态
#[put("/api/comments/{id}/status")]
pub async fn update_comment_status(
    path: web::Path<String>,
    status_data: web::Json<serde_json::Value>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let comment_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的评论ID".to_string(),
            data: None,
        }),
    };

    let status_str = status_data.get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let status = match status_str {
        "active" => crate::models::CommentStatus::Active,
        "hidden" => crate::models::CommentStatus::Hidden,
        "deleted" => crate::models::CommentStatus::Deleted,
        _ => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的状态值".to_string(),
            data: None,
        }),
    };

    let mut comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    
    if let Some(comment) = comments.iter_mut().find(|c| c.id == comment_id) {
        comment.status = status;
        comment.update_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match data.data_manager.save_comments(&comments) {
            Ok(_) => {
                HttpResponse::Ok().json(ApiResponse::<()> {
                    code: 200,
                    msg: "评论状态更新成功".to_string(),
                    data: None,
                })
            }
            Err(_) => {
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "更新评论状态失败".to_string(),
                    data: None,
                })
            }
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "评论不存在".to_string(),
            data: None,
        })
    }
}

// 更新评论内容
#[put("/api/comments/{id}")]
pub async fn update_comment(
    path: web::Path<String>,
    comment_data: web::Json<UpdateCommentRequest>,
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let comment_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的评论ID".to_string(),
            data: None,
        }),
    };

    let username = match req.headers().get("X-Username") {
        Some(name) => name.to_str().unwrap_or("").to_string(),
        None => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 401,
            msg: "未授权访问".to_string(),
            data: None,
        }),
    };

    let mut comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    
    if let Some(comment) = comments.iter_mut().find(|c| c.id == comment_id) {
        // 检查权限
        if comment.user_id != username {
            return HttpResponse::Forbidden().json(ApiResponse::<()> {
                code: 403,
                msg: "无权限修改此评论".to_string(),
                data: None,
            });
        }

        comment.content = comment_data.content.clone();
        comment.update_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match data.data_manager.save_comments(&comments) {
            Ok(_) => {
                HttpResponse::Ok().json(ApiResponse::<()> {
                    code: 200,
                    msg: "评论更新成功".to_string(),
                    data: None,
                })
            }
            Err(_) => {
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "更新评论失败".to_string(),
                    data: None,
                })
            }
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "评论不存在".to_string(),
            data: None,
        })
    }
}

// 回复评论
#[post("/api/comments/{id}/reply")]
pub async fn reply_comment(
    path: web::Path<String>,
    reply_data: web::Json<CreateCommentRequest>,
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let parent_id = match path.parse::<u32>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            code: 400,
            msg: "无效的父评论ID".to_string(),
            data: None,
        }),
    };

    let username = match req.headers().get("X-Username") {
        Some(name) => name.to_str().unwrap_or("").to_string(),
        None => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 401,
            msg: "未授权访问".to_string(),
            data: None,
        }),
    };

    let mut comments = match data.data_manager.load_comments() {
        Ok(comments) => comments,
        Err(_) => Vec::new(),
    };
    
    // 检查父评论是否存在
    let parent_comment_index = comments.iter().position(|c| c.id == parent_id);
    if let Some(index) = parent_comment_index {
        let new_id = comments.len() as u32 + 1;
        
        let reply_comment = crate::models::Comment {
            id: new_id,
            user_id: username.clone(),
            target_type: reply_data.target_type.clone(),
            target_id: reply_data.target_id,
            content: reply_data.content.clone(),
            create_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            likes: 0,
            dislikes: 0,
            parent_id: Some(parent_id),
            status: crate::models::CommentStatus::Active,
            replies: Vec::new(),
        };

        comments[index].replies.push(reply_comment.clone());

        match data.data_manager.save_comments(&comments) {
            Ok(_) => {
                HttpResponse::Ok().json(ApiResponse {
                    code: 200,
                    msg: "回复创建成功".to_string(),
                    data: Some(reply_comment),
                })
            }
            Err(_) => {
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    code: 500,
                    msg: "保存回复失败".to_string(),
                    data: None,
                })
            }
        }
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()> {
            code: 404,
            msg: "父评论不存在".to_string(),
            data: None,
        })
    }
} 