use crate::reporting::ReportData;
use crate::visualization::ChartConfig;

pub struct DashboardComponents {
    chart_renderer: Arc<ChartRenderer>,
    metrics_display: Arc<MetricsDisplay>,
    alert_panel: Arc<AlertPanel>,
    status_board: Arc<StatusBoard>,
}

impl DashboardComponents {
    pub async fn render_components(
        &self,
        data: &ReportData,
        config: &ComponentConfig,
    ) -> Result<RenderedComponents, RenderError> {
        // 1. Render Charts
        let charts = self.render_charts(
            &data.chart_data,
            &config.chart_config,
        ).await?;
        
        // 2. Display Metrics
        let metrics = self.display_metrics(
            &data.metrics,
            &config.metrics_config,
        ).await?;
        
        // 3. Show Alerts
        let alerts = self.show_alerts(
            &data.alerts,
            &config.alert_config,
        ).await?;
        
        // 4. Update Status
        let status = self.update_status(
            &data.status,
            &config.status_config,
        ).await?;

        Ok(RenderedComponents {
            charts,
            metrics,
            alerts,
            status,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn render_charts(
        &self,
        data: &ChartData,
        config: &ChartConfig,
    ) -> Result<Vec<RenderedChart>, RenderError> {
        let mut rendered_charts = Vec::new();

        for (chart_type, chart_data) in data.iter() {
            let chart = match chart_type {
                ChartType::TimeSeries => {
                    self.chart_renderer.render_time_series(chart_data, config)?
                },
                ChartType::Distribution => {
                    self.chart_renderer.render_distribution(chart_data, config)?
                },
                ChartType::Comparison => {
                    self.chart_renderer.render_comparison(chart_data, config)?
                },
            };
            rendered_charts.push(chart);
        }

        Ok(rendered_charts)
    }
} 