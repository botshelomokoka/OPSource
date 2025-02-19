use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationMetrics {
    // Core Financial Validation
    pub financial_metrics: FinancialValidationMetrics,
    // Protocol-Specific Validation
    pub protocol_metrics: ProtocolValidationMetrics,
    // Security Validation
    pub security_metrics: SecurityValidationMetrics,
    // Compliance Validation
    pub compliance_metrics: ComplianceValidationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialValidationMetrics {
    // Transaction Validation
    pub transaction_validation: TransactionValidation,
    // Balance Validation
    pub balance_validation: BalanceValidation,
    // Fee Validation
    pub fee_validation: FeeValidation,
    // Performance Validation
    pub performance_validation: PerformanceValidation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionValidation {
    pub total_volume_validated: f64,
    pub validation_success_rate: f64,
    pub validation_failures: Vec<ValidationFailure>,
    pub validation_latency: ValidationLatencyMetrics,
    pub double_spend_checks: DoubleSpendMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolValidationMetrics {
    // RSK Validation
    pub rsk_validation: RSKValidationMetrics,
    // Rollups Validation
    pub rollups_validation: RollupsValidationMetrics,
    // Lightning Validation
    pub lightning_validation: LightningValidationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RSKValidationMetrics {
    // Merge Mining Validation
    pub merge_mining_validation: MergeMiningValidation,
    // Federation Validation
    pub federation_validation: FederationValidation,
    // Smart Contract Validation
    pub smart_contract_validation: SmartContractValidation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityValidationMetrics {
    // Quantum Resistance Validation
    pub quantum_resistance: QuantumResistanceMetrics,
    // Privacy Validation
    pub privacy_validation: PrivacyValidationMetrics,
    // Key Security Validation
    pub key_security: KeySecurityMetrics,
    // Network Security
    pub network_security: NetworkSecurityMetrics,
}

impl ValidationMetrics {
    pub async fn validate_all(&mut self) -> Result<ValidationReport, ValidationError> {
        let mut report = ValidationReport::new();

        // Validate financial metrics
        self.validate_financial_metrics(&mut report).await?;
        
        // Validate protocol-specific metrics
        self.validate_protocol_metrics(&mut report).await?;
        
        // Validate security metrics
        self.validate_security_metrics(&mut report).await?;
        
        // Validate compliance metrics
        self.validate_compliance_metrics(&mut report).await?;

        Ok(report)
    }

    async fn validate_financial_metrics(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate transaction metrics
        self.validate_transactions(report).await?;
        
        // Validate balance metrics
        self.validate_balances(report).await?;
        
        // Validate fee metrics
        self.validate_fees(report).await?;

        Ok(())
    }

    async fn validate_protocol_metrics(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate RSK metrics
        if let Some(rsk_metrics) = &self.protocol_metrics.rsk_validation {
            self.validate_rsk_metrics(rsk_metrics, report).await?;
        }

        // Validate Rollups metrics
        if let Some(rollups_metrics) = &self.protocol_metrics.rollups_validation {
            self.validate_rollups_metrics(rollups_metrics, report).await?;
        }

        Ok(())
    }

    async fn validate_security_metrics(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate quantum resistance
        self.validate_quantum_resistance(report).await?;
        
        // Validate privacy measures
        self.validate_privacy_measures(report).await?;
        
        // Validate key security
        self.validate_key_security(report).await?;

        Ok(())
    }

    async fn validate_rsk_metrics(&self, metrics: &RSKValidationMetrics, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate merge mining metrics
        if metrics.merge_mining_validation.hashrate_distribution < 0.8 {
            report.add_warning(ValidationWarning {
                category: ValidationCategory::Protocol,
                severity: Severity::High,
                message: "RSK merge mining hashrate distribution is below threshold".to_string(),
            });
        }

        // Validate federation metrics
        if metrics.federation_validation.health_score < 0.9 {
            report.add_alert(ValidationAlert {
                category: ValidationCategory::Protocol,
                severity: Severity::Critical,
                message: "RSK federation health score is critical".to_string(),
            });
        }

        Ok(())
    }

    async fn validate_quantum_resistance(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        let metrics = &self.security_metrics.quantum_resistance;
        
        if metrics.resistance_score < 0.95 {
            report.add_alert(ValidationAlert {
                category: ValidationCategory::Security,
                severity: Severity::Critical,
                message: "Quantum resistance score is below critical threshold".to_string(),
            });
        }

        Ok(())
    }
} 