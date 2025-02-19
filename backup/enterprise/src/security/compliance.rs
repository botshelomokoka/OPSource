use ring::signature::{Ed25519KeyPair, Signature, KeyPair};
use compliance::ComplianceChecker;

pub struct SecurityManager {
    key_pair: Ed25519KeyPair,
    compliance_checker: ComplianceChecker,
}

impl SecurityManager {
    pub fn new() -> Self {
        let key_pair = Ed25519KeyPair::from_pkcs8(include_bytes!("ed25519_keypair.pk8")).unwrap();
        let compliance_checker = ComplianceChecker::new();

        Self { key_pair, compliance_checker }
    }

    pub fn sign_transaction(&self, transaction: &Transaction) -> Signature {
        self.key_pair.sign(transaction.hash().as_bytes())
    }

    pub async fn monitor_compliance(&self, transaction: &Transaction) -> Result<(), ComplianceError> {
        self.compliance_checker.check_transaction(transaction).await
    }
} 