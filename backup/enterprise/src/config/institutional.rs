use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionalConfig {
    pub custody: CustodyConfig,
    pub compliance: ComplianceConfig,
    pub policy: PolicyConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustodyConfig {
    pub multi_sig_threshold: u8,
    pub key_derivation_scheme: KeyDerivationScheme,
    pub backup_policy: BackupPolicy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub transaction_limits: TransactionLimits,
    pub required_approvals: u8,
    pub audit_level: AuditLevel,
} 