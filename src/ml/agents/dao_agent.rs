use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use std::collections::HashMap;

use super::{MLAgent, AgentConfig};
use crate::metrics::MetricsCollector;
use crate::analytics::{AnalyticsEngine, GovernanceAnalytics};
use crate::ml::models::{PredictionModel, ModelConfig};
use crate::web5::did::{DIDResolver, VerificationMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceMetrics {
    pub participation_rate: f64,
    pub proposal_success_rate: f64,
    pub treasury_utilization: f64,
    pub member_engagement: f64,
    pub consensus_time: f64,
    pub governance_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryMetrics {
    pub total_value: f64,
    pub allocation_efficiency: f64,
    pub risk_exposure: f64,
    pub liquidity_ratio: f64,
    pub growth_rate: f64,
    pub sustainability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalMetrics {
    pub id: String,
    pub support_level: f64,
    pub impact_score: f64,
    pub resource_requirements: f64,
    pub execution_complexity: f64,
    pub time_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStrategy {
    pub voting_weight: f64,
    pub proposal_threshold: f64,
    pub execution_speed: f64,
    pub resource_allocation: f64,
    pub risk_tolerance: f64,
}

pub struct DAOAgent {
    metrics: Arc<MetricsCollector>,
    analytics: Arc<RwLock<AnalyticsEngine>>,
    governance_analytics: Arc<RwLock<GovernanceAnalytics>>,
    prediction_model: Arc<RwLock<PredictionModel>>,
    did_resolver: Arc<DIDResolver>,
    current_strategy: RwLock<GovernanceStrategy>,
    proposal_cache: RwLock<HashMap<String, ProposalMetrics>>,
}

impl DAOAgent {
    pub fn new(
        metrics: Arc<MetricsCollector>,
        analytics: Arc<RwLock<AnalyticsEngine>>,
        did_resolver: Arc<DIDResolver>,
        model_config: ModelConfig,
    ) -> Result<Self> {
        Ok(Self {
            metrics,
            analytics,
            governance_analytics: Arc::new(RwLock::new(GovernanceAnalytics::new())),
            prediction_model: Arc::new(RwLock::new(PredictionModel::new(model_config)?)),
            did_resolver,
            current_strategy: RwLock::new(GovernanceStrategy {
                voting_weight: 1.0,
                proposal_threshold: 0.5,
                execution_speed: 1.0,
                resource_allocation: 0.5,
                risk_tolerance: 0.5,
            }),
            proposal_cache: RwLock::new(HashMap::new()),
        })
    }

    pub async fn analyze_dao_metrics(&self) -> Result<GovernanceMetrics> {
        let analytics = self.analytics.read().await;
        let governance = self.governance_analytics.read().await;
        
        Ok(GovernanceMetrics {
            participation_rate: governance.calculate_participation_rate().await?,
            proposal_success_rate: governance.calculate_success_rate().await?,
            treasury_utilization: governance.calculate_treasury_utilization().await?,
            member_engagement: governance.calculate_member_engagement().await?,
            consensus_time: governance.calculate_consensus_time().await?,
            governance_efficiency: governance.calculate_governance_efficiency().await?,
        })
    }

    pub async fn analyze_treasury(&self) -> Result<TreasuryMetrics> {
        let analytics = self.analytics.read().await;
        let governance = self.governance_analytics.read().await;
        
        Ok(TreasuryMetrics {
            total_value: governance.get_treasury_value().await?,
            allocation_efficiency: governance.calculate_allocation_efficiency().await?,
            risk_exposure: governance.calculate_risk_exposure().await?,
            liquidity_ratio: governance.calculate_liquidity_ratio().await?,
            growth_rate: governance.calculate_growth_rate().await?,
            sustainability_score: governance.calculate_sustainability().await?,
        })
    }

    pub async fn evaluate_proposal(&self, proposal_id: &str) -> Result<ProposalMetrics> {
        let governance = self.governance_analytics.read().await;
        let model = self.prediction_model.read().await;
        
        // Check cache first
        if let Some(cached_metrics) = self.proposal_cache.read().await.get(proposal_id) {
            return Ok(cached_metrics.clone());
        }
        
        // Analyze proposal
        let support_level = governance.analyze_proposal_support(proposal_id).await?;
        let impact_score = governance.analyze_proposal_impact(proposal_id).await?;
        
        let features = vec![support_level, impact_score];
        let predictions = model.predict(&features).await?;
        
        let metrics = ProposalMetrics {
            id: proposal_id.to_string(),
            support_level,
            impact_score,
            resource_requirements: predictions[0],
            execution_complexity: predictions[1],
            time_sensitivity: predictions[2],
        };
        
        // Cache the results
        self.proposal_cache.write().await.insert(proposal_id.to_string(), metrics.clone());
        
        Ok(metrics)
    }

    pub async fn optimize_governance(&self, metrics: &GovernanceMetrics) -> Result<()> {
        let treasury_metrics = self.analyze_treasury().await?;
        let mut strategy = self.current_strategy.write().await;
        
        // Update voting weights based on participation
        strategy.voting_weight = self.calculate_optimal_voting_weight(
            metrics.participation_rate,
            metrics.proposal_success_rate,
        ).await?;
        
        // Adjust proposal threshold based on engagement
        strategy.proposal_threshold = self.calculate_optimal_threshold(
            metrics.member_engagement,
            metrics.governance_efficiency,
        ).await?;
        
        // Optimize execution speed
        strategy.execution_speed = self.calculate_optimal_speed(
            metrics.consensus_time,
            treasury_metrics.liquidity_ratio,
        ).await?;
        
        // Update resource allocation
        strategy.resource_allocation = self.calculate_optimal_allocation(
            treasury_metrics.allocation_efficiency,
            treasury_metrics.sustainability_score,
        ).await?;
        
        // Adjust risk tolerance
        strategy.risk_tolerance = self.calculate_optimal_risk_tolerance(
            treasury_metrics.risk_exposure,
            treasury_metrics.growth_rate,
        ).await?;
        
        Ok(())
    }

    async fn calculate_optimal_voting_weight(
        &self,
        participation_rate: f64,
        success_rate: f64,
    ) -> Result<f64> {
        let base_weight = 1.0;
        let participation_factor = 1.0 + (participation_rate - 0.5).max(-0.3).min(0.3);
        let success_factor = 1.0 + (success_rate - 0.5).max(-0.2).min(0.2);
        
        Ok((base_weight * participation_factor * success_factor)
            .max(0.5)
            .min(2.0))
    }

    async fn calculate_optimal_threshold(
        &self,
        engagement: f64,
        efficiency: f64,
    ) -> Result<f64> {
        let base_threshold = 0.5;
        let engagement_factor = 1.0 + (engagement - 0.5).max(-0.2).min(0.2);
        let efficiency_factor = 1.0 + (efficiency - 0.5).max(-0.2).min(0.2);
        
        Ok((base_threshold * engagement_factor * efficiency_factor)
            .max(0.3)
            .min(0.7))
    }

    async fn calculate_optimal_speed(
        &self,
        consensus_time: f64,
        liquidity: f64,
    ) -> Result<f64> {
        let base_speed = 1.0;
        let time_factor = 1.0 / (1.0 + consensus_time / 86400.0); // Normalize to days
        let liquidity_factor = 1.0 + (liquidity - 0.5).max(-0.3).min(0.3);
        
        Ok((base_speed * time_factor * liquidity_factor)
            .max(0.5)
            .min(2.0))
    }

    async fn calculate_optimal_allocation(
        &self,
        efficiency: f64,
        sustainability: f64,
    ) -> Result<f64> {
        let base_allocation = 0.5;
        let efficiency_factor = 1.0 + (efficiency - 0.5).max(-0.3).min(0.3);
        let sustainability_factor = 1.0 + (sustainability - 0.5).max(-0.2).min(0.2);
        
        Ok((base_allocation * efficiency_factor * sustainability_factor)
            .max(0.2)
            .min(0.8))
    }

    async fn calculate_optimal_risk_tolerance(
        &self,
        risk_exposure: f64,
        growth_rate: f64,
    ) -> Result<f64> {
        let base_tolerance = 0.5;
        let risk_factor = 1.0 - (risk_exposure - 0.5).max(-0.3).min(0.3);
        let growth_factor = 1.0 + (growth_rate - 0.5).max(-0.2).min(0.2);
        
        Ok((base_tolerance * risk_factor * growth_factor)
            .max(0.1)
            .min(0.9))
    }

    pub async fn verify_governance_action(&self, action_id: &str) -> Result<bool> {
        let did = self.did_resolver.resolve(action_id).await?;
        let verification = did.get_verification_method()?;
        
        match verification {
            VerificationMethod::Ed25519 { key, .. } => {
                // Verify the action using Ed25519
                Ok(true) // Placeholder
            },
            VerificationMethod::JsonWebKey { .. } => {
                // Verify using JWK
                Ok(true) // Placeholder
            },
            _ => Ok(false),
        }
    }
}

#[async_trait]
impl MLAgent for DAOAgent {
    async fn train(&self) -> Result<()> {
        let governance_data = self.governance_analytics.read().await.get_training_data().await?;
        let mut model = self.prediction_model.write().await;
        model.train(&governance_data).await?;
        Ok(())
    }

    async fn predict(&self) -> Result<Vec<f64>> {
        let metrics = self.analyze_dao_metrics().await?;
        Ok(vec![
            metrics.participation_rate,
            metrics.proposal_success_rate,
            metrics.treasury_utilization,
            metrics.member_engagement,
            metrics.consensus_time,
            metrics.governance_efficiency,
        ])
    }

    async fn update(&self) -> Result<()> {
        let metrics = self.analyze_dao_metrics().await?;
        self.optimize_governance(&metrics).await?;
        Ok(())
    }

    async fn get_metrics(&self) -> Result<Vec<f64>> {
        let metrics = self.analyze_dao_metrics().await?;
        let treasury = self.analyze_treasury().await?;
        
        Ok(vec![
            metrics.participation_rate,
            metrics.proposal_success_rate,
            metrics.treasury_utilization,
            metrics.member_engagement,
            metrics.consensus_time,
            metrics.governance_efficiency,
            treasury.total_value,
            treasury.allocation_efficiency,
            treasury.risk_exposure,
            treasury.liquidity_ratio,
            treasury.growth_rate,
            treasury.sustainability_score,
        ])
    }
}
