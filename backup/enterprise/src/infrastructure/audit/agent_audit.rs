use crate::domain::ai::agent::AgentAction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub action: String,
    pub parameters: serde_json::Value,
    pub decision_path: Vec<String>,
    pub validation_results: Vec<String>,
    pub resource_usage: ResourceUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time: f64,
    pub memory_mb: f64,
    pub network_bytes: u64,
}

pub struct AgentAuditor {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
    resource_limits: ResourceLimits,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_cpu_time: f64,
    pub max_memory_mb: f64,
    pub max_network_bytes: u64,
}

impl AgentAuditor {
    pub fn new(resource_limits: ResourceLimits) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            resource_limits,
        }
    }

    pub async fn log_action(
        &self,
        agent_id: &str,
        action: &AgentAction,
        decision_path: Vec<String>,
        resource_usage: ResourceUsage,
    ) -> Result<(), Box<dyn Error>> {
        // Validate resource usage
        self.validate_resource_usage(&resource_usage)?;

        // Create audit entry
        let entry = AuditEntry {
            timestamp: Utc::now(),
            agent_id: agent_id.to_string(),
            action: action.action_type.clone(),
            parameters: serde_json::to_value(&action.parameters)?,
            decision_path,
            validation_results: vec![],
            resource_usage,
        };

        // Log the entry
        let mut entries = self.entries.write().await;
        entries.push(entry.clone());

        info!(
            "Audit log created for agent {} action {}",
            agent_id, action.action_type
        );

        Ok(())
    }

    fn validate_resource_usage(&self, usage: &ResourceUsage) -> Result<(), Box<dyn Error>> {
        if usage.cpu_time > self.resource_limits.max_cpu_time {
            warn!("CPU time exceeded limit: {}", usage.cpu_time);
            return Err("CPU time limit exceeded".into());
        }

        if usage.memory_mb > self.resource_limits.max_memory_mb {
            warn!("Memory usage exceeded limit: {} MB", usage.memory_mb);
            return Err("Memory limit exceeded".into());
        }

        if usage.network_bytes > self.resource_limits.max_network_bytes {
            warn!("Network usage exceeded limit: {} bytes", usage.network_bytes);
            return Err("Network usage limit exceeded".into());
        }

        Ok(())
    }

    pub async fn get_agent_history(&self, agent_id: &str) -> Result<Vec<AuditEntry>, Box<dyn Error>> {
        let entries = self.entries.read().await;
        Ok(entries
            .iter()
            .filter(|e| e.agent_id == agent_id)
            .cloned()
            .collect())
    }

    pub async fn get_action_history(&self, action_type: &str) -> Result<Vec<AuditEntry>, Box<dyn Error>> {
        let entries = self.entries.read().await;
        Ok(entries
            .iter()
            .filter(|e| e.action == action_type)
            .cloned()
            .collect())
    }

    pub async fn export_audit_log(&self) -> Result<String, Box<dyn Error>> {
        let entries = self.entries.read().await;
        Ok(serde_json::to_string_pretty(&*entries)?)
    }
}
