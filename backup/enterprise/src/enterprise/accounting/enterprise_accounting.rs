use crate::enterprise::auth::{EnterpriseAuthToken, EnterprisePermissions};
use crate::layer2::security::protocol_monitor::ProtocolMonitor;
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Debug)]
pub struct EnterpriseAccounting {
    protocol_monitor: Arc<ProtocolMonitor>,
    metrics_collector: Arc<MetricsCollector>,
    security_monitor: Arc<SecurityMonitor>,
}

#[derive(Debug, Serialize)]
pub struct AccountingMetrics {
    // Core Metrics
    pub timestamp: DateTime<Utc>,
    pub transaction_volume: f64,
    pub fee_metrics: FeeMetrics,
    
    // Layer-Specific Metrics
    pub layer1_metrics: Layer1Metrics,
    pub layer2_metrics: Layer2Metrics,
    
    // Protocol-Specific Accounting
    pub rsk_accounting: Option<RSKAccountingMetrics>,
    pub rollups_accounting: Option<RollupsAccountingMetrics>,
    pub lightning_accounting: Option<LightningAccountingMetrics>,
}

#[derive(Debug, Serialize)]
pub struct Layer2Metrics {
    // RSK Metrics
    pub rsk_volume: f64,
    pub rsk_fees: f64,
    pub rsk_peg_in_out: PegMetrics,
    
    // Rollups Metrics
    pub optimistic_volume: f64,
    pub zk_volume: f64,
    pub rollups_fees: RollupsFees,
    
    // Lightning Metrics
    pub lightning_volume: f64,
    pub channel_fees: f64,
    pub routing_fees: f64,
}

impl EnterpriseAccounting {
    pub async fn record_transaction(&self, 
        auth_token: &EnterpriseAuthToken,
        transaction: &Transaction
    ) -> Result<(), AccountingError> {
        // Verify permissions
        self.verify_accounting_permissions(auth_token, transaction)?;
        
        // Record based on transaction type
        match transaction.protocol_type {
            ProtocolType::RSK => {
                if auth_token.permissions.can_use_rsk {
                    self.record_rsk_transaction(transaction).await?;
                }
            },
            ProtocolType::Rollups(rollup_type) => {
                if auth_token.permissions.can_use_rollups {
                    match rollup_type {
                        RollupType::Optimistic => {
                            self.record_optimistic_transaction(transaction).await?;
                        },
                        RollupType::ZK => {
                            self.record_zk_transaction(transaction).await?;
                        }
                    }
                }
            },
            ProtocolType::Lightning => {
                self.record_lightning_transaction(transaction).await?;
            }
        }

        // Update security metrics
        self.update_security_metrics(transaction).await?;
        
        Ok(())
    }

    async fn record_rsk_transaction(&self, transaction: &Transaction) -> Result<(), AccountingError> {
        let metrics = RSKAccountingMetrics {
            merge_mining_fees: self.calculate_merge_mining_fees(transaction)?,
            federation_fees: self.calculate_federation_fees(transaction)?,
            peg_metrics: self.calculate_peg_metrics(transaction)?,
            gas_usage: self.calculate_gas_usage(transaction)?,
        };
        
        self.metrics_collector.record_rsk_metrics(metrics).await?;
        Ok(())
    }

    async fn record_zk_transaction(&self, transaction: &Transaction) -> Result<(), AccountingError> {
        let metrics = ZKAccountingMetrics {
            proof_generation_cost: self.calculate_proof_cost(transaction)?,
            verification_cost: self.calculate_verification_cost(transaction)?,
            data_availability_cost: self.calculate_data_cost(transaction)?,
        };
        
        self.metrics_collector.record_zk_metrics(metrics).await?;
        Ok(())
    }

    async fn generate_accounting_report(&self, 
        auth_token: &EnterpriseAuthToken,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    ) -> Result<AccountingReport, AccountingError> {
        // Verify reporting permissions
        if !auth_token.permissions.can_access_accounting_reports {
            return Err(AccountingError::PermissionDenied);
        }

        let report = AccountingReport {
            period_start: start_time,
            period_end: end_time,
            layer1_metrics: self.collect_layer1_metrics(start_time, end_time).await?,
            layer2_metrics: self.collect_layer2_metrics(start_time, end_time).await?,
            security_metrics: self.collect_security_metrics(start_time, end_time).await?,
        };

        Ok(report)
    }

    async fn update_security_metrics(&self, transaction: &Transaction) -> Result<(), AccountingError> {
        self.security_monitor.record_transaction_security_metrics(transaction).await?;
        Ok(())
    }
} 