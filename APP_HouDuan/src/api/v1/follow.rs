use actix_web::{web, HttpRequest, HttpResponse, Result};
use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::repositories::follow_repo::FollowRepository;
use crate::repositories::user_repo::UserRepository;
use crate::utils::jwt::JwtUtils;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowRequest {
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct FollowResponse {
    pub success: bool,
    pub is_following: bool,
    pub followers_count: i32,
    pub following_count: i32,
}

#[derive(Debug, Serialize)]
pub struct FollowUser {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub followed_at: String,
}

#[derive(Debug, Serialize)]
pub struct FollowListResponse {
    pub items: Vec<FollowUser>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct FollowQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// 关注用户
pub async fn follow_user(
    path: web::Path<i32>,
    req: HttpRequest,
    follow_repo: web::Data<Arc<FollowRepository>>,
    user_repo: web::Data<Arc<UserRepository>>,
    jwt_utils: web::Data<Arc<JwtUtils>>,
) -> Result<HttpResponse> {
    let followed_id = path.into_inner();
    
    // 验证JWT token
    let token = match req.headers().get("authorization") {
        Some(header) => match header.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    &auth_str[7..]
                } else {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid authorization header format"
                    })));
                }
            }
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid authorization header"
                })));
            }
        }
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing authorization header"
            })));
        }
    };

    let follower_id = match jwt_utils.verify_token(token) {
        Ok(claims) => claims.user_id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid token"
            })));
        }
    };

    // 检查被关注用户是否存在
    match user_repo.find_by_id(followed_id).await {
        Ok(Some(_)) => {},
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            })));
        }
        Err(e) => {
            error!("Database error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            })));
        }
    }

    // 不能关注自己
    if follower_id == followed_id {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Cannot follow yourself"
        })));
    }

    match follow_repo.follow_user(follower_id, followed_id).await {
        Ok(success) => {
            // 获取更新后的统计信息
            let followed_stats = follow_repo.get_follow_stats(followed_id).await.unwrap_or_default();
            let follower_stats = follow_repo.get_follow_stats(follower_id).await.unwrap_or_default();
            
            info!("User {} followed user {}", follower_id, followed_id);
            
            Ok(HttpResponse::Ok().json(FollowResponse {
                success,
                is_following: true,
                followers_count: followed_stats.followers_count,
                following_count: follower_stats.following_count,
            }))
        }
        Err(e) => {
            error!("Failed to follow user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to follow user"
            })))
        }
    }
}

/// 取消关注用户
pub async fn unfollow_user(
    path: web::Path<i32>,
    req: HttpRequest,
    follow_repo: web::Data<Arc<FollowRepository>>,
    jwt_utils: web::Data<Arc<JwtUtils>>,
) -> Result<HttpResponse> {
    let followed_id = path.into_inner();
    
    // 验证JWT token
    let token = match req.headers().get("authorization") {
        Some(header) => match header.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    &auth_str[7..]
                } else {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid authorization header format"
                    })));
                }
            }
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid authorization header"
                })));
            }
        }
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing authorization header"
            })));
        }
    };

    let follower_id = match jwt_utils.verify_token(token) {
        Ok(claims) => claims.user_id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid token"
            })));
        }
    };

    match follow_repo.unfollow_user(follower_id, followed_id).await {
        Ok(success) => {
            // 获取更新后的统计信息
            let followed_stats = follow_repo.get_follow_stats(followed_id).await.unwrap_or_default();
            let follower_stats = follow_repo.get_follow_stats(follower_id).await.unwrap_or_default();
            
            info!("User {} unfollowed user {}", follower_id, followed_id);
            
            Ok(HttpResponse::Ok().json(FollowResponse {
                success,
                is_following: false,
                followers_count: followed_stats.followers_count,
                following_count: follower_stats.following_count,
            }))
        }
        Err(e) => {
            error!("Failed to unfollow user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to unfollow user"
            })))
        }
    }
}

/// 检查关注状态
pub async fn check_follow_status(
    path: web::Path<i32>,
    req: HttpRequest,
    follow_repo: web::Data<Arc<FollowRepository>>,
    jwt_utils: web::Data<Arc<JwtUtils>>,
) -> Result<HttpResponse> {
    let followed_id = path.into_inner();
    
    // 验证JWT token
    let token = match req.headers().get("authorization") {
        Some(header) => match header.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    &auth_str[7..]
                } else {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid authorization header format"
                    })));
                }
            }
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid authorization header"
                })));
            }
        }
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing authorization header"
            })));
        }
    };

    let follower_id = match jwt_utils.verify_token(token) {
        Ok(claims) => claims.user_id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid token"
            })));
        }
    };

    match follow_repo.is_following(follower_id, followed_id).await {
        Ok(is_following) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "is_following": is_following
            })))
        }
        Err(e) => {
            error!("Failed to check follow status: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check follow status"
            })))
        }
    }
}

