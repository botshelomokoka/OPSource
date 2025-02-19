use crate::domain::ai::agent::{AIAgent, AgentAction, AgentCoordinator, AgentState};
use crate::infrastructure::metrics::agent_metrics::AgentMetrics;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct DefaultAgentCoordinator {
    agents: Arc<RwLock<Vec<Box<dyn AIAgent>>>>,
    metrics: AgentMetrics,
    task_history: Arc<RwLock<HashMap<String, Vec<AgentAction>>>>,
}

impl DefaultAgentCoordinator {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            agents: Arc::new(RwLock::new(Vec::new())),
            metrics: AgentMetrics::new()?,
            task_history: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    async fn distribute_task(&self, task: &str) -> Result<Vec<AgentAction>, Box<dyn Error>> {
        let mut all_actions = Vec::new();
        let agents = self.agents.read().await;
        
        for agent in agents.iter() {
            let start = std::time::Instant::now();
            let actions = agent.process_task(task.to_string()).await?;
            
            // Record metrics
            let duration = start.elapsed().as_secs_f64();
            let state = agent.get_state().await?;
            self.metrics.record_task_processing_time(&state.agent_id, duration);
            self.metrics.record_actions_generated(&state.agent_id, actions.len() as f64);
            
            all_actions.extend(actions);
        }
        
        Ok(all_actions)
    }
}

#[async_trait]
impl AgentCoordinator for DefaultAgentCoordinator {
    async fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<(), Box<dyn Error>> {
        let state = agent.get_state().await?;
        info!("Registering agent: {}", state.agent_id);
        
        let mut agents = self.agents.write().await;
        agents.push(agent);
        
        self.metrics.record_agent_registration(&state.agent_id);
        Ok(())
    }

    async fn coordinate_task(&mut self, task: String) -> Result<Vec<AgentAction>, Box<dyn Error>> {
        info!("Coordinating task: {}", task);
        
        let actions = self.distribute_task(&task).await?;
        
        // Store task history
        let mut history = self.task_history.write().await;
        history.insert(task.clone(), actions.clone());
        
        self.metrics.record_task_completion(&task);
        Ok(actions)
    }

    async fn get_agent_states(&self) -> Result<Vec<AgentState>, Box<dyn Error>> {
        let agents = self.agents.read().await;
        let mut states = Vec::new();
        
        for agent in agents.iter() {
            match agent.get_state().await {
                Ok(state) => states.push(state),
                Err(e) => warn!("Failed to get agent state: {}", e),
            }
        }
        
        Ok(states)
    }
}
