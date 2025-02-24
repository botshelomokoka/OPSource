use schnorr_fun::adaptor::{EncryptedSignature, HashTranscript};
use dlc_sled_storage::OracleStorage;

#[derive(Clone)]
pub struct DlcManager {
    hsm_client: Arc<dyn HsmSigner>,
    oracle_store: OracleStorage,
}

impl DlcManager {
    pub async fn sign_oracle_outcome(
        &self,
        contract_id: &ContractId,
        outcome: &str
    ) -> Result<EncryptedSignature> {
        let key_path = self.oracle_store.get_key_path(contract_id)?;
        self.hsm_client.sign_with_path(key_path, outcome)
            .await
            .map_err(Into::into)
    }
} 