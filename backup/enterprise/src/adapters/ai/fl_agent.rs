use crate::domain::ai::agent::{AIAgent, AgentAction, AgentCapability, AgentState};
use crate::domain::ai::ethics::{EthicsValidator, DefaultEthicsValidator};
use crate::domain::ai::bias::{BiasDetector, DefaultBiasDetector};
use crate::domain::fl::{FederatedModel, ModelMetrics};
use crate::infrastructure::audit::{AgentAuditor, ResourceUsage};
use crate::ports::fl::service::FederatedLearningPort;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

pub struct FederatedLearningAgent {
    agent_id: String,
    fl_port: Arc<dyn FederatedLearningPort + Send + Sync>,
    ethics_validator: DefaultEthicsValidator,
    bias_detector: DefaultBiasDetector,
    auditor: Arc<AgentAuditor>,
    capabilities: Vec<AgentCapability>,
    current_model: Option<FederatedModel>,
    metrics: HashMap<String, f64>,
}

impl FederatedLearningAgent {
    pub fn new(
        agent_id: String,
        fl_port: Arc<dyn FederatedLearningPort + Send + Sync>,
        auditor: Arc<AgentAuditor>,
    ) -> Self {
        Self {
            agent_id,
            fl_port,
            ethics_validator: DefaultEthicsValidator::new(),
            bias_detector: DefaultBiasDetector::new(),
            auditor,
            capabilities: vec![],
            current_model: None,
            metrics: HashMap::new(),
        }
    }

    async fn optimize_model(&mut self, model: &mut FederatedModel) -> Result<ModelMetrics, Box<dyn Error>> {
        let start = Instant::now();
        info!("Optimizing model {} with agent {}", model.id, self.agent_id);

        // Validate action ethics
        let mut context = HashMap::new();
        context.insert("model_id".to_string(), model.id.clone());
        context.insert("action".to_string(), "model_optimization".to_string());

        let validations = self.ethics_validator.validate_action("model_training", &context)?;
        let passed_validation = validations.iter().all(|v| v.passed);

        if !passed_validation {
            let failed = validations.iter().filter(|v| !v.passed).map(|v| &v.details).collect::<Vec<_>>();
            return Err(format!("Ethics validation failed: {:?}", failed).into());
        }

        // Check for bias
        if let Some(data) = model.training_data() {
            let bias_report = self.bias_detector.detect_bias(&model.id, data)?;
            for metric in &bias_report.metrics {
                if metric.value > metric.threshold {
                    warn!(
                        "Bias detected in model {}: {} = {} (threshold: {})",
                        model.id, metric.name, metric.value, metric.threshold
                    );
                }
            }
        }

        // Train the model
        let metrics = self.fl_port.train_model(model).await?;

        // Record resource usage
        let duration = start.elapsed();
        let resource_usage = ResourceUsage {
            cpu_time: duration.as_secs_f64(),
            memory_mb: 100.0, // TODO: Implement actual memory tracking
            network_bytes: 1000, // TODO: Implement actual network tracking
        };

        // Log the action
        self.auditor
            .log_action(
                &self.agent_id,
                &AgentAction {
                    action_type: "optimize_model".to_string(),
                    parameters: context,
                    priority: 1,
                    context: Some("model_optimization".to_string()),
                },
                vec!["validate_ethics".to_string(), "check_bias".to_string(), "train_model".to_string()],
                resource_usage,
            )
            .await?;

        Ok(metrics)
    }
}

#[async_trait]
impl AIAgent for FederatedLearningAgent {
    async fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Initializing FL agent: {}", self.agent_id);
        self.fl_port.initialize_training().await?;
        
        // Set up default capabilities
        self.capabilities = vec![
            AgentCapability {
                name: "model_optimization".to_string(),
                description: "Optimize federated learning models".to_string(),
                parameters: HashMap::new(),
                requirements: vec!["fl_port".to_string()],
            },
            AgentCapability {
                name: "model_evaluation".to_string(),
                description: "Evaluate model performance".to_string(),
                parameters: HashMap::new(),
                requirements: vec!["fl_port".to_string()],
            },
        ];
        
        Ok(())
    }

    async fn process_task(&mut self, task: String) -> Result<Vec<AgentAction>, Box<dyn Error>> {
        info!("Processing task for FL agent {}: {}", self.agent_id, task);
        
        let mut actions = Vec::new();
        
        // Parse task and create appropriate actions
        if task.contains("optimize") && self.current_model.is_some() {
            actions.push(AgentAction {
                action_type: "optimize_model".to_string(),
                parameters: HashMap::new(),
                priority: 1,
                context: Some("model_optimization".to_string()),
            });
        }
        
        Ok(actions)
    }

    async fn execute_action(&mut self, action: AgentAction) -> Result<(), Box<dyn Error>> {
        match action.action_type.as_str() {
            "optimize_model" => {
                if let Some(model) = &mut self.current_model {
                    let metrics = self.optimize_model(model).await?;
                    self.metrics.insert("accuracy".to_string(), metrics.accuracy);
                    self.metrics.insert("loss".to_string(), metrics.loss);
                }
            }
            _ => warn!("Unknown action type: {}", action.action_type),
        }
        Ok(())
    }

    async fn get_state(&self) -> Result<AgentState, Box<dyn Error>> {
        Ok(AgentState {
            agent_id: self.agent_id.clone(),
            capabilities: self.capabilities.clone(),
            current_task: None,
            performance_metrics: self.metrics.clone(),
        })
    }

    async fn update_capabilities(&mut self, capabilities: Vec<AgentCapability>) -> Result<(), Box<dyn Error>> {
        self.capabilities = capabilities;
        Ok(())
    }
}
