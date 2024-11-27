use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("Invalid pricing configuration")]
    InvalidPricing,
    #[error("Invalid revenue distribution")]
    InvalidDistribution,
    #[error("Resource allocation failed")]
    ResourceAllocation,
}

/// Revenue distribution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueConfig {
    dao_treasury_share: Decimal,    // Percentage to DAO treasury
    developer_pool_share: Decimal,  // Percentage to developer pool
    operational_share: Decimal,     // Percentage to operations
}

/// Service tier definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceTier {
    Basic {
        api_limit: u64,
        features: Vec<String>,
    },
    Professional {
        api_limit: u64,
        features: Vec<String>,
    },
    Enterprise {
        custom_limit: Option<u64>,
        features: Vec<String>,
    },
}

/// Usage metrics for pricing calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    api_calls: u64,
    data_processed: u64,
    compute_time: u64,
    storage_used: u64,
}

/// Dynamic pricing factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingFactor {
    name: String,
    multiplier: f64,
    conditions: Vec<PricingCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingCondition {
    metric: String,
    threshold: f64,
    adjustment: f64,
}

/// Revenue distribution system
pub struct RevenueDistribution {
    config: RevenueConfig,
    current_revenue: Decimal,
}

impl RevenueDistribution {
    pub fn new(config: RevenueConfig) -> Self {
        Self {
            config,
            current_revenue: Decimal::new(0, 0),
        }
    }

    pub fn add_revenue(&mut self, amount: Decimal) {
        self.current_revenue += amount;
    }

    pub fn distribute(&self) -> Result<Distribution, BusinessError> {
        let dao_amount = self.current_revenue * self.config.dao_treasury_share;
        let dev_amount = self.current_revenue * self.config.developer_pool_share;
        let ops_amount = self.current_revenue * self.config.operational_share;

        Ok(Distribution {
            dao_treasury: dao_amount,
            developer_pool: dev_amount,
            operational: ops_amount,
        })
    }
}

/// Service pricing system
pub struct ServicePricing {
    base_rate: Decimal,
    tier_multipliers: std::collections::HashMap<ServiceTier, f64>,
    custom_factors: Vec<PricingFactor>,
}

impl ServicePricing {
    pub fn new(
        base_rate: Decimal,
        tier_multipliers: std::collections::HashMap<ServiceTier, f64>,
        custom_factors: Vec<PricingFactor>,
    ) -> Self {
        Self {
            base_rate,
            tier_multipliers,
            custom_factors,
        }
    }

    pub fn calculate_price(
        &self,
        usage: &Usage,
        tier: &ServiceTier,
    ) -> Result<Decimal, BusinessError> {
        let tier_multiplier = self
            .tier_multipliers
            .get(tier)
            .ok_or(BusinessError::InvalidPricing)?;

        let base_price = self.base_rate * Decimal::from_f64(*tier_multiplier).unwrap();
        
        // Apply usage-based pricing
        let usage_price = self.calculate_usage_price(usage)?;
        
        // Apply custom factors
        let factor_adjustments = self.apply_custom_factors(usage)?;
        
        Ok(base_price + usage_price + factor_adjustments)
    }

    fn calculate_usage_price(&self, usage: &Usage) -> Result<Decimal, BusinessError> {
        // Implement usage-based pricing logic
        let api_cost = Decimal::from(usage.api_calls) * Decimal::new(1, 4); // $0.0001 per call
        let data_cost = Decimal::from(usage.data_processed) * Decimal::new(5, 6); // $0.000005 per byte
        let compute_cost = Decimal::from(usage.compute_time) * Decimal::new(1, 3); // $0.001 per second
        let storage_cost = Decimal::from(usage.storage_used) * Decimal::new(1, 5); // $0.00001 per byte

        Ok(api_cost + data_cost + compute_cost + storage_cost)
    }

    fn apply_custom_factors(&self, usage: &Usage) -> Result<Decimal, BusinessError> {
        let mut total_adjustment = Decimal::new(0, 0);

        for factor in &self.custom_factors {
            for condition in &factor.conditions {
                let metric_value = match condition.metric.as_str() {
                    "api_calls" => usage.api_calls as f64,
                    "data_processed" => usage.data_processed as f64,
                    "compute_time" => usage.compute_time as f64,
                    "storage_used" => usage.storage_used as f64,
                    _ => continue,
                };

                if metric_value > condition.threshold {
                    total_adjustment += Decimal::from_f64(condition.adjustment).unwrap();
                }
            }
        }

        Ok(total_adjustment)
    }
}

/// Auto-functionality vs DAO control manager
pub struct SystemController {
    auto_threshold: Decimal,
    dao_approval_required: bool,
}

impl SystemController {
    pub fn new(auto_threshold: Decimal, dao_approval_required: bool) -> Self {
        Self {
            auto_threshold,
            dao_approval_required,
        }
    }

    pub fn can_auto_adjust(&self, change_magnitude: Decimal) -> bool {
        if self.dao_approval_required {
            return false;
        }
        change_magnitude <= self.auto_threshold
    }

    pub fn requires_dao_approval(&self, change_magnitude: Decimal) -> bool {
        self.dao_approval_required || change_magnitude > self.auto_threshold
    }
}

#[derive(Debug)]
pub struct Distribution {
    dao_treasury: Decimal,
    developer_pool: Decimal,
    operational: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revenue_distribution() {
        let config = RevenueConfig {
            dao_treasury_share: Decimal::new(40, 2),  // 40%
            developer_pool_share: Decimal::new(30, 2), // 30%
            operational_share: Decimal::new(30, 2),    // 30%
        };

        let mut distribution = RevenueDistribution::new(config);
        distribution.add_revenue(Decimal::new(1000, 0)); // $1000

        let result = distribution.distribute().unwrap();
        assert_eq!(result.dao_treasury, Decimal::new(400, 0));
        assert_eq!(result.developer_pool, Decimal::new(300, 0));
        assert_eq!(result.operational, Decimal::new(300, 0));
    }

    #[test]
    fn test_service_pricing() {
        let mut tier_multipliers = std::collections::HashMap::new();
        tier_multipliers.insert(
            ServiceTier::Basic {
                api_limit: 1000,
                features: vec!["basic".to_string()],
            },
            1.0,
        );

        let pricing = ServicePricing::new(
            Decimal::new(100, 0), // $100 base rate
            tier_multipliers,
            vec![],
        );

        let usage = Usage {
            api_calls: 100,
            data_processed: 1000,
            compute_time: 60,
            storage_used: 1000,
        };

        let price = pricing
            .calculate_price(
                &usage,
                &ServiceTier::Basic {
                    api_limit: 1000,
                    features: vec!["basic".to_string()],
                },
            )
            .unwrap();

        assert!(price > Decimal::new(100, 0));
    }
}
