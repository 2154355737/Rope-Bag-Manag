use actix_web::{HttpRequest, dev::ServiceRequest};
use serde_json::{json, Value};
use chrono::{DateTime, Utc};
use std::net::IpAddr;
use log::{info, warn, error};
use crate::models::user::User;

/// 审计日志条目
#[derive(Debug, Clone)]
pub struct AuditLog {
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub user_role: Option<String>,
    pub operation: String,
    pub resource_type: String,
    pub resource_id: Option<i32>,
    pub details: Option<Value>,
    pub result: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub severity: AuditSeverity,
}

/// 审计日志严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum AuditSeverity {
    Info,      // 一般操作
    Warning,   // 需要关注
    Critical,  // 高风险操作
    Security,  // 安全事件
}

impl AuditLog {
    /// 创建新的审计日志
    pub fn new(operation: String, resource_type: String) -> Self {
        Self {
            timestamp: Utc::now(),
            user_id: None,
            username: None,
            user_role: None,
            operation,
            resource_type,
            resource_id: None,
            details: None,
            result: "success".to_string(),
            ip_address: None,
            user_agent: None,
            session_id: None,
            severity: AuditSeverity::Info,
        }
    }
    
    /// 设置用户信息
    pub fn with_user(mut self, user: &User) -> Self {
        self.user_id = Some(user.id);
        self.username = Some(user.username.clone());
        self.user_role = Some(user.role.to_string());
        self
    }
    
    /// 设置资源ID
    pub fn with_resource_id(mut self, resource_id: i32) -> Self {
        self.resource_id = Some(resource_id);
        self
    }
    
    /// 设置操作详情
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
    
    /// 设置操作结果
    pub fn with_result(mut self, result: String) -> Self {
        self.result = result;
        self
    }
    
    /// 设置严重程度
    pub fn with_severity(mut self, severity: AuditSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    /// 从HTTP请求设置网络信息
    pub fn with_request_info(mut self, req: &HttpRequest) -> Self {
        self.ip_address = get_client_ip(req);
        self.user_agent = get_user_agent(req);
        self
    }
    
    /// 从ServiceRequest设置网络信息
    pub fn with_service_request_info(mut self, req: &ServiceRequest) -> Self {
        self.ip_address = get_client_ip_from_service_request(req);
        self.user_agent = get_user_agent_from_service_request(req);
        self
    }
    
    /// 转换为JSON格式
    pub fn to_json(&self) -> Value {
        json!({
            "timestamp": self.timestamp.to_rfc3339(),
            "user_id": self.user_id,
            "username": self.username,
            "user_role": self.user_role,
            "operation": self.operation,
            "resource_type": self.resource_type,
            "resource_id": self.resource_id,
            "details": self.details,
            "result": self.result,
            "ip_address": self.ip_address,
            "user_agent": self.user_agent,
            "session_id": self.session_id,
            "severity": format!("{:?}", self.severity)
        })
    }
}

/// 审计日志服务
pub struct AuditService;

impl AuditService {
    /// 记录审计日志
    pub async fn log(audit_log: &AuditLog) -> Result<(), Box<dyn std::error::Error>> {
        let log_entry = audit_log.to_json();
        
        // 根据严重程度选择日志级别
        match audit_log.severity {
            AuditSeverity::Info => {
                info!("📝 审计日志: {}", serde_json::to_string_pretty(&log_entry)?);
            },
            AuditSeverity::Warning => {
                warn!("⚠️ 审计警告: {}", serde_json::to_string_pretty(&log_entry)?);
            },
            AuditSeverity::Critical => {
                error!("🚨 审计严重: {}", serde_json::to_string_pretty(&log_entry)?);
            },
            AuditSeverity::Security => {
                error!("🛡️ 安全事件: {}", serde_json::to_string_pretty(&log_entry)?);
            },
        }
        
        // TODO: 将日志写入数据库或专门的审计日志文件
        // 现在先使用标准日志，后续可以扩展到数据库存储
        
        // 对于严重和安全事件，发送告警
        if matches!(audit_log.severity, AuditSeverity::Critical | AuditSeverity::Security) {
            Self::send_alert(audit_log).await?;
        }
        
        Ok(())
    }
    
    /// 记录敏感操作
    pub async fn log_sensitive_operation(
        user: &User,
        operation: &str,
        resource_type: &str,
        resource_id: Option<i32>,
        details: Option<Value>,
        result: &str,
        req: &HttpRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut audit_log = AuditLog::new(operation.to_string(), resource_type.to_string())
            .with_user(user)
            .with_result(result.to_string())
            .with_request_info(req);
        
        if let Some(id) = resource_id {
            audit_log = audit_log.with_resource_id(id);
        }
        
        if let Some(details) = details {
            audit_log = audit_log.with_details(details);
        }
        
        // 判断操作严重程度
        audit_log = audit_log.with_severity(classify_operation_severity(operation));
        
        Self::log(&audit_log).await
    }
    
