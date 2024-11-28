pub mod cross_encoder;
pub mod multimodal;
pub mod distributed;
pub mod bitcoin;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSearchConfig {
    pub cross_encoder_threshold: f32,
    pub multimodal_weights: MultiModalWeights,
    pub distributed_nodes: Vec<String>,
    pub bitcoin_script_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalWeights {
    pub text: f32,
    pub image: f32,
    pub code: f32,
    pub blockchain: f32,
}

#[async_trait]
pub trait SearchNode: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>>;
    async fn index(&self, document: Document) -> Result<()>;
    async fn health_check(&self) -> Result<bool>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: DocumentContent,
    pub metadata: DocumentMetadata,
    pub bitcoin_data: Option<BitcoinData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentContent {
    Text(String),
    Image(Vec<u8>),
    Code(String),
    BitcoinScript(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub content_type: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinData {
    pub script_type: ScriptType,
    pub taproot_data: Option<TaprootData>,
    pub validation_status: ValidationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptType {
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaprootData {
    pub internal_key: String,
    pub merkle_root: String,
    pub script_path: Vec<String>,
    pub control_block: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Valid,
    Invalid(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: Document,
    pub relevance_scores: RelevanceScores,
    pub bitcoin_validation: Option<ValidationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceScores {
    pub bi_encoder_score: f32,
    pub cross_encoder_score: f32,
    pub multimodal_score: f32,
    pub final_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub script_analysis: ScriptAnalysis,
    pub taproot_verification: Option<TaprootVerification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptAnalysis {
    pub script_type: ScriptType,
    pub op_codes: Vec<String>,
    pub stack_trace: Vec<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaprootVerification {
    pub merkle_proof_valid: bool,
    pub signature_valid: bool,
    pub script_path_valid: bool,
    pub control_block_valid: bool,
}

pub struct AdvancedSearchEngine {
    config: AdvancedSearchConfig,
    nodes: Arc<RwLock<Vec<Box<dyn SearchNode>>>>,
    cross_encoder: Arc<cross_encoder::CrossEncoder>,
    multimodal_encoder: Arc<multimodal::MultiModalEncoder>,
    bitcoin_validator: Arc<bitcoin::BitcoinValidator>,
}

impl AdvancedSearchEngine {
    pub async fn new(config: AdvancedSearchConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            nodes: Arc::new(RwLock::new(Vec::new())),
            cross_encoder: Arc::new(cross_encoder::CrossEncoder::new(
                config.cross_encoder_threshold,
            )?),
            multimodal_encoder: Arc::new(multimodal::MultiModalEncoder::new(
                config.multimodal_weights,
            )?),
            bitcoin_validator: Arc::new(bitcoin::BitcoinValidator::new(
                config.bitcoin_script_validation,
            )?),
        })
    }

    pub async fn add_node(&self, node: Box<dyn SearchNode>) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.push(node);
        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // 1. Collect results from all nodes
        let mut all_results = Vec::new();
        let nodes = self.nodes.read().await;
        
        for node in nodes.iter() {
            let node_results = node.search(query).await?;
            all_results.extend(node_results);
        }

        // 2. Apply cross-encoder reranking
        let reranked_results = self.cross_encoder.rerank(query, all_results).await?;

        // 3. Apply multimodal scoring
        let multimodal_results = self.multimodal_encoder
            .score_results(query, reranked_results)
            .await?;

        // 4. Validate Bitcoin scripts if present
        let final_results = self.validate_bitcoin_data(multimodal_results).await?;

        // 5. Sort by final score
        let mut sorted_results = final_results;
        sorted_results.sort_by(|a, b| {
            b.relevance_scores
                .final_score
                .partial_cmp(&a.relevance_scores.final_score)
                .unwrap()
        });

        Ok(sorted_results)
    }

    async fn validate_bitcoin_data(
        &self,
        results: Vec<SearchResult>,
    ) -> Result<Vec<SearchResult>> {
        let mut validated_results = Vec::new();

        for mut result in results {
            if let Some(bitcoin_data) = &result.document.bitcoin_data {
                let validation = self.bitcoin_validator
                    .validate_script(bitcoin_data)
                    .await?;
                result.bitcoin_validation = Some(validation);
            }
            validated_results.push(result);
        }

        Ok(validated_results)
    }
}
