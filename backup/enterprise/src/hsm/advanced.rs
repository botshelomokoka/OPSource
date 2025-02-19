use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use bitcoin::secp256k1::{Secp256k1, SecretKey};

pub struct AdvancedHSM {
    device_manager: Arc<HSMDeviceManager>,
    key_store: Arc<HSMKeyStore>,
    policy_engine: Arc<PolicyEngine>,
    metrics: HSMMetrics,
    quorum_manager: Arc<QuorumManager>,
    backup_system: Arc<BackupSystem>,
}

impl AdvancedHSM {
    pub async fn new(config: HSMConfig) -> Result<Self, HSMError> {
        Ok(Self {
            device_manager: Arc::new(HSMDeviceManager::new(config.device_config)?),
            key_store: Arc::new(HSMKeyStore::new(config.key_config)?),
            policy_engine: Arc::new(PolicyEngine::new(config.policy_config)?),
            metrics: HSMMetrics::new(),
            quorum_manager: Arc::new(QuorumManager::new(config.quorum_config)?),
            backup_system: Arc::new(BackupSystem::new(config.backup_config)?),
        })
    }

    pub async fn sign_with_quorum(
        &self,
        transaction: &Transaction,
        context: &SigningContext,
    ) -> Result<Signature, HSMError> {
        // Verify quorum requirements
        self.quorum_manager.verify_quorum(context).await?;
        
        // Get key from HSM with enhanced security
        let key = self.key_store
            .get_signing_key_secure(context)
            .await?;
            
        // Sign with HSM
        let signature = self.device_manager
            .sign_with_enhanced_security(&key, transaction)
            .await?;
            
        // Backup signature data
        self.backup_system
            .backup_signature_data(transaction, &signature, context)
            .await?;
            
        Ok(signature)
    }
} 