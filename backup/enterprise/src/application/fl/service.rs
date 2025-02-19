use crate::domain::fl::{FederatedModel, ModelMetrics};
use crate::infrastructure::metrics::fl_metrics::FederatedLearningMetrics;
use crate::ports::fl::service::FederatedLearningPort;
use std::error::Error;
use std::sync::Arc;
use tokio::time::Instant;
use tracing::{info, warn};

pub struct FederatedLearningService {
    port: Arc<dyn FederatedLearningPort + Send + Sync>,
    metrics: FederatedLearningMetrics,
}

impl FederatedLearningService {
    pub fn new(port: Arc<dyn FederatedLearningPort + Send + Sync>) -> Result<Self, Box<dyn Error>> {
        let metrics = FederatedLearningMetrics::new()?;
        Ok(Self { port, metrics })
    }

    pub async fn train_model(&self, mut model: FederatedModel) -> Result<FederatedModel, Box<dyn Error>> {
        let start = Instant::now();
        info!("Starting federated training for model {}", model.id);

        // Initialize training
        self.port.initialize_training().await?;

        // Train the model
        let metrics = self.port.train_model(&mut model).await?;
        
        // Record metrics
        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_training_duration(&model.id, model.version, duration);
        self.metrics.record_convergence_rate(&model.id, metrics.convergence_rate);
        
        info!(
            "Completed federated training for model {}. Duration: {:.2}s, Accuracy: {:.4}",
            model.id, duration, metrics.accuracy
        );

        Ok(model)
    }

    pub async fn evaluate_model(&self, model: &FederatedModel) -> Result<ModelMetrics, Box<dyn Error>> {
        info!("Evaluating model {}", model.id);
        let metrics = self.port.evaluate_model(model).await?;
        
        if metrics.accuracy < 0.8 {
            warn!(
                "Model {} accuracy ({:.4}) is below threshold",
                model.id, metrics.accuracy
            );
        }

        Ok(metrics)
    }

    pub async fn get_training_metrics(&self) -> Result<Vec<ModelMetrics>, Box<dyn Error>> {
        self.port.get_training_metrics().await
    }
}
