pub mod handlers;
pub mod routes;
pub mod middleware;

use actix_web::{web, HttpResponse};

/// 配置所有API路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // 添加一个根级别的测试路由
    cfg.route("/test-direct-upload", web::post().to(test_direct_upload));
    
    // 将storage路由直接配置在根级别，避免/api/v1 scope内可能的中间件干扰
    handlers::storage::configure(cfg);
    
    routes::configure_health_routes(cfg);
    routes::configure_api_routes(cfg);
}

/// 测试直接上传路由
async fn test_direct_upload(mut payload: actix_multipart::Multipart) -> Result<HttpResponse, actix_web::Error> {
    use futures::{StreamExt, TryStreamExt};
    
    tracing::info!("=== 直接测试上传路由被调用 ===");
    
    let mut files_info = Vec::new();
    
    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| {
            tracing::error!("解析multipart字段失败: {}", e);
            actix_web::error::ErrorBadRequest(format!("解析上传数据失败: {}", e))
        })?;
        
        let cd = field.content_disposition().cloned();
        let name = cd.as_ref().and_then(|c| c.get_name()).unwrap_or("").to_string();
        let filename = cd.as_ref().and_then(|c| c.get_filename()).map(|s| s.to_string());
        
        tracing::info!("收到字段: name={}, filename={:?}", name, filename);
        
        let mut bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| {
                tracing::error!("读取字段数据失败: {}", e);
                actix_web::error::ErrorBadRequest(e.to_string())
            })?;
            bytes.extend_from_slice(&data);
        }
        
        let size = bytes.len();
        tracing::info!("字段 {} 大小: {} 字节", name, size);
        
        if let Some(fname) = filename {
            files_info.push(serde_json::json!({
                "field_name": name,
                "file_name": fname,
                "size": size
            }));
        } else if !bytes.is_empty() {
            let value = String::from_utf8_lossy(&bytes);
            tracing::info!("字段 {} 值: {}", name, value);
            files_info.push(serde_json::json!({
                "field_name": name,
                "value": value
            }));
        }
    }
    
    tracing::info!("测试上传处理完成");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "测试上传成功",
        "data": {
            "files": files_info
        }
    })))
}
