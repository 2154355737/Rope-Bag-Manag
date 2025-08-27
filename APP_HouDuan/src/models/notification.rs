use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,                // 接收人
    pub title: String,
    pub content: String,
    pub link: Option<String>,         // 前端跳转链接
    pub notif_type: Option<String>,   // 例如 ResourceApproved / CommentReceived
    pub related_type: Option<String>, // 例如 Package / Comment
    pub related_id: Option<i32>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
} 