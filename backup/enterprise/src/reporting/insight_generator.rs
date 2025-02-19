pub struct InsightGenerator {
    ml_analyzer: Arc<MLAnalyzer>,
    trend_analyzer: Arc<TrendAnalyzer>,
    anomaly_detector: Arc<AnomalyDetector>,
}

impl InsightGenerator {
    pub async fn generate_insights(
        &self,
        data: &FormattedData,
        analysis: &PerformanceAnalysis,
        ml_insights: &MLInsights,
    ) -> Result<ReportInsights, InsightError> {
        // 1. Analyze Trends
        let trends = self.trend_analyzer
            .analyze_trends(data, analysis)
            .await?;
        
        // 2. Detect Anomalies
        let anomalies = self.anomaly_detector
            .detect_anomalies(data, &trends)
            .await?;
        
        // 3. Generate ML-based Insights
        let ml_recommendations = self.ml_analyzer
            .generate_recommendations(
                data,
                &trends,
                &anomalies,
                ml_insights,
            )
            .await?;

        Ok(ReportInsights {
            trends,
            anomalies,
            recommendations: ml_recommendations,
            timestamp: chrono::Utc::now(),
        })
    }
} 