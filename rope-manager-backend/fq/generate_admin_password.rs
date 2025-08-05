use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "admin123";
    match hash(password, DEFAULT_COST) {
        Ok(hash) => println!("Password hash for '{}': {}", password, hash),
        Err(e) => eprintln!("Error generating hash: {}", e),
    }
} 