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
use super::IdentityError;
use crate::auth::web5::data_manager::{DataRecord, Web5DataManager};
use did_key::{DIDCore, Ed25519KeyPair, KeyMaterial};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Debug)]
pub struct CredentialManager {
    db: PgPool,
    data_manager: Web5DataManager,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub issuer: String,
    pub holder: String,
    pub type_: Vec<String>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub claims: serde_json::Value,
    pub proof: CredentialProof,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialProof {
    pub type_: String,
    pub created: DateTime<Utc>,
    pub verification_method: String,
    pub proof_purpose: String,
    pub proof_value: String,
}

impl CredentialManager {
    pub fn new(db: PgPool, data_manager: Web5DataManager) -> Self {
        Self { db, data_manager }
    }

    pub async fn initialize(&self) -> Result<(), IdentityError> {
        // Run any necessary initialization
        Ok(())
    }

    pub async fn issue_credential(
        &self,
        issuer: &Ed25519KeyPair,
        holder: &str,
        claims: serde_json::Value,
        type_: Vec<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<Credential, IdentityError> {
        let credential = Credential {
            id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            issuer: issuer.get_did().to_string(),
            holder: holder.to_string(),
            type_,
            issued_at: Utc::now(),
            expires_at,
            claims,
            proof: self.create_proof(issuer, &claims).await?,
        };

        // Store in database
        sqlx::query!(
            r#"
            INSERT INTO identity_credentials 
            (did, credential_type, credential_data, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            credential.holder,
            credential.type_.join(","),
            serde_json::to_value(&credential)?,
            credential.expires_at,
        )
        .execute(&self.db)
        .await?;

        // Store in Web5 DWN
        let record = DataRecord {
            protocol_id: super::IDENTITY_PROTOCOL_ID.to_string(),
            schema: "IdentityCredential".to_string(),
            data: serde_json::to_value(&credential)?,
            timestamp: Utc::now(),
        };
        self.data_manager.store_data(record).await?;

        Ok(credential)
    }

    async fn create_proof(
        &self,
        issuer: &Ed25519KeyPair,
        claims: &serde_json::Value,
    ) -> Result<CredentialProof, IdentityError> {
        let proof_value = issuer.sign(claims.to_string().as_bytes());
        
        Ok(CredentialProof {
            type_: "Ed25519Signature2020".to_string(),
            created: Utc::now(),
            verification_method: format!("{}#key-1", issuer.get_did()),
            proof_purpose: "assertionMethod".to_string(),
            proof_value: hex::encode(proof_value),
        })
    }

    pub async fn verify_credential(&self, credential: &Credential) -> Result<bool, IdentityError> {
        // Check expiration
        if let Some(expires_at) = credential.expires_at {
            if expires_at < Utc::now() {
                return Ok(false);
            }
        }

        // Verify proof
        let issuer_did = credential.issuer.parse::<did_key::DID>()?;
        let verification_key = issuer_did.resolve().await?;
        
        let proof_bytes = hex::decode(&credential.proof.proof_value)?;
        let message = credential.claims.to_string();
        
        verification_key.verify(message.as_bytes(), &proof_bytes)
            .map_err(|e| IdentityError::VerificationError(e.to_string()))
    }
}


