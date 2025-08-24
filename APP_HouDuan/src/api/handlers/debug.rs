use actix_web::{web, HttpResponse};
use crate::infrastructure::database::repositories::{UserRepository, EmailVerificationRepository};
use tracing::{info, error};
use sqlx::Row;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/debug")
            .route("/db-status", web::get().to(check_database_status))
            .route("/verification-codes/{email}", web::get().to(get_verification_codes))
            .route("/user-exists/{identifier}", web::get().to(check_user_exists))
            .route("/test-insert", web::post().to(test_user_insert))
    );
}

/// 检查数据库连接状态
async fn check_database_status(app: web::Data<crate::core::AppState>) -> Result<HttpResponse, actix_web::Error> {
    info!("🔍 开始检查数据库状态");
    
    let user_repo = UserRepository::new(app.db.pool());
    let ev_repo = EmailVerificationRepository::new(app.db.pool());
    
    let mut checks = Vec::new();
    
    // 检查用户表
    match user_repo.health_check().await {
        Ok(_) => {
            checks.push(serde_json::json!({
                "table": "users",
                "status": "ok",
                "message": "用户表连接正常"
            }));
        }
        Err(e) => {
            checks.push(serde_json::json!({
                "table": "users", 
                "status": "error",
                "message": format!("用户表连接失败: {}", e)
            }));
        }
    }
    
    // 检查验证码表
    match sqlx::query("SELECT COUNT(*) FROM email_verifications").fetch_one(app.db.pool()).await {
        Ok(row) => {
            let count: i64 = row.try_get(0).unwrap_or(0);
            checks.push(serde_json::json!({
                "table": "email_verifications",
                "status": "ok", 
                "message": format!("验证码表连接正常，当前记录数: {}", count)
            }));
        }
        Err(e) => {
            checks.push(serde_json::json!({
                "table": "email_verifications",
                "status": "error",
                "message": format!("验证码表连接失败: {}", e)
            }));
        }
    }
    
    // 检查表结构
    match sqlx::query("PRAGMA table_info(users)").fetch_all(app.db.pool()).await {
        Ok(rows) => {
            let mut columns = Vec::new();
            for row in rows {
                let name: String = row.try_get("name").unwrap_or_default();
                let data_type: String = row.try_get("type").unwrap_or_default();
                columns.push(format!("{}({})", name, data_type));
            }
            checks.push(serde_json::json!({
                "table": "users_schema",
                "status": "ok",
                "message": "用户表结构",
                "columns": columns
            }));
        }
        Err(e) => {
            checks.push(serde_json::json!({
                "table": "users_schema",
                "status": "error", 
                "message": format!("获取表结构失败: {}", e)
            }));
        }
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "数据库状态检查完成",
        "data": {
            "checks": checks,
            "timestamp": chrono::Utc::now()
        }
    })))
}

/// 获取指定邮箱的验证码记录
async fn get_verification_codes(
    app: web::Data<crate::core::AppState>, 
    path: web::Path<String>
) -> Result<HttpResponse, actix_web::Error> {
    let email = path.into_inner();
    info!("🔍 查询验证码记录: {}", email);
    
    match sqlx::query(
        "SELECT id, email, code, type, expires_at, used_at, created_at FROM email_verifications WHERE email = ? ORDER BY created_at DESC LIMIT 10"
    )
    .bind(&email)
    .fetch_all(app.db.pool())
    .await 
    {
        Ok(rows) => {
            let mut codes = Vec::new();
            for row in rows {
                codes.push(serde_json::json!({
                    "id": row.try_get::<i64, _>("id").unwrap_or(0),
                    "email": row.try_get::<String, _>("email").unwrap_or_default(),
                    "code": row.try_get::<String, _>("code").unwrap_or_default(),
                    "type": row.try_get::<String, _>("type").unwrap_or_default(),
                    "expires_at": row.try_get::<chrono::DateTime<chrono::Utc>, _>("expires_at").ok(),
                    "used_at": row.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("used_at").unwrap_or(None),
                    "created_at": row.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").ok(),
                    "is_expired": row.try_get::<chrono::DateTime<chrono::Utc>, _>("expires_at")
                        .map(|exp| exp < chrono::Utc::now())
                        .unwrap_or(true),
                    "is_used": row.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("used_at").unwrap_or(None).is_some()
                }));
            }
            
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "code": 0,
                "message": "获取验证码记录成功",
                "data": {
                    "email": email,
                    "codes": codes,
                    "total": codes.len()
                }
            })))
        }
        Err(e) => {
            error!("💥 查询验证码记录失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "code": 1,
                "message": format!("查询失败: {}", e)
            })))
        }
    }
}

/// 检查用户是否存在（用户名或邮箱）
async fn check_user_exists(
    app: web::Data<crate::core::AppState>,
    path: web::Path<String>
) -> Result<HttpResponse, actix_web::Error> {
    let identifier = path.into_inner();
    info!("🔍 检查用户是否存在: {}", identifier);
    
    let user_repo = UserRepository::new(app.db.pool());
    
    let username_exists = user_repo.exists_username(&identifier).await.unwrap_or(false);
    let email_exists = user_repo.exists_email(&identifier).await.unwrap_or(false);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "检查完成",
        "data": {
            "identifier": identifier,
            "username_exists": username_exists,
            "email_exists": email_exists,
            "any_exists": username_exists || email_exists
        }
    })))
}

#[derive(serde::Deserialize)]
struct TestInsertReq {
    username: String,
    email: String,
    password: String,
    nickname: Option<String>,
}

/// 测试用户插入操作
async fn test_user_insert(
    app: web::Data<crate::core::AppState>,
    payload: web::Json<TestInsertReq>
) -> Result<HttpResponse, actix_web::Error> {
    info!("🧪 测试用户插入操作");
    
    let user_repo = UserRepository::new(app.db.pool());
    
    // 测试插入
    match user_repo.insert_user(
        &payload.username,
        &payload.email, 
        "test_hash",
        payload.nickname.as_deref()
    ).await {
        Ok(user_id) => {
            info!("✅ 测试插入成功: user_id={}", user_id);
            
            // 立即删除测试用户
            match sqlx::query("DELETE FROM users WHERE id = ?")
                .bind(user_id)
                .execute(app.db.pool())
                .await 
            {
                Ok(_) => info!("🗑️ 测试用户已清理"),
                Err(e) => error!("⚠️ 清理测试用户失败: {}", e)
            }
            
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "code": 0,
                "message": "插入测试成功",
                "data": {
                    "test_user_id": user_id,
                    "cleaned": true
                }
            })))
        }
        Err(e) => {
            error!("💥 测试插入失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "code": 1,
                "message": format!("插入测试失败: {}", e),
                "debug_info": {
                    "error_type": "database",
                    "error_message": e.to_string()
                }
            })))
        }
    }
} 