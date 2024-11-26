use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use thiserror::Error;
use metrics::{counter, gauge};

#[derive(Error, Debug)]
pub enum RateLimiterError {
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub struct RateLimiter {
    capacity: u32,
    refill_rate: f64,
    tokens: f64,
    last_refill: Instant,
    metrics_prefix: String,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_rate: f64) -> Result<Self, RateLimiterError> {
        if capacity == 0 {
            return Err(RateLimiterError::InvalidConfig("Capacity cannot be zero".into()));
        }
        if refill_rate <= 0.0 {
            return Err(RateLimiterError::InvalidConfig("Refill rate must be positive".into()));
        }

        Ok(Self {
            capacity,
            refill_rate,
            tokens: capacity as f64,
            last_refill: Instant::now(),
            metrics_prefix: "rate_limiter".into(),
        })
    }

    pub async fn acquire(&mut self, tokens: u32) -> Result<(), RateLimiterError> {
        self.refill();
        
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            counter!(&format!("{}_tokens_acquired", self.metrics_prefix)).increment(tokens as u64);
            Ok(())
        } else {
            counter!(&format!("{}_rate_limit_exceeded", self.metrics_prefix)).increment(1);
            Err(RateLimiterError::RateLimitExceeded)
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        self.tokens = (self.tokens + elapsed.as_secs_f64() * self.refill_rate)
            .min(self.capacity as f64);
        self.last_refill = now;
        
        gauge!(&format!("{}_current_tokens", self.metrics_prefix)).set(self.tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(10, 1.0).unwrap();
        
        // Should succeed
        assert!(limiter.acquire(5).await.is_ok());
        
        // Should fail - not enough tokens
        assert!(limiter.acquire(6).await.is_err());
        
        // Wait for refill
        sleep(Duration::from_secs(5)).await;
        
        // Should succeed after refill
        assert!(limiter.acquire(5).await.is_ok());
    }
}
