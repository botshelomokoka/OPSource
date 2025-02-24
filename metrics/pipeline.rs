use chrono::Utc;
use prometheus::{Encoder, TextEncoder};

pub struct MetricsPipeline {
    registry: prometheus::Registry,
    ts_db: TimeSeriesDB,
}

impl MetricsPipeline {
    pub fn process(&self) -> Result<()> {
        let metric_fams = self.registry.gather();
        let mut buffer = vec![];
        TextEncoder::new().encode(&metric_fams, &mut buffer)?;
        
        self.ts_db.write(
            "anya_metrics", 
            Utc::now().timestamp_nanos(),
            &buffer
        )?;
        
        Ok(())
    }
    
    pub fn forward_to_ml(&self, model: &mut MLModel) {
        let data = self.ts_db.read_last_hours(24);
        model.update(data);
    }
} 