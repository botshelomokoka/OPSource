<<<<<<< HEAD
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use metrics::{Counter, Gauge, Histogram};

pub struct HSMManager {
    device_manager: Arc<HSMDeviceManager>,
    key_store: Arc<HSMKeyStore>,
    metrics: HSMMetrics,
    audit_logger: Arc<AuditLogger>,
    compliance_monitor: Arc<ComplianceMonitor>,
}

impl HSMManager {
    pub async fn new(config: HSMConfig) -> Result<Self, HSMError> {
        let metrics = HSMMetrics::new();
        let audit_logger = Arc::new(AuditLogger::new(config.audit_config)?);
        let compliance_monitor = Arc::new(ComplianceMonitor::new(config.compliance_config)?);
        
        Ok(Self {
            device_manager: Arc::new(HSMDeviceManager::new(config.device_config)?),
            key_store: Arc::new(HSMKeyStore::new(config.key_config)?),
            metrics,
            audit_logger,
            compliance_monitor,
        })
    }

    pub async fn sign_transaction(
        &self,
        tx: &Transaction,
        context: &HSMContext,
    ) -> Result<Signature, HSMError> {
        let start = Instant::now();
        
        // Compliance check
        self.compliance_monitor.check_operation(context).await?;
        
        // Get key from HSM
        let key = self.key_store.get_signing_key(context).await?;
        
        // Sign transaction
        let signature = self.device_manager
            .sign_with_key(&key, tx)
            .await?;
            
        // Audit logging
        self.audit_logger.log_signing_operation(tx, context).await?;
        
        // Update metrics
        self.metrics.signing_latency.record(start.elapsed());
        self.metrics.operations_total.increment(1);
        
        Ok(signature)
    }
} 
=======
 
>>>>>>> 0d30677a5063b7cf6fca105032e2c81bb0fb8ea7
