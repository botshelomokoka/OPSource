use super::RateLimitContext;
use chrono::{DateTime, Utc};

pub struct ComplianceChecker {
    status_checker: Arc<StatusChecker>,
    report_generator: Arc<ReportGenerator>,
    tracker: Arc<TransactionTracker>,
}

impl ComplianceChecker {
    pub async fn check_compliance(
        &self,
        context: &RateLimitContext,
    ) -> Result<ComplianceStatus, ComplianceError> {
        // Check current compliance status
        let status = self.status_checker.check(context).await?;
        
        // Generate regulatory report if needed
        if status.requires_reporting() {
            self.report_generator.generate_report(context).await?;
        }
        
        // Track transaction for compliance
        self.tracker.track_transaction(context).await?;
        
        Ok(status)
    }

    pub async fn generate_regulatory_report(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<RegulatoryReport, ComplianceError> {
        self.report_generator
            .generate_period_report(start_time, end_time)
            .await
    }
} 