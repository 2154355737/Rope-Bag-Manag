use actix_web::{web, HttpRequest, HttpResponse};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use crate::infrastructure::storage::LocalStorage;
use crate::shared::utils::jwt as jwt_util;
use serde::Deserialize;
use std::path::PathBuf;
use crate::infrastructure::database::repositories::StorageFileRepository;
use mime_guess::MimeGuess;
use actix_web::http::header::CONTENT_TYPE;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/storage")
            .route("/verify", web::post().to(verify))
            .route("/upload", web::post().to(upload))
            .route("/file/{path:.*}", web::get().to(get_file))
    );
}

#[derive(Deserialize)]
struct VerifyReq { filename: String, size: Option<i64>, sha256: Option<String> }

async fn verify(app: web::Data<crate::core::AppState>, req: HttpRequest, payload: web::Json<VerifyReq>) -> Result<HttpResponse, actix_web::Error> {
    let _uid = extract_uid(&req, &app)?;
    // 本地存储：预占位策略暂不实现，仅返回可以上传；sha256可用于快速秒传
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"canUpload": true}})))
}

async fn upload(app: web::Data<crate::core::AppState>, req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let uid = extract_uid(&req, &app)?;
    let base = PathBuf::from(&app.config.storage.upload_path);
    LocalStorage::ensure_base_dir(&base).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let target_root = LocalStorage::partitioned_dir(&base);

    let repo = StorageFileRepository::new(app.db.pool());
    let mut stored = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
        let cd = field.content_disposition().cloned();
        let mut bytes: Vec<u8> = Vec::new();
        while let Some(chunk) = field.next().await { let data = chunk.map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?; bytes.extend_from_slice(&data); }
        let original = cd.and_then(|c| c.get_filename().map(|s| s.to_string())).unwrap_or_else(|| "file".into());
        let sha256 = LocalStorage::sha256_hex(&bytes);
        if let Some(hit) = repo.find_by_sha256(&sha256).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))? {
            stored.push(serde_json::json!({
                "filename": original,
                "sha256": sha256,
                "path": format!("/api/v1/storage/file/{}", hit.relative_path.trim_start_matches('/')),
                "size": hit.size,
                "dedup": true
            }));
            continue;
        }
        // 相对路径字符串：YYYY/MM/DD/<sha8>/<safe_name>
        let safe_name = LocalStorage::sanitize_filename(&original);
        let hash_dir = LocalStorage::hash_path_component(&bytes);
        let date_dir = target_root.strip_prefix(&base).unwrap_or(target_root.as_path()).to_string_lossy().replace('\\', "/");
        let rel = format!("{}/{}/{}", date_dir.trim_start_matches('/'), hash_dir, safe_name);
        let rel_trim = rel.trim_start_matches('/').to_string();
        // 使用 OpenDAL 写入
        app.storage.write(&rel_trim, bytes.clone()).await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        let size = bytes.len() as i64;
        let mime = MimeGuess::from_path(&original).first_raw();
        let ext = std::path::Path::new(&original).extension().and_then(|e| e.to_str());
        let _ = repo.insert(Some(uid), &sha256, size, mime, Some(&original), ext, &rel_trim).await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        stored.push(serde_json::json!({
            "filename": original,
            "sha256": sha256,
            "path": format!("/api/v1/storage/file/{}", rel_trim),
            "size": size,
            "dedup": false
        }));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": stored})))
}

fn extract_uid(req: &HttpRequest, app: &crate::core::AppState) -> Result<i64, actix_web::Error> {
    if let Some(h) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = h.to_str() {
            if let Some(t) = s.strip_prefix("Bearer ") {
                if let Some(uid) = jwt_util::verify(t, &app.config.jwt) { return Ok(uid); }
            }
        }
    }
    Err(actix_web::error::ErrorUnauthorized("未登录"))
}

async fn get_file(app: web::Data<crate::core::AppState>, path: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    let rel = path.into_inner();
    let data = app.storage.read(&rel).await
        .map_err(|e| actix_web::error::ErrorNotFound(e.to_string()))?;
    let mime = MimeGuess::from_path(&rel).first_or_octet_stream();
    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, mime.as_ref()))
        .body(data))
} 