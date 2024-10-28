use bitcoin::FeeRate;

pub struct FeeEstimator {
    rpc_client: BitcoinRpcClient,
}

impl FeeEstimator {
    pub async fn estimate_fee(&self, target_blocks: u16) -> Result<FeeRate, BitcoinError> {
        let fee_rate = self.rpc_client
            .estimate_smart_fee(target_blocks)
            .await?;
            
        Ok(FeeRate::from_sat_per_vb(fee_rate.fee_rate))
    }
}
