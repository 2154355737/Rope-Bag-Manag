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
use sqlx::Row;

pub fn configure(cfg: &mut web::ServiceConfig) {
    tracing::info!("注册存储路由: /storage/*");
    
    // 使用scope组织路由，与其他处理器保持一致
    cfg.service(
        web::scope("/storage")
            .route("/verify", web::post().to(verify))
            .route("/upload", web::post().to(upload))
            .route("/file/{path:.*}", web::get().to(get_file))
    );
    
    tracing::info!("存储路由注册完成");
}

#[derive(Deserialize)]
struct VerifyReq { filename: Option<String>, size: Option<i64>, sha256: Option<String>, file_path: Option<String>, url: Option<String> }

fn normalize_rel_from_input(input: &str) -> String {
    // 支持 /storage/file/<rel> 或 绝对URL
    let mut rel = input.trim();
    if rel.starts_with("http://") || rel.starts_with("https://") {
        if let Some(pos) = rel.find("/storage/file/") {
            rel = &rel[pos..];
        }
    }
    if let Some(stripped) = rel.strip_prefix("/storage/file/") { return stripped.to_string(); }
    rel.trim_start_matches('/').to_string()
}

async fn verify(app: web::Data<crate::core::AppState>, req: HttpRequest, payload: web::Json<VerifyReq>) -> Result<HttpResponse, actix_web::Error> {
    let _uid = extract_uid(&req, &app)?;
    // 优先使用 file_path/url/sha256 验证
    if let Some(fp) = payload.file_path.as_ref().or(payload.url.as_ref()) {
        let rel = normalize_rel_from_input(fp);
        let base = PathBuf::from(&app.config.storage.upload_path);
        let full = base.join(&rel);
        let fs_exists = tokio::fs::metadata(full).await.is_ok();
        let db_exists: bool = match sqlx::query("SELECT 1 FROM storage_files WHERE relative_path = ? LIMIT 1")
            .bind(&rel)
            .fetch_optional(app.db.pool())
            .await {
            Ok(row_opt) => row_opt.is_some(),
            Err(_) => false,
        };
        let exists = fs_exists && db_exists;
        let download_url = if exists { Some(format!("/storage/file/{}", rel)) } else { None };
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"exists": exists, "file_size": payload.size, "download_url": download_url} })))
    }
    if let Some(sha) = payload.sha256.as_ref() {
        let row = sqlx::query("SELECT relative_path, size FROM storage_files WHERE sha256 = ?")
            .bind(sha)
            .fetch_optional(app.db.pool())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        if let Some(r) = row {
            let rel: String = r.try_get::<String, _>("relative_path").unwrap_or_default();
            let size: i64 = r.try_get::<i64, _>("size").unwrap_or(0);
            return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"exists": true, "file_size": size, "download_url": format!("/storage/file/{}", rel)} })))
        }
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"exists": false} })))
    }
    // 未提供 file_path/url/sha256 时，返回 exists=false，避免误判
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"exists": false}})))
}

