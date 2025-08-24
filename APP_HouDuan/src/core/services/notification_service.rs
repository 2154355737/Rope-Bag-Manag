use std::sync::Arc;
use tracing::instrument;
use crate::infrastructure::database::repositories::NotificationRepository;

#[derive(Debug)]
pub struct NotificationService {
    _notification_repo: Arc<NotificationRepository>,
}

impl NotificationService {
    pub fn new(notification_repo: Arc<NotificationRepository>) -> Self {
        Self { _notification_repo: notification_repo }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        tracing::debug!("Notification service health check");
        true
    }
} 