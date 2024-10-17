use anya_core::ml::{MLError, MLInput, MLOutput, MLModel};
use ndarray::{Array1, Array2};
use tch::{nn, Device, Tensor, Kind};
use std::collections::HashMap;
use crate::user_metrics::UserMetrics;
use crate::data::{InternalDataProvider, HistoricalDataAnalyzer};
use crate::research::{ResearchPaperDatabase, AIModelUpgrader};
use crate::libraries::LibraryVersionManager;
use crate::blockchain::BlockchainInterface;
use crate::tokenizer::Tokenizer;
use crate::embedding::Embedding;
use crate::lightning::{LightningInterface, TaroInterface};
use crate::dlc::DLCInterface;
use crate::ordinals::OrdinalInterface;

pub struct AdvancedBitcoinPricePredictor {
    model: nn::Sequential,
    optimizer: Box<dyn nn::Optimizer>,
    user_metrics: UserMetrics,
    internal_data: InternalDataProvider,
    historical_analyzer: HistoricalDataAnalyzer,
    research_db: ResearchPaperDatabase,
    library_manager: LibraryVersionManager,
    blockchain: BlockchainInterface,
}

impl AdvancedBitcoinPricePredictor {
    pub fn new(user_metrics: UserMetrics, blockchain: BlockchainInterface) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let model = nn::seq()
            .add(nn::linear(&vs.root(), 30, 128, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 128, 64, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 64, 1, Default::default()));

        let optimizer = Box::new(nn::Adam::default().build(&vs, 1e-4).unwrap());

        Self {
            model,
            optimizer,
            user_metrics,
            internal_data: InternalDataProvider::new(),
            historical_analyzer: HistoricalDataAnalyzer::new(),
            research_db: ResearchPaperDatabase::new(),
            library_manager: LibraryVersionManager::new(),
            blockchain,
        }
    }

    fn adjust_learning_rate(&mut self) {
        let usage_level = self.user_metrics.usage_level;
        let base_lr = 1e-4;
        let market_volatility = self.internal_data.get_market_volatility();
        let adjusted_lr = base_lr * (1.0 + (usage_level as f64 * 0.1)) * (1.0 + market_volatility);
        self.optimizer.set_lr(adjusted_lr);
    }

    fn upgrade_model(&mut self) {
        let latest_research = self.research_db.get_latest_bitcoin_prediction_papers();
        let model_upgrader = AIModelUpgrader::new();
        self.model = model_upgrader.upgrade_model(&self.model, &latest_research);
    }
}

impl MLModel for AdvancedBitcoinPricePredictor {
    fn update(&mut self, input: &[MLInput]) -> Result<(), MLError> {
        self.adjust_learning_rate();
        self.upgrade_model();
        self.library_manager.update_libraries();

        let additional_features = self.internal_data.get_additional_features();
        let x = Tensor::of_slice(&input.iter()
            .flat_map(|i| i.features.iter()
                .chain(additional_features.iter())
                .cloned()
                .collect::<Vec<f64>>())
            .collect::<Vec<f64>>())
            .view([-1, 30]);
        let y = Tensor::of_slice(&input.iter().map(|i| i.label).collect::<Vec<f64>>()).view([-1, 1]);

        let loss = self.model.forward(&x).mse_loss(&y, tch::Reduction::Mean);
        self.optimizer.backward_step(&loss);

        Ok(())
    }

    fn predict(&self, input: &MLInput) -> Result<MLOutput, MLError> {
        let additional_features = self.internal_data.get_additional_features();
        let x = Tensor::of_slice(&input.features.iter()
            .chain(additional_features.iter())
            .cloned()
            .collect::<Vec<f64>>())
            .view([1, -1]);
        let output = self.model.forward(&x);
        let prediction = output.double_value(&[0]);
        
        let confidence = self.calculate_confidence(prediction);

        Ok(MLOutput {
            prediction,
            confidence,
        })
    }