// 将函数改为公开，以便可以从其他模块访问
pub async fn upload(app: web::Data<crate::core::AppState>, req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("=== UPLOAD FUNCTION CALLED ==="); // 修改为info级别的日志
    let uid = extract_uid(&req, &app)?;
    tracing::info!("用户 {} 开始文件上传", uid);
    
    let base = PathBuf::from(&app.config.storage.upload_path);
    LocalStorage::ensure_base_dir(&base).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let target_root = LocalStorage::partitioned_dir(&base);

    let repo = StorageFileRepository::new(app.db.pool());
    let mut stored: Vec<serde_json::Value> = Vec::new();

    // 解析可能的绑定ID
    let mut pending_files: Vec<(String, Vec<u8>)> = Vec::new();
    let mut package_id: Option<i64> = None;
    let mut post_id: Option<i64> = None;

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| {
            tracing::error!("解析multipart字段失败: {}", e);
            actix_web::error::ErrorBadRequest(format!("解析上传数据失败: {}", e))
        })?;
        
        let cd = field.content_disposition().cloned();
        let name = cd.as_ref().and_then(|c| c.get_name()).unwrap_or("").to_string();
        
        if name == "package_id" {
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await { 
                let data = chunk.map_err(|e| {
                    tracing::error!("读取package_id字段失败: {}", e);
                    actix_web::error::ErrorBadRequest(e.to_string())
                })?; 
                bytes.extend_from_slice(&data); 
            }
            let s = String::from_utf8(bytes).unwrap_or_default();
            package_id = s.trim().parse::<i64>().ok();
            tracing::debug!("解析到package_id: {:?}", package_id);
            continue;
        } else if name == "post_id" {
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await { 
                let data = chunk.map_err(|e| {
                    tracing::error!("读取post_id字段失败: {}", e);
                    actix_web::error::ErrorBadRequest(e.to_string())
                })?; 
                bytes.extend_from_slice(&data); 
            }
            let s = String::from_utf8(bytes).unwrap_or_default();
            post_id = s.trim().parse::<i64>().ok();
            tracing::debug!("解析到post_id: {:?}", post_id);
            continue;
        }

        // 常规文件域（file）
        let mut bytes: Vec<u8> = Vec::new();
        let mut total_size = 0usize;
        const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
        
        while let Some(chunk) = field.next().await { 
            let data = chunk.map_err(|e| {
                tracing::error!("读取文件数据失败: {}", e);
                actix_web::error::ErrorBadRequest(format!("文件上传中断: {}", e))
            })?; 
            
            total_size += data.len();
            if total_size > MAX_FILE_SIZE {
                tracing::warn!("文件过大: {} bytes", total_size);
                return Err(actix_web::error::ErrorPayloadTooLarge("文件大小超过100MB限制"));
            }
            
            bytes.extend_from_slice(&data); 
        }
        
        let original = cd.and_then(|c| c.get_filename().map(|s| s.to_string())).unwrap_or_else(|| "file".into());
        tracing::info!("接收到文件: {} ({} bytes)", original, bytes.len());
        pending_files.push((original, bytes));
    }

    tracing::info!("开始处理 {} 个文件", pending_files.len());

    for (original, bytes) in pending_files {
        tracing::info!("处理文件: {} ({} bytes)", original, bytes.len());
        
        let sha256 = LocalStorage::sha256_hex(&bytes);
        if let Some(hit) = repo.find_by_sha256(&sha256).await.map_err(|e| {
            tracing::error!("查询文件SHA256失败: {}", e);
            actix_web::error::ErrorInternalServerError(e.to_string())
        })? {
            tracing::info!("文件去重命中: {} -> {}", original, hit.relative_path);
            let file_path = hit.relative_path.trim_start_matches('/').to_string();
            let download_url = format!("/storage/file/{}", file_path);
            stored.push(serde_json::json!({
                "file_path": file_path,
                "download_url": download_url,
                "file_size": hit.size,
                "dedup": true
            }));
            // 绑定到资源/帖子（去重命中也同样绑定）
            if let Some(pid) = package_id {
                bind_to_package(app.db.pool(), pid, &original, hit.size, &hit.mime.clone().unwrap_or_default(), &hit.relative_path, &hit.sha256).await?;
            }
            if let Some(pid) = post_id {
                bind_to_post(app.db.pool(), pid, &hit.relative_path).await?;
            }
            continue;
        }
        
        // 相对路径字符串：YYYY/MM/DD/<sha8>/<safe_name>
        let safe_name = LocalStorage::sanitize_filename(&original);
        let hash_dir = LocalStorage::hash_path_component(&bytes);
        let date_dir = target_root.strip_prefix(&base).unwrap_or(target_root.as_path()).to_string_lossy().replace('\\', "/");
        let rel = format!("{}/{}/{}", date_dir.trim_start_matches('/'), hash_dir, safe_name);
        let rel_trim = rel.trim_start_matches('/').to_string();
        
        tracing::info!("存储文件到: {}", rel_trim);
        
        // 使用 OpenDAL 写入
        app.storage.write(&rel_trim, bytes.clone()).await
            .map_err(|e| {
                tracing::error!("存储文件失败: {}", e);
                actix_web::error::ErrorInternalServerError(format!("文件存储失败: {}", e))
            })?;
            
        let size = bytes.len() as i64;
        let mime = MimeGuess::from_path(&original).first_raw();
        let ext = std::path::Path::new(&original).extension().and_then(|e| e.to_str());
        let _ = repo.insert(Some(uid), &sha256, size, mime, Some(&original), ext, &rel_trim).await
            .map_err(|e| {
                tracing::error!("记录文件到数据库失败: {}", e);
                actix_web::error::ErrorInternalServerError(e.to_string())
            })?;
            
        let download_url = format!("/storage/file/{}", rel_trim);
        stored.push(serde_json::json!({
            "file_path": rel_trim,
            "download_url": download_url,
            "file_size": size,
            "dedup": false
        }));
        
        tracing::info!("文件存储成功: {} -> {}", original, rel_trim);
        
        // 绑定
        if let Some(pid) = package_id {
            bind_to_package(app.db.pool(), pid, &original, size, &mime.unwrap_or("").to_string(), &rel_trim, &sha256).await?;
        }
        if let Some(pid) = post_id {
            bind_to_post(app.db.pool(), pid, &rel_trim).await?;
        }
    }

    tracing::info!("文件上传完成，共处理 {} 个文件", stored.len());

    // 若仅上传单文件，则返回对象，便于前端按对象读取
    if stored.len() == 1 {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": stored.remove(0)})))
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

async fn bind_to_package(pool: &sqlx::Pool<sqlx::Sqlite>, package_id: i64, original_name: &str, size: i64, mime: &str, rel_path: &str, sha256: &str) -> Result<(), actix_web::Error> {
    tracing::info!("绑定文件到资源 {}: {} ({})", package_id, original_name, rel_path);
    
    // 开始事务处理文件绑定
    let mut tx = pool.begin().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("开始事务失败: {}", e)))?;

    // 资源文件（zip）写入 file_path 与 file_size；图片加入 screenshots JSON 数组
    let is_zip = mime.contains("zip") || original_name.to_ascii_lowercase().ends_with(".zip");
    if is_zip {
        // 更新主包文件信息并设置为已发布状态（文件验证通过）
        sqlx::query("UPDATE packages SET file_path = ?, file_size = ?, status = 'published', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(rel_path)
            .bind(size)
            .bind(package_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("更新资源主文件失败: {}", e);
                actix_web::error::ErrorInternalServerError(format!("更新资源文件失败: {}", e))
            })?;
            
        tracing::info!("资源 {} 主文件已更新并发布", package_id);
    } else {
        // 更新 screenshots JSON 数组
        sqlx::query("UPDATE packages SET screenshots = json_insert(COALESCE(screenshots, '[]'), '$[#]', ?) , updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(format!("/storage/file/{}", rel_path.trim_start_matches('/')))
            .bind(package_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("更新资源截图失败: {}", e);
                actix_web::error::ErrorInternalServerError(format!("更新资源截图失败: {}", e))
            })?;
            
        tracing::info!("资源 {} 截图已更新", package_id);
    }
    
    // 记录到 package_files
    sqlx::query("INSERT INTO package_files (package_id, filename, file_path, file_size, mime_type, file_hash, storage_type, storage_config) VALUES (?, ?, ?, ?, ?, ?, 'local', '{}')")
        .bind(package_id)
        .bind(original_name)
        .bind(rel_path)
        .bind(size)
        .bind(mime)
        .bind(sha256)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("记录包文件失败: {}", e);
            actix_web::error::ErrorInternalServerError(format!("记录包文件失败: {}", e))
        })?;
    
    // 提交事务
    tx.commit().await
        .map_err(|e| {
            tracing::error!("提交文件绑定事务失败: {}", e);
            actix_web::error::ErrorInternalServerError(format!("文件绑定失败: {}", e))
        })?;
    
    tracing::info!("文件成功绑定到资源 {}: {}", package_id, original_name);
    Ok(())
}

async fn bind_to_post(pool: &sqlx::Pool<sqlx::Sqlite>, post_id: i64, rel_path: &str) -> Result<(), actix_web::Error> {
    tracing::info!("绑定图片到帖子 {}: {}", post_id, rel_path);
    
    // 开始事务处理图片绑定
    let mut tx = pool.begin().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("开始事务失败: {}", e)))?;

    // 更新帖子图片并设置为已发布状态（图片验证通过）
    sqlx::query("UPDATE posts SET images = json_insert(COALESCE(images, '[]'), '$[#]', ?), status = 'published', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(format!("/storage/file/{}", rel_path.trim_start_matches('/')))
        .bind(post_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("更新帖子图片失败: {}", e);
            actix_web::error::ErrorInternalServerError(format!("更新帖子图片失败: {}", e))
        })?;

    // 提交事务
    tx.commit().await
        .map_err(|e| {
            tracing::error!("提交图片绑定事务失败: {}", e);
            actix_web::error::ErrorInternalServerError(format!("图片绑定失败: {}", e))
        })?;
    
    tracing::info!("图片成功绑定到帖子 {}", post_id);
    Ok(())
} 