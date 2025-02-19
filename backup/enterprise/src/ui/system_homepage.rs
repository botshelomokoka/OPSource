use crate::dashboard::{DashboardManager, DashboardComponents};
use crate::reporting::ReportingEngine;
use crate::monitoring::MonitoringManager;

pub struct SystemHomepage {
    dashboard_manager: Arc<DashboardManager>,
    reporting_engine: Arc<ReportingEngine>,
    monitoring_manager: Arc<MonitoringManager>,
    layout_engine: Arc<LayoutEngine>,
}

impl SystemHomepage {
    pub async fn render_homepage(
        &self,
        config: HomepageConfig,
    ) -> Result<Homepage, HomepageError> {
        // 1. Initialize Layout
        let layout = self.layout_engine
            .create_homepage_layout(&config.layout)
            .await?;
        
        // 2. Setup Dashboard Components
        let dashboard = self.setup_dashboard_components(&config).await?;
        
        // 3. Initialize Real-time Monitoring
        let monitoring = self.setup_monitoring_components(&config).await?;
        
        // 4. Setup Reporting Widgets
        let reporting = self.setup_reporting_widgets(&config).await?;

        Ok(Homepage {
            layout,
            dashboard,
            monitoring,
            reporting,
            config: config.clone(),
        })
    }

    async fn setup_dashboard_components(
        &self,
        config: &HomepageConfig,
    ) -> Result<DashboardComponents, HomepageError> {
        let components = DashboardComponents::new()
            // Key Performance Indicators
            .add_component("kpi-overview", KPIWidget::new(
                self.dashboard_manager.get_kpi_metrics().await?,
            ))?
            // Security Overview
            .add_component("security-overview", SecurityWidget::new(
                self.dashboard_manager.get_security_metrics().await?,
            ))?
            // ML System Status
            .add_component("ml-status", MLStatusWidget::new(
                self.dashboard_manager.get_ml_metrics().await?,
            ))?
            // System Health
            .add_component("system-health", HealthWidget::new(
                self.dashboard_manager.get_health_metrics().await?,
            ))?;

        Ok(components)
    }
} 