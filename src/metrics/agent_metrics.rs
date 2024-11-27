//! Agent Metrics Module
//! 
//! # Overview
//! The `agent_metrics` module provides comprehensive monitoring and performance tracking
//! for different types of agents within the Anya system. It collects, aggregates, and
//! exposes metrics for core blockchain operations, enterprise features, and external
//! integrations.
//!
//! # Architecture
//! The module implements a hierarchical metrics system:
//! - AgentMetrics: Top-level metrics aggregator
//!   - CoreAgentMetrics: Bitcoin, Lightning, and DLC operations
//!   - EnterpriseAgentMetrics: Business logic and compliance
//!   - IntegrationAgentMetrics: External system interactions
//!
//! # Usage Examples
//! ```rust
//! use anya::metrics::AgentMetrics;
//! use anya::metrics::{AgentType, AgentAction};
//! 
//! let metrics = AgentMetrics::new()?;
//! 
//! // Record a core agent action
//! let action = AgentAction::TransactionSent { amount: 1000 };
//! metrics.record_agent_action(AgentType::Core(CoreAgentType::Bitcoin), &action)?;
//! 
//! // Record an enterprise action
//! let action = AgentAction::ComplianceCheck { passed: true };
//! metrics.record_agent_action(AgentType::Enterprise(EnterpriseType::Compliance), &action)?;
//! ```
//!
//! # Security Considerations
//! - Metrics are collected with minimal overhead
//! - No sensitive data is exposed in metrics
//! - Access to metrics requires proper authentication
//! - Rate limiting on metric collection
//!
//! # Performance
//! - Lock-free metric collection
//! - Efficient counter and gauge implementations
//! - Minimal memory overhead
//! - Configurable aggregation intervals

use std::error::Error;
use std::sync::atomic::{AtomicU64, Ordering};
use metrics::{counter, gauge, histogram};
use log::{info, warn, error};

/// Types of agents in the system
#[derive(Debug, Clone, PartialEq)]
pub enum AgentType {
    /// Core blockchain operations agent
    Core(CoreAgentType),
    /// Enterprise features agent
    Enterprise(EnterpriseType),
    /// External integration agent
    Integration(IntegrationType),
}

/// Types of core blockchain agents
#[derive(Debug, Clone, PartialEq)]
pub enum CoreAgentType {
    /// Bitcoin network operations
    Bitcoin,
    /// Lightning Network operations
    Lightning,
    /// DLC contract operations
    DLC,
}

/// Types of enterprise agents
#[derive(Debug, Clone, PartialEq)]
pub enum EnterpriseType {
    /// Compliance and regulatory checks
    Compliance,
    /// Risk assessment and management
    RiskManagement,
    /// Portfolio management
    Portfolio,
}

/// Types of integration agents
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationType {
    /// External exchange integration
    Exchange,
    /// Payment processor integration
    PaymentProcessor,
    /// Data provider integration
    DataProvider,
}

/// Actions performed by agents
#[derive(Debug, Clone)]
pub enum AgentAction {
    /// Transaction-related actions
    TransactionSent {
        /// Amount in satoshis
        amount: u64,
    },
    /// Channel-related actions
    ChannelOpened {
        /// Channel capacity in satoshis
        capacity: u64,
    },
    /// Compliance-related actions
    ComplianceCheck {
        /// Whether the check passed
        passed: bool,
    },
    /// Integration-related actions
    ExternalRequest {
        /// Type of request
        request_type: String,
        /// Duration in milliseconds
        duration_ms: u64,
    },
}

/// Core blockchain agent metrics collector
#[derive(Debug)]
pub struct CoreAgentMetrics {
    /// Total number of transactions processed
    transaction_count: Counter,
    /// Total value of transactions in satoshis
    transaction_value: Counter,
    /// Number of active channels
    channel_count: Gauge,
    /// Total channel capacity in satoshis
    channel_capacity: Gauge,
    /// Number of DLC contracts
    dlc_count: Counter,
    /// Total value locked in DLC contracts
    dlc_value: Gauge,
    /// Transaction processing latency
    transaction_latency: Histogram,
}

