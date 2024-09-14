use crate::user_metrics::UserMetrics;
use crate::ml::advanced_models::AdvancedBitcoinPricePredictor;
use crate::bitcoin::BitcoinClient;
use crate::lightning::LightningClient;
use crate::ml_logic::data_processing::process_market_data;
use crate::market_data::MarketDataFetcher;
use crate::ml::MLInput;

pub struct HighVolumeTrading {
    price_predictor: AdvancedBitcoinPricePredictor,
    bitcoin_client: BitcoinClient,
    lightning_client: LightningClient,
    user_metrics: UserMetrics,
}

impl HighVolumeTrading {
    pub fn new(user_metrics: UserMetrics, bitcoin_client: BitcoinClient, lightning_client: LightningClient) -> Self {
        let price_predictor = AdvancedBitcoinPricePredictor::new(user_metrics.clone());
        Self {
            price_predictor,
            bitcoin_client,
            lightning_client,
            user_metrics,
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
        let invoice = self.lightning_client.create_invoice(1000, "Buy order", 3600)?;
        println!("Lightning invoice created: {}", invoice.to_string());
        Ok(())
    }

    fn place_sell_order(&self) -> Result<(), Box<dyn Error>> {
        println!("Placing sell order...");
        // Implement sell order logic using Lightning Network for faster settlement
        let invoice = self.lightning_client.create_invoice(1000, "Sell order", 3600)?;
        println!("Lightning invoice created: {}", invoice.to_string());
        Ok(())
    }

    fn process_lightning_payments(&self) -> Result<(), Box<dyn Error>> {
        println!("Processing Lightning Network payments...");
        let pending_invoices = self.lightning_client.list_invoices()?;
        for invoice_str in pending_invoices {
            let invoice = Invoice::from_str(&invoice_str)?;
            if invoice.is_expired() {
                println!("Invoice {} has expired", invoice.payment_hash());
            } else if invoice.is_paid() {
                println!("Payment received for invoice {}", invoice.payment_hash());
                // Process the payment (e.g., update order status, release funds)
            }
        }
        Ok(())
    }
}

pub fn init(user_metrics: &UserMetrics, bitcoin_client: BitcoinClient, lightning_client: LightningClient) -> HighVolumeTrading {
    HighVolumeTrading::new(user_metrics.clone(), bitcoin_client, lightning_client)
}