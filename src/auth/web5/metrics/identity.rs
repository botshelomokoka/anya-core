use metrics::{Counter, Gauge, Histogram};

#[derive(Clone)]
pub struct IdentityMetrics {
    pub did_resolutions: Counter,
    pub credential_verifications: Counter,
    pub verification_duration: Histogram,
    pub active_credentials: Gauge,
    pub failed_verifications: Counter,
}

impl IdentityMetrics {
    pub fn new() -> Self {
        Self {
            did_resolutions: register_counter!("web5_did_resolutions_total"),
            credential_verifications: register_counter!("web5_credential_verifications_total"),
            verification_duration: register_histogram!("web5_verification_duration_seconds"),
            active_credentials: register_gauge!("web5_active_credentials"),
            failed_verifications: register_counter!("web5_failed_verifications_total"),
        }
    }
}
