use crate::monitoring::EnhancedMonitoring;
use crate::ml::MLAnalytics;
use crate::performance::PerformanceMetrics;

pub struct SystemReporter {
    monitoring: Arc<EnhancedMonitoring>,
    ml_analytics: Arc<MLAnalytics>,
    performance_metrics: Arc<PerformanceMetrics>,
    report_generator: Arc<ReportGenerator>,
}

impl SystemReporter {
    pub async fn generate_system_report(
        &self,
        config: ReportConfig,
    ) -> Result<SystemReport, ReportError> {
        // 1. Collect System Data
        let system_metrics = self.collect_system_data(&config).await?;
        
        // 2. Analyze Performance
        let performance_analysis = self.analyze_performance_data(
            &system_metrics,
            &config,
        ).await?;
        
        // 3. Generate ML Insights
        let ml_insights = self.generate_ml_insights(
            &system_metrics,
            &performance_analysis,
        ).await?;
        
        // 4. Create Report
        let report = self.report_generator
            .create_report(
                system_metrics,
                performance_analysis,
                ml_insights,
                &config,
            )
            .await?;

        Ok(report)
    }

    async fn collect_system_data(
        &self,
        config: &ReportConfig,
    ) -> Result<SystemMetrics, ReportError> {
        Ok(SystemMetrics {
            performance: self.performance_metrics.collect().await?,
            monitoring: self.monitoring.collect_metrics().await?,
            ml_metrics: self.ml_analytics.collect_metrics().await?,
            timestamp: chrono::Utc::now(),
        })
    }
} 