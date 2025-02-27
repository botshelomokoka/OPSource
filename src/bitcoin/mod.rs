// Bitcoin module
// Integrates both Python and Rust implementations behind a common interface

pub mod interface;
#[cfg(feature = "python-bitcoin")]
pub mod python;
#[cfg(feature = "rust-bitcoin")]
pub mod rust;
pub mod adapter;
pub mod shadow;
pub mod test;

// Re-export the main interface types for convenience
pub use interface::{
    BitcoinInterface, BitcoinError, BitcoinResult, BitcoinTransaction,
    BitcoinAddress, AddressType, TransactionInput, TransactionOutput,
    BlockHeader, BitcoinImplementationType,
    create_bitcoin_interface, get_current_bitcoin_interface
};
pub use adapter::BitcoinAdapter;
pub use shadow::{ShadowModeImplementation, create_shadow_mode_implementation};

use std::sync::Arc;

/// Module initialization
pub fn init() {
    println!("Initializing Bitcoin module...");
    
    // This will be used to perform any module-level initialization
    #[cfg(feature = "python-bitcoin")]
    println!("Python Bitcoin implementation available");
    
    #[cfg(feature = "rust-bitcoin")]
    println!("Rust Bitcoin implementation available");
}

/// Create a Bitcoin interface with the adapter
pub fn create_bitcoin_interface_with_adapter(config: &crate::config::Config) -> Arc<BitcoinAdapter> {
    let config_arc = Arc::new(config.clone());
    let mut adapter = BitcoinAdapter::new(config_arc);
    
    // Initialize the adapter
    if let Err(e) = adapter.initialize() {
        eprintln!("Warning: Failed to initialize Bitcoin adapter: {:?}", e);
    }
    
    Arc::new(adapter)
}

/// Create a Bitcoin interface in shadow mode
pub fn create_bitcoin_interface_shadow_mode(
    config: &crate::config::Config,
    primary_implementation: BitcoinImplementationType,
    log_file: Option<String>,
    log_all: bool,
) -> BitcoinResult<Arc<dyn BitcoinInterface>> {
    shadow::create_shadow_mode_implementation(config, primary_implementation, log_file, log_all)
} 