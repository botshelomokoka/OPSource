use std::error::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub action_type: String,
    pub parameters: HashMap<String, String>,
    pub priority: u32,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub agent_id: String,
    pub capabilities: Vec<AgentCapability>,
    pub current_task: Option<String>,
    pub performance_metrics: HashMap<String, f64>,
}

#[async_trait]
pub trait AIAgent: Send + Sync {
    async fn initialize(&mut self) -> Result<(), Box<dyn Error>>;
    async fn process_task(&mut self, task: String) -> Result<Vec<AgentAction>, Box<dyn Error>>;
    async fn execute_action(&mut self, action: AgentAction) -> Result<(), Box<dyn Error>>;
    async fn get_state(&self) -> Result<AgentState, Box<dyn Error>>;
    async fn update_capabilities(&mut self, capabilities: Vec<AgentCapability>) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait AgentCoordinator: Send + Sync {
    async fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<(), Box<dyn Error>>;
    async fn coordinate_task(&mut self, task: String) -> Result<Vec<AgentAction>, Box<dyn Error>>;
    async fn get_agent_states(&self) -> Result<Vec<AgentState>, Box<dyn Error>>;
}
