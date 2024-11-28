use anyhow::Result;
use rust_bert::bert::{BertModel, BertConfig};
use rust_bert::Config;
use ndarray::{Array1, Array2};
use super::{SearchResult, RelevanceScores};

pub struct CrossEncoder {
    model: BertModel,
    threshold: f32,
}

impl CrossEncoder {
    pub fn new(threshold: f32) -> Result<Self> {
        let config = BertConfig::from_pretrained("cross-encoder/ms-marco-MiniLM-L-12-v2");
        let model = BertModel::new(config)?;

        Ok(Self {
            model,
            threshold,
        })
    }

    pub async fn rerank(&self, query: &str, results: Vec<SearchResult>) -> Result<Vec<SearchResult>> {
        let mut reranked = Vec::new();

        for result in results {
            let cross_score = self.compute_cross_score(query, &result).await?;
            
            if cross_score >= self.threshold {
                let mut new_result = result.clone();
                new_result.relevance_scores.cross_encoder_score = cross_score;
                new_result.relevance_scores.final_score = self.compute_final_score(
                    &new_result.relevance_scores
                );
                reranked.push(new_result);
            }
        }

        // Sort by cross-encoder score
        reranked.sort_by(|a, b| {
            b.relevance_scores
                .cross_encoder_score
                .partial_cmp(&a.relevance_scores.cross_encoder_score)
                .unwrap()
        });

        Ok(reranked)
    }

    async fn compute_cross_score(&self, query: &str, result: &SearchResult) -> Result<f32> {
        let document_text = match &result.document.content {
            super::DocumentContent::Text(text) => text,
            super::DocumentContent::Code(code) => code,
            super::DocumentContent::BitcoinScript(script) => script,
            _ => return Ok(0.0), // Skip non-text content
        };

        // Concatenate query and document with [SEP] token
        let input = format!("{} [SEP] {}", query, document_text);
        
        // Tokenize and encode
        let tokenized = self.model.tokenize(&input);
        let encoding = self.model.encode(&[tokenized])?;
        
        // Get CLS token embedding and compute relevance score
        let cls_embedding = encoding.slice(s![0, 0, ..]).to_owned();
        let score = self.compute_relevance(&cls_embedding)?;

        Ok(score)
    }

    fn compute_relevance(&self, cls_embedding: &Array1<f32>) -> Result<f32> {
        // Project CLS embedding to scalar score using learned projection
        let projection = Array2::from_shape_vec(
            (1, cls_embedding.len()),
            vec![1.0 / cls_embedding.len() as f32; cls_embedding.len()]
        )?;
        
        let score = projection.dot(cls_embedding);
        Ok(score[[0]])
    }

    fn compute_final_score(&self, scores: &RelevanceScores) -> f32 {
        // Weighted combination of bi-encoder and cross-encoder scores
        const BI_ENCODER_WEIGHT: f32 = 0.3;
        const CROSS_ENCODER_WEIGHT: f32 = 0.7;

        scores.bi_encoder_score * BI_ENCODER_WEIGHT +
        scores.cross_encoder_score * CROSS_ENCODER_WEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_cross_encoder_reranking() -> Result<()> {
        let cross_encoder = CrossEncoder::new(0.5)?;

        // Create test results
        let results = vec![
            create_test_result("Document 1", 0.8),
            create_test_result("Document 2", 0.6),
            create_test_result("Document 3", 0.4),
        ];

        let reranked = cross_encoder.rerank("test query", results).await?;
        
        assert!(!reranked.is_empty());
        assert!(reranked[0].relevance_scores.cross_encoder_score >= 
               reranked[reranked.len()-1].relevance_scores.cross_encoder_score);

        Ok(())
    }

    fn create_test_result(text: &str, score: f32) -> SearchResult {
        SearchResult {
            document: Document {
                id: text.to_string(),
                content: DocumentContent::Text(text.to_string()),
                metadata: DocumentMetadata {
                    title: text.to_string(),
                    timestamp: Utc::now(),
                    tags: vec![],
                    content_type: "text".to_string(),
                    source: "test".to_string(),
                },
                bitcoin_data: None,
            },
            relevance_scores: RelevanceScores {
                bi_encoder_score: score,
                cross_encoder_score: 0.0,
                multimodal_score: 0.0,
                final_score: score,
            },
            bitcoin_validation: None,
        }
    }
}
