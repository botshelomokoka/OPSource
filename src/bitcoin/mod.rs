// Bitcoin module
// Integrates both Python and Rust implementations behind a common interface

pub mod interface;
#[cfg(feature = "python-bitcoin")]
pub mod python;
#[cfg(feature = "rust-bitcoin")]
pub mod rust;

// Re-export the main interface types for convenience
pub use interface::{
    BitcoinInterface, BitcoinError, BitcoinResult, BitcoinTransaction,
    BitcoinAddress, AddressType, TransactionInput, TransactionOutput,
    BlockHeader, BitcoinImplementationType,
    create_bitcoin_interface, get_current_bitcoin_interface
};

/// Module initialization
pub fn init() {
    println!("Initializing Bitcoin module...");
    
    // This will be used to perform any module-level initialization
    #[cfg(feature = "python-bitcoin")]
    println!("Python Bitcoin implementation available");
    
    #[cfg(feature = "rust-bitcoin")]
    println!("Rust Bitcoin implementation available");
} 