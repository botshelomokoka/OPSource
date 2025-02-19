use prometheus::{register_gauge_vec, register_histogram_vec, GaugeVec, HistogramVec};
use std::error::Error;

pub struct NodeMetrics {
    node_health: GaugeVec,
    peer_count: GaugeVec,
    message_size: HistogramVec,
    computation_time: HistogramVec,
}

impl NodeMetrics {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let node_health = register_gauge_vec!(
            "node_health_status",
            "Current health status of the node",
            &["node_id", "status"]
        )?;

        let peer_count = register_gauge_vec!(
            "node_peer_count",
            "Number of connected peers",
            &["node_id"]
        )?;

        let message_size = register_histogram_vec!(
            "node_message_size_bytes",
            "Size of messages exchanged between nodes",
            &["node_id", "message_type"]
        )?;

        let computation_time = register_histogram_vec!(
            "node_computation_time_seconds",
            "Time spent on local computations",
            &["node_id", "operation"]
        )?;

        Ok(Self {
            node_health,
            peer_count,
            message_size,
            computation_time,
        })
    }

    pub fn set_node_health(&self, node_id: &str, status: &str, value: f64) {
        self.node_health
            .with_label_values(&[node_id, status])
            .set(value);
    }

    pub fn set_peer_count(&self, node_id: &str, count: f64) {
        self.peer_count
            .with_label_values(&[node_id])
            .set(count);
    }

    pub fn observe_message_size(&self, node_id: &str, message_type: &str, size: f64) {
        self.message_size
            .with_label_values(&[node_id, message_type])
            .observe(size);
    }

    pub fn observe_computation_time(&self, node_id: &str, operation: &str, duration: f64) {
        self.computation_time
            .with_label_values(&[node_id, operation])
            .observe(duration);
    }
}
