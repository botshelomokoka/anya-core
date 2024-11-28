use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use rust_bert::bert::{BertModel, BertConfig};
use rust_bert::Config;
use ndarray::{Array1, Array2};
use crate::web5::cache::Web5Cache;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document_id: String,
    pub score: f32,
    pub relevance: f32,
    pub context_match: f32,
    pub metadata: SearchMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    pub title: String,
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub embedding_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub threshold: f32,
    pub max_results: usize,
    pub include_context: bool,
    pub filter_tags: Option<Vec<String>>,
    pub sort_by: SortCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortCriteria {
    Relevance,
    Timestamp,
    Score,
}

pub struct SemanticSearch {
    model: Arc<BertModel>,
    cache: Arc<Web5Cache>,
    embeddings: Arc<RwLock<HashMap<String, Array1<f32>>>>,
    metadata: Arc<RwLock<HashMap<String, SearchMetadata>>>,
}

impl SemanticSearch {
    pub async fn new(cache: Arc<Web5Cache>) -> Result<Self> {
        let config = BertConfig::from_pretrained("bert-base-uncased");
        let model = BertModel::new(config)?;

        Ok(Self {
            model: Arc::new(model),
            cache,
            embeddings: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn index_document(
        &self,
        document_id: &str,
        content: &str,
        metadata: SearchMetadata,
    ) -> Result<()> {
        // Generate embedding
        let embedding = self.generate_embedding(content).await?;
        
        // Store embedding and metadata
        {
            let mut embeddings = self.embeddings.write().await;
            embeddings.insert(document_id.to_string(), embedding);
            
            let mut metadata_store = self.metadata.write().await;
            metadata_store.insert(document_id.to_string(), metadata);
        }

        // Cache the result
        self.cache.set(
            &format!("embedding:{}", document_id),
            &embedding,
            None,
        ).await?;

        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        options: SearchOptions,
    ) -> Result<Vec<SearchResult>> {
        // Generate query embedding
        let query_embedding = self.generate_embedding(query).await?;
        
        // Calculate similarities and gather results
        let mut results = Vec::new();
        let embeddings = self.embeddings.read().await;
        let metadata_store = self.metadata.read().await;

        for (doc_id, doc_embedding) in embeddings.iter() {
            let similarity = self.calculate_similarity(&query_embedding, doc_embedding);
            
            if similarity >= options.threshold {
                if let Some(metadata) = metadata_store.get(doc_id) {
                    // Apply tag filtering
                    if let Some(filter_tags) = &options.filter_tags {
                        if !metadata.tags.iter().any(|tag| filter_tags.contains(tag)) {
                            continue;
                        }
                    }

                    let context_match = if options.include_context {
                        self.calculate_context_match(doc_id, query).await?
                    } else {
                        0.0
                    };

                    results.push(SearchResult {
                        document_id: doc_id.clone(),
                        score: similarity,
                        relevance: (similarity + context_match) / 2.0,
                        context_match,
                        metadata: metadata.clone(),
                    });
                }
            }
        }

        // Sort results
        results.sort_by(|a, b| match options.sort_by {
            SortCriteria::Relevance => b.relevance.partial_cmp(&a.relevance).unwrap(),
            SortCriteria::Timestamp => b.metadata.timestamp.cmp(&a.metadata.timestamp),
            SortCriteria::Score => b.score.partial_cmp(&a.score).unwrap(),
        });

        // Limit results
        results.truncate(options.max_results);

        Ok(results)
    }

    async fn generate_embedding(&self, text: &str) -> Result<Array1<f32>> {
        // Check cache first
        let cache_key = format!("embedding:{}", xxhash_rust::xxh3::xxh3_64(text.as_bytes()));
        if let Ok(Some(cached)) = self.cache.get::<Array1<f32>>(&cache_key).await {
            return Ok(cached);
        }

        // Generate new embedding
        let tokenized = self.model.tokenize(text);
        let embedding = self.model.encode(&tokenized)?;
        let mean_embedding = embedding.mean_axis(0).unwrap();

        // Cache the result
        self.cache.set(&cache_key, &mean_embedding, None).await?;

        Ok(mean_embedding)
    }

    fn calculate_similarity(&self, embedding1: &Array1<f32>, embedding2: &Array1<f32>) -> f32 {
        let dot_product = embedding1.dot(embedding2);
        let norm1 = (embedding1.dot(embedding1)).sqrt();
        let norm2 = (embedding2.dot(embedding2)).sqrt();
        
        dot_product / (norm1 * norm2)
    }

    async fn calculate_context_match(&self, document_id: &str, query: &str) -> Result<f32> {
        // Implement context matching logic
        // For now, return a placeholder value
        Ok(0.8)
    }

    pub async fn batch_index(
        &self,
        documents: Vec<(String, String, SearchMetadata)>,
    ) -> Result<()> {
        let mut handles = Vec::new();

        for (doc_id, content, metadata) in documents {
            let self_clone = self.clone();
            let handle = tokio::spawn(async move {
                self_clone.index_document(&doc_id, &content, metadata).await
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_semantic_search() -> Result<()> {
        let cache = Arc::new(Web5Cache::new(Default::default()));
        let search = SemanticSearch::new(cache).await?;

        // Index test documents
        let metadata = SearchMetadata {
            title: "Test Document".to_string(),
            source: "test".to_string(),
            timestamp: chrono::Utc::now(),
            tags: vec!["test".to_string()],
            embedding_type: "bert".to_string(),
        };

        search.index_document(
            "doc1",
            "This is a test document about AI",
            metadata.clone(),
        ).await?;

        // Search
        let options = SearchOptions {
            threshold: 0.5,
            max_results: 10,
            include_context: true,
            filter_tags: Some(vec!["test".to_string()]),
            sort_by: SortCriteria::Relevance,
        };

        let results = search.search("AI document", options).await?;
        assert!(!results.is_empty());
        assert!(results[0].score > 0.5);

        Ok(())
    }
}
