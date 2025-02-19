//! Anya Enterprise - Advanced Bitcoin-First ML Platform
//! Enterprise-grade extensions for Anya Core

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

// Module declarations
pub mod api;
pub mod auth;
pub mod ml;
pub mod web5;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod ports;
pub mod adapters;

// Re-export commonly used items
pub use api::{EnterpriseEndpoints, EnterpriseIntegration};
pub use auth::EnterpriseSecurity;
pub use ml::EnterpriseMLProcessor;
pub use web5::EnterpriseWeb5;
pub use domain::fl::{FederatedModel, ModelMetrics};
pub use ports::fl::service::FederatedLearningPort;
pub use adapters::fl::openfl_adapter::OpenFLAdapter;

// Feature flags for optional components
#[cfg(feature = "full")]
pub mod full {
    pub use super::api::*;
    pub use super::auth::*;
    pub use super::ml::*;
    pub use super::web5::*;
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

// Testing utilities
#[cfg(test)]
pub mod testing {
    use super::*;
    
    pub struct MockEnterpriseEndpoints;
    pub struct MockEnterpriseSecurity;
    pub struct MockEnterpriseMLProcessor;
    pub struct MockEnterpriseWeb5;
    
    // Implement mock traits for testing
}
