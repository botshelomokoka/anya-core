use std::env;
use std::time::{Duration, SystemTime};
use reqwest;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub key: String,
    pub features: Vec<String>,
    pub expiration: String,
}

#[derive(Error, Debug)]
pub enum LicenseError {
    #[error("Failed to verify license: {0}")]
    VerificationFailed(String),
    #[error("License expired")]
    Expired,
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Environment variable not set: {0}")]
    EnvVarNotSet(String),
}

const GRACE_PERIOD_DAYS: u64 = 7;
const OFFLINE_VERIFICATION_FILE: &str = "offline_license.json";

pub async fn verify_license(license_key: &str) -> Result<License, LicenseError> {
    let license_server_url = env::var("LICENSE_SERVER_URL")
        .map_err(|_| LicenseError::EnvVarNotSet("LICENSE_SERVER_URL".to_string()))?;

    match online_verification(&license_server_url, license_key).await {
        Ok(license) => {
            info!("License verified successfully");
            save_offline_license(&license)?;
            Ok(license)
        }
        Err(e) => {
            warn!("Online license verification failed: {}. Attempting offline verification.", e);
            offline_verification()
        }
    }
}

async fn online_verification(license_server_url: &str, license_key: &str) -> Result<License, LicenseError> {
    let client = reqwest::Client::new();
    let response = client
        .post(license_server_url)
        .json(&serde_json::json!({ "licenseKey": license_key }))
        .send()
        .await?;

    if response.status().is_success() {
        let license: License = response.json().await?;
        Ok(license)
    } else {
        Err(LicenseError::VerificationFailed(response.status().to_string()))
    }
}

fn offline_verification() -> Result<License, LicenseError> {
    let offline_license = std::fs::read_to_string(OFFLINE_VERIFICATION_FILE)
        .map_err(|e| LicenseError::VerificationFailed(format!("Failed to read offline license: {}", e)))?;

    let license: License = serde_json::from_str(&offline_license)
        .map_err(|e| LicenseError::VerificationFailed(format!("Failed to parse offline license: {}", e)))?;

    let expiration = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * GRACE_PERIOD_DAYS);
    let expiration = expiration.duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| LicenseError::VerificationFailed(format!("Failed to calculate expiration: {}", e)))?;

    if expiration.as_secs().to_string() < license.expiration {
        Ok(license)
    } else {
        Err(LicenseError::Expired)
    }
}

fn save_offline_license(license: &License) -> Result<(), LicenseError> {
    let offline_license = serde_json::to_string(license)
        .map_err(|e| LicenseError::VerificationFailed(format!("Failed to serialize license: {}", e)))?;

    std::fs::write(OFFLINE_VERIFICATION_FILE, offline_license)
        .map_err(|e| LicenseError::VerificationFailed(format!("Failed to save offline license: {}", e)))?;

    Ok(())
}