use plotly::Plot;
use resvg::usvg::Tree;

pub struct VisualizationEngine {
    plotly_generator: Arc<PlotlyGenerator>,
    chart_builder: Arc<ChartBuilder>,
    image_renderer: Arc<ImageRenderer>,
}

impl VisualizationEngine {
    pub async fn create_visualizations(
        &self,
        data: &ProcessedData,
        statistics: &Statistics,
        options: &VisualizationOptions,
    ) -> Result<Visualizations, VisualizationError> {
        // 1. Generate Performance Charts
        let performance_charts = self.generate_performance_charts(
            data,
            options,
        ).await?;
        
        // 2. Create ML Insights Visualizations
        let ml_visualizations = self.create_ml_visualizations(
            data,
            statistics,
            options,
        ).await?;
        
        // 3. Generate System Health Dashboard
        let health_dashboard = self.generate_health_dashboard(
            data,
            statistics,
            options,
        ).await?;
        
        // 4. Create Interactive Plots
        let interactive_plots = self.create_interactive_plots(
            data,
            options,
        ).await?;

        Ok(Visualizations {
            performance_charts,
            ml_visualizations,
            health_dashboard,
            interactive_plots,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn create_interactive_plots(
        &self,
        data: &ProcessedData,
        options: &VisualizationOptions,
    ) -> Result<Vec<Plot>, VisualizationError> {
        let mut plots = Vec::new();

        // 1. Performance Trends Plot
        plots.push(self.plotly_generator
            .create_performance_trend_plot(data)?);

        // 2. Resource Usage Plot
        plots.push(self.plotly_generator
            .create_resource_usage_plot(data)?);

        // 3. ML Metrics Plot
        plots.push(self.plotly_generator
            .create_ml_metrics_plot(data)?);

        Ok(plots)
    }
} 