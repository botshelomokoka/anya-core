pub mod subscription;
pub mod compliance;
pub mod analytics;

use bitcoin::Network;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::MobileError;
use crate::SecurityManager;

#[derive(Error, Debug)]
pub enum EnterpriseError {
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
    #[error("Compliance error: {0}")]
    ComplianceError(String),
    #[error("Analytics error: {0}")]
    AnalyticsError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    pub network: Network,
    pub compliance_checks: bool,
    pub analytics_enabled: bool,
    pub subscription_tier: String,
    pub api_key: Option<String>,
    pub webhook_url: Option<String>,
}

pub struct EnterpriseManager {
    config: EnterpriseConfig,
    security_manager: SecurityManager,
    subscription_manager: Option<subscription::SubscriptionManager>,
    compliance_manager: Option<compliance::ComplianceManager>,
    analytics_manager: Option<analytics::AnalyticsManager>,
}

impl EnterpriseManager {
    pub fn new(
        config: EnterpriseConfig,
        security_manager: SecurityManager,
    ) -> Result<Self, MobileError> {
        let subscription_manager = if !config.subscription_tier.is_empty() {
            Some(subscription::SubscriptionManager::new(
                config.network,
                security_manager.clone(),
                subscription::SubscriptionPlan::from_tier(&config.subscription_tier)?,
                config.webhook_url.clone(),
            )?)
        } else {
            None
        };

        let compliance_manager = if config.compliance_checks {
            Some(compliance::ComplianceManager::new(
                config.network,
                security_manager.clone(),
                config.api_key.clone(),
            )?)
        } else {
            None
        };

        let analytics_manager = if config.analytics_enabled {
            Some(analytics::AnalyticsManager::new(
                config.network,
                security_manager.clone(),
            )?)
        } else {
            None
        };

        Ok(Self {
            config,
            security_manager,
            subscription_manager,
            compliance_manager,
            analytics_manager,
        })
    }

    pub async fn check_transaction_compliance(&self, tx_data: &[u8]) -> Result<bool, MobileError> {
        if let Some(compliance) = &self.compliance_manager {
            compliance.check_transaction(tx_data).await
        } else {
            Ok(true)
        }
    }

    pub async fn track_event(&self, event_type: &str, data: serde_json::Value) -> Result<(), MobileError> {
        if let Some(analytics) = &self.analytics_manager {
            analytics.track_event(event_type, data).await
        } else {
            Ok(())
        }
    }

    pub async fn process_subscription(&mut self) -> Result<(), MobileError> {
        if let Some(subscription) = &mut self.subscription_manager {
            subscription.process_streaming_payment().await?;
        }
        Ok(())
    }

    pub async fn validate_enterprise_status(&self) -> Result<bool, MobileError> {
        // Check subscription status
        if let Some(subscription) = &self.subscription_manager {
            if !subscription.is_active() {
                return Ok(false);
            }
        }

        // Check compliance status
        if let Some(compliance) = &self.compliance_manager {
            if !compliance.is_compliant().await? {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
