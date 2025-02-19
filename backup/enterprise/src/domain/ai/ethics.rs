use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPrinciple {
    pub name: String,
    pub description: String,
    pub validation_rules: Vec<String>,
    pub severity: PrincipleSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrincipleSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalValidation {
    pub principle: String,
    pub passed: bool,
    pub details: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionBoundary {
    pub allowed_actions: Vec<String>,
    pub restricted_actions: Vec<String>,
    pub required_validations: Vec<String>,
}

pub trait EthicsValidator: Send + Sync {
    fn validate_action(&self, action: &str, context: &HashMap<String, String>) -> Result<Vec<EthicalValidation>, Box<dyn Error>>;
    fn get_boundaries(&self) -> Result<ActionBoundary, Box<dyn Error>>;
    fn update_principles(&mut self, principles: Vec<EthicalPrinciple>) -> Result<(), Box<dyn Error>>;
}

pub struct DefaultEthicsValidator {
    principles: Vec<EthicalPrinciple>,
    boundaries: ActionBoundary,
}

impl DefaultEthicsValidator {
    pub fn new() -> Self {
        Self {
            principles: vec![
                EthicalPrinciple {
                    name: "Transparency".to_string(),
                    description: "All decisions must be auditable".to_string(),
                    validation_rules: vec!["audit_log".to_string(), "decision_path".to_string()],
                    severity: PrincipleSeverity::Critical,
                },
                EthicalPrinciple {
                    name: "Privacy".to_string(),
                    description: "User data must be protected".to_string(),
                    validation_rules: vec!["data_encryption".to_string(), "minimal_access".to_string()],
                    severity: PrincipleSeverity::Critical,
                },
                EthicalPrinciple {
                    name: "Determinism".to_string(),
                    description: "Actions must be reproducible".to_string(),
                    validation_rules: vec!["version_check".to_string(), "seed_validation".to_string()],
                    severity: PrincipleSeverity::High,
                },
                EthicalPrinciple {
                    name: "Fairness".to_string(),
                    description: "Model decisions must be unbiased".to_string(),
                    validation_rules: vec!["bias_check".to_string(), "fairness_metrics".to_string()],
                    severity: PrincipleSeverity::Critical,
                },
                EthicalPrinciple {
                    name: "Accountability".to_string(),
                    description: "Clear responsibility for AI decisions".to_string(),
                    validation_rules: vec!["decision_owner".to_string(), "impact_assessment".to_string()],
                    severity: PrincipleSeverity::High,
                },
                EthicalPrinciple {
                    name: "Security".to_string(),
                    description: "Protection against adversarial attacks".to_string(),
                    validation_rules: vec!["attack_detection".to_string(), "model_robustness".to_string()],
                    severity: PrincipleSeverity::Critical,
                },
                EthicalPrinciple {
                    name: "Reliability".to_string(),
                    description: "Consistent and dependable operation".to_string(),
                    validation_rules: vec!["error_rate".to_string(), "confidence_check".to_string()],
                    severity: PrincipleSeverity::High,
                },
                EthicalPrinciple {
                    name: "DataQuality".to_string(),
                    description: "High-quality training data".to_string(),
                    validation_rules: vec!["data_validation".to_string(), "completeness_check".to_string()],
                    severity: PrincipleSeverity::High,
                },
            ],
            boundaries: ActionBoundary {
                allowed_actions: vec![
                    "model_training".to_string(),
                    "model_evaluation".to_string(),
                    "data_validation".to_string(),
                    "model_monitoring".to_string(),
                    "feature_analysis".to_string(),
                    "performance_tracking".to_string(),
                ],
                restricted_actions: vec![
                    "raw_data_access".to_string(),
                    "model_deployment".to_string(),
                    "config_override".to_string(),
                    "security_bypass".to_string(),
                ],
                required_validations: vec![
                    "bias_check".to_string(),
                    "privacy_check".to_string(),
                    "security_scan".to_string(),
                    "impact_assessment".to_string(),
                    "quality_validation".to_string(),
                ],
            },
        }
    }
}

impl EthicsValidator for DefaultEthicsValidator {
    fn validate_action(&self, action: &str, context: &HashMap<String, String>) -> Result<Vec<EthicalValidation>, Box<dyn Error>> {
        let mut validations = Vec::new();
        let now = chrono::Utc::now();

        // Check if action is restricted
        if self.boundaries.restricted_actions.contains(&action.to_string()) {
            return Ok(vec![EthicalValidation {
                principle: "Action Control".to_string(),
                passed: false,
                details: format!("Action '{}' is restricted", action),
                timestamp: now,
            }]);
        }

        // Check if action is allowed
        if !self.boundaries.allowed_actions.contains(&action.to_string()) {
            return Ok(vec![EthicalValidation {
                principle: "Action Control".to_string(),
                passed: false,
                details: format!("Action '{}' is not in allowed list", action),
                timestamp: now,
            }]);
        }

        // Validate each principle
        for principle in &self.principles {
            let mut passed = true;
            let mut details = String::new();

            // Check required validations
            for rule in &principle.validation_rules {
                if !context.contains_key(rule) {
                    passed = false;
                    details = format!("Missing required validation: {}", rule);
                    break;
                }

                // Additional validation based on rule type
                if let Some(value) = context.get(rule) {
                    match rule.as_str() {
                        "error_rate" => {
                            if let Ok(rate) = value.parse::<f64>() {
                                if rate > 0.1 {
                                    passed = false;
                                    details = format!("Error rate too high: {}", rate);
                                    break;
                                }
                            }
                        }
                        "confidence_check" => {
                            if let Ok(confidence) = value.parse::<f64>() {
                                if confidence < 0.9 {
                                    passed = false;
                                    details = format!("Confidence too low: {}", confidence);
                                    break;
                                }
                            }
                        }
                        "data_validation" => {
                            if value != "passed" {
                                passed = false;
                                details = format!("Data validation failed: {}", value);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }

            validations.push(EthicalValidation {
                principle: principle.name.clone(),
                passed,
                details,
                timestamp: now,
            });
        }

        Ok(validations)
    }

    fn get_boundaries(&self) -> Result<ActionBoundary, Box<dyn Error>> {
        Ok(self.boundaries.clone())
    }

    fn update_principles(&mut self, principles: Vec<EthicalPrinciple>) -> Result<(), Box<dyn Error>> {
        self.principles = principles;
        Ok(())
    }
}
