// Enterprise Module Structure
pub mod api;
pub mod auth;
pub mod ml;
pub mod web5;

// Re-exports
pub use api::{EnterpriseEndpoints, EnterpriseIntegration};
pub use auth::EnterpriseSecurity;
pub use ml::EnterpriseMLProcessor;
pub use web5::EnterpriseWeb5;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnterpriseError {
    #[error("API Error: {0}")]
    ApiError(String),

    #[error("Authentication Error: {0}")]
    AuthError(String),

    #[error("ML Processing Error: {0}")]
    MLError(String),

    #[error("Web5 Error: {0}")]
    Web5Error(String),
}

// Metrics and monitoring
#[cfg(feature = "metrics")]
pub mod metrics {
    use metrics::{counter, gauge, histogram};
    
    pub fn register_enterprise_metrics() {
        counter!("enterprise_api_requests_total");
        counter!("enterprise_auth_attempts_total");
        counter!("enterprise_ml_processes_total");
        counter!("enterprise_web5_operations_total");
        
        gauge!("enterprise_active_connections");
        gauge!("enterprise_ml_model_version");
        
        histogram!("enterprise_request_duration_seconds");
        histogram!("enterprise_ml_processing_duration_seconds");
    }
} 