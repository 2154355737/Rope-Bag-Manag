use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use crate::services::{post_service::PostService, package_service::PackageService};
use crate::models::TagQueryParams;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/search")
			.route("", web::get().to(search))
			.route("/trending", web::get().to(trending))
			.route("/suggest", web::get().to(suggest))
	);
}

#[derive(Deserialize)]
struct SearchQuery { query: Option<String>, r#type: Option<String>, page: Option<u32>, pageSize: Option<u32> }

async fn search(
	query: web::Query<SearchQuery>,
	post_service: web::Data<PostService>,
	package_service: web::Data<PackageService>,
) -> Result<HttpResponse, actix_web::Error> {
	let q = query.query.clone().unwrap_or_default();
	let page = query.page.unwrap_or(1);
	let page_size = query.pageSize.unwrap_or(10);
	let t = query.r#type.clone().unwrap_or_default().to_lowercase();

	let mut items: Vec<serde_json::Value> = Vec::new();

	if t.is_empty() || t == "post" {
		let mut params = crate::models::PostQueryParams { page: Some(page), page_size: Some(page_size), category_id: None, author_id: None, status: Some("Published".into()), search: None, tags: None, is_pinned: None, is_featured: None };
		if !q.is_empty() { params.search = Some(q.clone()); }
		if let Ok(resp) = post_service.get_posts(params).await {
			for p in resp.list {
				items.push(json!({
					"id": p.id,
					"type": "post",
					"title": p.title,
					"description": p.content,
					"author": {"id": p.author_id, "name": p.author_name },
					"stats": {"likes": p.like_count, "comments": p.comment_count, "views": p.view_count},
					"publishedAt": p.created_at.to_rfc3339()
				}));
			}
		}
	}

	if t.is_empty() || t == "resource" || t == "package" {
		if let Ok((packages, _total)) = package_service.get_packages_advanced(page, page_size, None, if q.is_empty(){None}else{Some(q.clone())}, Some("active".into())).await {
			for p in packages {
				items.push(json!({
					"id": p.id,
					"type": "resource",
					"title": p.name,
					"description": p.description,
					"author": {"name": p.author},
					"stats": {"downloads": p.download_count, "likes": p.like_count},
					"publishedAt": p.created_at.to_rfc3339()
				}));
			}
		}
	}

	// 简单分页：items 已按两源合并，直接裁剪
	let total = items.len() as u32;
	let start = ((page - 1) * page_size) as usize;
	let end = std::cmp::min(start + page_size as usize, items.len());
	let slice = if start < items.len() { &items[start..end] } else { &[] };

	Ok(HttpResponse::Ok().json(json!({
		"code": 0,
		"message": "success",
		"data": {
			"items": slice,
			"total": total,
			"hasMore": (end as u32) < total
		}
	})))
}

async fn trending(tag_service: web::Data<crate::services::tag_service::TagService>) -> Result<HttpResponse, actix_web::Error> {
	match tag_service.get_popular_tags(Some(10)).await {
		Ok(tags) => {
			let keywords: Vec<String> = tags.into_iter().map(|t| t.name).collect();
			Ok(HttpResponse::Ok().json(json!({"code":0, "data": {"keywords": keywords}})))
		}
		Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
	}
}

#[derive(Deserialize)]
struct SuggestQuery { query: Option<String> }

async fn suggest(query: web::Query<SuggestQuery>, tag_service: web::Data<crate::services::tag_service::TagService>) -> Result<HttpResponse, actix_web::Error> {
	let q = query.query.clone().unwrap_or_default();
	let params = TagQueryParams { page: Some(1), page_size: Some(10), search: if q.is_empty(){None}else{Some(q)}, sort_by: None, sort_order: None };
	match tag_service.get_tags(params).await {
		Ok(resp) => {
			let suggestions: Vec<String> = resp.list.into_iter().map(|t| t.name).collect();
			Ok(HttpResponse::Ok().json(json!({"code":0, "data": {"suggestions": suggestions}})))
		}
		Err(e) => Ok(HttpResponse::InternalServerError().json(json!({"code":500, "message": e.to_string()})))
	}
} 