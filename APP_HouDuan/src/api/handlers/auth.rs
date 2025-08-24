use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use bcrypt::{verify, hash};
use crate::infrastructure::database::repositories::UserRepository;
use crate::shared::utils::jwt as jwt_util;
use crate::infrastructure::email::EmailSender;
use crate::infrastructure::database::repositories::{EmailVerificationRepository, MailSettingsRepository};
use rand::Rng;
use chrono::{Utc, Duration};
use tracing::warn;
use sqlx::Error as SqlxError;

#[derive(Deserialize)]
struct LoginReq { username: String, password: String }

#[derive(Deserialize)]
struct RegisterReq { username: String, email: String, password: String, nickname: Option<String>, #[serde(alias = "verification_code")] code: Option<String> }

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/send-register-code", web::post().to(send_register_code))
            .route("/logout", web::post().to(logout))
            .route("/user-info", web::get().to(user_info))
            .route("/refresh", web::post().to(refresh))
            .route("/verify", web::get().to(verify_token))
    );
}

async fn login(app: web::Data<crate::core::AppState>, payload: web::Json<LoginReq>) -> Result<HttpResponse, actix_web::Error> {
    let repo = UserRepository::new(app.db.pool());
    let user_opt = repo.find_by_username_or_email(&payload.username).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let user = match user_opt { Some(u) => u, None => return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"用户不存在"}))) };

    let ok = verify(&payload.password, &user.password_hash).unwrap_or(false);
    if !ok { return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"密码错误"}))) }

    let token = jwt_util::sign_access(user.id, &app.config.jwt);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "登录成功",
        "data": {"token": token, "user": {"id": user.id, "username": user.username, "nickname": user.nickname, "avatar_url": user.avatar_url}}
    })))
}

async fn register(app: web::Data<crate::core::AppState>, payload: web::Json<RegisterReq>) -> Result<HttpResponse, actix_web::Error> {
    use tracing::{info, error, warn, debug, instrument};
    
    info!("🚀 开始处理用户注册请求");
    debug!("注册请求参数: username={}, email={}, nickname={:?}, has_code={}", 
        payload.username, payload.email, payload.nickname, payload.code.is_some());
    
    let repo = UserRepository::new(app.db.pool());
    let username = payload.username.trim();
    let email = payload.email.trim();
    
    // 步骤1: 检查用户名是否存在
    info!("📝 步骤1: 检查用户名唯一性 - {}", username);
    match repo.exists_username(username).await {
        Ok(exists) => {
            if exists {
                warn!("❌ 用户名已存在: {}", username);
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"用户名已存在"})))
            }
            debug!("✅ 用户名可用: {}", username);
        }
        Err(e) => {
            error!("💥 检查用户名时数据库错误: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({"code":1,"message":"检查用户名时发生错误"})));
        }
    }
    
    // 步骤2: 检查邮箱是否存在
    info!("📧 步骤2: 检查邮箱唯一性 - {}", email);
    match repo.exists_email(email).await {
        Ok(exists) => {
            if exists {
                warn!("❌ 邮箱已注册: {}", email);
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"邮箱已注册"})))
            }
            debug!("✅ 邮箱可用: {}", email);
        }
        Err(e) => {
            error!("💥 检查邮箱时数据库错误: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({"code":1,"message":"检查邮箱时发生错误"})));
        }
    }
    
    // 步骤3: 校验验证码
    info!("🔐 步骤3: 验证邮箱验证码");
    let code = match &payload.code { Some(c) => c.trim(), None => "" };
    if code.is_empty() {
        warn!("❌ 验证码为空");
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"请输入验证码"})))
    }
    debug!("🔍 查找验证码: email={}, code={}, type=register", email, code);
    
    let ev_repo = EmailVerificationRepository::new(app.db.pool());
    let valid = match ev_repo.find_valid(email, code, "register").await {
        Ok(v) => v,
        Err(e) => {
            error!("💥 查询验证码时数据库错误: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({"code":1,"message":"验证验证码时发生错误"})));
        }
    };
    
    let ev = match valid { 
        Some(v) => {
            info!("✅ 验证码有效: id={}", v.id);
            v
        }, 
        None => {
            warn!("❌ 验证码无效或已过期: email={}, code={}", email, code);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"验证码无效或已过期"})));
        }
    };

    // 步骤4: 密码加密
    info!("🔒 步骤4: 加密用户密码");
    let pwd_hash = match hash(&payload.password, 10) {
        Ok(hash) => {
            debug!("✅ 密码加密成功");
            hash
        },
        Err(e) => {
            error!("💥 密码加密失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({"code":1,"message":"密码加密失败"})));
        }
    };
    
    // 步骤5: 插入用户记录
    info!("👤 步骤5: 创建用户记录 - username={}, email={}, nickname={:?}", username, email, payload.nickname);
    let uid = match repo.insert_user(username, email, &pwd_hash, payload.nickname.as_deref()).await {
        Ok(id) => {
            info!("✅ 用户创建成功: user_id={}", id);
            id
        },
        Err(e) => {
            error!("💥 创建用户失败: {}", e);
            if let SqlxError::Database(db_err) = &e {
                let msg = db_err.message();
                error!("💾 数据库错误详情: {}", msg);
                if msg.contains("UNIQUE") || msg.contains("unique") {
                    warn!("❌ 违反唯一约束: {}", msg);
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"用户名或邮箱已存在"})))
                }
                // 其他数据库错误：返回 500 且携带详细信息（便于开发调试）
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "code":1,
                    "message":format!("数据库错误: {}", msg),
                    "debug_info": {
                        "error_type": "database",
                        "sql_error": msg,
                        "step": "insert_user"
                    }
                })));
            }
            // 非数据库错误：兜底为 500 JSON
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "code":1,
                "message":format!("注册失败: {}", e),
                "debug_info": {
                    "error_type": "unknown",
                    "error_message": e.to_string(),
                    "step": "insert_user"
                }
            })));
        }
    };
    
    // 步骤6: 标记验证码已使用
    info!("📝 步骤6: 标记验证码已使用 - verification_id={}", ev.id);
    match ev_repo.mark_used(ev.id).await {
        Ok(_) => debug!("✅ 验证码标记成功"),
        Err(e) => {
            // 这个错误不应该阻止注册成功，只记录警告
            warn!("⚠️ 标记验证码失败（不影响注册）: {}", e);
        }
    }

    // 步骤7: 生成JWT令牌
    info!("🎟️ 步骤7: 生成JWT令牌 - user_id={}", uid);
    let token = jwt_util::sign_access(uid, &app.config.jwt);
    
    info!("🎉 用户注册成功完成: user_id={}, username={}", uid, username);
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "code": 0,
        "message": "注册成功",
        "data": {
            "token": token, 
            "user": {
                "id": uid, 
                "username": username,
                "email": email,
                "nickname": payload.nickname
            }
        }
    })))
}

