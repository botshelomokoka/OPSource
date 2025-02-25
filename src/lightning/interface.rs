// Lightning Network Interface Layer
// Provides a unified interface for Lightning Network operations.
//
// This module implements the "ports and adapters" pattern from hexagonal architecture,
// allowing different implementations to be swapped while maintaining a consistent API.

use std::sync::Arc;
use crate::bitcoin::BitcoinResult;

/// Lightning implementation type selection enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightningImplementationType {
    /// Use LDK implementation
    LDK,
    /// Mock implementation for testing
    Mock,
}

/// Common error type for Lightning operations
#[derive(Debug, thiserror::Error)]
pub enum LightningError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Channel error: {0}")]
    ChannelError(String),
    
    #[error("Payment error: {0}")]
    PaymentError(String),
    
    #[error("Invoice error: {0}")]
    InvoiceError(String),
    
    #[error("Implementation error: {0}")]
    ImplementationError(String),
    
    #[error("Bitcoin error: {0}")]
    BitcoinError(#[from] crate::bitcoin::BitcoinError),
}

/// Result type for Lightning operations
pub type LightningResult<T> = Result<T, LightningError>;

/// Lightning Network node information
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Node public key
    pub pubkey: String,
    /// Network addresses (host:port)
    pub addresses: Vec<String>,
    /// Node alias (name)
    pub alias: Option<String>,
    /// Color of the node (hex)
    pub color: Option<String>,
    /// Node features
    pub features: Vec<String>,
}

/// Lightning Network channel information
#[derive(Debug, Clone)]
pub struct ChannelInfo {
    /// Channel ID
    pub channel_id: String,
    /// Funding transaction ID
    pub funding_txid: String,
    /// Funding output index
    pub funding_output_idx: u32,
    /// Channel capacity in satoshis
    pub capacity: u64,
    /// Local balance in satoshis
    pub local_balance: u64,
    /// Remote balance in satoshis
    pub remote_balance: u64,
    /// Remote node public key
    pub remote_pubkey: String,
    /// Whether the channel is active
    pub is_active: bool,
    /// Whether the channel is public
    pub is_public: bool,
    /// Short channel ID (once confirmed)
    pub short_channel_id: Option<String>,
}

/// Lightning Network invoice
#[derive(Debug, Clone)]
pub struct Invoice {
    /// BOLT-11 invoice string
    pub bolt11: String,
    /// Payment hash
    pub payment_hash: String,
    /// Description
    pub description: String,
    /// Amount in millisatoshis
    pub amount_msat: Option<u64>,
    /// Expiry time in seconds from creation
    pub expiry: u32,
    /// Creation timestamp
    pub timestamp: u64,
    /// Minimum final CLTV expiry delta
    pub min_final_cltv_expiry: u32,
}

/// Lightning Network payment information
#[derive(Debug, Clone)]
pub struct PaymentInfo {
    /// Payment ID
    pub payment_id: String,
    /// Payment hash
    pub payment_hash: String,
    /// Payment preimage (if payment is complete)
    pub preimage: Option<String>,
    /// Amount in millisatoshis
    pub amount_msat: u64,
    /// Fee paid in millisatoshis
    pub fee_msat: u64,
    /// Payment status
    pub status: PaymentStatus,
    /// Creation timestamp
    pub created_at: u64,
    /// Resolved timestamp (if complete or failed)
    pub resolved_at: Option<u64>,
    /// Payment description or purpose
    pub description: Option<String>,
}

/// Payment status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    /// Payment is in progress
    Pending,
    /// Payment succeeded
    Succeeded,
    /// Payment failed
    Failed,
}

/// Common interface for Lightning Network operations
pub trait LightningInterface: Send + Sync {
    /// Get information about the local node
    fn get_node_info(&self) -> LightningResult<NodeInfo>;
    
    /// Connect to a remote node
    fn connect_peer(&self, node_pubkey: &str, host: &str, port: u16) -> LightningResult<()>;
    
