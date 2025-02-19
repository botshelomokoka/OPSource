use crate::domain::fl::model::{FederatedModel, ModelMetrics};
use async_trait::async_trait;
use std::error::Error;
use std::collections::HashMap;

#[async_trait]
pub trait FederatedLearningPort: Send + Sync {
    /// Initialize the training environment and resources
    async fn initialize_training(&self) -> Result<(), Box<dyn Error>>;
    
    /// Train a model with local data
    async fn train_model(&self, model: &mut FederatedModel) -> Result<ModelMetrics, Box<dyn Error>>;
    
    /// Aggregate multiple model updates into a single model
    async fn aggregate_models(&self, models: Vec<FederatedModel>) -> Result<FederatedModel, Box<dyn Error>>;
    
    /// Evaluate model performance
    async fn evaluate_model(&self, model: &FederatedModel) -> Result<ModelMetrics, Box<dyn Error>>;
    
    /// Get training metrics history
    async fn get_training_metrics(&self) -> Result<Vec<ModelMetrics>, Box<dyn Error>>;
    
    /// Validate model against ethical principles and bias metrics
    async fn validate_model(&self, model: &FederatedModel) -> Result<HashMap<String, f64>, Box<dyn Error>>;
    
    /// Monitor resource usage during training
    async fn monitor_resources(&self) -> Result<HashMap<String, f64>, Box<dyn Error>>;
    
    /// Check model convergence
    async fn check_convergence(&self, metrics_history: &[ModelMetrics]) -> Result<bool, Box<dyn Error>>;
    
    /// Verify data quality and privacy requirements
    async fn verify_data_requirements(&self, model: &FederatedModel) -> Result<bool, Box<dyn Error>>;
}