    fn calculate_model_diversity(&self) -> f64 {
        let params: Vec<f64> = self.model
            .parameters()
            .iter()
            .flat_map(|t| t.flatten(0, -1).into_iter::<f64>().unwrap().collect::<Vec<f64>>())
            .collect();

        let mean = params.iter().sum::<f64>() / params.len() as f64;
        let variance = params.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / params.len() as f64;
        
        variance.sqrt() // Return standard deviation as a measure of diversity
    }

    fn optimize_model(&mut self) -> Result<(), MLError> {
        let historical_performance = self.historical_analyzer.analyze_model_performance(&self.model);
        
        if historical_performance < 0.7 {
            // Restructure the model based on historical performance
            let vs = nn::VarStore::new(Device::Cpu);
            self.model = nn::seq()
                .add(nn::linear(&vs.root(), 30, 256, Default::default()))
                .add_fn(|x| x.relu())
                .add(nn::dropout(&vs.root(), 0.3))
                .add(nn::linear(&vs.root(), 256, 128, Default::default()))
                .add_fn(|x| x.relu())
                .add(nn::dropout(&vs.root(), 0.3))
                .add(nn::linear(&vs.root(), 128, 64, Default::default()))
                .add_fn(|x| x.relu())
                .add(nn::linear(&vs.root(), 64, 1, Default::default()));
        }

        let optimizer: Box<dyn nn::Optimizer> = if self.user_metrics.usage_level > 3 {
            // Use a more sophisticated optimizer for high-usage users
            Box::new(nn::RmsProp::default().build(&self.model.vs(), 1e-4).unwrap())
        } else {
            // Use default optimizer for lower-usage users
            Box::new(nn::Adam::default().build(&self.model.vs(), 1e-3).unwrap())
        };
        self.optimizer = optimizer;

        Ok(())
    }
}

impl AdvancedBitcoinPricePredictor {
    fn calculate_confidence(&self, prediction: f64) -> f64 {
        let base_confidence = 0.9;
        let usage_factor = 1.0 + (self.user_metrics.usage_level as f64 * 0.02);
        let contribution_factor = 1.0 + (self.user_metrics.contributions as f64 * 0.01);
        let market_sentiment = self.internal_data.get_market_sentiment();
        let historical_accuracy = self.historical_analyzer.get_model_accuracy();
        let network_health = self.blockchain.get_network_health().unwrap_or(0.5);
        
        (base_confidence * usage_factor * contribution_factor * market_sentiment * historical_accuracy * network_health).min(1.0)
    }
}

pub struct AdvancedMarketSentimentAnalyzer {
    sentiment_model: nn::Sequential,
    user_metrics: UserMetrics,
    optimizer: Box<dyn nn::Optimizer>,
    internal_data: InternalDataProvider,
    research_db: ResearchPaperDatabase,
    blockchain: BlockchainInterface,
    tokenizer: Tokenizer,
    embedding: Embedding,
}

impl AdvancedMarketSentimentAnalyzer {
    fn new(user_metrics: UserMetrics, blockchain: BlockchainInterface) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let sentiment_model = nn::seq()
            .add(nn::linear(&vs.root(), 768, 512, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 512, 256, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 256, 3, Default::default()));
        
        let optimizer = Box::new(nn::Adam::default().build(&vs, 1e-4).unwrap());

