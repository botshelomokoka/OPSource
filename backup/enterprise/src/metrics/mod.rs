use std::sync::Arc;
use metrics::{Counter, Gauge, Histogram};
use tokio::sync::RwLock;

pub struct EnterpriseMetrics {
    transaction_counter: Counter,
    active_connections: Gauge,
    transaction_latency: Histogram,
    memory_usage: Gauge,
    cpu_usage: Gauge,
    error_counter: Counter,
    state: RwLock<MetricsState>,
}

impl EnterpriseMetrics {
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            transaction_counter: Counter::new("transactions_total"),
            active_connections: Gauge::new("active_connections"),
            transaction_latency: Histogram::new("transaction_latency_ms"),
            memory_usage: Gauge::new("memory_usage_bytes"),
            cpu_usage: Gauge::new("cpu_usage_percent"),
            error_counter: Counter::new("errors_total"),
            state: RwLock::new(MetricsState::new()),
        }
    }

    pub async fn record_transaction(&self, result: &TransactionResult) -> Result<(), MetricsError> {
        self.transaction_counter.increment(1);
        self.transaction_latency.record(result.latency);
        
        let mut state = self.state.write().await;
        state.update_transaction_metrics(result)?;
        
        Ok(())
    }

    pub async fn collect_metrics(&self) -> Result<SystemMetrics, MetricsError> {
        let state = self.state.read().await;
        Ok(SystemMetrics {
            transactions: self.transaction_counter.get(),
            connections: self.active_connections.get(),
            latency: self.transaction_latency.get_statistics(),
            memory: self.memory_usage.get(),
            cpu: self.cpu_usage.get(),
            errors: self.error_counter.get(),
            state: state.clone(),
        })
    }
} 