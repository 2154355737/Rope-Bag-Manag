use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("data.db")?;
    
    println!("🔍 检查用户状态...");
    
    // 查询所有用户的状态
    let mut stmt = conn.prepare("SELECT id, username, role, ban_status FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,     // id
            row.get::<_, String>(1)?,  // username  
            row.get::<_, String>(2)?,  // role
            row.get::<_, String>(3)?,  // ban_status
        ))
    })?;

    println!("当前用户状态:");
    println!("ID\t用户名\t\t角色\t\t状态");
    println!("------------------------------------------");
    
    for user in user_iter {
        let (id, username, role, ban_status) = user?;
        println!("{}\t{}\t\t{}\t\t{}", id, username, role, ban_status);
    }
    
    println!("\n🔧 修复用户状态 - 将所有用户状态设置为 'normal'");
    
    // 更新所有用户的ban_status为normal
    let updated_count = conn.execute(
        "UPDATE users SET ban_status = 'normal' WHERE ban_status != 'normal'",
        []
    )?;
    
    println!("✅ 已修复 {} 个用户的状态", updated_count);
    
    // 再次查询确认
    println!("\n✅ 修复后的用户状态:");
    println!("ID\t用户名\t\t角色\t\t状态");
    println!("------------------------------------------");
    
    let mut stmt2 = conn.prepare("SELECT id, username, role, ban_status FROM users")?;
    let user_iter2 = stmt2.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,     // id
            row.get::<_, String>(1)?,  // username  
            row.get::<_, String>(2)?,  // role
            row.get::<_, String>(3)?,  // ban_status
        ))
    })?;
    
    for user in user_iter2 {
        let (id, username, role, ban_status) = user?;
        println!("{}\t{}\t\t{}\t\t{}", id, username, role, ban_status);
    }
    
    Ok(())
} 