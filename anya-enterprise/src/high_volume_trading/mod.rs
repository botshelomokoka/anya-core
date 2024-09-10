use crate::user_metrics::UserMetrics;
use crate::ml::advanced_models::AdvancedBitcoinPricePredictor;
use crate::bitcoin::BitcoinClient;
use crate::lightning::LightningClient;
use crate::ml_logic::data_processing::process_market_data;
use crate::market_data::MarketDataFetcher;
use crate::ml::MLInput;
use tokio; // Ensure you have tokio in your dependencies
use std::sync::{Arc, Mutex}; // For shared state
use crate::ml_logic::metrics::Metrics; // Assuming you have a Metrics struct in ml_logic

pub struct HighVolumeTrading {
    price_predictor: AdvancedBitcoinPricePredictor,
    bitcoin_client: BitcoinClient,
    lightning_client: LightningClient,
    user_metrics: UserMetrics,
    metrics: Arc<Mutex<Metrics>>, // Shared metrics
}

impl HighVolumeTrading {
    pub fn new(user_metrics: UserMetrics, bitcoin_client: BitcoinClient, lightning_client: LightningClient, metrics: Metrics) -> Self {
        let price_predictor = AdvancedBitcoinPricePredictor::new(user_metrics.clone());
        Self {
            price_predictor,
            bitcoin_client,
            lightning_client,
            user_metrics,
            metrics: Arc::new(Mutex::new(metrics)), // Initialize shared metrics
        }
    }

    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("Executing high volume trading strategy with Lightning Network support...");
        
        let price_prediction = self.price_predictor.predict(&self.get_market_data())?;
        
        if price_prediction.confidence > 0.8 {
            if price_prediction.prediction > self.bitcoin_client.get_current_price()? {
                self.place_buy_order()?;
            } else {
                self.place_sell_order()?;
            }
        }

        // Check for any incoming Lightning payments
        self.process_lightning_payments()?;

        Ok(())
    }

    fn get_market_data(&self) -> Result<MLInput, Box<dyn Error>> {
        let market_data_fetcher = MarketDataFetcher::new();
        let raw_data = market_data_fetcher.fetch_latest_data()?;
        let processed_data = process_market_data(raw_data)?;
        
        Ok(MLInput {
            features: processed_data.features,
            label: processed_data.label,
        })
    }

    fn place_buy_order(&self) -> Result<(), Box<dyn Error>> {
        println!("Placing buy order...");
        // Implement buy order logic using Lightning Network for faster settlement
        let invoice_amount = config.get::<u32>("invoice.amount")?;
        let invoice = self.lightning_client.create_invoice(invoice_amount, "Buy order", 3600)?;
        println!("Lightning invoice created: {}", invoice.to_string());
        Ok(())
    }

    fn place_sell_order(&self) -> Result<(), Box<dyn Error>> {
        println!("Placing sell order...");
        // Implement sell order logic using Lightning Network for faster settlement
        let invoice_amount = config.get::<u32>("invoice.amount")?;
        let invoice = self.lightning_client.create_invoice(invoice_amount, "Sell order", 3600)?;
        println!("Lightning invoice created: {}", invoice.to_string());
        Ok(())
    }

    async fn process_lightning_payments(&self) -> Result<(), Box<dyn Error>> {
        println!("Processing Lightning Network payments...");
        let pending_invoices = self.lightning_client.list_invoices()?;
        
        // Use futures to process invoices concurrently
        let tasks: Vec<_> = pending_invoices.into_iter().map(|invoice_str| {
            let metrics = Arc::clone(&self.metrics); // Clone the Arc for each task
            tokio::spawn(async move {
                let invoice = Invoice::from_str(&invoice_str)?;
                if invoice.is_expired() {
                    println!("Invoice {} has expired", invoice.payment_hash());
                } else if invoice.is_paid() {
                    println!("Payment received for invoice {}", invoice.payment_hash());
                    // Update metrics in a thread-safe manner
                    let mut metrics = metrics.lock().unwrap();
                    metrics.update_payment_received(invoice.payment_hash()); // Example method
                }
                Ok::<(), Box<dyn Error>>(())
            })
        }).collect();

        // Await all tasks to complete
        for task in tasks {
            task.await??; // Handle errors from tasks
        }

        Ok(())
    }
}

pub mod trading_engine;
pub mod risk_management;
pub mod order_execution;
pub mod market_data;
pub mod analytics;

pub fn init(user_metrics: &UserMetrics, bitcoin_client: BitcoinClient, lightning_client: LightningClient) -> HighVolumeTrading {
    HighVolumeTrading::new(user_metrics.clone(), bitcoin_client, lightning_client)
}