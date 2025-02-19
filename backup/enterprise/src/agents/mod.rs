use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

pub mod trading;
pub mod security;
pub mod monitoring;
pub mod risk;
pub mod compliance;

pub struct AgentSystem {
    // Core agents
    trading_agent: Arc<TradingAgent>,
    security_agent: Arc<SecurityAgent>,
    monitoring_agent: Arc<MonitoringAgent>,
    risk_agent: Arc<RiskAgent>,
    compliance_agent: Arc<ComplianceAgent>,
    
    // Coordination
    coordinator: Arc<AgentCoordinator>,
    event_bus: broadcast::Sender<AgentEvent>,
    
    // State management
    state: RwLock<AgentState>,
    metrics: AgentMetrics,
}

impl AgentSystem {
    pub async fn process_enterprise_event(
        &self,
        event: EnterpriseEvent,
        context: &AgentContext,
    ) -> Result<AgentResponse, AgentError> {
        // Distribute event to relevant agents
        let responses = self.coordinator
            .distribute_event(event, context)
            .await?;
            
        // Aggregate responses
        let aggregated = self.aggregate_responses(responses).await?;
        
        // Update system state
        self.update_state(&aggregated, context).await?;
        
        Ok(aggregated)
    }
} 