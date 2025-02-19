use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedValidationMetrics {
    // Core Financial Validation
    pub financial_metrics: FinancialValidationMetrics,
    // Protocol-Specific Validation
    pub protocol_metrics: ProtocolValidationMetrics,
    // Security Validation
    pub security_metrics: SecurityValidationMetrics,
    // Compliance Validation
    pub compliance_metrics: ComplianceValidationMetrics,
    // New: Performance Validation
    pub performance_metrics: PerformanceValidationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialValidationMetrics {
    // Transaction Metrics
    pub transaction_volume: TransactionVolumeMetrics,
    pub fee_analysis: FeeAnalysisMetrics,
    pub liquidity_metrics: LiquidityMetrics,
    // New: Risk Metrics
    pub risk_assessment: RiskAssessmentMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAssessmentMetrics {
    pub volatility_index: f64,
    pub exposure_metrics: ExposureMetrics,
    pub counterparty_risk: CounterpartyRiskMetrics,
    pub market_risk: MarketRiskMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolValidationMetrics {
    // RSK Validation
    pub rsk_metrics: RSKValidationMetrics,
    // Rollups Validation
    pub rollups_metrics: RollupsValidationMetrics,
    // Lightning Validation
    pub lightning_metrics: LightningValidationMetrics,
    // New: Cross-Protocol Metrics
    pub cross_protocol_metrics: CrossProtocolMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityValidationMetrics {
    // Core Security
    pub quantum_resistance: QuantumResistanceMetrics,
    pub encryption_strength: EncryptionMetrics,
    pub key_security: KeySecurityMetrics,
    // New: Advanced Security
    pub zero_knowledge_proofs: ZKProofMetrics,
    pub privacy_preservation: PrivacyMetrics,
    pub network_security: NetworkSecurityMetrics,
}

impl EnhancedValidationMetrics {
    pub async fn validate_all(&self) -> Result<ValidationReport, ValidationError> {
        let mut report = ValidationReport::new();

        // Validate financial metrics
        self.validate_financial_metrics(&mut report).await?;
        
        // Validate protocol metrics
        self.validate_protocol_metrics(&mut report).await?;
        
        // Validate security metrics
        self.validate_security_metrics(&mut report).await?;
        
        // New: Validate cross-protocol interactions
        self.validate_cross_protocol_metrics(&mut report).await?;

        Ok(report)
    }

    async fn validate_cross_protocol_metrics(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate interactions between different protocols
        self.validate_rsk_rollups_interaction(report).await?;
        self.validate_lightning_rollups_interaction(report).await?;
        self.validate_rsk_lightning_interaction(report).await?;
        Ok(())
    }

    async fn validate_rsk_rollups_interaction(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate RSK and Rollups interaction
        if let Some(interaction_metrics) = &self.protocol_metrics.cross_protocol_metrics.rsk_rollups {
            if interaction_metrics.compatibility_score < 0.95 {
                report.add_warning(ValidationWarning {
                    category: ValidationCategory::Protocol,
                    severity: Severity::High,
                    message: "RSK-Rollups compatibility score below threshold".to_string(),
                });
            }
        }
        Ok(())
    }

    async fn validate_security_metrics(&self, report: &mut ValidationReport) -> Result<(), ValidationError> {
        // Validate quantum resistance
        if self.security_metrics.quantum_resistance.resistance_score < 0.98 {
            report.add_alert(ValidationAlert {
                category: ValidationCategory::Security,
                severity: Severity::Critical,
                message: "Quantum resistance score critical".to_string(),
            });
        }

        // Validate privacy preservation
        if self.security_metrics.privacy_preservation.privacy_score < 0.95 {
            report.add_alert(ValidationAlert {
                category: ValidationCategory::Security,
                severity: Severity::High,
                message: "Privacy preservation score below threshold".to_string(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossProtocolMetrics {
    pub rsk_rollups: Option<ProtocolInteractionMetrics>,
    pub lightning_rollups: Option<ProtocolInteractionMetrics>,
    pub rsk_lightning: Option<ProtocolInteractionMetrics>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolInteractionMetrics {
    pub compatibility_score: f64,
    pub interaction_latency: Duration,
    pub success_rate: f64,
    pub error_rate: f64,
} 