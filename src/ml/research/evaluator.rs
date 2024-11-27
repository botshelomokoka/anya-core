use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEvaluation {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub auc_roc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentEvaluation {
    pub value_consistency: f64,
    pub goal_alignment: f64,
    pub safety_metrics: f64,
    pub ethical_score: f64,
    pub transparency_level: f64,
}

pub struct ResearchEvaluator {
    model_evaluations: Arc<RwLock<Vec<ModelEvaluation>>>,
    alignment_evaluations: Arc<RwLock<Vec<AlignmentEvaluation>>>,
}

impl ResearchEvaluator {
    pub fn new() -> Self {
        Self {
            model_evaluations: Arc::new(RwLock::new(Vec::new())),
            alignment_evaluations: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn evaluate_model_performance(&self, predictions: &[f64], targets: &[f64]) -> Result<ModelEvaluation> {
        let accuracy = self.calculate_accuracy(predictions, targets);
        let precision = self.calculate_precision(predictions, targets);
        let recall = self.calculate_recall(predictions, targets);
        let f1_score = self.calculate_f1_score(precision, recall);
        let auc_roc = self.calculate_auc_roc(predictions, targets);

        let evaluation = ModelEvaluation {
            accuracy,
            precision,
            recall,
            f1_score,
            auc_roc,
        };

        self.model_evaluations.write().await.push(evaluation.clone());
        Ok(evaluation)
    }

    fn calculate_accuracy(&self, predictions: &[f64], targets: &[f64]) -> f64 {
        let mut correct = 0;
        for (pred, target) in predictions.iter().zip(targets.iter()) {
            if (pred - target).abs() < 0.5 {
                correct += 1;
            }
        }
        correct as f64 / predictions.len() as f64
    }

    fn calculate_precision(&self, predictions: &[f64], targets: &[f64]) -> f64 {
        let mut true_positives = 0;
        let mut false_positives = 0;

        for (pred, target) in predictions.iter().zip(targets.iter()) {
            if *pred >= 0.5 {
                if *target >= 0.5 {
                    true_positives += 1;
                } else {
                    false_positives += 1;
                }
            }
        }

        if true_positives + false_positives == 0 {
            0.0
        } else {
            true_positives as f64 / (true_positives + false_positives) as f64
        }
    }

    fn calculate_recall(&self, predictions: &[f64], targets: &[f64]) -> f64 {
        let mut true_positives = 0;
        let mut false_negatives = 0;

        for (pred, target) in predictions.iter().zip(targets.iter()) {
            if *target >= 0.5 {
                if *pred >= 0.5 {
                    true_positives += 1;
                } else {
                    false_negatives += 1;
                }
            }
        }

        if true_positives + false_negatives == 0 {
            0.0
        } else {
            true_positives as f64 / (true_positives + false_negatives) as f64
        }
    }

    fn calculate_f1_score(&self, precision: f64, recall: f64) -> f64 {
        if precision + recall == 0.0 {
            0.0
        } else {
            2.0 * (precision * recall) / (precision + recall)
        }
    }

    fn calculate_auc_roc(&self, predictions: &[f64], targets: &[f64]) -> f64 {
        // Simplified AUC-ROC calculation
        let mut auc = 0.0;
        let n_positive = targets.iter().filter(|&&x| x >= 0.5).count();
        let n_negative = targets.len() - n_positive;

        if n_positive == 0 || n_negative == 0 {
            return 0.5;
        }

        let mut true_positive_rate = 0.0;
        let mut false_positive_rate = 0.0;
        let mut prev_tpr = 0.0;
        let mut prev_fpr = 0.0;

        let mut sorted_pairs: Vec<_> = predictions.iter().zip(targets.iter()).collect();
        sorted_pairs.sort_by(|a, b| b.0.partial_cmp(a.0).unwrap());

        for (_, target) in sorted_pairs {
            if *target >= 0.5 {
                true_positive_rate += 1.0 / n_positive as f64;
            } else {
                false_positive_rate += 1.0 / n_negative as f64;
            }

            auc += (true_positive_rate + prev_tpr) * (false_positive_rate - prev_fpr) / 2.0;
            prev_tpr = true_positive_rate;
            prev_fpr = false_positive_rate;
        }

        auc
    }

    pub async fn evaluate_alignment(&self, model_behavior: &[f64], expected_behavior: &[f64]) -> Result<AlignmentEvaluation> {
        let value_consistency = self.calculate_value_consistency(model_behavior, expected_behavior);
        let goal_alignment = self.calculate_goal_alignment(model_behavior, expected_behavior);
        let safety_metrics = self.calculate_safety_metrics(model_behavior);
        let ethical_score = self.calculate_ethical_score(model_behavior);
        let transparency_level = self.calculate_transparency_level(model_behavior);

        let evaluation = AlignmentEvaluation {
            value_consistency,
            goal_alignment,
            safety_metrics,
            ethical_score,
            transparency_level,
        };

        self.alignment_evaluations.write().await.push(evaluation.clone());
        Ok(evaluation)
    }

    fn calculate_value_consistency(&self, behavior: &[f64], expected: &[f64]) -> f64 {
        let mut consistency = 0.0;
        for (b, e) in behavior.iter().zip(expected.iter()) {
            consistency += 1.0 - (b - e).abs();
        }
        consistency / behavior.len() as f64
    }

    fn calculate_goal_alignment(&self, behavior: &[f64], expected: &[f64]) -> f64 {
        let mut alignment = 0.0;
        for (b, e) in behavior.iter().zip(expected.iter()) {
            alignment += if (b - e).abs() < 0.1 { 1.0 } else { 0.0 };
        }
        alignment / behavior.len() as f64
    }

    fn calculate_safety_metrics(&self, behavior: &[f64]) -> f64 {
        // Implement safety metrics calculation
        // For now, using a simple threshold-based approach
        behavior.iter().filter(|&&x| x >= 0.8).count() as f64 / behavior.len() as f64
    }

    fn calculate_ethical_score(&self, behavior: &[f64]) -> f64 {
        // Implement ethical score calculation
        // For now, using a simple average-based approach
        behavior.iter().sum::<f64>() / behavior.len() as f64
    }

    fn calculate_transparency_level(&self, behavior: &[f64]) -> f64 {
        // Implement transparency level calculation
        // For now, using variance as a proxy for transparency
        let mean = behavior.iter().sum::<f64>() / behavior.len() as f64;
        let variance = behavior.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / behavior.len() as f64;
        1.0 / (1.0 + variance)
    }

    pub async fn get_latest_model_evaluation(&self) -> Option<ModelEvaluation> {
        self.model_evaluations.read().await.last().cloned()
    }

    pub async fn get_latest_alignment_evaluation(&self) -> Option<AlignmentEvaluation> {
        self.alignment_evaluations.read().await.last().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_evaluation() {
        let evaluator = ResearchEvaluator::new();
        let predictions = vec![0.8, 0.2, 0.9, 0.1];
        let targets = vec![1.0, 0.0, 1.0, 0.0];

        let evaluation = evaluator.evaluate_model_performance(&predictions, &targets).await.unwrap();
        assert!(evaluation.accuracy >= 0.0 && evaluation.accuracy <= 1.0);
        assert!(evaluation.precision >= 0.0 && evaluation.precision <= 1.0);
        assert!(evaluation.recall >= 0.0 && evaluation.recall <= 1.0);
        assert!(evaluation.f1_score >= 0.0 && evaluation.f1_score <= 1.0);
        assert!(evaluation.auc_roc >= 0.0 && evaluation.auc_roc <= 1.0);
    }

    #[tokio::test]
    async fn test_alignment_evaluation() {
        let evaluator = ResearchEvaluator::new();
        let model_behavior = vec![0.8, 0.9, 0.7, 0.85];
        let expected_behavior = vec![0.85, 0.9, 0.75, 0.8];

        let evaluation = evaluator.evaluate_alignment(&model_behavior, &expected_behavior).await.unwrap();
        assert!(evaluation.value_consistency >= 0.0 && evaluation.value_consistency <= 1.0);
        assert!(evaluation.goal_alignment >= 0.0 && evaluation.goal_alignment <= 1.0);
        assert!(evaluation.safety_metrics >= 0.0 && evaluation.safety_metrics <= 1.0);
        assert!(evaluation.ethical_score >= 0.0 && evaluation.ethical_score <= 1.0);
        assert!(evaluation.transparency_level >= 0.0 && evaluation.transparency_level <= 1.0);
    }
}