#[derive(Deserialize)]
struct SendCodeReq { email: String }

async fn send_register_code(app: web::Data<crate::core::AppState>, payload: web::Json<SendCodeReq>) -> Result<HttpResponse, actix_web::Error> {
    use tracing::{info, error, warn, debug};
    
    info!("📧 开始处理发送注册验证码请求");
    let email = payload.email.trim();
    debug!("请求邮箱: {}", email);
    
    // 步骤1: 检查邮箱是否已注册
    info!("🔍 步骤1: 检查邮箱是否已注册 - {}", email);
    let user_repo = UserRepository::new(app.db.pool());
    match user_repo.exists_email(email).await {
        Ok(exists) => {
            if exists {
                warn!("❌ 邮箱已注册: {}", email);
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({"code":1,"message":"邮箱已注册"})))
            }
            debug!("✅ 邮箱可用: {}", email);
        }
        Err(e) => {
            error!("💥 检查邮箱时数据库错误: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({"code":1,"message":"检查邮箱时发生错误"})));
        }
    }
    
    // 步骤2: 生成验证码
    info!("🎲 步骤2: 生成6位数验证码");
         let mut rng = rand::rng();
     let code = format!("{:06}", rng.random_range(0..=999999));
    let expires_at = Utc::now() + Duration::minutes(10);
    debug!("验证码生成成功: {} (过期时间: {})", code, expires_at);
    
    // 步骤3: 保存验证码到数据库
    info!("💾 步骤3: 保存验证码到数据库");
    let ev_repo = EmailVerificationRepository::new(app.db.pool());
    match ev_repo.create_code(email, &code, "register", expires_at).await {
        Ok(verification_id) => {
            info!("✅ 验证码保存成功: verification_id={}", verification_id);
        }
        Err(e) => {
            error!("💥 保存验证码失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "code":1,
                "message":"保存验证码失败",
                "debug_info": {
                    "error_type": "database",
                    "error_message": e.to_string(),
                    "step": "create_verification_code"
                }
            })));
        }
    }

    // 步骤4: 发送邮件
    info!("📮 步骤4: 发送验证码邮件到 {}", email);
    let subject = "【绳包管理器】注册验证码";
    let html = format!(
        "<div style='font-family:Arial,sans-serif'>\n<h2>注册验证码</h2>\n<p>您的验证码是：<b style='font-size:20px'>{}</b></p>\n<p>10分钟内有效，请勿泄露。</p>\n</div>",
        code
    );
    
    match EmailSender::send(&app.db, &app.config.email, email, None, subject, &html).await {
        Ok(_) => {
            info!("✅ 验证码邮件发送成功: {}", email);
        }
        Err(e) => {
            error!("💥 邮件发送失败: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "code":1,
                "message":format!("邮件发送失败: {}", e),
                "debug_info": {
                    "error_type": "email",
                    "error_message": e.to_string(),
                    "step": "send_email"
                }
            })));
        }
    }
    
    info!("🎉 验证码发送流程完成: {}", email);
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"验证码已发送"})))
}

async fn logout(_app: web::Data<crate::core::AppState>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "message":"已登出"})))
}

async fn user_info(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = parse_user_id_from_auth(&req, &app);
    let uid = match uid { Some(x) => x, None => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({"code":401,"message":"未登录"}))) };
    let repo = UserRepository::new(app.db.pool());
    if let Some(u) = repo.find_by_id(uid).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))? {
        return Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"id": u.id, "username": u.username, "nickname": u.nickname, "avatar_url": u.avatar_url}})))
    }
    Ok(HttpResponse::NotFound().json(serde_json::json!({"code":404, "message":"用户不存在"})))
}

async fn refresh(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let uid = parse_user_id_from_auth(&req, &app);
    let uid = match uid { Some(x) => x, None => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({"code":401,"message":"未登录"}))) };
    let token = jwt_util::sign_access(uid, &app.config.jwt);
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"token": token}})))
}

async fn verify_token(app: web::Data<crate::core::AppState>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let ok = parse_user_id_from_auth(&req, &app).is_some();
    Ok(HttpResponse::Ok().json(serde_json::json!({"code":0, "data": {"valid": ok}})))
}

fn parse_user_id_from_auth(req: &HttpRequest, app: &crate::core::AppState) -> Option<i64> {
    if let Some(hv) = req.headers().get(actix_web::http::header::AUTHORIZATION) {
        if let Ok(s) = hv.to_str() {
            if let Some(t) = s.strip_prefix("Bearer ") {
                return jwt_util::verify(t, &app.config.jwt)
            }
        }
    }
    None
} 