use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedFinancialMetrics {
    // Core Financial Metrics
    pub volume_metrics: VolumeMetrics,
    pub fee_metrics: FeeMetrics,
    pub performance_metrics: PerformanceMetrics,
    pub risk_metrics: RiskMetrics,
    pub liquidity_metrics: LiquidityMetrics,
    pub efficiency_metrics: EfficiencyMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeMetrics {
    // Transaction Volume Analysis
    pub total_volume_btc: f64,
    pub daily_volume_btc: f64,
    pub hourly_breakdown: Vec<HourlyVolume>,
    pub volume_by_protocol: HashMap<String, f64>,
    pub volume_trends: VolumeTrends,
    pub peak_volume_metrics: PeakVolumeMetrics,
    pub volatility_metrics: VolatilityMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolSpecificValidation {
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
    pub merge_mining_health: MergeMiningHealth,
    pub federation_status: FederationStatus,
    pub peg_mechanism_health: PegMechanismHealth,
    pub smart_contract_validation: SmartContractValidation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeMiningHealth {
    pub hashrate_distribution: f64,
    pub miner_participation: f64,
    pub block_validation_rate: f64,
    pub security_score: f64,
    pub decentralization_index: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollupsValidationMetrics {
    // Optimistic Rollups
    pub optimistic_validation: OptimisticValidation,
    // ZK Rollups
    pub zk_validation: ZKValidation,
    // Common Metrics
    pub data_availability: DataAvailabilityMetrics,
    pub state_validation: StateValidationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityValidationMetrics {
    // Core Security Metrics
    pub quantum_resistance_score: f64,
    pub encryption_strength: f64,
    pub key_security: KeySecurityMetrics,
    pub privacy_score: PrivacyMetrics,
    pub protocol_security: ProtocolSecurityMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivacyMetrics {
    pub transaction_privacy: f64,
    pub metadata_protection: f64,
    pub identity_preservation: f64,
    pub network_privacy: f64,
    pub temporal_analysis_resistance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub timestamp: DateTime<Utc>,
    pub financial_validation: AdvancedFinancialMetrics,
    pub protocol_validation: ProtocolSpecificValidation,
    pub security_validation: SecurityValidationMetrics,
    pub recommendations: Vec<ValidationRecommendation>,
    pub alerts: Vec<ValidationAlert>,
}

impl ValidationReport {
    pub fn generate_alerts(&self) -> Vec<ValidationAlert> {
        let mut alerts = Vec::new();
        
        // Check financial metrics
        if let Some(alert) = self.check_financial_metrics() {
            alerts.push(alert);
        }

        // Check protocol-specific metrics
        if let Some(alert) = self.check_protocol_metrics() {
            alerts.push(alert);
        }

        // Check security metrics
        if let Some(alert) = self.check_security_metrics() {
            alerts.push(alert);
        }

        alerts
    }

    pub fn generate_recommendations(&self) -> Vec<ValidationRecommendation> {
        let mut recommendations = Vec::new();

        // Financial recommendations
        if self.financial_validation.efficiency_metrics.cost_efficiency < 0.8 {
            recommendations.push(ValidationRecommendation {
                category: RecommendationCategory::Financial,
                priority: Priority::High,
                description: "Improve cost efficiency".to_string(),
                action_items: vec![
                    "Optimize batch processing".to_string(),
                    "Review fee structures".to_string(),
                ],
            });
        }

        // Protocol-specific recommendations
        if self.protocol_validation.rsk_validation.merge_mining_health.security_score < 0.9 {
            recommendations.push(ValidationRecommendation {
                category: RecommendationCategory::Protocol,
                priority: Priority::Critical,
                description: "Enhance RSK merge mining security".to_string(),
                action_items: vec![
                    "Increase miner participation".to_string(),
                    "Improve hashrate distribution".to_string(),
                ],
            });
        }

        recommendations
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationAlert {
    pub severity: AlertSeverity,
    pub category: AlertCategory,
    pub description: String,
    pub metrics: HashMap<String, f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationRecommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub description: String,
    pub action_items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertCategory {
    Financial,
    Protocol,
    Security,
    Performance,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Financial,
    Protocol,
    Security,
    Optimization,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
} 