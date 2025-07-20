use actix_web::{web, HttpResponse};
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheItem {
    key: String,
    value: serde_json::Value,
    ttl: Option<i64>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cache")
            .route("/{cache_key}", web::delete().to(clear_cache))
            .route("/{cache_key}", web::get().to(get_cache))
            .route("", web::post().to(set_cache))
            .route("", web::delete().to(clear_all_cache))
            .route("/stats", web::get().to(get_cache_stats))
    );
}

// 清除缓存的处理函数
async fn clear_cache(path: web::Path<String>) -> HttpResponse {
    let cache_key = path.into_inner();
    
    // 记录缓存清除操作
    println!("[CACHE] 清除缓存: {}", cache_key);
    
    // 实际应用中，这里可以实现真正的缓存清除逻辑
    // 例如：从Redis、Memcached等缓存系统中删除指定键的缓存
    
    // 返回成功响应
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": format!("缓存 '{}' 已成功清除", cache_key),
        "data": null
    }))
}

// 获取缓存的处理函数
async fn get_cache(path: web::Path<String>) -> HttpResponse {
    let cache_key = path.into_inner();
    
    println!("[CACHE] 获取缓存: {}", cache_key);
    
    // 在实际应用中，这里应该从缓存系统中获取数据
    // 这里只是模拟返回一些数据
    
    // 模拟一些常用缓存键的数据
    let mock_data = match cache_key.as_str() {
        "getPackages" => json!({
            "list": [],
            "total": 0,
            "page": 1,
            "pageSize": 10,
            "totalPages": 0
        }),
        "getCategories" => json!({
            "list": [
                {"id": 1, "name": "基础绳结", "description": "基础绳结技巧", "enabled": true, "created_at": "2023-01-01T00:00:00Z"},
                {"id": 2, "name": "高级绳结", "description": "复杂绳结技巧", "enabled": true, "created_at": "2023-01-01T00:00:00Z"},
                {"id": 3, "name": "装饰绳结", "description": "装饰性绳结", "enabled": true, "created_at": "2023-01-01T00:00:00Z"},
                {"id": 4, "name": "实用绳结", "description": "日常实用绳结", "enabled": true, "created_at": "2023-01-01T00:00:00Z"},
                {"id": 5, "name": "救援绳结", "description": "救援和安全绳结", "enabled": true, "created_at": "2023-01-01T00:00:00Z"}
            ]
        }),
        _ => json!(null)
    };
    
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": mock_data
    }))
}

// 设置缓存的处理函数
async fn set_cache(cache_item: web::Json<CacheItem>) -> HttpResponse {
    println!("[CACHE] 设置缓存: {}", cache_item.key);
    
    // 在实际应用中，这里应该将数据存入缓存系统
    
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "缓存设置成功",
        "data": null
    }))
}

// 清除所有缓存的处理函数
async fn clear_all_cache() -> HttpResponse {
    println!("[CACHE] 清除所有缓存");
    
    // 在实际应用中，这里应该清除所有缓存
    
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "所有缓存已清除",
        "data": null
    }))
}

// 获取缓存统计信息的处理函数
async fn get_cache_stats() -> HttpResponse {
    println!("[CACHE] 获取缓存统计");
    
    // 在实际应用中，这里应该返回真实的缓存统计数据
    
    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "total_keys": 0,
            "total_size": 0,
            "hit_rate": 0.0
        }
    }))
} 