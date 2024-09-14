use crate::market_data::RawMarketData;
use crate::ml::MLInput;

pub fn process_market_data(raw_data: RawMarketData) -> Result<MLInput, Box<dyn std::error::Error>> {
    // Process the raw market data into features
    let features = vec![
        raw_data.price,
        raw_data.volume,
        raw_data.timestamp.timestamp() as f64,
    ];

    Ok(MLInput {
        features,
        label: raw_data.price, // Using current price as label for this example
    })
}

pub struct ProcessedData {
    pub features: Vec<f64>,
    pub label: f64,
}