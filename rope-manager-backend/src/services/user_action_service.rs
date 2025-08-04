use anyhow::Result;
use crate::repositories::user_action_repo::UserActionRepository;
use crate::models::user_action::{
    UserAction, UserActionWithUser, CreateUserActionRequest, UserActionQueryParams, UserActionStats
};

#[derive(Clone)]
pub struct UserActionService {
    user_action_repo: UserActionRepository,
}

impl UserActionService {
    pub fn new(user_action_repo: UserActionRepository) -> Self {
        Self { user_action_repo }
    }

    // 记录用户行为
    pub async fn log_user_action(&self, req: &CreateUserActionRequest) -> Result<UserAction> {
        self.user_action_repo.create_user_action(req).await
    }

    // 获取用户行为记录列表（带用户信息）
    pub async fn get_user_actions_with_user(&self, params: &UserActionQueryParams) -> Result<(Vec<UserActionWithUser>, i64)> {
        self.user_action_repo.get_user_actions_with_user(params).await
    }

    // 获取用户行为记录列表（原方法，保持兼容性）
    pub async fn get_user_actions(&self, params: &UserActionQueryParams) -> Result<(Vec<UserAction>, i64)> {
        self.user_action_repo.get_user_actions(params).await
    }

    // 删除用户行为记录
    pub async fn delete_user_action(&self, action_id: i32) -> Result<()> {
        self.user_action_repo.delete_user_action(action_id).await
    }

    // 批量删除用户行为记录
    pub async fn batch_delete_user_actions(&self, action_ids: &[i32]) -> Result<()> {
        self.user_action_repo.batch_delete_user_actions(action_ids).await
    }

    // 获取用户行为统计数据
    pub async fn get_action_stats(&self, params: &UserActionQueryParams) -> Result<UserActionStats> {
        self.user_action_repo.get_action_stats(params).await
    }
} 