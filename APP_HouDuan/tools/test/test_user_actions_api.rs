use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::Connection;
use rope_manager_backend::services::user_action_service::UserActionService;
use rope_manager_backend::repositories::user_action_repo::UserActionRepository;
use rope_manager_backend::models::user_action::UserActionQueryParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试用户行为记录API...");
    
    // 连接数据库
    let conn = Connection::open("data.db")?;
    let conn = Arc::new(Mutex::new(conn));
    
    // 创建仓库和服务
    let user_action_repo = UserActionRepository::new(conn);
    let user_action_service = UserActionService::new(user_action_repo);
    
    // 测试获取用户行为记录
    println!("📋 测试获取用户行为记录...");
    
    let params = UserActionQueryParams {
        page: Some(1),
        page_size: Some(5),
        user_id: None,
        action_type: None,
        target_type: None,
        target_id: None,
        start_time: None,
        end_time: None,
    };
    
    match user_action_service.get_user_actions(&params).await {
        Ok((actions, total)) => {
            println!("✅ 成功获取用户行为记录");
            println!("📊 总记录数: {}", total);
            println!("📝 当前页记录数: {}", actions.len());
            
            for (i, action) in actions.iter().enumerate() {
                println!("  {}. ID:{} 用户:{:?} 行为:{} 时间:{}", 
                    i + 1, action.id, action.user_id, action.action_type, action.created_at);
            }
        },
        Err(e) => {
            println!("❌ 获取用户行为记录失败: {}", e);
            return Err(e.into());
        }
    }
    
    // 测试获取用户行为记录（带用户信息）
    println!("\n📋 测试获取用户行为记录（带用户信息）...");
    
    match user_action_service.get_user_actions_with_user(&params).await {
        Ok((actions, total)) => {
            println!("✅ 成功获取用户行为记录（带用户信息）");
            println!("📊 总记录数: {}", total);
            println!("📝 当前页记录数: {}", actions.len());
            
            for (i, action) in actions.iter().enumerate() {
                println!("  {}. ID:{} 用户:{:?}({:?}) 行为:{} 时间:{}", 
                    i + 1, action.id, action.user_id, action.username, action.action_type, action.created_at);
            }
        },
        Err(e) => {
            println!("❌ 获取用户行为记录（带用户信息）失败: {}", e);
            return Err(e.into());
        }
    }
    
    println!("\n✅ 所有测试完成！");
    Ok(())
} 