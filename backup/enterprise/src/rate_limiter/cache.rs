use lru::LruCache;
use std::num::NonZeroUsize;
use tokio::sync::Mutex;

pub struct InstitutionalCache {
    factors: Mutex<LruCache<String, f64>>,
    metrics: Mutex<LruCache<String, MetricsBatch>>,
}

impl InstitutionalCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            factors: Mutex::new(LruCache::new(NonZeroUsize::new(capacity).unwrap())),
            metrics: Mutex::new(LruCache::new(NonZeroUsize::new(capacity).unwrap())),
        }
    }

    pub async fn cache_institutional_factor(&self, key: String, factor: f64) {
        let mut cache = self.factors.lock().await;
        cache.put(key, factor);
    }

    pub async fn batch_metrics(&self, metrics: Vec<MetricEvent>) -> Result<(), MetricsError> {
        let mut batch = MetricsBatch::new();
        for metric in metrics {
            batch.add(metric);
        }
        
        if batch.is_ready_to_flush() {
            self.flush_metrics_batch(&batch).await?;
        }
        Ok(())
    }
} 