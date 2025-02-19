use proptest::prelude::*;
use anya_enterprise::{
    wallet::WalletManager,
    security::SecuritySystem,
    transaction::TransactionProcessor,
};

proptest! {
    #[test]
    fn test_transaction_validation(
        amount in 1_000..1_000_000_000u64,
        inputs in 1..10usize,
        outputs in 1..10usize,
    ) {
        let tx = generate_test_transaction(amount, inputs, outputs);
        let security = SecuritySystem::new(Default::default());
        
        prop_assert!(security.validate_transaction(&tx).is_ok());
    }

    #[test]
    fn test_institutional_policies(
        tx_amount in 1_000..1_000_000_000u64,
        policy_limit in 1_000..2_000_000_000u64,
    ) {
        let policy = InstitutionalPolicy::new(policy_limit);
        let tx = generate_test_transaction(tx_amount, 1, 1);
        
        prop_assert_eq!(
            policy.validate_transaction(&tx).is_ok(),
            tx_amount <= policy_limit
        );
    }
} 