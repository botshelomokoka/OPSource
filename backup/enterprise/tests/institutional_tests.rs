#[tokio::test]
async fn test_institutional_transaction_flow() {
    let config = InstitutionalConfig::default();
    let wallet = InstitutionalWallet::new(config).await?;
    
    // Test transaction creation with institutional policies
    let tx = wallet
        .create_institutional_transaction(
            test_inputs(),
            test_outputs(),
            &test_policy(),
        )
        .await?;
        
    // Verify compliance
    let compliance_result = wallet
        .security
        .compliance_manager
        .validate_transaction(&tx, &test_context())
        .await;
        
    assert!(compliance_result.is_ok());
} 