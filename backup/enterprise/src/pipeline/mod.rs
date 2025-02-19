use tokio::sync::mpsc;
use std::sync::Arc;

pub struct TransactionPipeline {
    wallet_manager: Arc<WalletManager>,
    layer_manager: Arc<LayerManager>,
    tx_sender: mpsc::Sender<TransactionEvent>,
    tx_receiver: mpsc::Receiver<TransactionEvent>,
}

impl TransactionPipeline {
    pub async fn process_transaction_stream(&mut self) -> Result<(), PipelineError> {
        while let Some(event) = self.tx_receiver.recv().await {
            match event {
                TransactionEvent::New(tx_params) => {
                    self.handle_new_transaction(tx_params).await?;
                },
                TransactionEvent::LayerBridge(bridge_params) => {
                    self.handle_layer_bridge(bridge_params).await?;
                },
                TransactionEvent::Status(tx_id) => {
                    self.handle_status_check(tx_id).await?;
                },
            }
        }
        Ok(())
    }

    async fn handle_new_transaction(
        &self,
        params: TransactionParams,
    ) -> Result<(), PipelineError> {
        let context = self.create_institutional_context(&params);
        self.wallet_manager
            .execute_cross_layer_transaction(params, &context)
            .await
            .map_err(PipelineError::from)
    }

    async fn handle_layer_bridge(
        &self,
        params: BridgeParams,
    ) -> Result<(), PipelineError> {
        let context = self.create_institutional_context(&params);
        self.layer_manager
            .execute_cross_layer_operation(params.into(), &context)
            .await
            .map_err(PipelineError::from)
    }
} 