use crate::license;
use crate::interlink::Interlink;
use log::{info, error};

pub mod advanced_analytics;
pub mod high_volume_trading;

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let license_key = std::env::var("ANYA_LICENSE_KEY")
        .map_err(|_| "ANYA_LICENSE_KEY not set")?;

    match license::verify_license(&license_key).await {
        Ok(license) => {
            info!("Enterprise license verified successfully");

            let mut interlink = Interlink::new();

            if license.features.contains(&"advanced_analytics".to_string()) {
                info!("Initializing advanced analytics module");
                advanced_analytics::init(&mut interlink)?;
            }

            if license.features.contains(&"high_volume_trading".to_string()) {
                info!("Initializing high volume trading module");
                high_volume_trading::init(&mut interlink)?;
            }

            // Schedule regular financial reporting
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await; // Daily report
                    match interlink.generate_report(Utc::now() - chrono::Duration::days(1), Utc::now()) {
                        Ok(report) => info!("Daily financial report generated: {:?}", report),
                        Err(e) => error!("Failed to generate daily financial report: {}", e),
                    }
                }
            });

            Ok(())
        }
        Err(e) => {
            error!("Failed to verify enterprise license: {}", e);
            Err(Box::new(e))
        }
    }
}