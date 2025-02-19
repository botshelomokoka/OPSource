use std::error::Error;

#[derive(Debug, Clone)]
pub struct FederatedModel {
    pub id: String,
    pub version: u32,
    pub metrics: ModelMetrics,
}

#[derive(Debug, Clone)]
pub struct ModelMetrics {
    pub accuracy: f32,
    pub loss: f32,
    pub convergence_rate: f32,
    pub training_rounds: u32,
}

pub trait FederatedLearningService {
    fn train(&self, model: &mut FederatedModel, data: &[u8]) -> Result<(), Box<dyn Error>>;
    fn evaluate(&self, model: &FederatedModel, data: &[u8]) -> Result<ModelMetrics, Box<dyn Error>>;
    fn aggregate(&self, models: Vec<FederatedModel>) -> Result<FederatedModel, Box<dyn Error>>;
}