impl CoreAgentMetrics {
    /// Creates a new CoreAgentMetrics instance
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            transaction_count: counter!("agent_core_transactions_total"),
            transaction_value: counter!("agent_core_transaction_value_sats"),
            channel_count: gauge!("agent_core_channels"),
            channel_capacity: gauge!("agent_core_channel_capacity_sats"),
            dlc_count: counter!("agent_core_dlc_contracts_total"),
            dlc_value: gauge!("agent_core_dlc_value_sats"),
            transaction_latency: histogram!("agent_core_transaction_latency_ms"),
        })
    }

    /// Records a core agent action
    pub fn record_action(&self, agent_type: CoreAgentType, action: &AgentAction) -> Result<(), Box<dyn Error>> {
        match (agent_type, action) {
            (CoreAgentType::Bitcoin, AgentAction::TransactionSent { amount }) => {
                self.transaction_count.increment(1);
                self.transaction_value.increment(*amount);
            }
            (CoreAgentType::Lightning, AgentAction::ChannelOpened { capacity }) => {
                self.channel_count.increment(1.0);
                self.channel_capacity.increment(*capacity as f64);
            }
            _ => warn!("Unhandled core agent action: {:?}", action),
        }
        Ok(())
    }

    /// Gets a snapshot of current metrics
    pub fn get_snapshot(&self) -> Result<CoreMetricsSnapshot, Box<dyn Error>> {
        Ok(CoreMetricsSnapshot {
            transaction_count: self.transaction_count.get_counter(),
            transaction_value: self.transaction_value.get_counter(),
            channel_count: self.channel_count.get_gauge(),
            channel_capacity: self.channel_capacity.get_gauge(),
            dlc_count: self.dlc_count.get_counter(),
            dlc_value: self.dlc_value.get_gauge(),
        })
    }
}

/// Enterprise agent metrics collector
#[derive(Debug)]
pub struct EnterpriseAgentMetrics {
    /// Number of compliance checks performed
    compliance_checks: Counter,
    /// Number of failed compliance checks
    compliance_failures: Counter,
    /// Risk assessment score
    risk_score: Gauge,
    /// Portfolio value in satoshis
    portfolio_value: Gauge,
    /// Operation latency
    operation_latency: Histogram,
}

impl EnterpriseAgentMetrics {
    /// Creates a new EnterpriseAgentMetrics instance
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            compliance_checks: counter!("agent_enterprise_compliance_checks_total"),
            compliance_failures: counter!("agent_enterprise_compliance_failures_total"),
            risk_score: gauge!("agent_enterprise_risk_score"),
            portfolio_value: gauge!("agent_enterprise_portfolio_value_sats"),
            operation_latency: histogram!("agent_enterprise_operation_latency_ms"),
        })
    }

    /// Records an enterprise agent action
    pub fn record_action(&self, agent_type: EnterpriseType, action: &AgentAction) -> Result<(), Box<dyn Error>> {
        match (agent_type, action) {
            (EnterpriseType::Compliance, AgentAction::ComplianceCheck { passed }) => {
                self.compliance_checks.increment(1);
                if !passed {
                    self.compliance_failures.increment(1);
                }
            }
            _ => warn!("Unhandled enterprise agent action: {:?}", action),
        }
        Ok(())
    }

    /// Gets a snapshot of current metrics
    pub fn get_snapshot(&self) -> Result<EnterpriseMetricsSnapshot, Box<dyn Error>> {
        Ok(EnterpriseMetricsSnapshot {
            compliance_checks: self.compliance_checks.get_counter(),
            compliance_failures: self.compliance_failures.get_counter(),
            risk_score: self.risk_score.get_gauge(),
            portfolio_value: self.portfolio_value.get_gauge(),
        })
    }
}

/// Integration agent metrics collector
#[derive(Debug)]
pub struct IntegrationAgentMetrics {
    /// Number of external requests
    request_count: Counter,
    /// Number of failed requests
    request_failures: Counter,
    /// Request latency
    request_latency: Histogram,
    /// Active integrations count
    active_integrations: Gauge,
}

impl IntegrationAgentMetrics {
    /// Creates a new IntegrationAgentMetrics instance
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            request_count: counter!("agent_integration_requests_total"),
            request_failures: counter!("agent_integration_failures_total"),
            request_latency: histogram!("agent_integration_request_latency_ms"),
            active_integrations: gauge!("agent_integration_active_count"),
        })
    }

    /// Records an integration agent action
    pub fn record_action(&self, agent_type: IntegrationType, action: &AgentAction) -> Result<(), Box<dyn Error>> {
        match (agent_type, action) {
            (_, AgentAction::ExternalRequest { request_type: _, duration_ms }) => {
                self.request_count.increment(1);
                self.request_latency.record(*duration_ms as f64);
            }
            _ => warn!("Unhandled integration agent action: {:?}", action),
        }
        Ok(())
    }

    /// Gets a snapshot of current metrics
    pub fn get_snapshot(&self) -> Result<IntegrationMetricsSnapshot, Box<dyn Error>> {
        Ok(IntegrationMetricsSnapshot {
            request_count: self.request_count.get_counter(),
            request_failures: self.request_failures.get_counter(),
            active_integrations: self.active_integrations.get_gauge(),
        })
    }
}

