use anyhow::Result;
use chrono::Utc;
use crate::models::notification::{Notification, NotificationQuery};
use crate::repositories::notification_repo::NotificationRepository;

#[derive(Clone)]
pub struct NotificationService {
    repo: NotificationRepository,
}

impl NotificationService {
    pub fn new(repo: NotificationRepository) -> Self { Self { repo } }

    pub async fn notify(&self, user_id: i32, title: &str, content: &str, link: Option<&str>, notif_type: Option<&str>, related_type: Option<&str>, related_id: Option<i32>) -> Result<i32> {
        let n = Notification {
            id: 0,
            user_id,
            title: title.to_string(),
            content: content.to_string(),
            link: link.map(|s| s.to_string()),
            notif_type: notif_type.map(|s| s.to_string()),
            related_type: related_type.map(|s| s.to_string()),
            related_id,
            is_read: false,
            created_at: Utc::now(),
        };
        self.repo.create(&n).await
    }

    pub async fn list(&self, user_id: i32, q: NotificationQuery) -> Result<Vec<Notification>> {
        self.repo.list(user_id, q.page.unwrap_or(1), q.size.unwrap_or(20)).await
    }

    pub async fn mark_read(&self, user_id: i32, id: i32) -> Result<()> { self.repo.mark_read(user_id, id).await }
    pub async fn unread_count(&self, user_id: i32) -> Result<i32> { self.repo.unread_count(user_id).await }
} 