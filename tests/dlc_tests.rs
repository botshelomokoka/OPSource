#[tokio::test]
async fn test_hsm_signing_flow() {
    let hsm = MockHsm::new();
    let oracle = TestOracle::new("BTC/USD", 50000);
    let dlc = DlcManager::new(hsm.clone(), oracle.store());
    
    let sig = dlc.sign_oracle_outcome(&contract_id, "50000")
        .await
        .expect("HSM signing failed");
    
    assert!(verify_adaptor_sig(
        &sig,
        &contract_params,
        oracle.public_key()
    ));
} 