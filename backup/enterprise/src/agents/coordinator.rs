pub struct AgentCoordinator {
    agent_registry: Arc<AgentRegistry>,
    event_router: Arc<EventRouter>,
    state_manager: Arc<StateManager>,
}

impl AgentCoordinator {
    pub async fn distribute_event(
        &self,
        event: EnterpriseEvent,
        context: &AgentContext,
    ) -> Result<Vec<AgentResponse>, AgentError> {
        // Determine relevant agents
        let relevant_agents = self.agent_registry
            .get_relevant_agents(&event)
            .await?;
            
        // Route event to agents
        let responses = self.event_router
            .route_event(event, &relevant_agents, context)
            .await?;
            
        // Update state
        self.state_manager
            .update_agent_states(&responses)
            .await?;
            
        Ok(responses)
    }
} 