use super::model::{FederatedModel, ModelMetrics};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let metrics = ModelMetrics {
            accuracy: 0.95,
            loss: 0.05,
            convergence_rate: 0.8,
            training_rounds: 10,
        };

        let model = FederatedModel {
            id: "test-model".to_string(),
            version: 1,
            metrics,
        };

        assert_eq!(model.id, "test-model");
        assert_eq!(model.version, 1);
        assert_eq!(model.metrics.accuracy, 0.95);
        assert_eq!(model.metrics.loss, 0.05);
        assert_eq!(model.metrics.convergence_rate, 0.8);
        assert_eq!(model.metrics.training_rounds, 10);
    }
}
