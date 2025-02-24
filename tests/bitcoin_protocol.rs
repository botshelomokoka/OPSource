#[test]
fn test_dlc_dispute_flow() {
    let mut oracle = TestOracle::new();
    let (alice, bob) = setup_parties();
    
    // Simulate oracle failure
    oracle.go_offline();
    
    let dispute = alice.init_dispute(bob.id());
    let outcome = dispute.resolve_after_timeout();
    
    assert_eq!(outcome, DisputeResult::MultisigEscrow);
} 