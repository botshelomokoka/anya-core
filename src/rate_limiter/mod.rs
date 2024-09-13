use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    network_load: Arc<Mutex<f32>>,
    base_limit: u32,
    max_limit: u32,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            limits: Arc::new(Mutex::new(HashMap::new())),
            network_load: Arc::new(Mutex::new(0.5)), // Start with 50% load
            base_limit: 100,
            max_limit: 1000,
        }
    }

    pub async fn check_rate_limit(&self, identifier: &str) -> bool {
        let mut limits = self.limits.lock().await;
        let now = Instant::now();
        let load = *self.network_load.lock().await;

        let max_requests = self.calculate_max_requests(load);
        let window = Duration::from_secs(60); // 1 minute window

        let (count, last_reset) = limits.entry(identifier.to_string()).or_insert((0, now));

        if now.duration_since(*last_reset) >= window {
            *count = 1;
            *last_reset = now;
            true
        } else if *count < max_requests {
            *count += 1;
            true
        } else {
            false
        }
    }

    fn calculate_max_requests(&self, load: f32) -> u32 {
        let dynamic_limit = (self.base_limit as f32 * (1.0 - load)) as u32;
        dynamic_limit.clamp(10, self.max_limit)
    }

    pub async fn update_network_load(&self, load: f32) {
        let mut current_load = self.network_load.lock().await;
        *current_load = load.clamp(0.0, 1.0);
    }

    pub async fn auto_adjust(&mut self) {
        let system = System::new_all();
        let total_memory = system.total_memory();
        let num_cores = system.processors().len();

        // Adjust base limit based on system resources
        self.base_limit = (num_cores * 10).max(100).min(1000);

        // Adjust max limit based on available memory
        self.max_limit = (total_memory / 1024 / 1024).min(10000) as u32;
    }
}