        Self {
            sentiment_model,
            user_metrics,
            optimizer,
            internal_data: InternalDataProvider::new(),
            research_db: ResearchPaperDatabase::new(),
            blockchain,
            tokenizer: Tokenizer::new(),
            embedding: Embedding::new(),
        }
    }

    fn analyze_sentiment(&self, text: &str) -> Result<MLOutput, MLError> {
        let input_tensor = self.preprocess_text(text);
        let output = self.sentiment_model.forward(&input_tensor);
        let sentiment_score = output.double_value(&[0]);
        
        let confidence = self.calculate_confidence(sentiment_score);

        Ok(MLOutput {
            prediction: sentiment_score,
            confidence,
        })
    }

    fn preprocess_text(&self, text: &str) -> Tensor {
        let tokens = self.tokenizer.encode(text, true).unwrap();
        let token_ids: Vec<i64> = tokens.get_ids().iter().map(|&id| id as i64).collect();

        let max_length = 512;
        let padded_ids = if token_ids.len() >= max_length {
            token_ids[..max_length].to_vec()
        } else {
            let mut padded = token_ids;
            padded.resize(max_length, 0);
            padded
        };

        let input_tensor = Tensor::of_slice(&padded_ids).view([1, max_length]);
        let embedded = self.embedding.forward(&input_tensor);
        let pooled = embedded.mean_dim(&[1], true, Kind::Float);

        pooled.view([1, 768])
    }

    fn calculate_confidence(&self, sentiment_score: f64) -> f64 {
        let base_confidence = 0.85;
        let usage_factor = 1.0 + (self.user_metrics.usage_level as f64 * 0.03);
        let contribution_factor = 1.0 + (self.user_metrics.contributions as f64 * 0.02);
        let market_volatility = self.internal_data.get_market_volatility();
        let network_health = self.blockchain.get_network_health().unwrap_or(0.5);
        
        (base_confidence * usage_factor * contribution_factor * (1.0 - market_volatility) * network_health).min(1.0)
    }
}

struct AdvancedBlockchainDataPredictor {
    blockchain_model: nn::Sequential,
    user_metrics: UserMetrics,
    optimizer: Box<dyn nn::Optimizer>,
    internal_data: InternalDataProvider,
    historical_analyzer: HistoricalDataAnalyzer,
}

impl AdvancedBlockchainDataPredictor {
    fn new(user_metrics: UserMetrics) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let blockchain_model = nn::seq()
            .add(nn::linear(&vs.root(), 150, 128, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 128, 64, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 64, 1, Default::default()));
        
        let optimizer = Box::new(nn::RmsProp::default().build(&vs, 1e-4).unwrap());

        Self {
            blockchain_model,
            user_metrics,
            optimizer,
            internal_data: InternalDataProvider::new(),
            historical_analyzer: HistoricalDataAnalyzer::new(),
        }
    }
    fn predict_blockchain_data(&self, input_data: &[f64]) -> Result<MLOutput, MLError> {
        let input_tensor = Tensor::of_slice(input_data).view([-1, 150]);
        let output = self.blockchain_model.forward(&input_tensor);
        let prediction = output.double_value(&[0]) as f64;
        
        let confidence = self.calculate_confidence(prediction);

        let market_sentiment = self.internal_data.get_market_sentiment();
        let adjusted_prediction = prediction * (1.0 + market_sentiment * 0.1);

        let historical_trend = self.historical_analyzer.get_trend_factor();
        let final_prediction = adjusted_prediction * historical_trend;

        Ok(MLOutput {
            prediction: final_prediction,
            confidence,
        })
    }

    fn calculate_confidence(&self, prediction: f64) -> f64 {
        let base_confidence = 0.8;
        let usage_factor = 1.0 + (self.user_metrics.usage_level as f64 * 0.04);
        let contribution_factor = 1.0 + (self.user_metrics.contributions as f64 * 0.03);
        let historical_accuracy = self.historical_analyzer.get_model_accuracy();
        
        (base_confidence * usage_factor * contribution_factor * historical_accuracy).min(1.0)
    }
}

struct AdvancedCryptoPortfolioOptimizer {
    portfolio_model: nn::Sequential,
    user_metrics: UserMetrics,
    optimizer: Box<dyn nn::Optimizer>,
    internal_data: InternalDataProvider,
    research_db: ResearchPaperDatabase,
    lightning: LightningInterface,
    taro: TaroInterface,
    dlc: DLCInterface,
    ordinals: OrdinalInterface,
}

