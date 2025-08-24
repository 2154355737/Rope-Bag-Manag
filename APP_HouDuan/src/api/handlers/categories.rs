use actix_web::{web, HttpResponse};
use crate::infrastructure::database::repositories::CategoryRepository;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::get().to(get_categories))
    );
}

async fn get_categories(app: web::Data<crate::core::AppState>) -> Result<HttpResponse, actix_web::Error> {
    let repo = CategoryRepository::new(app.db.pool());
    let cats = repo.list_all().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let count_map = repo.counts_map().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let list: Vec<serde_json::Value> = cats.into_iter().map(|c| {
        let cnt = count_map.get(&c.id).cloned().unwrap_or(0);
        serde_json::json!({
            "id": c.id,
            "name": c.name,
            "description": c.description,
            "enabled": c.enabled == 1,
            "icon": c.icon,
            "sort_order": c.sort_order,
            "created_at": c.created_at,
            "updated_at": c.updated_at,
            "count": cnt
        })
    }).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "success",
        "data": {"list": list}
    })))
} 