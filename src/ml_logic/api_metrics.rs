pub struct ApiMetricsCollector {
    payment_processor: PaymentProcessor,
    usage_tracker: UsageTracker,
}

impl ApiMetricsCollector {
    pub async fn collect_and_process(&self, license_key: &str) -> Result<UsageMetrics, MetricsError> {
        let usage = self.usage_tracker.get_metrics(license_key).await?;
        self.payment_processor.process_charges(&usage).await?;
        Ok(usage)
    }
}

