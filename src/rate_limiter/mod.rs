use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RateLimiterError {
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

pub struct RateLimiter {
    capacity: u32,
    refill_rate: f64,
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            limits: Arc::new(Mutex::new(HashMap::new())),
            network_load: Arc::new(Mutex::new(0.25)), // Start with 25% load
            base_limit: 100,
            max_limit: 1000,
        }
    }

    pub async fn acquire(&mut self, tokens: u32) -> Result<(), RateLimiterError> {
        self.refill();
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            Ok(())
        } else {
            let wait_time = Duration::from_secs_f64((tokens as f64 - self.tokens) / self.refill_rate);
            sleep(wait_time).await;
            self.refill();
            self.tokens -= tokens as f64;
            Ok(())
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        self.tokens = (self.tokens + elapsed.as_secs_f64() * self.refill_rate).min(self.capacity as f64);
        self.last_refill = now;
    }
}