    /// 记录安全事件
    pub async fn log_security_event(
        operation: &str,
        details: Value,
        req: &HttpRequest,
        user: Option<&User>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut audit_log = AuditLog::new(operation.to_string(), "security".to_string())
            .with_severity(AuditSeverity::Security)
            .with_details(details)
            .with_result("security_event".to_string())
            .with_request_info(req);
        
        if let Some(user) = user {
            audit_log = audit_log.with_user(user);
        }
        
        Self::log(&audit_log).await
    }
    
    /// 发送告警通知
    async fn send_alert(audit_log: &AuditLog) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: 实现告警通知机制
        // 可以通过邮件、钉钉、企业微信等方式发送告警
        
        warn!("🚨 安全告警需要发送: 操作={}, 用户={:?}, IP={:?}", 
            audit_log.operation, 
            audit_log.username, 
            audit_log.ip_address
        );
        
        Ok(())
    }
}

/// 判断操作的严重程度
fn classify_operation_severity(operation: &str) -> AuditSeverity {
    match operation {
        // 安全相关操作
        "unauthorized_access_attempt" | "admin_access_attempt" | 
        "brute_force_attempt" | "suspicious_activity" => AuditSeverity::Security,
        
        // 高风险操作
        "user_ban" | "user_delete" | "admin_create" | 
        "system_config_change" | "backup_restore" | "data_export" |
        "mass_delete" | "privilege_escalation" => AuditSeverity::Critical,
        
        // 需要关注的操作
        "user_role_change" | "password_reset" | "email_change" |
        "package_delete" | "comment_batch_delete" => AuditSeverity::Warning,
        
        // 一般操作
        _ => AuditSeverity::Info,
    }
}

/// 获取客户端IP地址
fn get_client_ip(req: &HttpRequest) -> Option<String> {
    // 1. 尝试从X-Forwarded-For头获取
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            let ip = forwarded_str.split(',').next().unwrap_or("").trim();
            if !ip.is_empty() {
                return Some(ip.to_string());
            }
        }
    }
    
    // 2. 尝试从X-Real-IP头获取
    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            return Some(ip_str.to_string());
        }
    }
    
    // 3. 从连接信息获取
    if let Some(peer_addr) = req.peer_addr() {
        return Some(peer_addr.ip().to_string());
    }
    
    None
}

/// 从ServiceRequest获取客户端IP
fn get_client_ip_from_service_request(req: &ServiceRequest) -> Option<String> {
    get_client_ip(req.request())
}

/// 获取User-Agent
fn get_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("User-Agent")
        .and_then(|ua| ua.to_str().ok())
        .map(|s| s.to_string())
}

/// 从ServiceRequest获取User-Agent
fn get_user_agent_from_service_request(req: &ServiceRequest) -> Option<String> {
    get_user_agent(req.request())
}

/// 便捷宏：记录敏感操作
#[macro_export]
macro_rules! audit_log {
    ($user:expr, $operation:expr, $resource_type:expr, $req:expr) => {
        $crate::middleware::audit::AuditService::log_sensitive_operation(
            $user,
            $operation,
            $resource_type,
            None,
            None,
            "success",
            $req,
        )
    };
    
    ($user:expr, $operation:expr, $resource_type:expr, $resource_id:expr, $req:expr) => {
        $crate::middleware::audit::AuditService::log_sensitive_operation(
            $user,
            $operation,
            $resource_type,
            Some($resource_id),
            None,
            "success",
            $req,
        )
    };
    
    ($user:expr, $operation:expr, $resource_type:expr, $resource_id:expr, $details:expr, $req:expr) => {
        $crate::middleware::audit::AuditService::log_sensitive_operation(
            $user,
            $operation,
            $resource_type,
            Some($resource_id),
            Some($details),
            "success",
            $req,
        )
    };
}

/// 便捷宏：记录安全事件
#[macro_export]
macro_rules! security_log {
    ($operation:expr, $details:expr, $req:expr) => {
        $crate::middleware::audit::AuditService::log_security_event(
            $operation,
            $details,
            $req,
            None,
        )
    };
    
    ($operation:expr, $details:expr, $req:expr, $user:expr) => {
        $crate::middleware::audit::AuditService::log_security_event(
            $operation,
            $details,
            $req,
            Some($user),
        )
    };
} 