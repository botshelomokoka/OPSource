#[tokio::test]
async fn test_key_rotation() {
    let hsm = MockHsm::new();
    let manager = HsmKeyManager::new(hsm.clone());
    
    let initial_key = manager.current_key().await;
    manager.rotate_keys().await;
    
    assert_ne!(manager.current_key().await, initial_key);
    assert_eq!(hsm.rotation_count(), 1);
} 