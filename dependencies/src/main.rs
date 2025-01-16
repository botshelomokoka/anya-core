mod ml_logic;

async fn setup_license_management(config: Config) -> LicenseManager {
    let auth_provider: Box<dyn BlockchainAuth> = match config.auth.provider_type {
        AuthProviderType::Stacks => Box::new(StacksAuth::new(config.auth.credentials)),
        AuthProviderType::Lightning => Box::new(LightningAuth::new(config.auth.credentials)),
        AuthProviderType::Web5 => Box::new(Web5Auth::new(config.auth.credentials)),
        _ => Box::new(DefaultAuth::new(config.auth.credentials)),
    };

    LicenseManager::new(
        auth_provider,
        ApiMetricsCollector::new()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    let license_manager = setup_license_management(config).await?;
    // ... rest of application
    Ok(())
}
