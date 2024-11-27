//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use super::{IdentityError, Credential, CredentialProof};
use crate::auth::web5::metrics::identity::IdentityMetrics;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use did_key::{DIDCore, Ed25519KeyPair};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct VerificationManager {
    db: PgPool,
    metrics: IdentityMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub credential_id: String,
    pub verifier: String,
    pub timestamp: DateTime<Utc>,
    pub is_valid: bool,
    pub verification_method: String,
    pub metadata: Option<serde_json::Value>,
}

impl VerificationManager {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            metrics: IdentityMetrics::new(),
        }
    }

    pub async fn initialize(&self) -> Result<(), IdentityError> {
        // Initialize verification tables if needed
        Ok(())
    }

    pub async fn verify_credential(
        &self,
        credential: &Credential,
        verifier: &Ed25519KeyPair,
    ) -> Result<VerificationResult, IdentityError> {
        let start = std::time::Instant::now();
        
        // Check expiration
        if let Some(expires_at) = credential.expires_at {
            if expires_at < Utc::now() {
                self.metrics.failed_verifications.increment(1);
                return Err(IdentityError::CredentialExpired);
            }
        }

        // Verify proof
        let result = self.verify_proof(credential, &credential.proof).await?;

        // Record verification
        let verification = VerificationResult {
            credential_id: credential.id.clone(),
            verifier: verifier.get_did().to_string(),
            timestamp: Utc::now(),
            is_valid: result,
            verification_method: "Ed25519VerificationKey2020".to_string(),
            metadata: None,
        };

        // Store verification result
        sqlx::query!(
            r#"
            INSERT INTO verification_records 
            (credential_id, verifier_did, verification_method, verified_at, verification_result, proof)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            credential.id,
            verification.verifier,
            verification.verification_method,
            verification.timestamp,
            verification.is_valid,
            serde_json::to_value(&credential.proof)?,
        )
        .execute(&self.db)
        .await?;

        // Record metrics
        self.metrics.verification_duration
            .record(start.elapsed().as_secs_f64(), &[]);
        self.metrics.credential_verifications.increment(1);
        if !result {
            self.metrics.failed_verifications.increment(1);
        }

        Ok(verification)
    }

    async fn verify_proof(
        &self,
        credential: &Credential,
        proof: &CredentialProof,
    ) -> Result<bool, IdentityError> {
        // Resolve issuer DID
        let issuer_did = credential.issuer.parse::<did_key::DID>()?;
        let verification_key = issuer_did.resolve().await?;

        // Verify signature
        let proof_bytes = hex::decode(&proof.proof_value)?;
        let message = credential.claims.to_string();

        verification_key.verify(message.as_bytes(), &proof_bytes)
            .map_err(|e| IdentityError::VerificationError(e.to_string()))
    }

    pub async fn get_verification_history(
        &self,
        credential_id: &str,
    ) -> Result<Vec<VerificationResult>, IdentityError> {
        let records = sqlx::query_as!(
            VerificationResult,
            r#"
            SELECT 
                credential_id,
                verifier_did as verifier,
                verified_at as timestamp,
                verification_result as is_valid,
                verification_method,
                metadata
            FROM verification_records
            WHERE credential_id = $1
            ORDER BY verified_at DESC
            "#,
            credential_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(records)
    }
}