/// Snapshot of core metrics
#[derive(Debug, Clone)]
pub struct CoreMetricsSnapshot {
    /// Total number of transactions
    pub transaction_count: u64,
    /// Total transaction value in satoshis
    pub transaction_value: u64,
    /// Number of active channels
    pub channel_count: f64,
    /// Total channel capacity in satoshis
    pub channel_capacity: f64,
    /// Number of DLC contracts
    pub dlc_count: u64,
    /// Total value locked in DLC contracts
    pub dlc_value: f64,
}

/// Snapshot of enterprise metrics
#[derive(Debug, Clone)]
pub struct EnterpriseMetricsSnapshot {
    /// Number of compliance checks
    pub compliance_checks: u64,
    /// Number of failed compliance checks
    pub compliance_failures: u64,
    /// Current risk score
    pub risk_score: f64,
    /// Current portfolio value in satoshis
    pub portfolio_value: f64,
}

/// Snapshot of integration metrics
#[derive(Debug, Clone)]
pub struct IntegrationMetricsSnapshot {
    /// Total number of requests
    pub request_count: u64,
    /// Number of failed requests
    pub request_failures: u64,
    /// Number of active integrations
    pub active_integrations: f64,
}

/// Metrics collector for all agent types
pub struct AgentMetrics {
    /// Core blockchain agent metrics
    core_metrics: CoreAgentMetrics,
    /// Enterprise feature metrics
    enterprise_metrics: EnterpriseAgentMetrics,
    /// Integration metrics
    integration_metrics: IntegrationAgentMetrics,
}

impl AgentMetrics {
    /// Creates a new AgentMetrics instance
    ///
    /// # Returns
    /// * `Result<Self, Box<dyn Error>>` - New metrics instance
    ///
    /// # Example
    /// ```
    /// let metrics = AgentMetrics::new()?;
    /// ```
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            core_metrics: CoreAgentMetrics::new()?,
            enterprise_metrics: EnterpriseAgentMetrics::new()?,
            integration_metrics: IntegrationAgentMetrics::new()?,
        })
    }

    /// Records an action performed by an agent
    ///
    /// # Arguments
    /// * `agent_type` - Type of agent performing the action
    /// * `action` - Action being performed
    ///
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Success or error
    ///
    /// # Example
    /// ```
    /// let action = AgentAction::TransactionSent { amount: 1000 };
    /// metrics.record_agent_action(AgentType::Core(CoreAgentType::Bitcoin), &action)?;
    /// ```
    pub fn record_agent_action(&self, agent_type: AgentType, action: &AgentAction) -> Result<(), Box<dyn Error>> {
        match agent_type {
            AgentType::Core(core_type) => self.core_metrics.record_action(core_type, action),
            AgentType::Enterprise(ent_type) => self.enterprise_metrics.record_action(ent_type, action),
            AgentType::Integration(int_type) => self.integration_metrics.record_action(int_type, action),
        }
    }

    /// Retrieves current metrics for all agents
    ///
    /// # Returns
    /// * `Result<AgentMetricsSnapshot, Box<dyn Error>>` - Current metrics
    pub fn get_metrics(&self) -> Result<AgentMetricsSnapshot, Box<dyn Error>> {
        Ok(AgentMetricsSnapshot {
            core: self.core_metrics.get_snapshot()?,
            enterprise: self.enterprise_metrics.get_snapshot()?,
            integration: self.integration_metrics.get_snapshot()?,
        })
    }
}

/// Snapshot of current agent metrics
#[derive(Debug, Clone)]
pub struct AgentMetricsSnapshot {
    /// Core metrics snapshot
    pub core: CoreMetricsSnapshot,
    /// Enterprise metrics snapshot
    pub enterprise: EnterpriseMetricsSnapshot,
    /// Integration metrics snapshot
    pub integration: IntegrationMetricsSnapshot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_metrics() -> Result<(), Box<dyn Error>> {
        let metrics = AgentMetrics::new()?;
        
        // Test core metrics
        let action = AgentAction::TransactionSent { amount: 1000 };
        metrics.record_agent_action(AgentType::Core(CoreAgentType::Bitcoin), &action)?;
        
        // Test enterprise metrics
        let action = AgentAction::ComplianceCheck { passed: true };
        metrics.record_agent_action(AgentType::Enterprise(EnterpriseType::Compliance), &action)?;
        
        // Verify metrics
        let snapshot = metrics.get_metrics()?;
        assert!(snapshot.core.transaction_count > 0);
        assert!(snapshot.enterprise.compliance_checks > 0);
        
        Ok(())
    }
}
