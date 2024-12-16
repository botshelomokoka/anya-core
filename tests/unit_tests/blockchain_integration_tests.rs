use anya_core::{
    blockchain::{BitcoinCore, Lightning, Stacks},
    config::Config,
    ml_logic::{MLLogic, MLFeeManager},
};
use anyhow::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bitcoin_integration() -> Result<()> {
        let config = Config::load_test_config()?;
        let bitcoin = BitcoinCore::new(
            &config.bitcoin_rpc_url,
            config.bitcoin_auth.clone(),
            config.bitcoin_network.clone()
        )?;
        
        let block_count = bitcoin.get_block_count()?;
        assert!(block_count > 0);
        Ok(())
    }

    // Add other tests...
}
