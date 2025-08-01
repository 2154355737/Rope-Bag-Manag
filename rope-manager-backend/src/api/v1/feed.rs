use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::services::{package_service::PackageService, post_service::PostService};
// use crate::models::*;  // 移除未使用的导入

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/community/feed")
            .route(web::get().to(get_feed))
    );
}

#[derive(Debug, Deserialize)]
pub struct FeedQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category_id: Option<i32>,
    pub search: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize)]
#[derive(Clone)]
pub struct FeedItem {
    pub id: i32,
    pub item_type: String, // "post" | "package"
    pub title: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i32>,
    pub created_at: String,
    pub tags: Vec<String>,
}

async fn get_feed(
    package_service: web::Data<PackageService>,
    post_service: web::Data<PostService>,
    query: web::Query<FeedQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    // 获取资源（已上架）
    let (packages, _) = match package_service.get_packages_advanced(
        page,
        page_size,
        query.category_id,
        query.search.clone(),
        Some("active".to_string()),
    ).await {
        Ok(res) => res,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取资源失败: {}", e)
            })));
        }
    };

    // 获取帖子（已发布）
    let post_query = crate::models::PostQueryParams {
        page: Some(page),
        page_size: Some(page_size),
        category_id: query.category_id,
        author_id: None,
        status: Some("Published".to_string()),
        search: query.search.clone(),
        tags: query.tag.as_ref().map(|t| vec![t.clone()]),
        is_pinned: None,
        is_featured: None,
    };
    let posts_resp = post_service.get_posts(post_query).await;
    let posts = match posts_resp {
        Ok(resp) => resp.list,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取帖子失败: {}", e)
            })));
        }
    };

    // 合并
    let mut items: Vec<FeedItem> = Vec::new();
    for p in packages {
        items.push(FeedItem {
            id: p.id,
            item_type: "package".to_string(),
            title: p.name,
            author: p.author,
            download_count: Some(p.download_count),
            view_count: None,
            created_at: p.created_at.to_rfc3339(),
            tags: p.tags.unwrap_or_default(),
        });
    }
    for post in posts {
        items.push(FeedItem {
            id: post.id,
            item_type: "post".to_string(),
            title: post.title,
            author: post.author_name.unwrap_or_else(|| "匿名".to_string()),
            download_count: None,
            view_count: Some(post.view_count),
            created_at: post.created_at.to_rfc3339(),
            tags: vec![], // TODO: 获取帖子标签
        });
    }

    // 按时间降序
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // 分页
    let total = items.len() as i64;
    let start = ((page - 1) * page_size) as usize;
    let end = std::cmp::min(start + page_size as usize, items.len());
    let paged_items = if start < items.len() { items[start..end].to_vec() } else { vec![] };

    Ok(HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "list": paged_items,
            "total": total,
            "page": page,
            "page_size": page_size
        }
    })))
} 