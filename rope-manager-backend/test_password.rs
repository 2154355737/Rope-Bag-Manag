use bcrypt::verify;

fn main() {
    let password = "admin123";
    let hash = "$2a$10$kazltxN8wRULpZb4tMWqDO9Ie9BaH5z.O1/XuEZ.fdSDzwfMOO5pa";
    
    match verify(password, hash) {
        Ok(is_valid) => {
            println!("Password verification result: {}", is_valid);
            if is_valid {
                println!("✅ 密码验证成功！");
            } else {
                println!("❌ 密码验证失败！");
            }
        },
        Err(e) => {
            println!("❌ 密码验证错误: {}", e);
        }
    }
} 