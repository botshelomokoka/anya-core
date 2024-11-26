use crate::research::bitcoin_layers_crawler::{BitcoinLayersCrawler, ResearchData, MLAnalysis};
use crate::ml_core::{MLCore, MLInput, MLOutput};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};

pub struct BitcoinLayersMLIntegration {
    crawler: BitcoinLayersCrawler,
    ml_core: Arc<Mutex<MLCore>>,
    update_interval: Duration,
}

impl BitcoinLayersMLIntegration {
    pub fn new(ml_core: Arc<Mutex<MLCore>>) -> Self {
        Self {
            crawler: BitcoinLayersCrawler::new(),
            ml_core,
            update_interval: Duration::from_secs(3600), // Update every hour
        }
    }

    pub async fn start_monitoring(&self) {
        info!("Starting Bitcoin Layers monitoring");
        
        loop {
            match self.update_ml_models().await {
                Ok(_) => info!("Successfully updated ML models with Bitcoin Layers data"),
                Err(e) => error!("Failed to update ML models: {}", e),
            }

            sleep(self.update_interval).await;
        }
    }

    async fn update_ml_models(&self) -> Result<(), Box<dyn std::error::Error>> {
        let research_data = self.crawler.crawl_repositories().await?;
        
        let ml_inputs = self.prepare_ml_inputs(&research_data);
        let mut ml_core = self.ml_core.lock().await;
        
        // Update models with new data
        for input in ml_inputs {
            ml_core.train(&input)?;
        }

        // Update security parameters based on research
        self.update_security_parameters(&research_data).await?;

        Ok(())
    }

    fn prepare_ml_inputs(&self, research_data: &[ResearchData]) -> Vec<MLInput> {
        research_data.iter()
            .filter_map(|data| {
                data.analysis.as_ref().map(|analysis| MLInput {
                    features: vec![
                        analysis.sentiment_score,
                        analysis.technical_complexity,
                        analysis.security_impact,
                        analysis.adoption_potential,
                    ],
                    timestamp: chrono::Utc::now(),
                    label: calculate_importance_score(analysis),
                })
            })
            .collect()
    }

    async fn update_security_parameters(&self, research_data: &[ResearchData]) -> Result<(), Box<dyn std::error::Error>> {
        let security_scores: Vec<f64> = research_data.iter()
            .filter_map(|data| data.analysis.as_ref())
            .map(|analysis| analysis.security_impact)
            .collect();

        if !security_scores.is_empty() {
            let avg_security_score = security_scores.iter().sum::<f64>() / security_scores.len() as f64;
            
            // Update security thresholds based on research findings
            if avg_security_score > 0.8 {
                // Implement stricter security measures
                self.update_security_thresholds(avg_security_score).await?;
            }
        }

        Ok(())
    }

    async fn update_security_thresholds(&self, security_score: f64) -> Result<(), Box<dyn std::error::Error>> {
        // Implement security threshold updates
        Ok(())
    }
}

fn calculate_importance_score(analysis: &MLAnalysis) -> f64 {
    const SECURITY_WEIGHT: f64 = 0.4;
    const COMPLEXITY_WEIGHT: f64 = 0.3;
    const ADOPTION_WEIGHT: f64 = 0.2;
    const SENTIMENT_WEIGHT: f64 = 0.1;

    analysis.security_impact * SECURITY_WEIGHT +
    analysis.technical_complexity * COMPLEXITY_WEIGHT +
    analysis.adoption_potential * ADOPTION_WEIGHT +
    analysis.sentiment_score * SENTIMENT_WEIGHT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bitcoin_layers_integration() {
        let ml_core = Arc::new(Mutex::new(MLCore::new()));
        let integration = BitcoinLayersMLIntegration::new(ml_core);
        
        let result = integration.update_ml_models().await;
        assert!(result.is_ok());
    }
}
