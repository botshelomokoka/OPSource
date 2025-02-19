use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialMetricsDetail {
    // Volume Metrics
    pub total_volume_btc: f64,
    pub daily_volume_btc: f64,
    pub volume_by_protocol: HashMap<String, f64>,
    pub volume_trends: VolumeTrends,
    
    // Fee Analysis
    pub total_fees_btc: f64,
    pub fee_distribution: FeeDistribution,
    pub gas_costs: GasCosts,
    pub fee_efficiency: FeeEfficiency,
    
    // Performance Metrics
    pub transaction_success_rate: f64,
    pub average_confirmation_time: Duration,
    pub throughput: ThroughputMetrics,
    pub cost_efficiency: CostEfficiency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeTrends {
    pub hourly_volumes: Vec<HourlyVolume>,
    pub daily_volumes: Vec<DailyVolume>,
    pub weekly_volumes: Vec<WeeklyVolume>,
    pub monthly_growth_rate: f64,
    pub volume_volatility: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeDistribution {
    pub mining_fees: f64,
    pub protocol_fees: f64,
    pub rollup_fees: f64,
    pub gas_fees: f64,
    pub fee_trends: FeeTrends,
    pub fee_optimization_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasCosts {
    pub total_gas_used: u64,
    pub average_gas_price: f64,
    pub gas_optimization_score: f64,
    pub gas_usage_breakdown: HashMap<String, u64>,
    pub estimated_savings: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    pub transactions_per_second: f64,
    pub peak_tps: f64,
    pub average_tps: f64,
    pub tps_by_protocol: HashMap<String, f64>,
    pub throughput_efficiency: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostEfficiency {
    pub cost_per_transaction: f64,
    pub cost_trends: Vec<CostTrend>,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
    pub efficiency_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolMetricsDetail {
    // RSK Metrics
    pub rsk_metrics: RSKDetailedMetrics,
    // Rollups Metrics
    pub rollups_metrics: RollupsDetailedMetrics,
    // Lightning Metrics
    pub lightning_metrics: LightningDetailedMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RSKDetailedMetrics {
    // Volume and Transaction Metrics
    pub merge_mining_rewards: f64,
    pub peg_in_volume: f64,
    pub peg_out_volume: f64,
    pub smart_contract_interactions: u64,
    
    // Performance Metrics
    pub block_confirmation_time: Duration,
    pub transaction_finality_time: Duration,
    pub network_hashrate: f64,
    
    // Economic Metrics
    pub total_value_locked: f64,
    pub mining_profitability: f64,
    pub fee_revenue: f64,
    
    // Security Metrics
    pub federation_health_score: f64,
    pub merge_mining_security_score: f64,
    pub peg_mechanism_health: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollupsDetailedMetrics {
    // Batch Processing Metrics
    pub batch_size: usize,
    pub batch_processing_time: Duration,
    pub batch_compression_ratio: f64,
    pub batch_efficiency_score: f64,
    
    // Economic Metrics
    pub total_value_locked: f64,
    pub transaction_costs: f64,
    pub fee_savings: f64,
    
    // Performance Metrics
    pub proof_generation_time: Duration,
    pub verification_time: Duration,
    pub data_availability_score: f64,
    
    // Security Metrics
    pub fraud_proof_status: Option<FraudProofStatus>,
    pub challenge_success_rate: Option<f64>,
    pub security_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightningDetailedMetrics {
    // Channel Metrics
    pub total_channels: u64,
    pub active_channels: u64,
    pub channel_capacity: f64,
    pub channel_health: f64,
    
    // Routing Metrics
    pub routing_success_rate: f64,
    pub average_path_length: f64,
    pub routing_fees_earned: f64,
    pub path_finding_efficiency: f64,
    
    // Network Metrics
    pub network_liquidity: f64,
    pub peer_availability: f64,
    pub network_centralization: f64,
    pub network_reliability: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityMetricsDetail {
    pub quantum_resistance_score: f64,
    pub privacy_score: f64,
    pub encryption_strength: f64,
    pub key_security_score: f64,
    pub protocol_security_scores: HashMap<String, f64>,
    pub threat_assessment: ThreatAssessment,
    pub audit_compliance: AuditCompliance,
}

// Additional types for detailed metrics tracking
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub risk_level: RiskLevel,
    pub identified_threats: Vec<Threat>,
    pub mitigation_status: HashMap<String, MitigationStatus>,
    pub security_incidents: Vec<SecurityIncident>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditCompliance {
    pub compliance_score: f64,
    pub audit_findings: Vec<AuditFinding>,
    pub remediation_status: HashMap<String, RemediationStatus>,
    pub last_audit_date: DateTime<Utc>,
} 