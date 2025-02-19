#[cfg(test)]
mod tests {
    use super::super::ethics::{DefaultEthicsValidator, EthicsValidator, EthicalPrinciple, PrincipleSeverity};
    use std::collections::HashMap;

    #[test]
    fn test_ethics_validation() {
        let validator = DefaultEthicsValidator::new();
        let mut context = HashMap::new();
        context.insert("audit_log".to_string(), "enabled".to_string());
        context.insert("decision_path".to_string(), "recorded".to_string());
        
        let result = validator.validate_action("model_training", &context).unwrap();
        assert!(!result.is_empty());
        
        // Check transparency principle
        let transparency = result.iter().find(|v| v.principle == "Transparency").unwrap();
        assert!(transparency.passed);
    }

    #[test]
    fn test_restricted_action() {
        let validator = DefaultEthicsValidator::new();
        let context = HashMap::new();
        
        let result = validator.validate_action("raw_data_access", &context).unwrap();
        assert!(!result.is_empty());
        
        // Should fail due to restricted action
        let failed = result.iter().any(|v| !v.passed);
        assert!(failed);
    }

    #[test]
    fn test_action_boundaries() {
        let validator = DefaultEthicsValidator::new();
        let boundaries = validator.get_boundaries().unwrap();
        
        assert!(boundaries.allowed_actions.contains(&"model_training".to_string()));
        assert!(boundaries.restricted_actions.contains(&"raw_data_access".to_string()));
        assert!(boundaries.required_validations.contains(&"bias_check".to_string()));
    }

    #[test]
    fn test_update_principles() {
        let mut validator = DefaultEthicsValidator::new();
        let new_principles = vec![
            EthicalPrinciple {
                name: "TestPrinciple".to_string(),
                description: "Test Description".to_string(),
                validation_rules: vec!["test_rule".to_string()],
                severity: PrincipleSeverity::High,
            },
        ];
        
        assert!(validator.update_principles(new_principles.clone()).is_ok());
        
        let context = HashMap::new();
        let result = validator.validate_action("test_action", &context).unwrap();
        assert!(result.iter().any(|v| v.principle == "TestPrinciple"));
    }
}