/// 获取用户的关注者列表
pub async fn get_user_followers(
    path: web::Path<i32>,
    query: web::Query<FollowQuery>,
    follow_repo: web::Data<Arc<FollowRepository>>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    match follow_repo.get_followers(user_id, offset as i64, page_size as i64).await {
        Ok((followers, total)) => {
            let items: Vec<FollowUser> = followers.into_iter().map(|f| FollowUser {
                id: f.id,
                username: f.username,
                nickname: f.nickname,
                avatar_url: f.avatar_url,
                bio: f.bio,
                followed_at: f.followed_at.to_rfc3339(),
            }).collect();

            let total_pages = (total as f64 / page_size as f64).ceil() as u32;
            let has_more = page < total_pages;

            Ok(HttpResponse::Ok().json(FollowListResponse {
                items,
                total,
                page,
                page_size,
                has_more,
            }))
        }
        Err(e) => {
            error!("Failed to get followers: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get followers"
            })))
        }
    }
}

/// 获取用户的关注列表
pub async fn get_user_following(
    path: web::Path<i32>,
    query: web::Query<FollowQuery>,
    follow_repo: web::Data<Arc<FollowRepository>>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    match follow_repo.get_following(user_id, offset as i64, page_size as i64).await {
        Ok((following, total)) => {
            let items: Vec<FollowUser> = following.into_iter().map(|f| FollowUser {
                id: f.id,
                username: f.username,
                nickname: f.nickname,
                avatar_url: f.avatar_url,
                bio: f.bio,
                followed_at: f.followed_at.to_rfc3339(),
            }).collect();

            let total_pages = (total as f64 / page_size as f64).ceil() as u32;
            let has_more = page < total_pages;

            Ok(HttpResponse::Ok().json(FollowListResponse {
                items,
                total,
                page,
                page_size,
                has_more,
            }))
        }
        Err(e) => {
            error!("Failed to get following: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get following"
            })))
        }
    }
}

/// 获取我的关注者列表
pub async fn get_my_followers(
    query: web::Query<FollowQuery>,
    req: HttpRequest,
    follow_repo: web::Data<Arc<FollowRepository>>,
    jwt_utils: web::Data<Arc<JwtUtils>>,
) -> Result<HttpResponse> {
    // 验证JWT token
    let token = match req.headers().get("authorization") {
        Some(header) => match header.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    &auth_str[7..]
                } else {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid authorization header format"
                    })));
                }
            }
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid authorization header"
                })));
            }
        }
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing authorization header"
            })));
        }
    };

    let user_id = match jwt_utils.verify_token(token) {
        Ok(claims) => claims.user_id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid token"
            })));
        }
    };

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    match follow_repo.get_followers(user_id, offset as i64, page_size as i64).await {
        Ok((followers, total)) => {
            let items: Vec<FollowUser> = followers.into_iter().map(|f| FollowUser {
                id: f.id,
                username: f.username,
                nickname: f.nickname,
                avatar_url: f.avatar_url,
                bio: f.bio,
                followed_at: f.followed_at.to_rfc3339(),
            }).collect();

            let total_pages = (total as f64 / page_size as f64).ceil() as u32;
            let has_more = page < total_pages;

            Ok(HttpResponse::Ok().json(FollowListResponse {
                items,
                total,
                page,
                page_size,
                has_more,
            }))
        }
        Err(e) => {
            error!("Failed to get my followers: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get my followers"
            })))
        }
    }
}

/// 获取我的关注列表
pub async fn get_my_following(
    query: web::Query<FollowQuery>,
    req: HttpRequest,
    follow_repo: web::Data<Arc<FollowRepository>>,
    jwt_utils: web::Data<Arc<JwtUtils>>,
) -> Result<HttpResponse> {
    // 验证JWT token
    let token = match req.headers().get("authorization") {
        Some(header) => match header.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    &auth_str[7..]
                } else {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid authorization header format"
                    })));
                }
            }
            Err(_) => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid authorization header"
                })));
            }
        }
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing authorization header"
            })));
        }
    };

    let user_id = match jwt_utils.verify_token(token) {
        Ok(claims) => claims.user_id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid token"
            })));
        }
    };

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    match follow_repo.get_following(user_id, offset as i64, page_size as i64).await {
        Ok((following, total)) => {
            let items: Vec<FollowUser> = following.into_iter().map(|f| FollowUser {
                id: f.id,
                username: f.username,
                nickname: f.nickname,
                avatar_url: f.avatar_url,
                bio: f.bio,
                followed_at: f.followed_at.to_rfc3339(),
            }).collect();

            let total_pages = (total as f64 / page_size as f64).ceil() as u32;
            let has_more = page < total_pages;

            Ok(HttpResponse::Ok().json(FollowListResponse {
                items,
                total,
                page,
                page_size,
                has_more,
            }))
        }
        Err(e) => {
            error!("Failed to get my following: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get my following"
            })))
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{user_id}/follow", web::post().to(follow_user))
            .route("/{user_id}/follow", web::delete().to(unfollow_user))
            .route("/{user_id}/follow-status", web::get().to(check_follow_status))
            .route("/{user_id}/followers", web::get().to(get_user_followers))
            .route("/{user_id}/following", web::get().to(get_user_following))
    )
    .service(
        web::scope("/me")
            .route("/followers", web::get().to(get_my_followers))
            .route("/following", web::get().to(get_my_following))
    );
} 