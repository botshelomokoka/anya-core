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
    pub fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            capacity,
            refill_rate,
            tokens: capacity as f64,
            last_refill: Instant::now(),
        }
    }

    pub fn wrapped(capacity: u32, refill_rate: f64) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new(capacity, refill_rate)))
    }

    pub async fn acquire(rate_limiter: Arc<Mutex<Self>>, tokens: u32) -> Result<(), RateLimiterError> {
        loop {
            let mut rl = rate_limiter.lock().await;
            rl.refill();
            if rl.tokens >= tokens as f64 {
                rl.tokens -= tokens as f64;
                return Ok(());
            } else {
                let wait_time = Duration::from_secs_f64((tokens as f64 - rl.tokens) / rl.refill_rate);
                drop(rl); // Release the lock before sleeping
                sleep(wait_time).await;
            }
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        self.tokens = (self.tokens + elapsed.as_secs_f64() * self.refill_rate).min(self.capacity as f64);
        self.last_refill = now;
    }
}