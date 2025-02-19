pub struct HomepageComponents {
    kpi_widget: KPIWidget,
    security_widget: SecurityWidget,
    ml_widget: MLStatusWidget,
    health_widget: HealthWidget,
    reporting_widget: ReportingWidget,
}

impl HomepageComponents {
    pub async fn update_all(
        &mut self,
        metrics: &SystemMetrics,
    ) -> Result<(), ComponentError> {
        // 1. Update KPI Overview
        self.kpi_widget.update(KPIData {
            system_performance: metrics.performance_score,
            security_status: metrics.security_score,
            ml_accuracy: metrics.ml_accuracy,
            system_health: metrics.health_score,
        }).await?;

        // 2. Update Security Overview
        self.security_widget.update(SecurityData {
            threat_level: metrics.threat_level,
            security_incidents: metrics.security_incidents,
            risk_score: metrics.risk_score,
            compliance_status: metrics.compliance_status,
        }).await?;

        // 3. Update ML Status
        self.ml_widget.update(MLData {
            model_performance: metrics.model_performance,
            prediction_accuracy: metrics.prediction_accuracy,
            training_status: metrics.training_status,
            model_health: metrics.model_health,
        }).await?;

        // 4. Update System Health
        self.health_widget.update(HealthData {
            component_status: metrics.component_status,
            resource_usage: metrics.resource_usage,
            error_rates: metrics.error_rates,
            latency_metrics: metrics.latency_metrics,
        }).await?;

        Ok(())
    }
} 