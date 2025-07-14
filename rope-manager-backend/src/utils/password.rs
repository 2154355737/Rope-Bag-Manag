use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Clone)]
pub struct PasswordUtils;

impl PasswordUtils {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let hash = hash(password, DEFAULT_COST)?;
        Ok(hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let is_valid = verify(password, hash)?;
        Ok(is_valid)
    }
} 