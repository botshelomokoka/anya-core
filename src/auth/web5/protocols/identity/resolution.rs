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
use crate::auth::web5::metrics::identity::IdentityMetrics;
use did_key::{DIDCore, DID, KeyMaterial};
use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct ResolutionManager {
    db: PgPool,
    metrics: IdentityMetrics,
    cache_duration: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub did: String,
    pub document: serde_json::Value,
    pub metadata: ResolutionMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolutionMetadata {
    pub resolved: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub method: String,
    pub driver: String,
}

impl ResolutionManager {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            metrics: IdentityMetrics::new(),
            cache_duration: Duration::hours(24), // 24 hour cache by default
        }
    }

    pub async fn initialize(&self) -> Result<(), IdentityError> {
        // Initialize resolution cache if needed
        Ok(())
    }

    pub async fn resolve_did(&self, did: &str) -> Result<ResolutionResult, IdentityError> {
        let start = std::time::Instant::now();
        
        // Check cache first
        if let Some(cached) = self.get_cached_resolution(did).await? {
            if cached.metadata.valid_until > Utc::now() {
                return Ok(cached);
            }
        }

        // Perform resolution
        let did = did.parse::<DID>()?;
        let document = did.resolve().await?;

        let result = ResolutionResult {
            did: did.to_string(),
            document: serde_json::to_value(&document)?,
            metadata: ResolutionMetadata {
                resolved: Utc::now(),
                valid_until: Utc::now() + self.cache_duration,
                method: did.method().to_string(),
                driver: "did-key".to_string(),
            },
        };

        // Cache the result
        self.cache_resolution(&result).await?;

        // Record metrics
        self.metrics.did_resolutions.increment(1);
        self.metrics.verification_duration
            .record(start.elapsed().as_secs_f64(), &[]);

        Ok(result)
    }

    async fn get_cached_resolution(&self, did: &str) -> Result<Option<ResolutionResult>, IdentityError> {
        let record = sqlx::query!(
            r#"
            SELECT resolution_result, last_resolved, valid_until
            FROM did_resolution_cache
            WHERE did = $1
            "#,
            did,
        )
        .fetch_optional(&self.db)
        .await?;

        if let Some(record) = record {
            let result: ResolutionResult = serde_json::from_value(record.resolution_result)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    async fn cache_resolution(&self, result: &ResolutionResult) -> Result<(), IdentityError> {
        sqlx::query!(
            r#"
            INSERT INTO did_resolution_cache 
            (did, resolution_result, last_resolved, valid_until)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (did) 
            DO UPDATE SET 
                resolution_result = EXCLUDED.resolution_result,
                last_resolved = EXCLUDED.last_resolved,
                valid_until = EXCLUDED.valid_until
            "#,
            result.did,
            serde_json::to_value(&result)?,
            result.metadata.resolved,
            result.metadata.valid_until,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}


