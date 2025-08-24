use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::infrastructure::database::repositories::{MailSettingsRepository, MailSettings, UserRepository};
use crate::shared::utils::jwt as jwt_util;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/mail-settings", web::get().to(get_mail_settings))
            .route("/mail-settings", web::put().to(update_mail_settings))
    );
}

#[derive(Serialize, Deserialize)]
struct MailSettingsDTO {
    smtp_host: Option<String>,
    smtp_port: Option<i64>,
    username: Option<String>,
    password: Option<String>,
    from_address: Option<String>,
    from_name: Option<String>,
    enabled: Option<bool>,
}

async fn get_mail_settings(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let _admin_id = require_admin(&req, &app).await?;
    let repo = MailSettingsRepository::new(app.db.pool());
    let row = repo.get().await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let dto = if let Some(s) = row {
        MailSettingsDTO {
            smtp_host: s.smtp_host,
            smtp_port: s.smtp_port,
            username: s.username,
            password: None, // 不回传密码
            from_address: s.from_address,
            from_name: s.from_name,
            enabled: Some(s.enabled != 0),
        }
    } else {
        // 若DB无记录，返回默认配置占位（来自文件配置）
        let c = &app.config.email;
        MailSettingsDTO {
            smtp_host: Some(c.smtp_host.clone()),
            smtp_port: Some(c.smtp_port as i64),
            username: Some(c.smtp_username.clone()),
            password: None,
            from_address: Some(c.from_address.clone()),
            from_name: Some(c.from_name.clone()),
            enabled: Some(false),
        }
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": dto})))
}

async fn update_mail_settings(app: web::Data<crate::core::AppState>, req: HttpRequest, payload: web::Json<MailSettingsDTO>) -> Result<HttpResponse, actix_web::Error> {
    let _admin_id = require_admin(&req, &app).await?;
    let repo = MailSettingsRepository::new(app.db.pool());
    let p = payload.into_inner();
    let _ = repo.upsert(
        p.smtp_host.as_deref(), p.smtp_port, p.username.as_deref(), p.password.as_deref(),
        p.from_address.as_deref(), p.from_name.as_deref(), p.enabled
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"保存成功"})))
}

async fn require_admin(req: &HttpRequest, app: &crate::core::AppState) -> Result<i64, actix_web::Error> {
    let uid = if let Some(hv) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = hv.to_str() { if let Some(t) = s.strip_prefix("Bearer ") { jwt_util::verify(t, &app.config.jwt) } else { None } } else { None }
    } else { None };
    let uid = uid.ok_or_else(|| actix_web::error::ErrorUnauthorized("未登录"))?;
    let ur = UserRepository::new(app.db.pool());
    let u = ur.find_by_id(uid).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    if let Some(user) = u { if user.can_access_admin() { return Ok(uid); } }
    Err(actix_web::error::ErrorForbidden("无权访问"))
} 