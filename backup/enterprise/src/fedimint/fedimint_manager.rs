use fedimint_client::FedimintClient;
use fedimint_core::config::FederationConfig;
use crate::security::SecurityManager;

pub struct FedimintManager {
    fedimint_client: Arc<FedimintClient>,
    security_manager: Arc<SecurityManager>,
    federation_manager: Arc<FederationManager>,
    mint_monitor: Arc<MintMonitor>,
}

impl FedimintManager {
    pub async fn process_fedimint_operation(
        &self,
        operation: FedimintOperation,
        context: &SecurityContext,
    ) -> Result<FedimintResult, FedimintError> {
        // 1. Security Validation
        self.security_manager
            .validate_fedimint_operation(&operation, context)
            .await?;
        
        // 2. Process Operation
        let result = match operation {
            FedimintOperation::Deposit(deposit) => {
                self.process_deposit(deposit, context).await?
            },
            FedimintOperation::Withdraw(withdraw) => {
                self.process_withdrawal(withdraw, context).await?
            },
            FedimintOperation::Transfer(transfer) => {
                self.process_federation_transfer(transfer, context).await?
            },
            FedimintOperation::Join(join) => {
                self.process_federation_join(join, context).await?
            },
        };
        
        // 3. Monitor Operation
        self.mint_monitor
            .track_operation(&result)
            .await?;

        Ok(result)
    }

    async fn process_deposit(
        &self,
        deposit: DepositRequest,
        context: &SecurityContext,
    ) -> Result<FedimintResult, FedimintError> {
        // Handle deposit to federation
        let deposit_tx = self.fedimint_client
            .create_deposit(deposit.amount)
            .await?;
            
        // Monitor deposit status
        self.monitor_deposit_status(&deposit_tx).await?;

        Ok(FedimintResult::Deposit(deposit_tx))
    }

    async fn process_federation_join(
        &self,
        join: FederationJoinRequest,
        context: &SecurityContext,
    ) -> Result<FedimintResult, FedimintError> {
        // Validate federation config
        self.validate_federation_config(&join.config).await?;
        
        // Join federation
        let join_result = self.federation_manager
            .join_federation(join)
            .await?;

        Ok(FedimintResult::Join(join_result))
    }
} 