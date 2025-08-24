use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/search")
            .route("/trending", web::get().to(trending))
    );
}

async fn trending(app: web::Data<crate::core::AppState>) -> Result<HttpResponse, actix_web::Error> {
    let rows = sqlx::query_as::<_, (String,)>("SELECT name FROM tags ORDER BY usage_count DESC LIMIT 10")
        .fetch_all(app.db.pool())
        .await
        .unwrap_or_default();
    let keywords: Vec<String> = rows.into_iter().map(|t| t.0).collect();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "data": {"keywords": keywords}
    })))
} 