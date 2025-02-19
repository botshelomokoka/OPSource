use crate::ui::{DashboardConfig, WidgetConfig};
use crate::visualization::ChartEngine;
use crate::ui::dashboard_agent::DashboardAgent;
use metrics::{counter, histogram};
use log::{info, error};
use tokio::time::Instant;
use std::sync::Arc;

pub struct DashboardManager {
    dashboard_agent: Arc<DashboardAgent>,
    widget_manager: Arc<WidgetManager>,
    layout_engine: Arc<LayoutEngine>,
    chart_engine: Arc<ChartEngine>,
    interaction_handler: Arc<InteractionHandler>,
}

impl DashboardManager {
    pub fn new(
        metrics_collector: Arc<MetricsCollector>,
        ml_processor: Arc<MLProcessor>,
    ) -> Self {
        let dashboard_agent = Arc::new(DashboardAgent::new(
            metrics_collector,
            ml_processor,
            Duration::from_secs(1),
            4,
        ));

        Self {
            dashboard_agent,
            widget_manager: Arc::new(WidgetManager::new()),
            layout_engine: Arc::new(LayoutEngine::new()),
            chart_engine: Arc::new(ChartEngine::new()),
            interaction_handler: Arc::new(InteractionHandler::new()),
        }
    }

    pub async fn create_dashboard(
        &self,
        config: DashboardConfig,
    ) -> Result<Dashboard, DashboardError> {
        let start = Instant::now();
        counter!("dashboard_creation_attempts_total", 1);

        // 1. Initialize Layout
        let layout = self.layout_engine
            .create_layout(&config.layout)
            .await
            .map_err(|e| {
                counter!("dashboard_layout_failures_total", 1);
                DashboardError::LayoutError(e.to_string())
            })?;
        
        // 2. Create Widgets
        let widgets = self.create_dashboard_widgets(&config.widgets).await?;
        
        // 3. Setup Interactivity
        let interactions = self.interaction_handler
            .setup_interactions(&widgets, &config.interactions)
            .await
            .map_err(|e| {
                counter!("dashboard_interaction_setup_failures_total", 1);
                DashboardError::InteractionError(e.to_string())
            })?;
        
        // 4. Initialize Real-time Updates
        let update_channels = self.setup_real_time_updates(
            &widgets,
            &config.update_config,
        ).await?;

        // 5. Start Dashboard Agent
        let agent_handle = self.start_dashboard_agent(&widgets).await?;

        let elapsed = start.elapsed();
        histogram!("dashboard_creation_duration_seconds", elapsed.as_secs_f64());
        counter!("dashboard_creation_success_total", 1);

        Ok(Dashboard {
            layout,
            widgets,
            interactions,
            update_channels,
            agent_handle,
            config: config.clone(),
        })
    }

    async fn create_dashboard_widgets(
        &self,
        configs: &[WidgetConfig],
    ) -> Result<Vec<DashboardWidget>, DashboardError> {
        let start = Instant::now();
        counter!("widget_creation_attempts_total", 1);

        let mut widgets = Vec::new();

        for config in configs {
            let widget = match config.widget_type {
                WidgetType::Chart => {
                    self.create_chart_widget(config).await.map_err(|e| {
                        counter!("chart_widget_creation_failures_total", 1);
                        DashboardError::WidgetError(e.to_string())
                    })?
                },
                WidgetType::Metrics => {
                    self.create_metrics_widget(config).await.map_err(|e| {
                        counter!("metrics_widget_creation_failures_total", 1);
                        DashboardError::WidgetError(e.to_string())
                    })?
                },
                WidgetType::Alert => {
                    self.create_alert_widget(config).await.map_err(|e| {
                        counter!("alert_widget_creation_failures_total", 1);
                        DashboardError::WidgetError(e.to_string())
                    })?
                },
                WidgetType::Status => {
                    self.create_status_widget(config).await.map_err(|e| {
                        counter!("status_widget_creation_failures_total", 1);
                        DashboardError::WidgetError(e.to_string())
                    })?
                },
            };
            widgets.push(widget);
        }

        let elapsed = start.elapsed();
        histogram!("widget_creation_duration_seconds", elapsed.as_secs_f64());
        counter!("widget_creation_success_total", 1);

        Ok(widgets)
    }

    async fn start_dashboard_agent(
        &self,
        widgets: &[DashboardWidget],
    ) -> Result<AgentHandle, DashboardError> {
        let start = Instant::now();
        
        // Clone necessary data for the agent
        let widget_refs = widgets.iter()
            .map(|w| w.get_reference())
            .collect::<Vec<_>>();

        // Start the agent in a separate task
        let agent = self.dashboard_agent.clone();
        let handle = tokio::spawn(async move {
            agent.run().await
        });

        let elapsed = start.elapsed();
        histogram!("agent_start_duration_seconds", elapsed.as_secs_f64());

        Ok(AgentHandle {
            handle,
            widget_refs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_dashboard_creation() {
        let metrics_collector = Arc::new(MetricsCollector::new());
        let ml_processor = Arc::new(MLProcessor::new());
        
        let manager = DashboardManager::new(
            metrics_collector,
            ml_processor,
        );

        let config = DashboardConfig {
            layout: LayoutConfig::default(),
            widgets: vec![
                WidgetConfig::new(WidgetType::Chart),
                WidgetConfig::new(WidgetType::Metrics),
            ],
            interactions: InteractionConfig::default(),
            update_config: UpdateConfig::default(),
        };

        let dashboard = manager.create_dashboard(config).await.unwrap();
        assert_eq!(dashboard.widgets.len(), 2);
    }
} 