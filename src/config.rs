// Configuration module for OPSource
// Provides configuration settings for various components

/// Main configuration struct for the application
#[derive(Debug, Clone)]
pub struct Config {
    /// Whether to use the Rust Bitcoin implementation (true) or Python (false)
    pub use_rust_bitcoin: bool,
    
    /// Bitcoin network to connect to (mainnet, testnet, regtest)
    pub bitcoin_network: Option<String>,
    
    /// Bitcoin RPC connection URL
    pub bitcoin_rpc_url: Option<String>,
    
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
        
        // Set default feature flags
        features.insert("use_electrum".to_string(), true);
        features.insert("verify_blocks".to_string(), false);
        features.insert("lightning_enabled".to_string(), false);
        
        Config {
            use_rust_bitcoin: true, // Default to Rust implementation
            bitcoin_network: Some("testnet".to_string()),
            bitcoin_rpc_url: Some("http://localhost:18332".to_string()),
            bitcoin_rpc_user: None,
            bitcoin_rpc_pass: None,
            bitcoin_data_dir: None,
            wallet_path: None,
            lightning_implementation: Some("mock".to_string()), // Default to mock implementation
            lightning_node_pubkey: None,
            lightning_listen_addr: Some("0.0.0.0:9735".to_string()),
            lightning_data_dir: None,
            features,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        // Load from environment variables if available
        if let Ok(val) = std::env::var("USE_RUST_BITCOIN") {
            config.use_rust_bitcoin = val.to_lowercase() == "true";
        }
        
        if let Ok(val) = std::env::var("BITCOIN_NETWORK") {
            config.bitcoin_network = Some(val);
        }
        
        if let Ok(val) = std::env::var("BITCOIN_RPC_URL") {
            config.bitcoin_rpc_url = Some(val);
        }
        
        if let Ok(val) = std::env::var("BITCOIN_RPC_USER") {
            config.bitcoin_rpc_user = Some(val);
        }
        
        if let Ok(val) = std::env::var("BITCOIN_RPC_PASS") {
            config.bitcoin_rpc_pass = Some(val);
        }
        
        if let Ok(val) = std::env::var("BITCOIN_DATA_DIR") {
            config.bitcoin_data_dir = Some(val);
        }
        
        if let Ok(val) = std::env::var("WALLET_PATH") {
            config.wallet_path = Some(val);
        }
        
        // Lightning Network configuration
        if let Ok(val) = std::env::var("LIGHTNING_IMPLEMENTATION") {
            config.lightning_implementation = Some(val);
        }
        
        if let Ok(val) = std::env::var("LIGHTNING_NODE_PUBKEY") {
            config.lightning_node_pubkey = Some(val);
        }
        
        if let Ok(val) = std::env::var("LIGHTNING_LISTEN_ADDR") {
            config.lightning_listen_addr = Some(val);
        }
        
        if let Ok(val) = std::env::var("LIGHTNING_DATA_DIR") {
            config.lightning_data_dir = Some(val);
        }
        
        // Feature flags
        if let Ok(val) = std::env::var("LIGHTNING_ENABLED") {
            config.features.insert("lightning_enabled".to_string(), val.to_lowercase() == "true");
        }
        
        config
    }
    
    /// Check if a specific feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }
    
    /// Set a feature flag
    pub fn set_feature(&mut self, feature: &str, enabled: bool) {
        self.features.insert(feature.to_string(), enabled);
    }
    
    /// Get the Lightning implementation type based on configuration
    pub fn get_lightning_implementation_type(&self) -> crate::lightning::interface::LightningImplementationType {
        use crate::lightning::interface::LightningImplementationType;
        
        match self.lightning_implementation.as_deref() {
            Some("ldk") => LightningImplementationType::LDK,
            _ => LightningImplementationType::Mock,
        }
    }

    /// Determine which Bitcoin implementation to use based on configuration
    pub fn get_bitcoin_implementation_type(&self) -> crate::bitcoin::BitcoinImplementationType {
        // Check if we have an explicit setting in the config
        if self.use_rust_bitcoin {
            return crate::bitcoin::BitcoinImplementationType::Rust;
        } else {
            return crate::bitcoin::BitcoinImplementationType::Python;
        }
    }

    /// Check if we should use the Rust Bitcoin implementation
    pub fn uses_rust_bitcoin(&self) -> bool {
        self.use_rust_bitcoin
    }

    /// Set the Bitcoin implementation type
    pub fn set_bitcoin_implementation(&mut self, implementation_type: crate::bitcoin::BitcoinImplementationType) {
        match implementation_type {
            crate::bitcoin::BitcoinImplementationType::Rust => self.use_rust_bitcoin = true,
            crate::bitcoin::BitcoinImplementationType::Python => self.use_rust_bitcoin = false,
        }
    }
}

/// Create a default configuration for testing
pub fn test_config() -> Config {
    Config::default()
} 