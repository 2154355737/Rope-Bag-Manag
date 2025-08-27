use bcrypt::verify;

fn main() {
    // 从数据库中admin用户的密码哈希（从截图中看到的）
    let stored_hash = "$2b$12$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi";
    let test_passwords = vec!["admin123", "admin", "password", "123456"];
    
    println!("测试admin用户的密码...");
    for password in test_passwords {
        match verify(password, stored_hash) {
            Ok(true) => println!("✅ 密码 '{}' 验证成功!", password),
            Ok(false) => println!("❌ 密码 '{}' 验证失败", password),
            Err(e) => println!("🔥 验证密码 '{}' 时出错: {}", password, e),
        }
    }
    
    // 生成新的admin123哈希
    println!("\n生成新的admin123密码哈希:");
    match bcrypt::hash("admin123", bcrypt::DEFAULT_COST) {
        Ok(hash) => println!("新哈希: {}", hash),
        Err(e) => println!("生成哈希失败: {}", e),
    }
} 