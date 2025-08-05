use bcrypt::{hash, verify, DEFAULT_COST};

fn main() {
    // 测试密码哈希
    let password = "admin123";
    match hash(password, DEFAULT_COST) {
        Ok(hash) => {
            println!("Generated hash: {}", hash);
            
            // 验证密码
            match verify(password, &hash) {
                Ok(is_valid) => {
                    println!("Password verification: {}", is_valid);
                },
                Err(e) => eprintln!("Verification error: {}", e),
            }
        },
        Err(e) => eprintln!("Hash generation error: {}", e),
    }
} 