impl AdvancedCryptoPortfolioOptimizer {
    fn new(user_metrics: UserMetrics, lightning: LightningInterface, taro: TaroInterface, dlc: DLCInterface, ordinals: OrdinalInterface) -> Self {
        let vs = nn::VarStore::new(Device::Cpu);
        let portfolio_model = nn::seq()
            .add(nn::linear(&vs.root(), 130, 256, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 256, 128, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 128, 35, Default::default()));
        
        let optimizer = Box::new(nn::Adam::default().build(&vs, 1e-4).unwrap());

        Self {
            portfolio_model,
            user_metrics,
            optimizer,
            internal_data: InternalDataProvider::new(),
            research_db: ResearchPaperDatabase::new(),
            lightning,
            taro,
            dlc,
            ordinals,
        }
    }

    fn optimize_portfolio(&self, portfolio_data: &[f64]) -> Result<MLOutput, MLError> {
        let input_tensor = Tensor::of_slice(portfolio_data).view([-1, 130]);
        let output = self.portfolio_model.forward(&input_tensor);
        
        let weights = output.softmax(-1).double_value(&[0]);
        let mut optimized_weights: Vec<f64> = weights.iter().map(|&w| w as f64).collect();
        
        self.apply_constraints(&mut optimized_weights);
        
        let expected_return = self.calculate_expected_return(&optimized_weights);
        let portfolio_risk = self.calculate_portfolio_risk(&optimized_weights);
        
        let sharpe_ratio = (expected_return - self.internal_data.get_risk_free_rate()) / portfolio_risk;
        
        let market_trends = self.analyze_market_trends();
        let on_chain_metrics = self.analyze_on_chain_metrics();
        let lightning_metrics = self.analyze_lightning_network();
        let defi_metrics = self.analyze_defi_metrics();
        let ordinal_metrics = self.analyze_ordinal_market();
        let taro_metrics = self.analyze_taro_assets();

        let adjusted_weights = self.adjust_weights(&optimized_weights, &market_trends, &on_chain_metrics, &lightning_metrics, &defi_metrics, &ordinal_metrics, &taro_metrics);
        
        let confidence = self.calculate_confidence(&adjusted_weights);

        Ok(MLOutput {
            prediction: sharpe_ratio,
            confidence,
            additional_info: Some(HashMap::from([
                ("optimized_weights".to_string(), adjusted_weights),
                ("expected_return".to_string(), vec![expected_return]),
                ("portfolio_risk".to_string(), vec![portfolio_risk]),
                ("sharpe_ratio".to_string(), vec![sharpe_ratio]),
                ("market_trends".to_string(), market_trends),
                ("on_chain_metrics".to_string(), on_chain_metrics.values().cloned().collect()),
                ("lightning_metrics".to_string(), lightning_metrics.values().cloned().collect()),
                ("defi_metrics".to_string(), defi_metrics.values().cloned().collect()),
                ("ordinal_metrics".to_string(), ordinal_metrics.values().cloned().collect()),
                ("taro_metrics".to_string(), taro_metrics.values().cloned().collect()),
            ])),
        })
    }

    fn apply_constraints(&self, weights: &mut Vec<f64>) {
        // Ensure no short selling
        for w in weights.iter_mut() {
            *w = w.max(0.0);
        }
        
        // Normalize weights to sum to 1
        let sum: f64 = weights.iter().sum();
        for w in weights.iter_mut() {
            *w /= sum;
        }
        
        // Apply maximum allocation constraint (e.g., 30% per asset)
        let max_allocation = 0.3;
        for w in weights.iter_mut() {
            *w = w.min(max_allocation);
        }
        
        // Re-normalize after applying max allocation constraint
        let sum: f64 = weights.iter().sum();
        for w in weights.iter_mut() {
            *w /= sum;
        }
    }

