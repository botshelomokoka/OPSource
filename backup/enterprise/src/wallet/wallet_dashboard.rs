use crate::security::SecurityManager;
use crate::monitoring::{WalletMonitor, MetricsCollector};
use crate::ui::dashboard_agent::DashboardAgent;
use anya_core::wallet::WalletManager;
use metrics::{counter, histogram};
use log::{info, error};
use tokio::time::Instant;
use std::sync::Arc;

pub struct WalletDashboard {
    wallet_manager: Arc<WalletManager>,
    security_manager: Arc<SecurityManager>,
    wallet_monitor: Arc<WalletMonitor>,
    metrics_collector: Arc<MetricsCollector>,
    dashboard_agent: Arc<DashboardAgent>,
    dashboard_state: Arc<DashboardState>,
}

impl WalletDashboard {
    pub fn new(
        wallet_manager: Arc<WalletManager>,
        security_manager: Arc<SecurityManager>,
        metrics_collector: Arc<MetricsCollector>,
        ml_processor: Arc<MLProcessor>,
    ) -> Self {
        let dashboard_agent = Arc::new(DashboardAgent::new(
            metrics_collector.clone(),
            ml_processor,
            Duration::from_secs(1),
            4,
        ));

        Self {
            wallet_manager,
            security_manager,
            wallet_monitor: Arc::new(WalletMonitor::new()),
            metrics_collector,
            dashboard_agent,
            dashboard_state: Arc::new(DashboardState::default()),
        }
    }

    pub async fn process_dashboard_update(
        &self,
        request: DashboardRequest,
        context: &DashboardContext,
    ) -> Result<DashboardResult, DashboardError> {
        let start = Instant::now();
        counter!("wallet_dashboard_updates_total", 1);

        // 1. Security Validation
        self.security_manager
            .validate_dashboard_request(&request, context)
            .await
            .map_err(|e| {
                counter!("wallet_dashboard_security_failures_total", 1);
                DashboardError::SecurityError(e.to_string())
            })?;
        
        // 2. Collect Metrics
        let metrics = self.collect_dashboard_metrics(context).await?;
        
        // 3. Process Update through Agent
        let result = self.process_agent_update(request, metrics, context).await?;
        
        // 4. Update Dashboard State
        self.dashboard_state
            .update_state(&result)
            .await
            .map_err(|e| {
                counter!("wallet_dashboard_state_update_failures_total", 1);
                DashboardError::StateError(e.to_string())
            })?;

        let elapsed = start.elapsed();
        histogram!("wallet_dashboard_update_duration_seconds", elapsed.as_secs_f64());
        counter!("wallet_dashboard_update_success_total", 1);

        Ok(result)
    }

    async fn process_agent_update(
        &self,
        request: DashboardRequest,
        metrics: DashboardMetrics,
        context: &DashboardContext,
    ) -> Result<DashboardResult, DashboardError> {
        let start = Instant::now();
        
        // Create agent observations
        let observations = DashboardObservations {
            request: request.clone(),
            metrics,
            context: context.clone(),
            timestamp: chrono::Utc::now(),
        };

        // Let agent make decisions
        let decisions = self.dashboard_agent
            .make_decisions(observations)
            .await
            .map_err(|e| DashboardError::AgentError(e.to_string()))?;

        // Execute agent decisions
        let result = match request.update_type {
            DashboardUpdateType::WalletStatus => {
                self.execute_wallet_status_decisions(&decisions, context).await?
            },
            DashboardUpdateType::TransactionHistory => {
                self.execute_transaction_decisions(&decisions, context).await?
            },
            DashboardUpdateType::AssetOverview => {
                self.execute_asset_decisions(&decisions, context).await?
            },
            DashboardUpdateType::Analytics => {
                self.execute_analytics_decisions(&decisions, context).await?
            },
        };

        let elapsed = start.elapsed();
        histogram!("wallet_dashboard_agent_processing_duration_seconds", elapsed.as_secs_f64());

        Ok(result)
    }

    async fn collect_dashboard_metrics(
        &self,
        context: &DashboardContext,
    ) -> Result<DashboardMetrics, DashboardError> {
        let start = Instant::now();
        counter!("wallet_metrics_collection_attempts_total", 1);

        // Collect wallet metrics
        let wallet_metrics = self.metrics_collector
            .collect_wallet_metrics()
            .await
            .map_err(|e| {
                counter!("wallet_metrics_collection_failures_total", 1);
                DashboardError::MetricsError(e.to_string())
            })?;
            
        // Collect transaction metrics
        let tx_metrics = self.metrics_collector
            .collect_transaction_metrics()
            .await
            .map_err(|e| {
                counter!("transaction_metrics_collection_failures_total", 1);
                DashboardError::MetricsError(e.to_string())
            })?;
            
        // Collect asset metrics
        let asset_metrics = self.metrics_collector
            .collect_asset_metrics()
            .await
            .map_err(|e| {
                counter!("asset_metrics_collection_failures_total", 1);
                DashboardError::MetricsError(e.to_string())
            })?;

        let elapsed = start.elapsed();
        histogram!("wallet_metrics_collection_duration_seconds", elapsed.as_secs_f64());
        counter!("wallet_metrics_collection_success_total", 1);

        Ok(DashboardMetrics {
            wallet: wallet_metrics,
            transactions: tx_metrics,
            assets: asset_metrics,
            timestamp: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_wallet_dashboard_update() {
        let wallet_manager = Arc::new(WalletManager::new());
        let security_manager = Arc::new(SecurityManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let ml_processor = Arc::new(MLProcessor::new());

        let dashboard = WalletDashboard::new(
            wallet_manager,
            security_manager,
            metrics_collector,
            ml_processor,
        );

        let request = DashboardRequest {
            update_type: DashboardUpdateType::WalletStatus,
            params: Default::default(),
        };

        let context = DashboardContext::default();
        let result = dashboard.process_dashboard_update(request, &context).await;
        assert!(result.is_ok());
    }
} 