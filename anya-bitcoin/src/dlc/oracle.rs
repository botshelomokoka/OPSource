pub fn verify_outcome(
  &self,
  signatures: Vec<Signature>,
  public_keys: Vec<PublicKey>
) -> Result<bool> {
  let mut transcript = Transcript::new(b"DLC Outcome");
  let message = self.contract_id.as_ref();
  
  for (sig, pubkey) in signatures.iter().zip(public_keys) {
      verify_schnorr(&pubkey, &mut transcript, sig)
          .context("Invalid oracle signature")?;
  }
  Ok(true)
}

pub fn dispute_period(&self) -> Duration {
  Duration::from_secs(6 * 60 * 60) // 6 hours
}

const REQUIRED_SIGNATURES: usize = 3;
const TOTAL_ORACLES: usize = 5;

pub struct MultiOracleVerifier {
    pub public_keys: [PublicKey; TOTAL_ORACLES],
    hsm_client: Arc<dyn HsmSigner>,
}

impl MultiOracleVerifier {
    pub fn verify_outcome(
        &self,
        signatures: &[Signature]
    ) -> Result<()> {
        let valid_sigs = signatures.iter()
            .filter(|sig| self.public_keys.iter()
                .any(|pk| pk.verify(sig)))
            .count();
            
        if valid_sigs < REQUIRED_SIGNATURES {
            return Err(Error::InsufficientOracleSignatures);
        }
        
        self.hsm_client.audit_log("DLC verification")?;
        Ok(())
    }
} 