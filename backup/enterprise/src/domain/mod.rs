pub mod ai;
pub mod fl;

pub use ai::{AIAgent, AgentAction, AgentCapability, AgentState};
pub use fl::{FederatedModel, ModelMetrics};

pub mod web5;
pub mod network;
