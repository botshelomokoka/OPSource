use crate::domain::ai::agent::{AIAgent, AgentAction, AgentCapability, AgentCoordinator, AgentState};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct AgentSystem {
    agents: HashMap<String, Box<dyn AIAgent>>,
    coordinator: Box<dyn AgentCoordinator>,
    metrics: Arc<RwLock<HashMap<String, f64>>>,
    active_tasks: HashSet<String>,
}

impl AgentSystem {
    pub fn new(coordinator: Box<dyn AgentCoordinator>) -> Self {
        Self {
            agents: HashMap::new(),
            coordinator,
            metrics: Arc::new(RwLock::new(HashMap::new())),
            active_tasks: HashSet::new(),
        }
    }

    pub async fn register_ml_agent(&mut self, agent_id: String, agent: Box<dyn AIAgent>) -> Result<(), Box<dyn Error>> {
        info!("Registering ML agent: {}", agent_id);
        self.agents.insert(agent_id.clone(), agent);
        self.coordinator.register_agent(self.agents.get(&agent_id).unwrap().as_ref().clone()).await?;
        Ok(())
    }

    pub async fn process_ml_task(&mut self, task_id: String, task: String) -> Result<Vec<AgentAction>, Box<dyn Error>> {
        if self.active_tasks.contains(&task_id) {
            warn!("Task {} is already being processed", task_id);
            return Ok(vec![]);
        }

        info!("Processing ML task: {}", task_id);
        self.active_tasks.insert(task_id.clone());

        let actions = self.coordinator.coordinate_task(task).await?;
        
        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.insert(format!("task_{}_actions", task_id), actions.len() as f64);

        self.active_tasks.remove(&task_id);
        Ok(actions)
    }

    pub async fn get_agent_metrics(&self) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    pub async fn update_agent_capabilities(
        &mut self,
        agent_id: &str,
        capabilities: Vec<AgentCapability>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.update_capabilities(capabilities).await?;
            info!("Updated capabilities for agent: {}", agent_id);
        }
        Ok(())
    }
}
