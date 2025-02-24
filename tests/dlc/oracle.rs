#[test]
fn test_multi_oracle_verification() {
    let oracles = vec![PublicKey::random(), PublicKey::random()];
    let sigs = oracles.iter().map(|k| k.sign(&msg)).collect();
    
    assert!(verify_outcome(&sigs, &oracles).is_ok());
    assert!(verify_outcome(&sigs[..1], &oracles).is_err());
} 