use metrics::{counter, gauge, histogram, Counter, Gauge, Histogram};
use once_cell::sync::Lazy;
use std::sync::Arc;

/// Core system metrics
pub struct CoreMetrics {
    // Bitcoin metrics
    pub bitcoin_transactions: Counter,
    pub bitcoin_block_height: Gauge,
    pub bitcoin_mempool_size: Gauge,
    pub bitcoin_peer_count: Gauge,
    
    // Lightning metrics
    pub lightning_channels: Counter,
    pub lightning_capacity: Gauge,
    pub lightning_forwards: Counter,
    pub lightning_fees_earned: Counter,
    
    // DLC metrics
    pub dlc_contracts: Counter,
    pub dlc_active_contracts: Gauge,
    pub dlc_contract_value: Gauge,
    
    // Network metrics
    pub network_peers: Gauge,
    pub network_bandwidth: Gauge,
    pub network_latency: Histogram,
    pub network_errors: Counter,
}

/// ML system metrics
pub struct MLMetrics {
    pub training_iterations: Counter,
    pub model_accuracy: Gauge,
    pub prediction_latency: Histogram,
    pub model_size: Gauge,
    pub training_errors: Counter,
    pub validation_score: Gauge,
    pub inference_requests: Counter,
}

/// Security metrics
pub struct SecurityMetrics {
    pub auth_attempts: Counter,
    pub auth_failures: Counter,
    pub encryption_operations: Counter,
    pub security_violations: Counter,
    pub quantum_resistant_ops: Counter,
}

/// Storage metrics
pub struct StorageMetrics {
    pub storage_operations: Counter,
    pub storage_size: Gauge,
    pub operation_latency: Histogram,
    pub error_count: Counter,
}

/// Privacy metrics
pub struct PrivacyMetrics {
    pub zk_proofs_generated: Counter,
    pub mpc_operations: Counter,
    pub privacy_score: Gauge,
    pub encryption_time: Histogram,
}

impl CoreMetrics {
    pub fn new() -> Self {
        Self {
            // Bitcoin metrics
            bitcoin_transactions: counter!("bitcoin_transactions_total"),
            bitcoin_block_height: gauge!("bitcoin_block_height"),
            bitcoin_mempool_size: gauge!("bitcoin_mempool_size"),
            bitcoin_peer_count: gauge!("bitcoin_peer_count"),
            
            // Lightning metrics
            lightning_channels: counter!("lightning_channels_total"),
            lightning_capacity: gauge!("lightning_capacity_sats"),
            lightning_forwards: counter!("lightning_forwards_total"),
            lightning_fees_earned: counter!("lightning_fees_earned_sats"),
            
            // DLC metrics
            dlc_contracts: counter!("dlc_contracts_total"),
            dlc_active_contracts: gauge!("dlc_active_contracts"),
            dlc_contract_value: gauge!("dlc_contract_value_sats"),
            
            // Network metrics
            network_peers: gauge!("network_peers"),
            network_bandwidth: gauge!("network_bandwidth_bytes"),
            network_latency: histogram!("network_latency_ms"),
            network_errors: counter!("network_errors_total"),
        }
    }
}

impl MLMetrics {
    pub fn new() -> Self {
        Self {
            training_iterations: counter!("ml_training_iterations_total"),
            model_accuracy: gauge!("ml_model_accuracy"),
            prediction_latency: histogram!("ml_prediction_latency_ms"),
            model_size: gauge!("ml_model_size_bytes"),
            training_errors: counter!("ml_training_errors_total"),
            validation_score: gauge!("ml_validation_score"),
            inference_requests: counter!("ml_inference_requests_total"),
        }
    }
}

impl SecurityMetrics {
    pub fn new() -> Self {
        Self {
            auth_attempts: counter!("security_auth_attempts_total"),
            auth_failures: counter!("security_auth_failures_total"),
            encryption_operations: counter!("security_encryption_ops_total"),
            security_violations: counter!("security_violations_total"),
            quantum_resistant_ops: counter!("security_quantum_resistant_ops_total"),
        }
    }
}

impl StorageMetrics {
    pub fn new() -> Self {
        Self {
            storage_operations: counter!("storage_operations_total"),
            storage_size: gauge!("storage_size_bytes"),
            operation_latency: histogram!("storage_operation_latency_ms"),
            error_count: counter!("storage_errors_total"),
        }
    }
}

impl PrivacyMetrics {
    pub fn new() -> Self {
        Self {
            zk_proofs_generated: counter!("privacy_zk_proofs_total"),
            mpc_operations: counter!("privacy_mpc_operations_total"),
            privacy_score: gauge!("privacy_score"),
            encryption_time: histogram!("privacy_encryption_time_ms"),
        }
    }
}

// Global metrics instance
pub static METRICS: Lazy<Arc<Metrics>> = Lazy::new(|| {
    Arc::new(Metrics {
        core: CoreMetrics::new(),
        ml: MLMetrics::new(),
        security: SecurityMetrics::new(),
        storage: StorageMetrics::new(),
        privacy: PrivacyMetrics::new(),
    })
});

pub struct Metrics {
    pub core: CoreMetrics,
    pub ml: MLMetrics,
    pub security: SecurityMetrics,
    pub storage: StorageMetrics,
    pub privacy: PrivacyMetrics,
} 