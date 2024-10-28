use metrics::{Counter, Gauge, Histogram, Key, KeyName, Unit};
use opentelemetry::trace::{Tracer, TracerProvider};

pub struct Monitoring {
    tracer: Box<dyn Tracer>,
    metrics: MetricsRegistry,
}

impl Monitoring {
    pub fn new() -> Self {
        let provider = opentelemetry_jaeger::new_pipeline()
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Failed to initialize tracer");
            
        Self {
            tracer: provider.tracer("anya"),
            metrics: MetricsRegistry::new(),
        }
    }

    pub fn record_ml_prediction(&self, duration_ms: f64) {
        self.metrics.ml_prediction_duration
            .record(duration_ms, &[]);
    }

    pub fn record_key_generation(&self) {
        self.metrics.key_generation_total
            .increment(1);
    }
}

#[derive(Clone)]
pub struct MetricsRegistry {
    // ML metrics
    pub ml_prediction_duration: Histogram,
    pub ml_training_duration: Histogram,
    pub model_validation_score: Gauge,
    pub feature_extraction_errors: Counter,
    
    // Crypto metrics
    pub key_generation_total: Counter,
    pub signing_operations: Counter,
    pub verification_operations: Counter,
    pub failed_auth_attempts: Counter,
    
    // Database metrics
    pub db_query_duration: Histogram,
    pub db_connection_errors: Counter,
    pub backup_duration: Histogram,
    pub backup_size_bytes: Gauge,
    
    // System metrics
    pub memory_usage_bytes: Gauge,
    pub cpu_usage_percent: Gauge,
    pub open_file_descriptors: Gauge,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self {
            ml_prediction_duration: register_histogram!("ml_prediction_duration_seconds"),
            ml_training_duration: register_histogram!("ml_training_duration_seconds"),
            model_validation_score: register_gauge!("model_validation_score"),
            feature_extraction_errors: register_counter!("feature_extraction_errors_total"),
            
            key_generation_total: register_counter!("key_generation_total"),
            signing_operations: register_counter!("signing_operations_total"),
            verification_operations: register_counter!("verification_operations_total"),
            failed_auth_attempts: register_counter!("failed_auth_attempts_total"),
            
            db_query_duration: register_histogram!("db_query_duration_seconds"),
            db_connection_errors: register_counter!("db_connection_errors_total"),
            backup_duration: register_histogram!("backup_duration_seconds"),
            backup_size_bytes: register_gauge!("backup_size_bytes"),
            
            memory_usage_bytes: register_gauge!("memory_usage_bytes"),
            cpu_usage_percent: register_gauge!("cpu_usage_percent"),
            open_file_descriptors: register_gauge!("open_file_descriptors"),
        }
    }
}
