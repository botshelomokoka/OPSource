use super::build_cli;
use crate::wallet::WalletManager;
use crate::layers::LayerManager;

pub struct CommandHandler {
    wallet_manager: Arc<WalletManager>,
    layer_manager: Arc<LayerManager>,
}

impl CommandHandler {
    pub async fn handle_command(&self, args: &[String]) -> Result<(), CommandError> {
        let matches = build_cli().get_matches_from(args);

        match matches.subcommand() {
            ("tx", Some(tx_matches)) => {
                self.handle_transaction_command(tx_matches).await?
            },
            ("layer", Some(layer_matches)) => {
                self.handle_layer_command(layer_matches).await?
            },
            _ => return Err(CommandError::InvalidCommand),
        }

        Ok(())
    }

    async fn handle_transaction_command(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), CommandError> {
        match matches.subcommand() {
            ("create", Some(create_matches)) => {
                let layer = create_matches.value_of("LAYER").unwrap();
                let params = create_matches.value_of("PARAMS").unwrap();
                
                let tx_params = self.parse_transaction_params(layer, params)?;
                self.wallet_manager
                    .execute_cross_layer_transaction(tx_params, &self.get_context())
                    .await?;
            },
            ("status", Some(status_matches)) => {
                let txid = status_matches.value_of("TXID").unwrap();
                self.check_transaction_status(txid).await?;
            },
            _ => return Err(CommandError::InvalidSubcommand),
        }

        Ok(())
    }

    async fn handle_layer_command(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), CommandError> {
        match matches.subcommand() {
            ("bridge", Some(bridge_matches)) => {
                let from = bridge_matches.value_of("FROM").unwrap();
                let to = bridge_matches.value_of("TO").unwrap();
                let amount = bridge_matches.value_of("AMOUNT").unwrap();
                
                let bridge_params = self.parse_bridge_params(from, to, amount)?;
                self.layer_manager
                    .execute_cross_layer_operation(bridge_params, &self.get_context())
                    .await?;
            },
            _ => return Err(CommandError::InvalidSubcommand),
        }

        Ok(())
    }
} 