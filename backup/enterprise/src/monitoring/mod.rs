pub struct EnterpriseMetrics {
    transaction_counter: Counter,
    transaction_volume: Histogram,
    security_events: Counter,
    wallet_operations: Histogram,
}

impl EnterpriseMetrics {
    pub fn record_transaction(&self, tx: &Transaction) {
        self.transaction_counter.inc();
        self.transaction_volume.observe(tx.output.iter().map(|o| o.value).sum());
    }
} 