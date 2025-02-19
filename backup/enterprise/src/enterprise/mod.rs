use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use anya_core::{CoreSystem, config::{SystemConfig, SystemMode}};

pub struct EnterpriseManager {
    core: Arc<CoreSystem>,
    config: EnterpriseConfig,
    state: RwLock<EnterpriseState>,
    mode_handler: Box<dyn ModeHandler>,
}

#[async_trait]
pub trait ModeHandler: Send + Sync {
    async fn handle_transaction(&self, tx: Transaction) -> Result<(), EnterpriseError>;
    async fn scale_resources(&self) -> Result<(), EnterpriseError>;
    async fn collect_metrics(&self) -> Result<SystemMetrics, EnterpriseError>;
}

impl EnterpriseManager {
    pub async fn new(config: EnterpriseConfig) -> Result<Self, EnterpriseError> {
        let core = Arc::new(CoreSystem::new(config.core_config.clone()).await?);
        
        let mode_handler = match config.core_config.mode {
            SystemMode::Lightweight => Box::new(LightweightHandler::new()?),
            SystemMode::Standard => Box::new(StandardHandler::new()?),
            SystemMode::Enterprise => Box::new(EnterpriseHandler::new()?),
            SystemMode::Custom(custom_config) => Box::new(CustomHandler::new(custom_config)?),
        };

        Ok(Self {
            core,
            config,
            state: RwLock::new(EnterpriseState::new()),
            mode_handler,
        })
    }

    pub async fn process_transaction(
        &self,
        tx: Transaction,
    ) -> Result<TransactionResult, EnterpriseError> {
        let _span = info_span!("process_transaction", mode = ?self.config.core_config.mode);
        
        // Mode-specific handling
        self.mode_handler.handle_transaction(&tx).await?;
        
        // Core processing
        let result = self.core.process_transaction(tx).await?;
        
        // Update state
        self.state.write().await.update(&result)?;
        
        Ok(result)
    }

    pub async fn scale_if_needed(&self) -> Result<(), EnterpriseError> {
        if self.config.core_config.scaling.auto_scale {
            self.mode_handler.scale_resources().await?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EnterpriseError {
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    #[error("Rate limit error: {0}")]
    RateLimit(#[from] crate::rate_limiter::RateLimitError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Wallet error: {0}")]
    WalletError(String),
}
