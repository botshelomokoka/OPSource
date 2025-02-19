use crate::wallet::{
    EnterpriseWallet,
    EnterpriseLiquidWallet,
    EnterpriseTaroIntegration,
    EnterpriseMLWallet,
};

pub struct WalletAlignment {
    enterprise_wallet: Arc<EnterpriseWallet>,
    liquid_wallet: Arc<EnterpriseLiquidWallet>,
    taro_integration: Arc<EnterpriseTaroIntegration>,
    ml_wallet: Arc<EnterpriseMLWallet>,
}

impl WalletAlignment {
    pub async fn align_wallet_operations(
        &self,
        operation: WalletOperation,
        context: &SecurityContext,
    ) -> Result<WalletResult, WalletError> {
        // 1. Validate Operation
        self.validate_wallet_operation(&operation, context).await?;
        
        // 2. Process Based on Type
        let result = match operation {
            WalletOperation::Bitcoin(tx) => {
                self.enterprise_wallet
                    .process_enterprise_transaction(tx, context)
                    .await?
            },
            WalletOperation::Liquid(tx) => {
                self.liquid_wallet
                    .process_enterprise_liquid_transaction(tx, context)
                    .await?
            },
            WalletOperation::Taro(tx) => {
                self.taro_integration
                    .process_taro_transaction(tx, context)
                    .await?
            },
            WalletOperation::ML(tx) => {
                self.ml_wallet
                    .process_enterprise_ml_transaction(tx, context)
                    .await?
            },
        };
        
        // 3. Align State
        self.align_wallet_state(&result).await?;

        Ok(result)
    }

    async fn align_wallet_state(
        &self,
        result: &WalletResult,
    ) -> Result<(), WalletError> {
        // Align states across wallets
        self.align_balance_states(result).await?;
        self.align_transaction_histories(result).await?;
        self.align_asset_states(result).await?;
        self.align_ml_states(result).await?;

        Ok(())
    }
} 