use prometheus::{register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec};
use std::error::Error;

pub struct FederatedLearningMetrics {
    training_duration: HistogramVec,
    model_updates: IntCounterVec,
    convergence_rate: HistogramVec,
    network_latency: HistogramVec,
}

impl FederatedLearningMetrics {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let training_duration = register_histogram_vec!(
            "fl_training_duration_seconds",
            "Duration of federated learning training rounds",
            &["model", "round"]
        )?;

        let model_updates = register_int_counter_vec!(
            "fl_model_updates_total",
            "Total number of model updates",
            &["model", "node"]
        )?;

        let convergence_rate = register_histogram_vec!(
            "fl_convergence_rate",
            "Model convergence rate over training rounds",
            &["model"]
        )?;

        let network_latency = register_histogram_vec!(
            "fl_network_latency_seconds",
            "Network latency between nodes",
            &["source", "destination"]
        )?;

        Ok(Self {
            training_duration,
            model_updates,
            convergence_rate,
            network_latency,
        })
    }

    pub fn record_training_duration(&self, model: &str, round: u32, duration: f64) {
        self.training_duration
            .with_label_values(&[model, &round.to_string()])
            .observe(duration);
    }

    pub fn increment_model_updates(&self, model: &str, node: &str) {
        self.model_updates
            .with_label_values(&[model, node])
            .inc();
    }

    pub fn record_convergence_rate(&self, model: &str, rate: f64) {
        self.convergence_rate
            .with_label_values(&[model])
            .observe(rate);
    }

    pub fn record_network_latency(&self, source: &str, destination: &str, latency: f64) {
        self.network_latency
            .with_label_values(&[source, destination])
            .observe(latency);
    }
}
