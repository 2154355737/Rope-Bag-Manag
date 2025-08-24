use actix_web::{web, HttpResponse, Result};
use tracing::instrument;
use serde_json::json;

use crate::core::AppState;
use crate::shared::errors::ApiResponse;

#[instrument(skip(app_state))]
pub async fn health_check(app_state: web::Data<AppState>) -> Result<HttpResponse> {
    match app_state.health_check().await {
        Ok(_) => {
            let response = ApiResponse::success(json!({
                "status": "healthy",
                "version": "2.0.0",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "services": {
                    "database": "ok",
                    "user_service": "ok",
                    "auth_service": "ok"
                }
            }));
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()>::error(500, format!("健康检查失败: {}", e));
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

#[instrument]
pub async fn version() -> Result<HttpResponse> {
    let response = ApiResponse::success(json!({
        "name": "绳包管理器后端",
        "version": "2.0.0",
        "arch": std::env::consts::ARCH,
        "os": std::env::consts::OS
    }));
    Ok(HttpResponse::Ok().json(response))
} 