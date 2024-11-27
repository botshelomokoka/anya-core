use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use std::collections::HashMap;

use super::{MLAgent, AgentConfig};
use crate::metrics::MetricsCollector;
use crate::analytics::{AnalyticsEngine, UserAnalytics};
use crate::ml::models::{PredictionModel, ModelConfig};
use crate::web5::did::{DIDResolver, VerificationMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetrics {
    pub engagement_score: f64,
    pub activity_level: f64,
    pub trust_score: f64,
    pub contribution_value: f64,
    pub interaction_quality: f64,
    pub reputation_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizationProfile {
    pub preferences: HashMap<String, f64>,
    pub interaction_patterns: HashMap<String, f64>,
    pub content_affinities: HashMap<String, f64>,
    pub service_usage: HashMap<String, f64>,
    pub risk_tolerance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorMetrics {
    pub session_duration: f64,
    pub feature_usage: HashMap<String, f64>,
    pub response_patterns: HashMap<String, f64>,
    pub error_frequency: f64,
    pub completion_rates: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStrategy {
    pub personalization_level: f64,
    pub interaction_frequency: f64,
    pub content_depth: f64,
    pub assistance_level: f64,
    pub automation_level: f64,
}

pub struct UserAgent {
    metrics: Arc<MetricsCollector>,
    analytics: Arc<RwLock<AnalyticsEngine>>,
    user_analytics: Arc<RwLock<UserAnalytics>>,
    prediction_model: Arc<RwLock<PredictionModel>>,
    did_resolver: Arc<DIDResolver>,
    current_strategy: RwLock<UserStrategy>,
    user_profiles: RwLock<HashMap<String, PersonalizationProfile>>,
    behavior_cache: RwLock<HashMap<String, BehaviorMetrics>>,
}

impl UserAgent {
    pub fn new(
        metrics: Arc<MetricsCollector>,
        analytics: Arc<RwLock<AnalyticsEngine>>,
        did_resolver: Arc<DIDResolver>,
        model_config: ModelConfig,
    ) -> Result<Self> {
        Ok(Self {
            metrics,
            analytics,
            user_analytics: Arc::new(RwLock::new(UserAnalytics::new())),
            prediction_model: Arc::new(RwLock::new(PredictionModel::new(model_config)?)),
            did_resolver,
            current_strategy: RwLock::new(UserStrategy {
                personalization_level: 0.5,
                interaction_frequency: 1.0,
                content_depth: 0.5,
                assistance_level: 0.5,
                automation_level: 0.5,
            }),
            user_profiles: RwLock::new(HashMap::new()),
            behavior_cache: RwLock::new(HashMap::new()),
        })
    }

    pub async fn analyze_user_metrics(&self, user_id: &str) -> Result<UserMetrics> {
        let analytics = self.analytics.read().await;
        let user = self.user_analytics.read().await;
        
        Ok(UserMetrics {
            engagement_score: user.calculate_engagement(user_id).await?,
            activity_level: user.calculate_activity_level(user_id).await?,
            trust_score: user.calculate_trust_score(user_id).await?,
            contribution_value: user.calculate_contribution_value(user_id).await?,
            interaction_quality: user.calculate_interaction_quality(user_id).await?,
            reputation_score: user.calculate_reputation_score(user_id).await?,
        })
    }

    pub async fn get_personalization_profile(&self, user_id: &str) -> Result<PersonalizationProfile> {
        // Check cache first
        if let Some(profile) = self.user_profiles.read().await.get(user_id) {
            return Ok(profile.clone());
        }

        let analytics = self.analytics.read().await;
        let user = self.user_analytics.read().await;
        
        let profile = PersonalizationProfile {
            preferences: user.analyze_preferences(user_id).await?,
            interaction_patterns: user.analyze_interaction_patterns(user_id).await?,
            content_affinities: user.analyze_content_affinities(user_id).await?,
            service_usage: user.analyze_service_usage(user_id).await?,
            risk_tolerance: user.calculate_risk_tolerance(user_id).await?,
        };

        // Cache the profile
        self.user_profiles.write().await.insert(user_id.to_string(), profile.clone());
        
        Ok(profile)
    }

    pub async fn analyze_behavior(&self, user_id: &str) -> Result<BehaviorMetrics> {
        let analytics = self.analytics.read().await;
        let user = self.user_analytics.read().await;
        
        let metrics = BehaviorMetrics {
            session_duration: user.calculate_session_duration(user_id).await?,
            feature_usage: user.analyze_feature_usage(user_id).await?,
            response_patterns: user.analyze_response_patterns(user_id).await?,
            error_frequency: user.calculate_error_frequency(user_id).await?,
            completion_rates: user.calculate_completion_rates(user_id).await?,
        };

        // Cache behavior metrics
        self.behavior_cache.write().await.insert(user_id.to_string(), metrics.clone());
        
        Ok(metrics)
    }

    pub async fn optimize_user_experience(&self, user_id: &str) -> Result<()> {
        let metrics = self.analyze_user_metrics(user_id).await?;
        let profile = self.get_personalization_profile(user_id).await?;
        let behavior = self.analyze_behavior(user_id).await?;
        
        let mut strategy = self.current_strategy.write().await;
        
        // Update personalization level
        strategy.personalization_level = self.calculate_optimal_personalization(
            &metrics,
            &profile,
            &behavior,
        ).await?;
        
        // Adjust interaction frequency
        strategy.interaction_frequency = self.calculate_optimal_interaction(
            &metrics,
            &profile,
            &behavior,
        ).await?;
        
        // Update content depth
        strategy.content_depth = self.calculate_optimal_content_depth(
            &metrics,
            &profile,
            &behavior,
        ).await?;
        
        // Adjust assistance level
        strategy.assistance_level = self.calculate_optimal_assistance(
            &metrics,
            &profile,
            &behavior,
        ).await?;
        
        // Update automation level
        strategy.automation_level = self.calculate_optimal_automation(
            &metrics,
            &profile,
            &behavior,
        ).await?;
        
        Ok(())
    }

    async fn calculate_optimal_personalization(
        &self,
        metrics: &UserMetrics,
        profile: &PersonalizationProfile,
        behavior: &BehaviorMetrics,
    ) -> Result<f64> {
        let engagement_factor = metrics.engagement_score;
        let preference_diversity = calculate_diversity(&profile.preferences);
        let completion_impact = behavior.completion_rates;
        
        Ok(((engagement_factor + preference_diversity + completion_impact) / 3.0)
            .max(0.1)
            .min(1.0))
    }

    async fn calculate_optimal_interaction(
        &self,
        metrics: &UserMetrics,
        profile: &PersonalizationProfile,
        behavior: &BehaviorMetrics,
    ) -> Result<f64> {
        let activity_factor = metrics.activity_level;
        let pattern_intensity = calculate_intensity(&profile.interaction_patterns);
        let session_impact = behavior.session_duration / 3600.0; // Normalize to hours
        
        Ok(((activity_factor + pattern_intensity + session_impact) / 3.0)
            .max(0.5)
            .min(2.0))
    }

    async fn calculate_optimal_content_depth(
        &self,
        metrics: &UserMetrics,
        profile: &PersonalizationProfile,
        behavior: &BehaviorMetrics,
    ) -> Result<f64> {
        let engagement_factor = metrics.engagement_score;
        let affinity_depth = calculate_average(&profile.content_affinities);
        let feature_usage = calculate_average(&behavior.feature_usage);
        
        Ok(((engagement_factor + affinity_depth + feature_usage) / 3.0)
            .max(0.2)
            .min(1.0))
    }

    async fn calculate_optimal_assistance(
        &self,
        metrics: &UserMetrics,
        profile: &PersonalizationProfile,
        behavior: &BehaviorMetrics,
    ) -> Result<f64> {
        let error_factor = 1.0 - behavior.error_frequency;
        let completion_factor = behavior.completion_rates;
        let complexity_factor = calculate_average(&profile.service_usage);
        
        Ok(((2.0 - error_factor + completion_factor + complexity_factor) / 3.0)
            .max(0.2)
            .min(0.8))
    }

    async fn calculate_optimal_automation(
        &self,
        metrics: &UserMetrics,
        profile: &PersonalizationProfile,
        behavior: &BehaviorMetrics,
    ) -> Result<f64> {
        let trust_factor = metrics.trust_score;
        let usage_complexity = calculate_average(&profile.service_usage);
        let pattern_stability = calculate_stability(&behavior.response_patterns);
        
        Ok(((trust_factor + usage_complexity + pattern_stability) / 3.0)
            .max(0.1)
            .min(0.9))
    }

    pub async fn verify_user_action(&self, user_id: &str, action_id: &str) -> Result<bool> {
        let did = self.did_resolver.resolve(action_id).await?;
        let verification = did.get_verification_method()?;
        
        match verification {
            VerificationMethod::Ed25519 { key, .. } => {
                // Verify using Ed25519
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
impl MLAgent for UserAgent {
    async fn train(&self) -> Result<()> {
        let user_data = self.user_analytics.read().await.get_training_data().await?;
        let mut model = self.prediction_model.write().await;
        model.train(&user_data).await?;
        Ok(())
    }

    async fn predict(&self) -> Result<Vec<f64>> {
        let metrics = self.analyze_user_metrics("system").await?;
        Ok(vec![
            metrics.engagement_score,
            metrics.activity_level,
            metrics.trust_score,
            metrics.contribution_value,
            metrics.interaction_quality,
            metrics.reputation_score,
        ])
    }

    async fn update(&self) -> Result<()> {
        let metrics = self.analyze_user_metrics("system").await?;
        self.optimize_user_experience("system").await?;
        Ok(())
    }

    async fn get_metrics(&self) -> Result<Vec<f64>> {
        let metrics = self.analyze_user_metrics("system").await?;
        Ok(vec![
            metrics.engagement_score,
            metrics.activity_level,
            metrics.trust_score,
            metrics.contribution_value,
            metrics.interaction_quality,
            metrics.reputation_score,
        ])
    }
}

fn calculate_diversity(map: &HashMap<String, f64>) -> f64 {
    if map.is_empty() {
        return 0.0;
    }
    
    let values: Vec<f64> = map.values().cloned().collect();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
        
    variance.sqrt()
}

fn calculate_intensity(map: &HashMap<String, f64>) -> f64 {
    if map.is_empty() {
        return 0.0;
    }
    
    map.values().sum::<f64>() / map.len() as f64
}

fn calculate_average(map: &HashMap<String, f64>) -> f64 {
    if map.is_empty() {
        return 0.0;
    }
    
    map.values().sum::<f64>() / map.len() as f64
}

fn calculate_stability(map: &HashMap<String, f64>) -> f64 {
    if map.is_empty() {
        return 0.0;
    }
    
    let values: Vec<f64> = map.values().cloned().collect();
    let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    
    1.0 - ((max - min) / max).min(1.0)
}
