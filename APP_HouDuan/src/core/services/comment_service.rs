use std::sync::Arc;
use tracing::instrument;
use crate::infrastructure::database::repositories::{CommentRepository, UserRepository};

#[derive(Debug)]
pub struct CommentService {
    _comment_repo: Arc<CommentRepository>,
    _user_repo: Arc<UserRepository>,
}

impl CommentService {
    pub fn new(comment_repo: Arc<CommentRepository>, user_repo: Arc<UserRepository>) -> Self {
        Self { 
            _comment_repo: comment_repo, 
            _user_repo: user_repo 
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        tracing::debug!("Comment service health check");
        true
    }
} 