    fn calculate_expected_return(&self, weights: &[f64]) -> f64 {
        let historical_returns = self.internal_data.get_historical_returns();
        let asset_correlations = self.internal_data.get_asset_correlations();
        let market_trends = self.internal_data.get_market_trends();
        let lightning_growth = self.lightning.get_network_growth();
        let taro_adoption = self.taro.get_adoption_rate();
        let dlc_market_impact = self.dlc.get_market_impact();
        let ordinal_adoption = self.ordinals.get_adoption_rate();

        let mut expected_return = 0.0;
        for (i, &weight) in weights.iter().enumerate() {
            let asset_return = historical_returns[i] * (1.0 + market_trends[i]);
            let correlation_factor = asset_correlations[i].iter().zip(weights).map(|(&c, &w)| c * w).sum::<f64>();
            let lightning_factor = if i == 0 { lightning_growth } else { 1.0 };
            let taro_factor = if i < 5 { 1.0 + taro_adoption * 0.1 } else { 1.0 };
            let dlc_factor = 1.0 + dlc_market_impact * 0.05;
            let ordinal_factor = if i == 0 { 1.0 + ordinal_adoption * 0.15 } else { 1.0 };
            
            expected_return += weight * asset_return * (1.0 + correlation_factor) * 
                               lightning_factor * taro_factor * dlc_factor * ordinal_factor;
        }

        // Apply CAPM (Capital Asset Pricing Model) adjustment
        let market_return = self.internal_data.get_market_return();
        let risk_free_rate = self.internal_data.get_risk_free_rate();
        let portfolio_beta = self.calculate_portfolio_beta(weights);
        
        expected_return = risk_free_rate + portfolio_beta * (market_return - risk_free_rate);

        expected_return
    }

    fn calculate_portfolio_risk(&self, weights: &[f64]) -> f64 {
        let covariance_matrix = self.internal_data.get_covariance_matrix();
        let lightning_risk = self.lightning.get_network_risk();
        let taro_risk = self.taro.get_protocol_risk();
        let dlc_risk = self.dlc.get_contract_risk();
        let ordinal_risk = self.ordinals.get_market_risk();

        let mut portfolio_variance = 0.0;

        for (i, &w_i) in weights.iter().enumerate() {
            for (j, &w_j) in weights.iter().enumerate() {
                portfolio_variance += w_i * w_j * covariance_matrix[i][j];
            }
        }

        // Apply Conditional Value at Risk (CVaR) adjustment
        let confidence_level = 0.95;
        let historical_returns = self.internal_data.get_historical_returns();
        let portfolio_returns: Vec<f64> = historical_returns.iter()
            .map(|returns| returns.iter().zip(weights).map(|(&r, &w)| r * w).sum())
            .collect();
        let cvar = self.calculate_cvar(&portfolio_returns, confidence_level);

        let combined_risk = (portfolio_variance.sqrt() + cvar) * (1.0 + lightning_risk * 0.1 + taro_risk * 0.05 + 
                             dlc_risk * 0.03 + ordinal_risk * 0.07);
        combined_risk
    }

    fn calculate_portfolio_beta(&self, weights: &[f64]) -> f64 {
        let asset_betas = self.internal_data.get_asset_betas();
        weights.iter().zip(asset_betas.iter()).map(|(&w, &b)| w * b).sum()
    }

    fn calculate_cvar(&self, returns: &[f64], confidence_level: f64) -> f64 {
        let mut sorted_returns = returns.to_vec();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let cutoff_index = ((1.0 - confidence_level) * returns.len() as f64).floor() as usize;
        sorted_returns[..cutoff_index].iter().sum::<f64>() / cutoff_index as f64
    }

    fn calculate_confidence(&self, optimized_weights: &[f64]) -> f64 {
        let base_confidence = 0.75;
        let usage_factor = 1.0 + (self.user_metrics.usage_level as f64 * 0.05);
        let contribution_factor = 1.0 + (self.user_metrics.contributions as f64 * 0.04);
        let diversity_factor = 1.0 - (optimized_weights.iter().map(|&w| w.powi(2)).sum::<f64>().sqrt() / optimized_weights.len() as f64);
        let market_sentiment = self.internal_data.get_market_sentiment();
        let lightning_confidence = self.lightning.get_network_confidence();
        let taro_confidence = self.taro.get_protocol_confidence();
        let dlc_confidence = self.dlc.get_contract_confidence();
        let ordinal_confidence = self.ordinals.get_market_confidence();
        
        (base_confidence * usage_factor * contribution_factor * diversity_factor * 
         market_sentiment * lightning_confidence * taro_confidence * 
         dlc_confidence * ordinal_confidence).min(1.0)
    }

