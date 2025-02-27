// Configuration module for OPSource
// Provides configuration settings for various components

/// Main configuration struct for the application
#[derive(Debug, Clone)]
pub struct Config {
    /// Bitcoin network to connect to (mainnet, testnet, regtest)
    pub bitcoin_network: String,
    
    /// Bitcoin RPC connection URL
    pub bitcoin_rpc_url: String,
    
    /// Bitcoin RPC username
    pub bitcoin_rpc_user: Option<String>,
    
    /// Bitcoin RPC password
    pub bitcoin_rpc_pass: Option<String>,
    
    /// Path to Bitcoin data directory
    pub bitcoin_data_dir: Option<String>,
    
    /// Path to wallet file
    pub wallet_path: Option<String>,
    
    /// Lightning implementation type (ldk or mock)
    pub lightning_implementation: Option<String>,
    
    /// Lightning Network node public key (if already known)
    pub lightning_node_pubkey: Option<String>,
    
    /// Lightning Network listening address and port
    pub lightning_listen_addr: Option<String>,
    
    /// Lightning Network data directory
    pub lightning_data_dir: Option<String>,
    
    /// Feature flags for various components
    pub features: std::collections::HashMap<String, bool>,
}

impl Default for Config {
    fn default() -> Self {
        let mut features = std::collections::HashMap::new();
        features.insert("taproot".to_string(), true);
        features.insert("lightning".to_string(), false);
        features.insert("dlc".to_string(), false);
        
        Self {
            bitcoin_network: "testnet".to_string(),
            bitcoin_rpc_url: "http://localhost:18332".to_string(),
            bitcoin_rpc_user: None,
            bitcoin_rpc_pass: None,
            bitcoin_data_dir: None,
            wallet_path: None,
            lightning_implementation: Some("ldk".to_string()),
            lightning_node_pubkey: None,
            lightning_listen_addr: None,
            lightning_data_dir: None,
            features,
        }
    }
}

impl Config {
    /// Create a configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        // Bitcoin configuration
        if let Ok(network) = std::env::var("BITCOIN_NETWORK") {
            config.bitcoin_network = network;
        }
        
        if let Ok(rpc_url) = std::env::var("BITCOIN_RPC_URL") {
            config.bitcoin_rpc_url = rpc_url;
        }
        
        if let Ok(rpc_user) = std::env::var("BITCOIN_RPC_USER") {
            config.bitcoin_rpc_user = Some(rpc_user);
        }
        
        if let Ok(rpc_pass) = std::env::var("BITCOIN_RPC_PASS") {
            config.bitcoin_rpc_pass = Some(rpc_pass);
        }
        
        if let Ok(data_dir) = std::env::var("BITCOIN_DATA_DIR") {
            config.bitcoin_data_dir = Some(data_dir);
        }
        
        if let Ok(wallet_path) = std::env::var("WALLET_PATH") {
            config.wallet_path = Some(wallet_path);
        }
        
        // Lightning configuration
        if let Ok(lightning_impl) = std::env::var("LIGHTNING_IMPLEMENTATION") {
            config.lightning_implementation = Some(lightning_impl);
        }
        
        if let Ok(lightning_pubkey) = std::env::var("LIGHTNING_NODE_PUBKEY") {
            config.lightning_node_pubkey = Some(lightning_pubkey);
        }
        
        if let Ok(lightning_addr) = std::env::var("LIGHTNING_LISTEN_ADDR") {
            config.lightning_listen_addr = Some(lightning_addr);
        }
        
        if let Ok(lightning_dir) = std::env::var("LIGHTNING_DATA_DIR") {
            config.lightning_data_dir = Some(lightning_dir);
        }
        
        // Feature flags
        if let Ok(features_str) = std::env::var("ENABLED_FEATURES") {
            for feature in features_str.split(',') {
                let feature = feature.trim();
                if !feature.is_empty() {
                    config.features.insert(feature.to_string(), true);
                }
            }
        }
        
        config
    }
    
    /// Check if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }
    
    /// Set a feature flag
    pub fn set_feature(&mut self, feature: &str, enabled: bool) {
        self.features.insert(feature.to_string(), enabled);
    }
    
    /// Get the Lightning implementation type
    pub fn get_lightning_implementation_type(&self) -> crate::lightning::interface::LightningImplementationType {
        match self.lightning_implementation.as_deref() {
            Some("ldk") => crate::lightning::interface::LightningImplementationType::LDK,
            Some("mock") => crate::lightning::interface::LightningImplementationType::Mock,
            _ => crate::lightning::interface::LightningImplementationType::LDK,
        }
    }
    
    /// Get the Bitcoin implementation type
    pub fn get_bitcoin_implementation_type(&self) -> crate::bitcoin::interface::BitcoinImplementationType {
        crate::bitcoin::interface::BitcoinImplementationType::Rust
    }
}

/// Create a test configuration for unit tests
pub fn test_config() -> Config {
    let mut config = Config::default();
    config.bitcoin_network = "regtest".to_string();
    config.bitcoin_rpc_url = "http://localhost:18443".to_string();
    config
} 