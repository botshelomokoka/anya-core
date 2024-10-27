use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::time::{sleep, Duration};
use log::{info, warn, error};
use metrics::{counter, gauge};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitcoinLayersRepo {
    name: String,
    description: Option<String>,
    html_url: String,
    updated_at: String,
    stargazers_count: u32,
    topics: Vec<String>,
    language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchData {
    repo: BitcoinLayersRepo,
    content: String,
    analysis: Option<MLAnalysis>,
    layer_type: LayerType,
    security_score: f64,
    scalability_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LayerType {
    Layer1,
    Layer2,
    Layer3,
    Sidechain,
    StateChannel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MLAnalysis {
    sentiment_score: f64,
    technical_complexity: f64,
    security_impact: f64,
    adoption_potential: f64,
    scalability_metrics: ScalabilityMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalabilityMetrics {
    tps_estimate: f64,
    latency_ms: f64,
    cost_per_tx: f64,
    decentralization_score: f64,
}

pub struct BitcoinLayersCrawler {
    client: Client,
    base_url: String,
    rate_limit_delay: Duration,
    metrics: CrawlerMetrics,
}

impl BitcoinLayersCrawler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.github.com".to_string(),
            rate_limit_delay: Duration::from_secs(2),
            metrics: CrawlerMetrics::new(),
        }
    }

    pub async fn crawl_repositories(&self) -> Result<Vec<ResearchData>, Box<dyn Error>> {
        info!("Starting Bitcoin Layers repository crawl");
        
        let repos = self.fetch_bitcoin_layers_repos().await?;
        let mut research_data = Vec::new();

        for repo in repos {
            sleep(self.rate_limit_delay).await;
            
            match self.analyze_repository(&repo).await {
                Ok(data) => {
                    self.metrics.record_successful_analysis();
                    research_data.push(data);
                }
                Err(e) => {
                    self.metrics.record_failed_analysis();
                    error!("Failed to analyze repository {}: {}", repo.name, e);
                }
            }
        }

        Ok(research_data)
    }

    async fn fetch_bitcoin_layers_repos(&self) -> Result<Vec<BitcoinLayersRepo>, Box<dyn Error>> {
        let url = format!("{}/orgs/bitcoinlayers/repos", self.base_url);
        let response = self.client.get(&url)
            .header("User-Agent", "Anya-Research-Bot")
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    async fn analyze_repository(&self, repo: &BitcoinLayersRepo) -> Result<ResearchData, Box<dyn Error>> {
        let content = self.fetch_repository_content(repo).await?;
        let layer_type = self.determine_layer_type(repo)?;
        let security_score = self.analyze_security(&content)?;
        let scalability_score = self.analyze_scalability(&content)?;
        let analysis = self.perform_ml_analysis(&content).await?;

        Ok(ResearchData {
            repo: repo.clone(),
            content,
            analysis: Some(analysis),
            layer_type,
            security_score,
            scalability_score,
        })
    }

    fn determine_layer_type(&self, repo: &BitcoinLayersRepo) -> Result<LayerType, Box<dyn Error>> {
        let topics = &repo.topics;
        
        if topics.iter().any(|t| t.contains("layer1")) {
            Ok(LayerType::Layer1)
        } else if topics.iter().any(|t| t.contains("layer2")) {
            Ok(LayerType::Layer2)
        } else if topics.iter().any(|t| t.contains("layer3")) {
            Ok(LayerType::Layer3)
        } else if topics.iter().any(|t| t.contains("sidechain")) {
            Ok(LayerType::Sidechain)
        } else {
            Ok(LayerType::StateChannel)
        }
    }

    async fn fetch_repository_content(&self, repo: &BitcoinLayersRepo) -> Result<String, Box<dyn Error>> {
        let readme_url = format!("{}/contents/README.md", repo.html_url);
        let response = self.client.get(&readme_url)
            .header("User-Agent", "Anya-Research-Bot")
            .send()
            .await?;

        Ok(response.text().await?)
    }

    async fn perform_ml_analysis(&self, content: &str) -> Result<MLAnalysis, Box<dyn Error>> {
        // Implement ML analysis using your existing ML models
        Ok(MLAnalysis {
            sentiment_score: analyze_sentiment(content)?,
            technical_complexity: analyze_complexity(content)?,
            security_impact: analyze_security_impact(content)?,
            adoption_potential: analyze_adoption_potential(content)?,
            scalability_metrics: analyze_scalability_metrics(content)?,
        })
    }
}

struct CrawlerMetrics {
    successful_analyses: Counter,
    failed_analyses: Counter,
    security_scores: Gauge,
    scalability_scores: Gauge,
}

impl CrawlerMetrics {
    fn new() -> Self {
        Self {
            successful_analyses: counter!("crawler_successful_analyses_total"),
            failed_analyses: counter!("crawler_failed_analyses_total"),
            security_scores: gauge!("crawler_security_scores"),
            scalability_scores: gauge!("crawler_scalability_scores"),
        }
    }

    fn record_successful_analysis(&self) {
        self.successful_analyses.increment(1);
    }

    fn record_failed_analysis(&self) {
        self.failed_analyses.increment(1);
    }

    fn update_scores(&self, security: f64, scalability: f64) {
        self.security_scores.set(security);
        self.scalability_scores.set(scalability);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bitcoin_layers_crawler() {
        let crawler = BitcoinLayersCrawler::new();
        let result = crawler.crawl_repositories().await;
        assert!(result.is_ok());
        
        let research_data = result.unwrap();
        assert!(!research_data.is_empty());
        
        // Verify analysis results
        for data in research_data {
            assert!(data.security_score >= 0.0 && data.security_score <= 1.0);
            assert!(data.scalability_score >= 0.0 && data.scalability_score <= 1.0);
        }
    }
}