    fn analyze_market_trends(&self) -> Vec<f64> {
        let historical_data = self.internal_data.get_historical_data();
        let mut trends = Vec::new();

        for asset in historical_data {
            let trend = self.calculate_trend(asset);
            trends.push(trend);
        }

        trends
    }

    fn calculate_trend(&self, asset_data: Vec<f64>) -> f64 {
        let window_size = 14; // 14-day moving average
        let mut trend = 0.0;

        for i in window_size..asset_data.len() {
            let window = &asset_data[i - window_size..i];
            let avg = window.iter().sum::<f64>() / window_size as f64;
            trend += if asset_data[i] > avg { 1.0 } else { -1.0 };
        }

        trend / (asset_data.len() - window_size) as f64
    }

    fn analyze_on_chain_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        metrics.insert("active_addresses".to_string(), self.blockchain.get_active_addresses());
        metrics.insert("transaction_volume".to_string(), self.blockchain.get_transaction_volume());
        metrics.insert("mining_difficulty".to_string(), self.blockchain.get_mining_difficulty());
        metrics.insert("mempool_size".to_string(), self.blockchain.get_mempool_size());

        metrics
    }

    fn analyze_lightning_network(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        metrics.insert("channel_capacity".to_string(), self.lightning.get_total_channel_capacity());
        metrics.insert("node_count".to_string(), self.lightning.get_node_count() as f64);
        metrics.insert("payment_volume".to_string(), self.lightning.get_payment_volume());

        metrics
    }

    fn analyze_defi_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        metrics.insert("total_value_locked".to_string(), self.internal_data.get_total_value_locked());
        metrics.insert("yield_farming_returns".to_string(), self.internal_data.get_yield_farming_returns());
        metrics.insert("liquidity_pool_depth".to_string(), self.internal_data.get_liquidity_pool_depth());

        metrics
    }

    fn analyze_ordinal_market(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("total_inscriptions".to_string(), self.ordinals.get_total_inscriptions());
        metrics.insert("daily_inscription_rate".to_string(), self.ordinals.get_daily_inscription_rate());
        metrics.insert("average_inscription_fee".to_string(), self.ordinals.get_average_inscription_fee());
        metrics
    }

    fn analyze_taro_assets(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("total_taro_assets".to_string(), self.taro.get_total_assets());
        metrics.insert("daily_taro_transactions".to_string(), self.taro.get_daily_transactions());
        metrics.insert("taro_liquidity".to_string(), self.taro.get_total_liquidity());
        metrics
    }

    fn adjust_weights(&self, weights: &[f64], market_trends: &[f64], on_chain_metrics: &HashMap<String, f64>, lightning_metrics: &HashMap<String, f64>, defi_metrics: &HashMap<String, f64>, ordinal_metrics: &HashMap<String, f64>, taro_metrics: &HashMap<String, f64>) -> Vec<f64> {
        let mut adjusted_weights = weights.to_vec();

        for (i, weight) in adjusted_weights.iter_mut().enumerate() {
            let trend_factor = 1.0 + market_trends[i] * 0.1;
            let on_chain_factor = 1.0 + (on_chain_metrics["active_addresses"] / 1_000_000.0).min(0.1);
            let lightning_factor = 1.0 + (lightning_metrics["channel_capacity"] / 1_000_000_000.0).min(0.1);
            let defi_factor = 1.0 + (defi_metrics["total_value_locked"] / 10_000_000_000.0).min(0.1);
            let ordinal_factor = 1.0 + (ordinal_metrics["total_inscriptions"] / 1_000_000.0).min(0.05);
            let taro_factor = 1.0 + (taro_metrics["total_taro_assets"] / 1_000_000.0).min(0.05);

            *weight *= trend_factor * on_chain_factor * lightning_factor * defi_factor * ordinal_factor * taro_factor;
        }

        // Normalize weights
        let sum: f64 = adjusted_weights.iter().sum();
        for weight in adjusted_weights.iter_mut() {
            *weight /= sum;
        }

        adjusted_weights
    }
}