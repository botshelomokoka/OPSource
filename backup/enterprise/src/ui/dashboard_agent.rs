use crate::ml::MLProcessor;
use crate::monitoring::MetricsCollector;
use tokio::sync::broadcast;
use thiserror::Error;
use metrics::{counter, gauge, histogram};
use log::{info, warn, error};
use std::sync::Arc;
use tokio::time::{Instant, Duration};

#[derive(Error, Debug)]
pub enum DashboardAgentError {
    #[error("Perception Error: {0}")]
    PerceptionError(String),

    #[error("Decision Error: {0}")]
    DecisionError(String),

    #[error("Action Error: {0}")]
    ActionError(String),

    #[error("State Error: {0}")]
    StateError(String),
}

/// Dashboard Agent State
#[derive(Debug, Clone)]
pub struct DashboardState {
    pub last_update: chrono::DateTime<chrono::Utc>,
    pub active_widgets: Vec<WidgetState>,
    pub user_interactions: Vec<UserInteraction>,
    pub performance_metrics: PerformanceMetrics,
}

/// Dashboard Agent that follows AI principles
pub struct DashboardAgent {
    // Perception
    metrics_collector: Arc<MetricsCollector>,
    state_observer: Arc<StateObserver>,
    
    // Decision Making
    ml_processor: Arc<MLProcessor>,
    decision_engine: Arc<DecisionEngine>,
    
    // Action
    widget_manager: Arc<WidgetManager>,
    update_sender: broadcast::Sender<DashboardUpdate>,
    
    // State
    state: Arc<tokio::sync::RwLock<DashboardState>>,
    
    // Configuration
    update_interval: Duration,
    max_concurrent_updates: usize,
}

impl DashboardAgent {
    /// Initialize a new dashboard agent
    pub fn new(
        metrics_collector: Arc<MetricsCollector>,
        ml_processor: Arc<MLProcessor>,
        update_interval: Duration,
        max_concurrent_updates: usize,
    ) -> Self {
        let (update_sender, _) = broadcast::channel(100);
        
        Self {
            metrics_collector,
            state_observer: Arc::new(StateObserver::new()),
            ml_processor,
            decision_engine: Arc::new(DecisionEngine::new()),
            widget_manager: Arc::new(WidgetManager::new()),
            update_sender,
            state: Arc::new(tokio::sync::RwLock::new(DashboardState::default())),
            update_interval,
            max_concurrent_updates,
        }
    }

    /// Start the dashboard agent's perception-decision-action loop
    pub async fn run(&self) -> Result<(), DashboardAgentError> {
        info!("Starting dashboard agent");
        let start = Instant::now();
        counter!("dashboard_agent_starts_total", 1);

        loop {
            // 1. Perceive Environment
            let observations = self.perceive_environment().await?;
            
            // 2. Make Decisions
            let decisions = self.make_decisions(observations).await?;
            
            // 3. Take Actions
            self.execute_actions(decisions).await?;
            
            // 4. Update State
            self.update_state().await?;
            
            // Wait for next cycle
            tokio::time::sleep(self.update_interval).await;
            
            // Record metrics
            let elapsed = start.elapsed();
            histogram!("dashboard_agent_cycle_duration_seconds", elapsed.as_secs_f64());
        }
    }

    /// Perceive the dashboard environment
    async fn perceive_environment(&self) -> Result<DashboardObservations, DashboardAgentError> {
        let start = Instant::now();
        counter!("dashboard_observations_total", 1);

        // Collect metrics
        let metrics = self.metrics_collector.collect_dashboard_metrics().await
            .map_err(|e| DashboardAgentError::PerceptionError(e.to_string()))?;

        // Observe state
        let state = self.state_observer.observe_state().await
            .map_err(|e| DashboardAgentError::PerceptionError(e.to_string()))?;

        // Observe user interactions
        let interactions = self.state_observer.observe_interactions().await
            .map_err(|e| DashboardAgentError::PerceptionError(e.to_string()))?;

        let elapsed = start.elapsed();
        histogram!("dashboard_perception_duration_seconds", elapsed.as_secs_f64());

        Ok(DashboardObservations {
            metrics,
            state,
            interactions,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Make decisions based on observations
    async fn make_decisions(
        &self,
        observations: DashboardObservations,
    ) -> Result<Vec<DashboardDecision>, DashboardAgentError> {
        let start = Instant::now();
        counter!("dashboard_decisions_total", 1);

        // Process observations with ML
        let ml_insights = self.ml_processor.process_observations(&observations).await
            .map_err(|e| DashboardAgentError::DecisionError(e.to_string()))?;

        // Generate decisions
        let decisions = self.decision_engine.generate_decisions(ml_insights).await
            .map_err(|e| DashboardAgentError::DecisionError(e.to_string()))?;

        let elapsed = start.elapsed();
        histogram!("dashboard_decision_duration_seconds", elapsed.as_secs_f64());

        Ok(decisions)
    }

    /// Execute dashboard actions based on decisions
    async fn execute_actions(
        &self,
        decisions: Vec<DashboardDecision>,
    ) -> Result<(), DashboardAgentError> {
        let start = Instant::now();
        counter!("dashboard_actions_total", 1);

        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_concurrent_updates));
        let mut handles = Vec::new();

        for decision in decisions {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let widget_manager = self.widget_manager.clone();
            let update_sender = self.update_sender.clone();

            let handle = tokio::spawn(async move {
                let result = widget_manager.apply_decision(&decision).await;
                if let Ok(update) = result {
                    let _ = update_sender.send(update);
                }
                drop(permit);
                result
            });

            handles.push(handle);
        }

        for handle in handles {
            match handle.await {
                Ok(Ok(_)) => counter!("dashboard_action_success_total", 1),
                Ok(Err(e)) => {
                    counter!("dashboard_action_failures_total", 1);
                    error!("Dashboard action failed: {}", e);
                },
                Err(e) => {
                    counter!("dashboard_action_failures_total", 1);
                    error!("Dashboard task failed: {}", e);
                }
            }
        }

        let elapsed = start.elapsed();
        histogram!("dashboard_action_duration_seconds", elapsed.as_secs_f64());

        Ok(())
    }

    /// Update the dashboard state
    async fn update_state(&self) -> Result<(), DashboardAgentError> {
        let start = Instant::now();
        counter!("dashboard_state_updates_total", 1);

        let mut state = self.state.write().await;
        state.last_update = chrono::Utc::now();
        
        // Update performance metrics
        state.performance_metrics = self.metrics_collector
            .collect_performance_metrics()
            .await
            .map_err(|e| DashboardAgentError::StateError(e.to_string()))?;

        let elapsed = start.elapsed();
        histogram!("dashboard_state_update_duration_seconds", elapsed.as_secs_f64());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_dashboard_agent_cycle() {
        let agent = DashboardAgent::new(
            Arc::new(MetricsCollector::new()),
            Arc::new(MLProcessor::new()),
            Duration::from_secs(1),
            4,
        );

        // Run one cycle
        let observations = agent.perceive_environment().await.unwrap();
        let decisions = agent.make_decisions(observations).await.unwrap();
        assert!(agent.execute_actions(decisions).await.is_ok());
        assert!(agent.update_state().await.is_ok());
    }
} 