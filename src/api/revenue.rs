use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::business::{RevenueConfig, ServiceTier, Usage};

/// API Revenue tracking and management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRevenue {
    /// Current billing period revenue
    current_revenue: Decimal,
    /// Historical revenue data
    revenue_history: Vec<RevenueEntry>,
    /// Service configuration
    service_config: ServiceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEntry {
    timestamp: chrono::DateTime<chrono::Utc>,
    amount: Decimal,
    service_tier: ServiceTier,
    usage_metrics: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    base_rates: std::collections::HashMap<ServiceTier, Decimal>,
    usage_rates: UsageRates,
    volume_discounts: Vec<VolumeDiscount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRates {
    api_call_rate: Decimal,     // Cost per API call
    data_rate: Decimal,         // Cost per byte
    compute_rate: Decimal,      // Cost per second
    storage_rate: Decimal,      // Cost per byte
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDiscount {
    threshold: u64,
    discount_percentage: Decimal,
}

/// API Revenue Manager
pub struct ApiRevenueManager {
    revenue_data: Arc<RwLock<ApiRevenue>>,
    config: RevenueConfig,
}

impl ApiRevenueManager {
    pub async fn new(config: RevenueConfig, service_config: ServiceConfig) -> Self {
        let revenue = ApiRevenue {
            current_revenue: Decimal::new(0, 0),
            revenue_history: Vec::new(),
            service_config,
        };

        Self {
            revenue_data: Arc::new(RwLock::new(revenue)),
            config,
        }
    }

    /// Record API usage and calculate revenue
    pub async fn record_usage(&self, usage: Usage, tier: ServiceTier) -> Result<Decimal, anyhow::Error> {
        let mut revenue = self.revenue_data.write().await;
        
        // Calculate usage-based costs
        let usage_cost = self.calculate_usage_cost(&usage, &revenue.service_config.usage_rates)?;
        
        // Apply tier-based pricing
        let base_rate = revenue.service_config.base_rates
            .get(&tier)
            .ok_or_else(|| anyhow::anyhow!("Invalid service tier"))?;
        
        // Apply volume discounts
        let total_cost = self.apply_volume_discounts(
            usage_cost + *base_rate,
            usage.api_calls,
            &revenue.service_config.volume_discounts,
        )?;

        // Record revenue entry
        let entry = RevenueEntry {
            timestamp: chrono::Utc::now(),
            amount: total_cost,
            service_tier: tier,
            usage_metrics: usage,
        };
        
        revenue.revenue_history.push(entry);
        revenue.current_revenue += total_cost;

        Ok(total_cost)
    }

    /// Calculate cost based on usage metrics
    fn calculate_usage_cost(&self, usage: &Usage, rates: &UsageRates) -> Result<Decimal, anyhow::Error> {
        let api_cost = Decimal::from(usage.api_calls) * rates.api_call_rate;
        let data_cost = Decimal::from(usage.data_processed) * rates.data_rate;
        let compute_cost = Decimal::from(usage.compute_time) * rates.compute_rate;
        let storage_cost = Decimal::from(usage.storage_used) * rates.storage_rate;

        Ok(api_cost + data_cost + compute_cost + storage_cost)
    }

    /// Apply volume-based discounts
    fn apply_volume_discounts(
        &self,
        base_cost: Decimal,
        volume: u64,
        discounts: &[VolumeDiscount],
    ) -> Result<Decimal, anyhow::Error> {
        let mut applicable_discount = Decimal::new(0, 0);

        for discount in discounts {
            if volume >= discount.threshold {
                applicable_discount = discount.discount_percentage;
            }
        }

        let discount_amount = base_cost * applicable_discount;
        Ok(base_cost - discount_amount)
    }

    /// Get current revenue metrics
    pub async fn get_revenue_metrics(&self) -> Result<RevenueMetrics, anyhow::Error> {
        let revenue = self.revenue_data.read().await;
        
        Ok(RevenueMetrics {
            current_period: revenue.current_revenue,
            total_api_calls: revenue.revenue_history.iter()
                .map(|entry| entry.usage_metrics.api_calls)
                .sum(),
            total_data_processed: revenue.revenue_history.iter()
                .map(|entry| entry.usage_metrics.data_processed)
                .sum(),
            revenue_by_tier: self.calculate_revenue_by_tier(&revenue.revenue_history),
        })
    }

    /// Calculate revenue distribution by service tier
    fn calculate_revenue_by_tier(&self, history: &[RevenueEntry]) -> std::collections::HashMap<ServiceTier, Decimal> {
        let mut revenue_by_tier = std::collections::HashMap::new();

        for entry in history {
            let current = revenue_by_tier.entry(entry.service_tier.clone()).or_insert(Decimal::new(0, 0));
            *current += entry.amount;
        }

        revenue_by_tier
    }

    /// Distribute revenue according to configuration
    pub async fn distribute_revenue(&self) -> Result<RevenueDistribution, anyhow::Error> {
        let revenue = self.revenue_data.read().await;
        let current = revenue.current_revenue;

        Ok(RevenueDistribution {
            dao_treasury: current * self.config.dao_treasury_share,
            developer_pool: current * self.config.developer_pool_share,
            operational: current * self.config.operational_share,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RevenueMetrics {
    current_period: Decimal,
    total_api_calls: u64,
    total_data_processed: u64,
    revenue_by_tier: std::collections::HashMap<ServiceTier, Decimal>,
}

#[derive(Debug)]
pub struct RevenueDistribution {
    dao_treasury: Decimal,
    developer_pool: Decimal,
    operational: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_revenue_recording() {
        let config = RevenueConfig {
            dao_treasury_share: Decimal::new(40, 2),
            developer_pool_share: Decimal::new(30, 2),
            operational_share: Decimal::new(30, 2),
        };

        let mut base_rates = std::collections::HashMap::new();
        base_rates.insert(
            ServiceTier::Basic {
                api_limit: 1000,
                features: vec!["basic".to_string()],
            },
            Decimal::new(100, 0),
        );

        let service_config = ServiceConfig {
            base_rates,
            usage_rates: UsageRates {
                api_call_rate: Decimal::new(1, 4),
                data_rate: Decimal::new(5, 6),
                compute_rate: Decimal::new(1, 3),
                storage_rate: Decimal::new(1, 5),
            },
            volume_discounts: vec![
                VolumeDiscount {
                    threshold: 1000,
                    discount_percentage: Decimal::new(10, 2),
                },
            ],
        };

        let manager = ApiRevenueManager::new(config, service_config).await;

        let usage = Usage {
            api_calls: 100,
            data_processed: 1000,
            compute_time: 60,
            storage_used: 1000,
        };

        let revenue = manager
            .record_usage(
                usage,
                ServiceTier::Basic {
                    api_limit: 1000,
                    features: vec!["basic".to_string()],
                },
            )
            .await
            .unwrap();

        assert!(revenue > Decimal::new(100, 0));
    }
}
