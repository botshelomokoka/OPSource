use super::model::{FederatedModel, ModelMetrics};
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait FederatedLearningService: Send + Sync {
    async fn train_model(&self, model: &mut FederatedModel) -> Result<ModelMetrics, Box<dyn Error>>;
    async fn evaluate_model(&self, model: &FederatedModel) -> Result<ModelMetrics, Box<dyn Error>>;
    async fn aggregate_updates(&self, model: &mut FederatedModel) -> Result<(), Box<dyn Error>>;
    async fn validate_model(&self, model: &FederatedModel) -> Result<bool, Box<dyn Error>>;
}
