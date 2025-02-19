pub struct ReportGenerator {
    template_engine: Arc<TemplateEngine>,
    data_formatter: Arc<DataFormatter>,
    insight_generator: Arc<InsightGenerator>,
}

impl ReportGenerator {
    pub async fn create_report(
        &self,
        metrics: SystemMetrics,
        analysis: PerformanceAnalysis,
        insights: MLInsights,
        config: &ReportConfig,
    ) -> Result<SystemReport, ReportError> {
        // 1. Format Data
        let formatted_data = self.data_formatter
            .format_metrics(&metrics, &config.format)
            .await?;
        
        // 2. Generate Insights
        let report_insights = self.insight_generator
            .generate_insights(
                &formatted_data,
                &analysis,
                &insights,
            )
            .await?;
        
        // 3. Apply Template
        let report_content = self.template_engine
            .apply_template(
                &config.template,
                &formatted_data,
                &report_insights,
            )
            .await?;

        Ok(SystemReport {
            content: report_content,
            metrics: formatted_data,
            insights: report_insights,
            generated_at: chrono::Utc::now(),
            config: config.clone(),
        })
    }
} 