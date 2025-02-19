use super::fl_metrics::FederatedLearningMetrics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = FederatedLearningMetrics::new();
        assert!(metrics.is_ok());
    }

    #[test]
    fn test_record_metrics() {
        let metrics = FederatedLearningMetrics::new().unwrap();
        
        // Test recording training duration
        metrics.record_training_duration("test-model", 1, 10.5);
        
        // Test recording model updates
        metrics.increment_model_updates("test-model", "node-1");
        
        // Test recording convergence rate
        metrics.record_convergence_rate("test-model", 0.95);
        
        // Test recording network latency
        metrics.record_network_latency("node-1", "node-2", 0.05);
    }
}