    /// List connected peers
    fn list_peers(&self) -> LightningResult<Vec<NodeInfo>>;
    
    /// Open a channel with a peer
    fn open_channel(
        &self,
        node_pubkey: &str,
        capacity: u64,
        push_msat: Option<u64>,
        is_private: bool,
    ) -> LightningResult<ChannelInfo>;
    
    /// List all channels
    fn list_channels(&self) -> LightningResult<Vec<ChannelInfo>>;
    
    /// Close a channel
    fn close_channel(&self, channel_id: &str, force: bool) -> LightningResult<String>;
    
    /// Create an invoice
    fn create_invoice(
        &self,
        amount_msat: Option<u64>,
        description: &str,
        expiry: Option<u32>,
    ) -> LightningResult<Invoice>;
    
    /// Pay an invoice
    fn pay_invoice(&self, bolt11: &str, amount_msat: Option<u64>) -> LightningResult<PaymentInfo>;
    
    /// Decode an invoice
    fn decode_invoice(&self, bolt11: &str) -> LightningResult<Invoice>;
    
    /// Get a payment by hash
    fn get_payment(&self, payment_hash: &str) -> LightningResult<Option<PaymentInfo>>;
    
    /// List all payments
    fn list_payments(&self) -> LightningResult<Vec<PaymentInfo>>;
    
    /// Implementation type
    fn implementation_type(&self) -> LightningImplementationType;
}

/// Factory to create the appropriate Lightning implementation
pub fn create_lightning_interface(
    implementation_type: LightningImplementationType,
    config: &crate::config::Config,
    bitcoin_interface: Arc<dyn crate::bitcoin::BitcoinInterface>,
) -> Arc<dyn LightningInterface> {
    match implementation_type {
        #[cfg(feature = "ldk")]
        LightningImplementationType::LDK => {
            use crate::lightning::ldk::LdkLightningImplementation;
            Arc::new(LdkLightningImplementation::new(config, bitcoin_interface))
        }
        
        #[cfg(feature = "mock-lightning")]
        LightningImplementationType::Mock => {
            use crate::lightning::mock::MockLightningImplementation;
            Arc::new(MockLightningImplementation::new(config, bitcoin_interface))
        }
        
        #[cfg(not(feature = "ldk"))]
        LightningImplementationType::LDK => {
            eprintln!("Warning: LDK Lightning implementation requested but not available. Falling back to Mock implementation.");
            use crate::lightning::mock::MockLightningImplementation;
            Arc::new(MockLightningImplementation::new(config, bitcoin_interface))
        }
        
        #[cfg(not(feature = "mock-lightning"))]
        LightningImplementationType::Mock => {
            eprintln!("Warning: Mock Lightning implementation requested but not available. Falling back to LDK implementation.");
            use crate::lightning::ldk::LdkLightningImplementation;
            Arc::new(LdkLightningImplementation::new(config, bitcoin_interface))
        }
    }
}

/// Get the current default Lightning implementation
pub fn get_current_lightning_interface(
    config: &crate::config::Config,
    bitcoin_interface: Arc<dyn crate::bitcoin::BitcoinInterface>,
) -> Arc<dyn LightningInterface> {
    // Check if the LDK implementation is available and preferred
    #[cfg(feature = "ldk")]
    {
        return create_lightning_interface(LightningImplementationType::LDK, config, bitcoin_interface);
    }
    
    // Fallback to Mock implementation
    #[cfg(feature = "mock-lightning")]
    {
        return create_lightning_interface(LightningImplementationType::Mock, config, bitcoin_interface);
    }
    
    // If neither is available, panic
    #[cfg(not(any(feature = "ldk", feature = "mock-lightning")))]
    {
        panic!("No Lightning implementation available!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    
    #[test]
    fn test_interface_creation() {
        let config = config::Config::default();
        let bitcoin_interface = crate::bitcoin::get_current_bitcoin_interface(&config);
        
        // This will use whatever implementation is available based on features
        let _lightning = get_current_lightning_interface(&config, bitcoin_interface);
    }
} 