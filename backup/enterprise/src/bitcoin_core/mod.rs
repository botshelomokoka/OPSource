use bitcoin::Network;
use bitcoincore_rpc::{Auth, Client, RpcApi};

pub struct BitcoinCoreIntegration {
    network: Network,
    client: Client,
    chain_validator: ChainValidator,
}

impl BitcoinCoreIntegration {
    pub fn new(config: &BitcoinConfig) -> Result<Self, BitcoinError> {
        let client = Client::new(
            &config.rpc_url,
            Auth::UserPass(
                config.rpc_user.clone(),
                config.rpc_password.clone(),
            ),
        )?;

        Ok(Self {
            network: config.network,
            client,
            chain_validator: ChainValidator::new(config.network),
        })
    }

    pub async fn validate_block(&self, block_hash: &bitcoin::BlockHash) -> Result<(), BitcoinError> {
        let block = self.client.get_block(block_hash)?;
        self.chain_validator.validate_block(&block)?;
        Ok(())
    }
} 