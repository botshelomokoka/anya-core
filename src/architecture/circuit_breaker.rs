use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use crate::architecture::errors::{HexagonalError, HexagonalResult};
use crate::architecture::types::{CircuitBreakerMetrics, StateTransition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl std::fmt::Display for CircuitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "CLOSED"),
            CircuitState::Open => write!(f, "OPEN"),
            CircuitState::HalfOpen => write!(f, "HALF-OPEN"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub reset_timeout: Duration,
    pub half_open_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            reset_timeout: Duration::from_secs(60),
            half_open_timeout: Duration::from_secs(30),
        }
    }
}

pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitBreakerState>>,
    config: CircuitBreakerConfig,
    metrics: Arc<RwLock<CircuitBreakerMetrics>>,
}

#[derive(Debug)]
struct CircuitBreakerState {
    current_state: CircuitState,
    failure_count: u32,
    success_count: u32,
    last_failure: Option<Instant>,
    last_success: Option<Instant>,
    last_state_change: Instant,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitBreakerState {
                current_state: CircuitState::Closed,
                failure_count: 0,
                success_count: 0,
                last_failure: None,
                last_success: None,
                last_state_change: Instant::now(),
            })),
            config,
            metrics: Arc::new(RwLock::new(CircuitBreakerMetrics::new())),
        }
    }

    pub async fn execute<F, T>(&self, operation: F) -> HexagonalResult<T>
    where
        F: FnOnce() -> HexagonalResult<T>,
    {
        self.pre_execute().await?;

        let result = operation();
        self.post_execute(result).await
    }

    async fn pre_execute(&self) -> HexagonalResult<()> {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.write().await;

        metrics.total_requests += 1;

        match state.current_state {
            CircuitState::Open => {
                if state.last_state_change.elapsed() >= self.config.reset_timeout {
                    self.transition_to_half_open(&mut state, &mut metrics).await;
                } else {
                    return Err(HexagonalError::CircuitBreakerError(
                        "Circuit breaker is open".into(),
                    ));
                }
            }
            CircuitState::HalfOpen => {
                if state.last_state_change.elapsed() >= self.config.half_open_timeout {
                    self.transition_to_open(&mut state, &mut metrics).await;
                    return Err(HexagonalError::CircuitBreakerError(
                        "Circuit breaker timed out in half-open state".into(),
                    ));
                }
            }
            CircuitState::Closed => {}
        }

        Ok(())
    }

    async fn post_execute<T>(&self, result: HexagonalResult<T>) -> HexagonalResult<T> {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.write().await;

        match result {
            Ok(value) => {
                self.handle_success(&mut state, &mut metrics).await;
                Ok(value)
            }
            Err(error) => {
                self.handle_failure(&mut state, &mut metrics).await;
                Err(error)
            }
        }
    }

    async fn handle_success(
        &self,
        state: &mut CircuitBreakerState,
        metrics: &mut CircuitBreakerMetrics,
    ) {
        metrics.successful_requests += 1;
        state.last_success = Some(Instant::now());
        state.success_count += 1;
        state.failure_count = 0;

        if state.current_state == CircuitState::HalfOpen
            && state.success_count >= self.config.success_threshold
        {
            self.transition_to_closed(state, metrics).await;
        }
    }

    async fn handle_failure(
        &self,
        state: &mut CircuitBreakerState,
        metrics: &mut CircuitBreakerMetrics,
    ) {
        metrics.failed_requests += 1;
        state.last_failure = Some(Instant::now());
        state.failure_count += 1;
        state.success_count = 0;

        if state.current_state == CircuitState::Closed
            && state.failure_count >= self.config.failure_threshold
        {
            self.transition_to_open(state, metrics).await;
        } else if state.current_state == CircuitState::HalfOpen {
            self.transition_to_open(state, metrics).await;
        }
    }

    async fn transition_to_open(
        &self,
        state: &mut CircuitBreakerState,
        metrics: &mut CircuitBreakerMetrics,
    ) {
        let transition = StateTransition {
            timestamp: chrono::Utc::now(),
            from_state: state.current_state,
            to_state: CircuitState::Open,
            reason: "Failure threshold exceeded".into(),
        };

        state.current_state = CircuitState::Open;
        state.last_state_change = Instant::now();
        state.failure_count = 0;
        state.success_count = 0;

        metrics.state_transitions.push(transition);
    }

    async fn transition_to_half_open(
        &self,
        state: &mut CircuitBreakerState,
        metrics: &mut CircuitBreakerMetrics,
    ) {
        let transition = StateTransition {
            timestamp: chrono::Utc::now(),
            from_state: state.current_state,
            to_state: CircuitState::HalfOpen,
            reason: "Reset timeout elapsed".into(),
        };

        state.current_state = CircuitState::HalfOpen;
        state.last_state_change = Instant::now();
        state.failure_count = 0;
        state.success_count = 0;

        metrics.state_transitions.push(transition);
    }

    async fn transition_to_closed(
        &self,
        state: &mut CircuitBreakerState,
        metrics: &mut CircuitBreakerMetrics,
    ) {
        let transition = StateTransition {
            timestamp: chrono::Utc::now(),
            from_state: state.current_state,
            to_state: CircuitState::Closed,
            reason: "Success threshold reached".into(),
        };

        state.current_state = CircuitState::Closed;
        state.last_state_change = Instant::now();
        state.failure_count = 0;
        state.success_count = 0;

        metrics.state_transitions.push(transition);
    }

    pub async fn get_metrics(&self) -> CircuitBreakerMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn get_state(&self) -> CircuitState {
        self.state.read().await.current_state
    }

    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.write().await;

        let transition = StateTransition {
            timestamp: chrono::Utc::now(),
            from_state: state.current_state,
            to_state: CircuitState::Closed,
            reason: "Manual reset".into(),
        };

        state.current_state = CircuitState::Closed;
        state.last_state_change = Instant::now();
        state.failure_count = 0;
        state.success_count = 0;

        metrics.state_transitions.push(transition);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_circuit_breaker_transitions() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 1,
            reset_timeout: Duration::from_millis(100),
            half_open_timeout: Duration::from_millis(50),
        };

        let breaker = CircuitBreaker::new(config);

        // Test transition to open
        for _ in 0..2 {
            let result: HexagonalResult<()> = breaker
                .execute(|| Err(HexagonalError::NetworkError("Test error".into())))
                .await;
            assert!(result.is_err());
        }
        assert_eq!(breaker.get_state().await, CircuitState::Open);

        // Wait for reset timeout
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Test transition to half-open
        let result = breaker
            .execute(|| Ok::<_, HexagonalError>(()))
            .await;
        assert!(result.is_ok());
        assert_eq!(breaker.get_state().await, CircuitState::Closed);

        // Verify metrics
        let metrics = breaker.get_metrics().await;
        assert_eq!(metrics.total_requests, 3);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 2);
        assert_eq!(metrics.state_transitions.len(), 3);
    }
}
