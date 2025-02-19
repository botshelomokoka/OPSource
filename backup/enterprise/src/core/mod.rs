use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AnyaCore {
    bitcoin_core: Arc<BitcoinCore>,
    lightning_node: Arc<LightningNode>,
    dlc_manager: Arc<DLCManager>,
    rgb_core: Arc<RGBCore>,
    stacks_core: Arc<StacksCore>,
    rsk_core: Arc<RSKCore>,
    
    // Core services
    security_manager: Arc<SecurityManager>,
    policy_engine: Arc<PolicyEngine>,
    custody_manager: Arc<CustodyManager>,
    metrics_system: Arc<MetricsSystem>,
}

impl AnyaCore {
    pub async fn new(config: CoreConfig) -> Result<Self, CoreError> {
        let bitcoin_core = Arc::new(BitcoinCore::new(config.bitcoin_config).await?);
        let lightning_node = Arc::new(LightningNode::new(config.lightning_config).await?);
        
        Ok(Self {
            bitcoin_core,
            lightning_node,
            dlc_manager: Arc::new(DLCManager::new(config.dlc_config).await?),
            rgb_core: Arc::new(RGBCore::new(config.rgb_config).await?),
            stacks_core: Arc::new(StacksCore::new(config.stacks_config).await?),
            rsk_core: Arc::new(RSKCore::new(config.rsk_config).await?),
            security_manager: Arc::new(SecurityManager::new(config.security_config).await?),
            policy_engine: Arc::new(PolicyEngine::new(config.policy_config).await?),
            custody_manager: Arc::new(CustodyManager::new(config.custody_config).await?),
            metrics_system: Arc::new(MetricsSystem::new(config.metrics_config).await?),
        })
    }
} 