use thiserror::Error;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::{error, warn, info, Level};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;
use tokio::time;

static ERROR_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum HexagonalError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Authorization error: {0}")]
    AuthzError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Circuit breaker error: {0}")]
    CircuitBreakerError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Business logic error: {0}")]
    BusinessError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Data conversion error: {0}")]
    DataConversionError(String),

    #[error("State transition error: {0}")]
    StateTransitionError(String),

    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhaustedError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl HexagonalError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            HexagonalError::NetworkError(_) |
            HexagonalError::DatabaseError(_) |
            HexagonalError::ServiceUnavailable(_) |
            HexagonalError::TimeoutError(_) |
            HexagonalError::ExternalServiceError(_)
        )
    }

    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            HexagonalError::ValidationError(_) |
            HexagonalError::AuthError(_) |
            HexagonalError::AuthzError(_) |
            HexagonalError::NotFoundError(_) |
            HexagonalError::BusinessError(_) |
            HexagonalError::DataConversionError(_)
        )
    }

    pub fn requires_intervention(&self) -> bool {
        matches!(
            self,
            HexagonalError::ConfigError(_) |
            HexagonalError::ResourceExhaustedError(_) |
            HexagonalError::ConcurrencyError(_)
        )
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            HexagonalError::IoError(_) => "E001",
            HexagonalError::NetworkError(_) => "E002",
            HexagonalError::DatabaseError(_) => "E003",
            HexagonalError::CacheError(_) => "E004",
            HexagonalError::AuthError(_) => "E005",
            HexagonalError::AuthzError(_) => "E006",
            HexagonalError::ValidationError(_) => "E007",
            HexagonalError::CircuitBreakerError(_) => "E008",
            HexagonalError::RateLimitError(_) => "E009",
            HexagonalError::ConfigError(_) => "E010",
            HexagonalError::ServiceUnavailable(_) => "E011",
            HexagonalError::TimeoutError(_) => "E012",
            HexagonalError::NotFoundError(_) => "E013",
            HexagonalError::BusinessError(_) => "E014",
            HexagonalError::ExternalServiceError(_) => "E015",
            HexagonalError::DataConversionError(_) => "E016",
            HexagonalError::StateTransitionError(_) => "E017",
            HexagonalError::ConcurrencyError(_) => "E018",
            HexagonalError::ResourceExhaustedError(_) => "E019",
            HexagonalError::InvalidOperationError(_) => "E020",
            HexagonalError::SerializationError(_) => "E021",
            HexagonalError::DeserializationError(_) => "E022",
            HexagonalError::UnknownError(_) => "E999",
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self {
            HexagonalError::IoError(_) |
            HexagonalError::NetworkError(_) |
            HexagonalError::TimeoutError(_) |
            HexagonalError::ServiceUnavailable(_) => ErrorSeverity::Warning,

            HexagonalError::AuthError(_) |
            HexagonalError::AuthzError(_) |
            HexagonalError::ValidationError(_) |
            HexagonalError::NotFoundError(_) |
            HexagonalError::BusinessError(_) => ErrorSeverity::Error,

            HexagonalError::DatabaseError(_) |
            HexagonalError::ConfigError(_) |
            HexagonalError::ResourceExhaustedError(_) |
            HexagonalError::ConcurrencyError(_) => ErrorSeverity::Critical,

            _ => ErrorSeverity::Info,
        }
    }

    pub fn increment_count(&self) -> u64 {
        ERROR_COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    pub fn get_error_count() -> u64 {
        ERROR_COUNTER.load(Ordering::SeqCst)
    }

    pub fn should_retry(&self, attempt: u32) -> bool {
        if !self.is_retryable() {
            return false;
        }

        match self {
            HexagonalError::NetworkError(_) => attempt < 3,
            HexagonalError::DatabaseError(_) => attempt < 2,
            HexagonalError::ServiceUnavailable(_) => attempt < 5,
            HexagonalError::TimeoutError(_) => attempt < 3,
            HexagonalError::ExternalServiceError(_) => attempt < 2,
            _ => false,
        }
    }

    pub fn retry_delay(&self, attempt: u32) -> Duration {
        let base_delay = match self {
            HexagonalError::NetworkError(_) => Duration::from_millis(100),
            HexagonalError::DatabaseError(_) => Duration::from_millis(200),
            HexagonalError::ServiceUnavailable(_) => Duration::from_millis(500),
            HexagonalError::TimeoutError(_) => Duration::from_millis(1000),
            HexagonalError::ExternalServiceError(_) => Duration::from_millis(2000),
            _ => Duration::from_millis(100),
        };

        base_delay.saturating_mul(2_u32.saturating_pow(attempt - 1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub error_count: u64,
    pub error_type_distribution: std::collections::HashMap<String, u64>,
    pub severity_distribution: std::collections::HashMap<ErrorSeverity, u64>,
    pub retry_attempts: u64,
    pub recovery_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub timestamp: DateTime<Utc>,
    pub error: HexagonalError,
    pub severity: ErrorSeverity,
    pub source: Option<String>,
    pub trace_id: Option<String>,
    pub user_id: Option<String>,
    pub attempt: u32,
    pub retry_count: u32,
    pub last_retry_timestamp: Option<DateTime<Utc>>,
    pub additional_info: std::collections::HashMap<String, String>,
    pub stack_trace: Option<String>,
    pub metrics: Option<ErrorMetrics>,
}

impl ErrorContext {
    pub fn new(error: HexagonalError) -> Self {
        Self {
            timestamp: Utc::now(),
            severity: error.severity(),
            error: error.clone(),
            source: None,
            trace_id: None,
            user_id: None,
            attempt: 1,
            retry_count: 0,
            last_retry_timestamp: None,
            additional_info: std::collections::HashMap::new(),
            stack_trace: std::backtrace::Backtrace::capture().to_string().into(),
            metrics: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn add_info(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_info.insert(key.into(), value.into());
        self
    }

    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
        self.last_retry_timestamp = Some(Utc::now());
    }

    pub fn should_retry(&self) -> bool {
        self.error.should_retry(self.retry_count)
    }

    pub fn retry_delay(&self) -> Duration {
        self.error.retry_delay(self.retry_count)
    }

    pub fn with_metrics(mut self, metrics: ErrorMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn log(&self) {
        let level = match self.severity {
            ErrorSeverity::Info => Level::INFO,
            ErrorSeverity::Warning => Level::WARN,
            ErrorSeverity::Error => Level::ERROR,
            ErrorSeverity::Critical => Level::ERROR,
        };

        let span = tracing::span!(
            level,
            "error_context",
            error_code = self.error.error_code(),
            severity = %self.severity,
            trace_id = self.trace_id.as_deref().unwrap_or("unknown"),
            user_id = self.user_id.as_deref().unwrap_or("unknown"),
            retry_count = self.retry_count,
        );

        let _guard = span.enter();

        match level {
            Level::ERROR => error!(
                error = %self.error,
                stack_trace = self.stack_trace.as_deref().unwrap_or(""),
                "Error occurred"
            ),
            Level::WARN => warn!(error = %self.error, "Warning occurred"),
            _ => info!(error = %self.error, "Info logged"),
        }
    }
}

pub type HexagonalResult<T> = Result<T, HexagonalError>;

#[macro_export]
macro_rules! with_context {
    ($result:expr) => {
        match $result {
            Ok(value) => Ok(value),
            Err(error) => {
                let context = $crate::ErrorContext::new(error)
                    .with_trace_id(Uuid::new_v4().to_string());
                context.log();
                Err(context)
            }
        }
    };
}

#[macro_export]
macro_rules! retry_with_backoff {
    ($op:expr, $max_attempts:expr) => {
        {
            let mut attempt = 1;
            let mut last_error = None;

            while attempt <= $max_attempts {
                match $op {
                    Ok(value) => break Ok(value),
                    Err(error) => {
                        let mut context = $crate::ErrorContext::new(error);
                        if !context.should_retry() {
                            context.log();
                            break Err(context);
                        }

                        let delay = context.retry_delay();
                        time::sleep(delay).await;
                        
                        context.increment_retry();
                        last_error = Some(context);
                        attempt += 1;
                    }
                }
            }

            if let Some(context) = last_error {
                context.log();
                Err(context)
            } else {
                unreachable!("Loop should have either succeeded or returned an error")
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_error_properties() {
        let error = HexagonalError::NetworkError("connection failed".to_string());
        assert!(error.is_retryable());
        assert!(!error.is_permanent());
        assert_eq!(error.error_code(), "E002");
        assert_eq!(error.severity(), ErrorSeverity::Warning);
    }

    #[test]
    fn test_error_context() {
        let error = HexagonalError::DatabaseError("connection timeout".to_string());
        let context = ErrorContext::new(error)
            .with_trace_id("trace-123".to_string())
            .with_user_id("user-456".to_string())
            .add_info("database", "primary");

        assert_eq!(context.severity, ErrorSeverity::Critical);
        assert!(context.trace_id.is_some());
        assert!(context.user_id.is_some());
        assert!(context.additional_info.contains_key("database"));
    }

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let mut attempts = 0;
        let result: Result<(), ErrorContext> = retry_with_backoff!(
            {
                attempts += 1;
                if attempts < 3 {
                    Err(HexagonalError::NetworkError("test".to_string()))
                } else {
                    Ok(())
                }
            },
            5
        );

        assert!(result.is_ok());
        assert_eq!(attempts, 3);
    }

    #[test]
    fn test_error_metrics() {
        let error = HexagonalError::TimeoutError("operation timed out".to_string());
        let mut distribution = std::collections::HashMap::new();
        distribution.insert("TimeoutError".to_string(), 1);
        
        let mut severity_dist = std::collections::HashMap::new();
        severity_dist.insert(ErrorSeverity::Warning, 1);

        let metrics = ErrorMetrics {
            error_count: 1,
            error_type_distribution: distribution,
            severity_distribution: severity_dist,
            retry_attempts: 2,
            recovery_success_rate: 0.5,
        };

        let context = ErrorContext::new(error).with_metrics(metrics);
        assert!(context.metrics.is_some());
    }
}
