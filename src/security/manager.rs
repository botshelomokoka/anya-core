use argon2::{self, Config};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn, error};
use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    role: String,
}

pub struct SecurityManager {
    jwt_secret: String,
    jwt_expiration: u64,
    password_pepper: String,
}

impl SecurityManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = CONFIG.read().await;
        let jwt_secret = config.get_string("security.jwt_secret")
            .ok_or("JWT secret not configured")?;
        let jwt_expiration = config.get_number("security.jwt_expiration")
            .unwrap_or(3600) as u64;
        let password_pepper = config.get_string("security.password_pepper")
            .ok_or("Password pepper not configured")?;

        Ok(Self {
            jwt_secret,
            jwt_expiration,
            password_pepper,
        })
    }

    pub fn hash_password(&self, password: &str) -> Result<String, argon2::Error> {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config::default();
        let peppered_password = format!("{}{}", password, self.password_pepper);
        
        argon2::hash_encoded(
            peppered_password.as_bytes(),
            &salt,
            &config
        )
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, argon2::Error> {
        let peppered_password = format!("{}{}", password, self.password_pepper);
        argon2::verify_encoded(hash, peppered_password.as_bytes())
    }

    pub fn generate_jwt(&self, user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + self.jwt_expiration as usize,
            iat: now,
            role: role.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )
    }

    pub fn verify_jwt(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation
        )?;

        Ok(token_data.claims)
    }

    pub fn generate_api_key(&self) -> String {
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        base64::encode(key)
    }

    pub async fn rotate_credentials(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement credential rotation logic
        todo!("Implement credential rotation")
    }

    pub fn sanitize_input(&self, input: &str) -> String {
        // Basic input sanitization
        input
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('&', "&amp;")
    }

    pub fn validate_input(&self, input: &str, pattern: &str) -> bool {
        // Implement input validation against regex pattern
        regex::Regex::new(pattern)
            .map(|re| re.is_match(input))
            .unwrap_or(false)
    }

    pub async fn audit_log(&self, event_type: &str, user_id: &str, action: &str, details: &str) {
        // Log security events
        info!(
            event_type = event_type,
            user_id = user_id,
            action = action,
            details = details,
            timestamp = chrono::Utc::now().to_rfc3339(),
            "Security event logged"
        );
    }
}
