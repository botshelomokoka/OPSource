use super::service::FederatedLearningService;
use crate::domain::fl::{FederatedModel, ModelMetrics};
use crate::ports::fl::service::FederatedLearningPort;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

struct MockFederatedLearningPort {
    accuracy: f32,
    loss: f32,
}

#[async_trait]
impl FederatedLearningPort for MockFederatedLearningPort {
    async fn initialize_training(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn train_model(&self, model: &mut FederatedModel) -> Result<ModelMetrics, Box<dyn Error>> {
        Ok(ModelMetrics {
            accuracy: self.accuracy,
            loss: self.loss,
            convergence_rate: 0.8,
            training_rounds: 1,
        })
    }

    async fn aggregate_models(&self, models: Vec<FederatedModel>) -> Result<FederatedModel, Box<dyn Error>> {
        Ok(models[0].clone())
    }

    async fn evaluate_model(&self, _model: &FederatedModel) -> Result<ModelMetrics, Box<dyn Error>> {
        Ok(ModelMetrics {
            accuracy: self.accuracy,
            loss: self.loss,
            convergence_rate: 0.8,
            training_rounds: 1,
        })
    }

    async fn get_training_metrics(&self) -> Result<Vec<ModelMetrics>, Box<dyn Error>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_successful_training() {
        let port = Arc::new(MockFederatedLearningPort {
            accuracy: 0.95,
            loss: 0.05,
        });

        let service = FederatedLearningService::new(port).unwrap();

        let model = FederatedModel {
            id: "test-model".to_string(),
            version: 1,
            metrics: ModelMetrics {
                accuracy: 0.0,
                loss: 1.0,
                convergence_rate: 0.0,
                training_rounds: 0,
            },
        };

        let result = service.train_model(model).await.unwrap();
        assert_eq!(result.id, "test-model");
        assert_eq!(result.version, 1);
    }

    #[tokio::test]
    async fn test_model_evaluation() {
        let port = Arc::new(MockFederatedLearningPort {
            accuracy: 0.95,
            loss: 0.05,
        });

        let service = FederatedLearningService::new(port).unwrap();

        let model = FederatedModel {
            id: "test-model".to_string(),
            version: 1,
            metrics: ModelMetrics {
                accuracy: 0.95,
                loss: 0.05,
                convergence_rate: 0.8,
                training_rounds: 1,
            },
        };

        let metrics = service.evaluate_model(&model).await.unwrap();
        assert_eq!(metrics.accuracy, 0.95);
        assert_eq!(metrics.loss, 0.05);
    }
}
