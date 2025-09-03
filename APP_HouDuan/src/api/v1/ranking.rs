use actix_web::{web, HttpResponse, Result};
use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::repositories::user_repo::UserRepository;
use crate::repositories::package_repo::PackageRepository;
use crate::repositories::post_repo::PostRepository;
use std::sync::Arc;

/// 构建完整的头像URL
fn build_avatar_url(avatar_url: Option<String>) -> Option<String> {
    avatar_url.map(|url| {
        if url.starts_with("http://") || url.starts_with("https://") {
            url // 已经是完整URL
        } else if url.starts_with("/") {
            format!("http://localhost:15201{}", url) // 相对路径转换为完整URL
        } else {
            format!("http://localhost:15201/{}", url) // 添加前缀斜杠
        }
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RankingQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub period: Option<String>, // "week", "month", "year", "all"
}

#[derive(Debug, Serialize)]
pub struct UserRankingItem {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub score: i64,
    pub posts_count: i64,
    pub resources_count: i64,
    pub followers_count: i64,
    pub likes_count: i64,
    pub total_views: i64,
    pub total_downloads: i64,
}

#[derive(Debug, Serialize)]
pub struct ResourceRankingItem {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub author: AuthorInfo,
    pub downloads: i64,
    pub likes: i64,
    pub rating: f64,
    pub category: String,
    pub created_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PostRankingItem {
    pub id: i64,
    pub title: String,
    pub content_preview: Option<String>,
    pub author: AuthorInfo,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    pub created_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthorInfo {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RankingResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: i64,
}

/// 获取用户排行榜
pub async fn get_user_ranking(
    query: web::Query<RankingQuery>,
    user_repo: web::Data<Arc<UserRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100); // 限制最大页面大小
    let offset = (page - 1) * page_size;

    info!("获取用户排行榜: page={}, page_size={}", page, page_size);

    match user_repo.get_user_ranking(offset as i64, page_size as i64).await {
        Ok((users, total)) => {
            let items: Vec<UserRankingItem> = users.into_iter().map(|user| {
                UserRankingItem {
                    id: user.id as i64,
                    username: user.username,
                    nickname: user.nickname,
                    avatar_url: build_avatar_url(user.avatar_url),
                    score: user.star as i64,
                    posts_count: 0, // TODO: 从统计表获取
                    resources_count: 0, // TODO: 从统计表获取
                    followers_count: 0, // TODO: 从关注表获取
                    likes_count: 0, // TODO: 从点赞统计获取
                    total_views: 0, // TODO: 从浏览统计获取
                    total_downloads: user.download_count as i64,
                }
            }).collect();

            let total_pages = (total as f64 / page_size as f64).ceil() as i64;

            Ok(HttpResponse::Ok().json(RankingResponse {
                items,
                total: total as i64,
                page,
                page_size,
                total_pages,
            }))
        }
        Err(e) => {
            error!("获取用户排行榜失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "获取用户排行榜失败"
            })))
        }
    }
}

/// 获取资源排行榜
pub async fn get_resource_ranking(
    query: web::Query<RankingQuery>,
    package_repo: web::Data<Arc<PackageRepository>>,
    user_repo: web::Data<Arc<UserRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    info!("获取资源排行榜: page={}, page_size={}", page, page_size);

    match package_repo.get_package_ranking(offset as i64, page_size as i64).await {
        Ok((packages, total)) => {
            let mut items = Vec::new();

            for package in packages {
                // 暂时使用简化的作者信息，避免数据库查询问题
                let author = AuthorInfo {
                    id: 1, // 暂时固定ID
                    username: package.author.clone(),
                    nickname: Some(package.author.clone()),
                    avatar_url: None,
                };

                // 获取分类名称
                let category_name = package.category_id
                    .and_then(|id| {
                        // TODO: 从category表获取分类名称
                        Some(format!("分类{}", id))
                    })
                    .unwrap_or_else(|| "未分类".to_string());

                items.push(ResourceRankingItem {
                    id: package.id as i64,
                    title: package.name,
                    description: package.description,
                    author,
                    downloads: package.download_count as i64,
                    likes: package.like_count as i64,
                    rating: 4.5, // TODO: 从评分表获取
                    category: category_name,
                    created_at: package.created_at.to_rfc3339(),
                    tags: package.tags.unwrap_or_default(),
                });
            }

            let total_pages = (total as f64 / page_size as f64).ceil() as i64;

            Ok(HttpResponse::Ok().json(RankingResponse {
                items,
                total: total as i64,
                page,
                page_size,
                total_pages,
            }))
        }
        Err(e) => {
            error!("获取资源排行榜失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "获取资源排行榜失败"
            })))
        }
    }
}

/// 获取帖子排行榜
pub async fn get_post_ranking(
    query: web::Query<RankingQuery>,
    post_repo: web::Data<Arc<PostRepository>>,
    user_repo: web::Data<Arc<UserRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);

    info!("获取帖子排行榜: page={}, page_size={}", page, page_size);

    match post_repo.get_post_ranking(page, page_size) {
        Ok((posts, total)) => {
            let mut items = Vec::new();

            for post in posts {
                // 获取作者信息
                let author = match user_repo.find_by_id(post.author_id).await {
                    Ok(Some(user)) => AuthorInfo {
                        id: user.id as i64,
                        username: user.username,
                        nickname: user.nickname,
                        avatar_url: build_avatar_url(user.avatar_url),
                    },
                    _ => {
                        // 如果找不到用户，使用post中的author_name作为fallback
                        AuthorInfo {
                            id: post.author_id as i64,
                            username: post.author_name.clone().unwrap_or_else(|| format!("user_{}", post.author_id)),
                            nickname: post.author_name.clone(),
                            avatar_url: Some(format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", post.author_id)),
                        }
                    }
                };

                // 生成内容预览（前100个字符，安全处理UTF-8）
                let content_preview = if post.content.chars().count() > 100 {
                    let truncated: String = post.content.chars().take(100).collect();
                    Some(format!("{}...", truncated))
                } else {
                    Some(post.content.clone())
                };

                items.push(PostRankingItem {
                    id: post.id as i64,
                    title: post.title,
                    content_preview,
                    author,
                    views: post.view_count as i64,
                    likes: post.like_count as i64,
                    comments: post.comment_count as i64,
                    created_at: post.created_at.to_rfc3339(),
                    tags: post.tags.unwrap_or_default(),
                });
            }

            let total_pages = (total as f64 / page_size as f64).ceil() as i64;

            Ok(HttpResponse::Ok().json(RankingResponse {
                items,
                total,
                page,
                page_size,
                total_pages,
            }))
        }
        Err(e) => {
            error!("获取帖子排行榜失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "获取帖子排行榜失败"
            })))
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ranking")
            .route("/users", web::get().to(get_user_ranking))
            .route("/resources", web::get().to(get_resource_ranking))
            .route("/posts", web::get().to(get_post_ranking))
    );
} 