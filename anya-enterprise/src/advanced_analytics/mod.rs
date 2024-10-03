use crate::user_metrics::UserMetrics;
use crate::blockchain::BlockchainInterface;
use crate::data_feed::{DataFeed, DataSource};
use crate::ml_logic::dao_rules::{DAORule, DAOContext};
use crate::market_data::MarketDataFetcher;
use crate::ml_logic::data_processing::process_market_data;
use crate::ml::{MLInput, MLOutput};
use tch::{nn, Device, Tensor};
use std::error::Error;
use std::collections::HashMap;

pub struct AdvancedAnalytics {
    model: nn::Sequential,
    user_metrics: UserMetrics,
    blockchain: BlockchainInterface,
    data_feeds: HashMap<DataSource, DataFeed>,
    dao_rules: Vec<DAORule>,
}

impl AdvancedAnalytics {
    pub fn new(
        user_metrics: UserMetrics,
        blockchain: BlockchainInterface,
        data_feeds: HashMap<DataSource, DataFeed>,
        dao_rules: Vec<DAORule>,
    ) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let model = nn::seq()
            .add(nn::linear(&vs.root(), 100, 64, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(&vs.root(), 64, 32, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(&vs.root(), 32, 1, Default::default()));

        Self {
            model,
            user_metrics,
            blockchain,
            data_feeds,
            dao_rules,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Running advanced analytics...");

        let market_sentiment = self.analyze_market_sentiment()?;
        println!("Market sentiment score: {}", market_sentiment);

        let user_behavior = self.analyze_user_behavior()?;
        println!("User behavior score: {}", user_behavior);

        let blockchain_metrics = self.analyze_blockchain_metrics()?;
        println!("Blockchain health score: {}", blockchain_metrics);

        let dao_effectiveness = self.analyze_dao_effectiveness()?;
        println!("DAO effectiveness score: {}", dao_effectiveness);

        let combined_score = (market_sentiment + user_behavior + blockchain_metrics + dao_effectiveness) / 4.0;
        println!("Combined analytics score: {}", combined_score);

        Ok(())
    }

    fn analyze_market_sentiment(&self) -> Result<f64, Box<dyn Error>> {
        let market_data = self.data_feeds.get(&DataSource::Market)
            .ok_or("Market data feed not found")?
            .get_latest_data()?;

        let input = Tensor::of_slice(&market_data).view([1, -1]);
        let output = self.model.forward(&input);
        let sentiment_score = output.double_value(&[0]);

        // Normalize the sentiment score to a range of 0 to 1
        let normalized_score = (sentiment_score + 1.0) / 2.0;

        Ok(normalized_score)
    }

    fn analyze_user_behavior(&self) -> Result<f64, Box<dyn Error>> {
        let usage_level = self.user_metrics.get_usage_level()?;
        let contribution_score = self.user_metrics.get_contribution_score()?;
        let loyalty_score = self.user_metrics.get_loyalty_score()?;

        // Combine the metrics with weighted importance
        let behavior_score = (usage_level * 0.3 + contribution_score * 0.4 + loyalty_score * 0.3) / 3.0;

        Ok(behavior_score)
    }

    fn analyze_blockchain_metrics(&self) -> Result<f64, Box<dyn Error>> {
        let transaction_volume = self.blockchain.get_transaction_volume()?;
        let network_hashrate = self.blockchain.get_network_hashrate()?;
        let mempool_size = self.blockchain.get_mempool_size()?;

        // Normalize and combine metrics
        let volume_score = (transaction_volume / 1_000_000.0).min(1.0); // Assume 1M transactions is a perfect score
        let hashrate_score = (network_hashrate / 1_000_000_000_000.0).min(1.0); // Assume 1 TH/s is a perfect score
        let mempool_score = 1.0 - (mempool_size as f64 / 10_000.0).min(1.0); // Assume 0 is perfect, 10k is worst

        let blockchain_health = (volume_score * 0.4 + hashrate_score * 0.4 + mempool_score * 0.2);

        Ok(blockchain_health)
    }

    fn analyze_dao_effectiveness(&self) -> Result<f64, Box<dyn Error>> {
        let mut context = DAOContext {
            current_fee: self.blockchain.get_current_fee()?,
            vote_count: self.blockchain.get_total_votes()?,
            parameters: self.blockchain.get_dao_parameters()?,
        };

        let mut effectiveness_score = 0.0;
        for rule in &self.dao_rules {
            if rule.apply_rule(&mut context).is_ok() {
                effectiveness_score += 1.0;
            }
        }

        let normalized_score = effectiveness_score / self.dao_rules.len() as f64;

        Ok(normalized_score)
    }

    pub fn perform_analysis(&self) -> Result<MLOutput, Box<dyn Error>> {
        let market_data_fetcher = MarketDataFetcher::new();
        let raw_data = market_data_fetcher.fetch_latest_data()?;
        let processed_data = process_market_data(raw_data)?;
        
        let input = MLInput {
            features: processed_data.features,
            label: processed_data.label,
        };

        let input_tensor = Tensor::of_slice(&input.features).view([1, -1]);
        let output = self.model.forward(&input_tensor);
        let prediction = output.double_value(&[0]);
        let confidence = self.calculate_confidence()?;

        Ok(MLOutput {
            prediction,
            confidence,
        })
    }

    fn calculate_confidence(&self) -> Result<f64, Box<dyn Error>> {
        let market_sentiment = self.analyze_market_sentiment()?;
        let user_behavior = self.analyze_user_behavior()?;
        let blockchain_metrics = self.analyze_blockchain_metrics()?;
        let dao_effectiveness = self.analyze_dao_effectiveness()?;

        // Combine all factors with weighted importance
        let confidence = (
            market_sentiment * 0.3 +
            user_behavior * 0.2 +
            blockchain_metrics * 0.3 +
            dao_effectiveness * 0.2
        );

        Ok(confidence)
    }
}

pub fn init(
    user_metrics: &UserMetrics,
    blockchain: &BlockchainInterface,
    data_feeds: &HashMap<DataSource, DataFeed>,
    dao_rules: &[DAORule],
) -> AdvancedAnalytics {
    AdvancedAnalytics::new(
        user_metrics.clone(),
        blockchain.clone(),
        data_feeds.clone(),
        dao_rules.to_vec(),
    )
}