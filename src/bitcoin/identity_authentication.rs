use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct IdentityAuthenticationModule;

impl IdentityAuthenticationModule {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_token(&self, user_id: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
        let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
        Ok(token)
    }

    pub fn validate_token(&self, token: &str, secret: &str) -> Result<Claims, Box<dyn std::error::Error>> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
        Ok(token_data.claims)
    }
}