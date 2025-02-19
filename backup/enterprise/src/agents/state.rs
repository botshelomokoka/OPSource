pub struct AgentState {
    trading_state: TradingState,
    security_state: SecurityState,
    monitoring_state: MonitoringState,
    risk_state: RiskState,
    compliance_state: ComplianceState,
}

impl AgentState {
    pub async fn update_state(
        &mut self,
        event: &AgentEvent,
        context: &AgentContext,
    ) -> Result<(), StateError> {
        match event {
            AgentEvent::Trading(e) => {
                self.trading_state.update(e, context).await?;
            },
            AgentEvent::Security(e) => {
                self.security_state.update(e, context).await?;
            },
            AgentEvent::Monitoring(e) => {
                self.monitoring_state.update(e, context).await?;
            },
            // Handle other event types
        }
        
        Ok(())
    }
} 