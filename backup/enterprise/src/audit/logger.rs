use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub context: AuditContext,
    pub metadata: serde_json::Value,
    pub severity: AuditSeverity,
}

pub struct AuditLogger {
    storage: Arc<dyn AuditStorage>,
    metrics: AuditMetrics,
    config: AuditConfig,
}

impl AuditLogger {
    pub async fn log_event(
        &self,
        event_type: AuditEventType,
        context: AuditContext,
        metadata: serde_json::Value,
    ) -> Result<(), AuditError> {
        let event = AuditEvent {
            timestamp: Utc::now(),
            event_type,
            context,
            metadata,
            severity: event_type.severity(),
        };

        // Store event
        self.storage.store_event(&event).await?;

        // Update metrics
        self.metrics.events_logged.increment(1);
        self.metrics.events_by_type
            .get_or_create(&event_type.to_string())
            .increment(1);

        // Handle high severity events
        if event.severity >= AuditSeverity::High {
            self.handle_high_severity_event(&event).await?;
        }

        Ok(())
    }

    async fn handle_high_severity_event(&self, event: &AuditEvent) -> Result<(), AuditError> {
        // Alert relevant systems
        self.alert_system.send_alert(event).await?;

        // Additional logging for high severity events
        self.metrics.high_severity_events.increment(1);

        Ok(())
    }
} 