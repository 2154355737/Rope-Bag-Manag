use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::config::JwtConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
}

pub fn sign_access(user_id: i64, cfg: &JwtConfig) -> String {
    let exp = Utc::now() + Duration::seconds(cfg.expires_in);
    let claims = Claims { sub: user_id, exp: exp.timestamp() };
    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(cfg.secret.as_bytes())).unwrap_or_default()
}

pub fn verify(token: &str, cfg: &JwtConfig) -> Option<i64> {
    match decode::<Claims>(token, &DecodingKey::from_secret(cfg.secret.as_bytes()), &Validation::new(Algorithm::HS256)) {
        Ok(data) => Some(data.claims.sub),
        Err(_) => None,
    }
} 