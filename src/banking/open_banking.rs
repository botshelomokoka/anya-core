use crate::privacy::zksnarks::ZKSnarkSystem;
use crate::security::enhanced_security::SecurityModule;
use crate::metrics::{counter, gauge};
use thiserror::Error;
use log::{info, warn, error};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum OpenBankingError {
    #[error("Authentication failed: {0}")]
    AuthError(String),
    #[error("API request failed: {0}")]
    APIError(String),
    #[error("Data validation failed: {0}")]
    ValidationError(String),
    #[error("Privacy constraint violation: {0}")]
    PrivacyError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    account_id: String,
    account_type: AccountType,
    balance: f64,
    currency: String,
    status: AccountStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Savings,
    Investment,
    Custody,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Inactive,
    Frozen,
}

pub struct OpenBankingAPI {
    client: Client,
    security: Arc<SecurityModule>,
    zk_system: Arc<ZKSnarkSystem>,
    metrics: BankingMetrics,
    base_url: String,
    api_key: String,
}

impl OpenBankingAPI {
    pub fn new(
        security: Arc<SecurityModule>,
        zk_system: Arc<ZKSnarkSystem>,
        base_url: String,
        api_key: String,
    ) -> Self {
        Self {
            client: Client::new(),
            security,
            zk_system,
            metrics: BankingMetrics::new(),
            base_url,
            api_key,
        }
    }

    pub async fn get_accounts(&self, user_id: &str) -> Result<Vec<BankAccount>, OpenBankingError> {
        // Generate ZK proof for user authentication
        let auth_proof = self.zk_system.create_proof(&[
            user_id.as_bytes(),
            &chrono::Utc::now().timestamp().to_le_bytes(),
        ]).map_err(|e| OpenBankingError::PrivacyError(e.to_string()))?;

        // Make authenticated API request
        let response = self.client.get(&format!("{}/accounts", self.base_url))
            .header("Authorization", &self.api_key)
            .header("X-User-Id", user_id)
            .header("X-Auth-Proof", hex::encode(&auth_proof))
            .send()
            .await
            .map_err(|e| OpenBankingError::APIError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(OpenBankingError::APIError(format!("API request failed: {}", response.status())));
        }

        let accounts: Vec<BankAccount> = response.json()
            .await
            .map_err(|e| OpenBankingError::APIError(e.to_string()))?;

        self.metrics.record_successful_request();
        Ok(accounts)
    }

    pub async fn initiate_payment(&self, 
        from_account: &str, 
        to_account: &str, 
        amount: f64,
        currency: &str,
    ) -> Result<PaymentStatus, OpenBankingError> {
        // Validate payment parameters
        self.validate_payment(from_account, to_account, amount)?;

        // Create payment request with ZK proof
        let payment_proof = self.create_payment_proof(from_account, to_account, amount)?;

        let payment_request = PaymentRequest {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
            amount,
            currency: currency.to_string(),
            proof: payment_proof,
        };

        // Submit payment request
        let response = self.client.post(&format!("{}/payments", self.base_url))
            .header("Authorization", &self.api_key)
            .json(&payment_request)
            .send()
            .await
            .map_err(|e| OpenBankingError::APIError(e.to_string()))?;

        if !response.status().is_success() {
            self.metrics.record_failed_payment();
            return Err(OpenBankingError::APIError("Payment initiation failed".into()));
        }

        let status: PaymentStatus = response.json()
            .await
            .map_err(|e| OpenBankingError::APIError(e.to_string()))?;

        self.metrics.record_successful_payment(amount);
        Ok(status)
    }

    fn validate_payment(&self, from_account: &str, to_account: &str, amount: f64) -> Result<(), OpenBankingError> {
        if amount <= 0.0 {
            return Err(OpenBankingError::ValidationError("Invalid amount".into()));
        }

        if from_account == to_account {
            return Err(OpenBankingError::ValidationError("Same account transfer not allowed".into()));
        }

        Ok(())
    }

    fn create_payment_proof(&self, from_account: &str, to_account: &str, amount: f64) -> Result<Vec<u8>, OpenBankingError> {
        self.zk_system.create_proof(&[
            from_account.as_bytes(),
            to_account.as_bytes(),
            &amount.to_le_bytes(),
        ]).map_err(|e| OpenBankingError::PrivacyError(e.to_string()))
    }
}

#[derive(Debug, Serialize)]
struct PaymentRequest {
    from_account: String,
    to_account: String,
    amount: f64,
    currency: String,
    proof: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentStatus {
    pub transaction_id: String,
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

struct BankingMetrics {
    successful_requests: Counter,
    failed_requests: Counter,
    successful_payments: Counter,
    failed_payments: Counter,
    total_payment_volume: Gauge,
}

impl BankingMetrics {
    fn new() -> Self {
        Self {
            successful_requests: counter!("banking_requests_successful_total"),
            failed_requests: counter!("banking_requests_failed_total"),
            successful_payments: counter!("banking_payments_successful_total"),
            failed_payments: counter!("banking_payments_failed_total"),
            total_payment_volume: gauge!("banking_payment_volume_total"),
        }
    }

    fn record_successful_request(&self) {
        self.successful_requests.increment(1);
    }

    fn record_failed_request(&self) {
        self.failed_requests.increment(1);
    }

    fn record_successful_payment(&self, amount: f64) {
        self.successful_payments.increment(1);
        self.total_payment_volume.add(amount);
    }

    fn record_failed_payment(&self) {
        self.failed_payments.increment(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_accounts() {
        let api = setup_test_api();
        let accounts = api.get_accounts("test_user").await;
        assert!(accounts.is_ok());
    }

    #[tokio::test]
    async fn test_payment_validation() {
        let api = setup_test_api();
        let result = api.validate_payment("acc1", "acc2", 100.0);
        assert!(result.is_ok());

        let result = api.validate_payment("acc1", "acc1", 100.0);
        assert!(result.is_err());

        let result = api.validate_payment("acc1", "acc2", -100.0);
        assert!(result.is_err());
    }
}
