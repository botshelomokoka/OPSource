use std::sync::Arc;
use bitcoin::Transaction;
use tokio::sync::RwLock;

pub mod bitcoin_wallet;
pub mod lightning_wallet;
pub mod multi_sig_wallet;
pub mod institutional_wallet;
pub mod layer_wallet;

pub use bitcoin_wallet::BitcoinWallet;
pub use lightning_wallet::LightningWallet;
pub use multi_sig_wallet::MultiSigWallet;
pub use institutional_wallet::InstitutionalWallet;
pub use layer_wallet::LayerWallet;

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Bitcoin error: {0}")]
    BitcoinError(#[from] bitcoin::Error),
    #[error("Lightning error: {0}")]
    LightningError(#[from] lightning::Error),
    #[error("RGB error: {0}")]
    RGBError(#[from] rgb::Error),
    #[error("DLC error: {0}")]
    DLCError(#[from] dlc::Error),
    #[error("Layer error: {0}")]
    LayerError(#[from] crate::layers::LayerError),
}

pub struct WalletManager {
    bitcoin_wallet: Arc<BitcoinWallet>,
    lightning_wallet: Arc<LightningWallet>,
    rgb_module: Arc<RGBModule>,
    dlc_manager: Arc<DLCManager>,
    layer_manager: Arc<LayerManager>,
    institutional_controls: Arc<InstitutionalControls>,
}

impl WalletManager {
    pub async fn execute_cross_layer_transaction(
        &self,
        params: CrossLayerTransactionParams,
        context: &InstitutionalContext,
    ) -> Result<TransactionResult, WalletError> {
        // Validate institutional requirements
        self.institutional_controls.validate_transaction(&params)?;
        
        // Execute the cross-layer transaction
        let result = match params.layer_type {
            LayerType::Lightning => {
                self.lightning_wallet.process_transaction(params).await?
            },
            LayerType::RGB => {
                self.rgb_module.process_transaction(params).await?
            },
            LayerType::DLC => {
                self.dlc_manager.process_transaction(params).await?
            },
            // Add other layer types
        };

        // Log and monitor
        self.log_transaction(&result, context).await?;
        
        Ok(result)
    }
} 