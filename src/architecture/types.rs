use std::time::Duration;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    pub version: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub accuracy: f64,
    pub training_progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub connected_peers: usize,
    pub sync_progress: f64,
    pub bandwidth_usage: f64,
    pub latency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub is_connected: bool,
    pub uptime: Duration,
    pub throughput: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub low: u64,
    pub medium: u64,
    pub high: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub id: String,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct SpanGuard {
    span: Span,
}

impl SpanGuard {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        self.span.end_time = Some(chrono::Utc::now());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub latency: Vec<Duration>,
    pub throughput: f64,
    pub error_rate: f64,
    pub resource_utilization: ResourceUtilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub error_count: u64,
    pub error_types: std::collections::HashMap<String, u64>,
    pub error_rates: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMetrics {
    pub transactions_processed: u64,
    pub active_users: u64,
    pub success_rate: f64,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMetrics {
    pub performance: PerformanceMetrics,
    pub errors: ErrorMetrics,
    pub resources: ResourceMetrics,
    pub business: BusinessMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub network_bandwidth: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthType {
    Basic,
    OAuth2,
    JWT,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub additional: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub resource: String,
    pub action: String,
    pub conditions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub user: String,
    pub resource: String,
    pub action: String,
    pub outcome: AuditOutcome,
    pub details: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Failure,
    Warning,
}

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: HealthState,
    pub components: std::collections::HashMap<String, ComponentHealth>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone)]
pub struct ComponentHealth {
    pub state: HealthState,
    pub message: Option<String>,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub metrics: std::collections::HashMap<String, f64>,
}

impl HealthStatus {
    pub fn from_results(results: Vec<ComponentHealth>) -> Self {
        let mut components = std::collections::HashMap::new();
        let mut overall = HealthState::Healthy;

        for (i, result) in results.into_iter().enumerate() {
            if result.state == HealthState::Unhealthy {
                overall = HealthState::Unhealthy;
            } else if result.state == HealthState::Degraded && overall == HealthState::Healthy {
                overall = HealthState::Degraded;
            }
            components.insert(format!("component_{}", i), result);
        }

        Self {
            status: overall,
            components,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    async fn check(&self) -> Result<ComponentHealth, crate::HexagonalError>;
}

#[async_trait::async_trait]
pub trait ErrorRecovery: Send + Sync {
    async fn attempt_recovery(&self, error: &crate::HexagonalError) -> Result<(), crate::HexagonalError>;
    async fn get_recovery_status(&self) -> RecoveryStatus;
}

#[derive(Debug, Clone)]
pub struct RecoveryStatus {
    pub attempts: u32,
    pub last_attempt: chrono::DateTime<chrono::Utc>,
    pub success_rate: f64,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub state_transitions: Vec<StateTransition>,
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub from_state: CircuitState,
    pub to_state: CircuitState,
    pub reason: String,
}

impl CircuitBreakerMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            state_transitions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtomicState(std::sync::atomic::AtomicU8);

impl AtomicState {
    pub fn new(initial: CircuitState) -> Self {
        Self(std::sync::atomic::AtomicU8::new(initial as u8))
    }

    pub fn load(&self, order: std::sync::atomic::Ordering) -> CircuitState {
        match self.0.load(order) {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => panic!("Invalid circuit state"),
        }
    }

    pub fn store(&self, state: CircuitState, order: std::sync::atomic::Ordering) {
        self.0.store(state as u8, order);
    }
}

#[derive(Debug, Clone)]
pub struct Cache<T> {
    storage: std::sync::Arc<parking_lot::RwLock<std::collections::HashMap<String, CacheEntry<T>>>>,
    config: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    value: T,
    expiry: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub ttl: Duration,
    pub max_size: usize,
}

impl<T: Clone + Send + Sync + 'static> Cache<T> {
    pub fn new() -> Self {
        Self {
            storage: std::sync::Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
            config: CacheConfig {
                ttl: Duration::from_secs(3600),
                max_size: 1000,
            },
        }
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        let storage = self.storage.read();
        storage.get(key).and_then(|entry| {
            if entry.expiry > chrono::Utc::now() {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }

    pub async fn set(&self, key: String, value: T) -> Result<(), crate::HexagonalError> {
        let mut storage = self.storage.write();
        if storage.len() >= self.config.max_size {
            return Err(crate::HexagonalError::CacheError("Cache full".into()));
        }

        storage.insert(key, CacheEntry {
            value,
            expiry: chrono::Utc::now() + chrono::Duration::from_std(self.config.ttl).unwrap(),
        });
        Ok(())
    }
}
