use super::agent_audit::{AgentAuditor, ResourceLimits, ResourceUsage};
use crate::domain::ai::agent::AgentAction;
use std::collections::HashMap;

#[tokio::test]
async fn test_audit_logging() {
    let limits = ResourceLimits {
        max_cpu_time: 10.0,
        max_memory_mb: 1000.0,
        max_network_bytes: 1000000,
    };
    
    let auditor = AgentAuditor::new(limits);
    let agent_id = "test-agent";
    
    let mut params = HashMap::new();
    params.insert("param1".to_string(), "value1".to_string());
    
    let action = AgentAction {
        action_type: "test-action".to_string(),
        parameters: params,
        priority: 1,
        context: Some("test-context".to_string()),
    };
    
    let resource_usage = ResourceUsage {
        cpu_time: 1.0,
        memory_mb: 100.0,
        network_bytes: 1000,
    };
    
    let decision_path = vec!["step1".to_string(), "step2".to_string()];
    
    // Test logging
    let result = auditor
        .log_action(agent_id, &action, decision_path.clone(), resource_usage)
        .await;
    assert!(result.is_ok());
    
    // Test retrieval
    let history = auditor.get_agent_history(agent_id).await.unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].agent_id, agent_id);
    assert_eq!(history[0].action, "test-action");
    assert_eq!(history[0].decision_path, decision_path);
}

#[tokio::test]
async fn test_resource_limits() {
    let limits = ResourceLimits {
        max_cpu_time: 1.0,
        max_memory_mb: 100.0,
        max_network_bytes: 1000,
    };
    
    let auditor = AgentAuditor::new(limits);
    
    let action = AgentAction {
        action_type: "test-action".to_string(),
        parameters: HashMap::new(),
        priority: 1,
        context: None,
    };
    
    // Test exceeding CPU limit
    let resource_usage = ResourceUsage {
        cpu_time: 2.0, // Exceeds limit
        memory_mb: 50.0,
        network_bytes: 500,
    };
    
    let result = auditor
        .log_action("test-agent", &action, vec![], resource_usage)
        .await;
    assert!(result.is_err());
    
    // Test exceeding memory limit
    let resource_usage = ResourceUsage {
        cpu_time: 0.5,
        memory_mb: 200.0, // Exceeds limit
        network_bytes: 500,
    };
    
    let result = auditor
        .log_action("test-agent", &action, vec![], resource_usage)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_export_audit_log() {
    let limits = ResourceLimits {
        max_cpu_time: 10.0,
        max_memory_mb: 1000.0,
        max_network_bytes: 1000000,
    };
    
    let auditor = AgentAuditor::new(limits);
    
    // Add some audit entries
    let action = AgentAction {
        action_type: "test-action".to_string(),
        parameters: HashMap::new(),
        priority: 1,
        context: None,
    };
    
    let resource_usage = ResourceUsage {
        cpu_time: 1.0,
        memory_mb: 100.0,
        network_bytes: 1000,
    };
    
    auditor
        .log_action("agent1", &action, vec![], resource_usage.clone())
        .await
        .unwrap();
    auditor
        .log_action("agent2", &action, vec![], resource_usage)
        .await
        .unwrap();
    
    // Test export
    let log = auditor.export_audit_log().await.unwrap();
    assert!(log.contains("agent1"));
    assert!(log.contains("agent2"));
    assert!(log.contains("test-action"));
}
