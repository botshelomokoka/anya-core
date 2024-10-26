use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::time::{sleep, Duration};
use log::{info, warn, error};
use metrics::{counter, gauge};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Request failed: {0}")]
    RequestError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Rate limit exceeded")]
    RateLimitError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchData {
    source: DataSource,
    content: String,
    analysis: Option<MLAnalysis>,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataSource {
    BitcoinLayers,
    FederatedLearning,
    BIPs,
    Research,
    GitHub,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    timestamp: chrono::DateTime<chrono::Utc>,
    source_url: String,
    tags: Vec<String>,
}

pub struct UnifiedCrawler {
    client: Client,
    rate_limiter: RateLimiter,
    metrics: CrawlerMetrics,
}

impl UnifiedCrawler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            rate_limiter: RateLimiter::new(100, 1.0), // 100 requests per second
            metrics: CrawlerMetrics::new(),
        }
    }

    pub async fn crawl_all_sources(&self) -> Result<Vec<ResearchData>, CrawlerError> {
        let mut all_data = Vec::new();

        // Crawl Bitcoin Layers repositories
        match self.crawl_bitcoin_layers().await {
            Ok(data) => all_data.extend(data),
            Err(e) => error!("Failed to crawl Bitcoin Layers: {}", e),
        }

        // Crawl Federated Learning research
        match self.crawl_federated_learning().await {
            Ok(data) => all_data.extend(data),
            Err(e) => error!("Failed to crawl Federated Learning: {}", e),
        }

        // Crawl BIPs
        match self.crawl_bips().await {
            Ok(data) => all_data.extend(data),
            Err(e) => error!("Failed to crawl BIPs: {}", e),
        }

        // Crawl research papers
        match self.crawl_research_papers().await {
            Ok(data) => all_data.extend(data),
            Err(e) => error!("Failed to crawl research papers: {}", e),
        }

        self.metrics.record_crawl_completion(all_data.len());
        Ok(all_data)
    }

    async fn crawl_bitcoin_layers(&self) -> Result<Vec<ResearchData>, CrawlerError> {
        self.rate_limiter.acquire(1).await?;
        
        let url = "https://api.github.com/orgs/bitcoinlayers/repos";
        let response = self.client.get(url)
            .header("User-Agent", "Anya-Research-Bot")
            .send()
            .await
            .map_err(|e| CrawlerError::RequestError(e.to_string()))?;

        let repos: Vec<serde_json::Value> = response.json().await
            .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        let mut research_data = Vec::new();
        for repo in repos {
            let data = self.analyze_repository(&repo).await?;
            research_data.push(data);
        }

        Ok(research_data)
    }

    async fn crawl_federated_learning(&self) -> Result<Vec<ResearchData>, CrawlerError> {
        self.rate_limiter.acquire(1).await?;
        
        let url = "https://raw.githubusercontent.com/innovation-cat/Awesome-Federated-Machine-Learning/master/README.md";
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e| CrawlerError::RequestError(e.to_string()))?;

        let content = response.text().await
            .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        let research_data = self.parse_fl_research(&content).await?;
        Ok(research_data)
    }

    async fn crawl_bips(&self) -> Result<Vec<ResearchData>, CrawlerError> {
        self.rate_limiter.acquire(1).await?;
        
        let url = "https://github.com/bitcoin/bips/blob/master/README.mediawiki";
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e| CrawlerError::RequestError(e.to_string()))?;

        let content = response.text().await
            .map_err(|e| CrawlerError::ParseError(e.to_string()))?;

        let research_data = self.parse_bips(&content).await?;
        Ok(research_data)
    }

    async fn crawl_research_papers(&self) -> Result<Vec<ResearchData>, CrawlerError> {
        self.rate_limiter.acquire(1).await?;
        
        let papers = self.fetch_research_papers().await?;
        let mut research_data = Vec::new();

        for paper in papers {
            if let Ok(analysis) = self.analyze_paper(&paper).await {
                research_data.push(ResearchData {
                    source: DataSource::Research,
                    content: paper.content,
                    analysis: Some(analysis),
                    metadata: paper.metadata,
                });
            }
        }

        Ok(research_data)
    }

    async fn analyze_repository(&self, repo: &serde_json::Value) -> Result<ResearchData, CrawlerError> {
        // Implement repository analysis
        Ok(ResearchData {
            source: DataSource::GitHub,
            content: repo.to_string(),
            analysis: None,
            metadata: Metadata {
                timestamp: chrono::Utc::now(),
                source_url: repo["html_url"].as_str().unwrap_or("").to_string(),
                tags: vec![],
            },
        })
    }
}

struct CrawlerMetrics {
    successful_crawls: Counter,
    failed_crawls: Counter,
    data_points_collected: Gauge,
}

impl CrawlerMetrics {
    fn new() -> Self {
        Self {
            successful_crawls: counter!("crawler_successful_crawls_total"),
            failed_crawls: counter!("crawler_failed_crawls_total"),
            data_points_collected: gauge!("crawler_data_points_total"),
        }
    }

    fn record_crawl_completion(&self, data_points: usize) {
        self.successful_crawls.increment(1);
        self.data_points_collected.set(data_points as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_crawler() {
        let crawler = UnifiedCrawler::new();
        let result = crawler.crawl_all_sources().await;
        assert!(result.is_ok());
        
        let research_data = result.unwrap();
        assert!(!research_data.is_empty());
    }
}
