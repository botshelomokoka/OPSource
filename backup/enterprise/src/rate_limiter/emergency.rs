use super::{RateLimiter, RateLimitError};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

pub struct EmergencyControl {
    circuit_breaker: AtomicBool,
    emergency_bypass: RwLock<EmergencyBypass>,
    audit_logger: Arc<AuditLogger>,
}

impl EmergencyControl {
    pub fn new(audit_logger: Arc<AuditLogger>) -> Self {
        Self {
            circuit_breaker: AtomicBool::new(false),
            emergency_bypass: RwLock::new(EmergencyBypass::default()),
            audit_logger,
        }
    }

    pub async fn trigger_circuit_breaker(&self, reason: &str) {
        self.circuit_breaker.store(true, Ordering::SeqCst);
        self.audit_logger.log_emergency_event("CIRCUIT_BREAKER_TRIGGERED", reason).await;
    }

    pub async fn enable_emergency_bypass(&self, context: EmergencyContext) -> Result<(), RateLimitError> {
        let mut bypass = self.emergency_bypass.write().await;
        bypass.enable(context.clone())?;
        self.audit_logger.log_emergency_event("EMERGENCY_BYPASS_ENABLED", &context.reason).await;
        Ok(())
    }
} 