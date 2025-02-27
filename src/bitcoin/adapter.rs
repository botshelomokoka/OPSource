// src/bitcoin/adapter.rs
//
// This module provides adapter functionality to facilitate the transition
// between Python and Rust Bitcoin implementations.

use std::sync::Arc;
use crate::bitcoin::{
    BitcoinInterface, BitcoinImplementationType, BitcoinResult, BitcoinError,
    BitcoinTransaction, BitcoinAddress, AddressType
};
use crate::config::Config;

/// Adapter for switching between Bitcoin implementations
pub struct BitcoinAdapter {
    /// Current configuration
    config: Arc<Config>,
    
    /// Python implementation (if available)
    #[cfg(feature = "python-bitcoin")]
    python_impl: Option<Arc<dyn BitcoinInterface>>,
    
    /// Rust implementation (if available)
    #[cfg(feature = "rust-bitcoin")]
    rust_impl: Option<Arc<dyn BitcoinInterface>>,
}

impl BitcoinAdapter {
    /// Create a new Bitcoin adapter
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            #[cfg(feature = "python-bitcoin")]
            python_impl: None,
            #[cfg(feature = "rust-bitcoin")]
            rust_impl: None,
        }
    }
    
    /// Initialize the adapter with both implementations
    pub fn initialize(&mut self) -> BitcoinResult<()> {
        // Initialize Python implementation if available
        #[cfg(feature = "python-bitcoin")]
        {
            let python_impl = crate::bitcoin::create_bitcoin_interface(
                BitcoinImplementationType::Python,
                &self.config
            );
            self.python_impl = Some(python_impl);
        }
        
        // Initialize Rust implementation if available
        #[cfg(feature = "rust-bitcoin")]
        {
            let rust_impl = crate::bitcoin::create_bitcoin_interface(
                BitcoinImplementationType::Rust,
                &self.config
            );
            self.rust_impl = Some(rust_impl);
        }
        
        Ok(())
    }
    
    /// Get the current implementation based on configuration
    pub fn get_implementation(&self) -> BitcoinResult<Arc<dyn BitcoinInterface>> {
        match self.config.get_bitcoin_implementation_type() {
            BitcoinImplementationType::Python => {
                #[cfg(feature = "python-bitcoin")]
                {
                    if let Some(impl_ref) = &self.python_impl {
                        return Ok(impl_ref.clone());
                    }
                }
                Err(BitcoinError::Implementation("Python implementation not available".to_string()))
            },
            BitcoinImplementationType::Rust => {
                #[cfg(feature = "rust-bitcoin")]
                {
                    if let Some(impl_ref) = &self.rust_impl {
                        return Ok(impl_ref.clone());
                    }
                }
                Err(BitcoinError::Implementation("Rust implementation not available".to_string()))
            }
        }
    }
    
    /// Run the same operation on both implementations and compare results
    pub fn compare_implementations<T, F>(&self, operation: F) -> BitcoinResult<(T, T)>
    where
        T: std::fmt::Debug + PartialEq,
        F: Fn(&dyn BitcoinInterface) -> BitcoinResult<T>,
    {
        let mut python_result = None;
        let mut rust_result = None;
        
        // Run on Python implementation if available
        #[cfg(feature = "python-bitcoin")]
        {
            if let Some(impl_ref) = &self.python_impl {
                match operation(impl_ref.as_ref()) {
                    Ok(result) => python_result = Some(result),
                    Err(e) => return Err(BitcoinError::Implementation(
                        format!("Python implementation error: {:?}", e)
                    )),
                }
            }
        }
        
        // Run on Rust implementation if available
        #[cfg(feature = "rust-bitcoin")]
        {
            if let Some(impl_ref) = &self.rust_impl {
                match operation(impl_ref.as_ref()) {
                    Ok(result) => rust_result = Some(result),
                    Err(e) => return Err(BitcoinError::Implementation(
                        format!("Rust implementation error: {:?}", e)
                    )),
                }
            }
        }
        
        // Return results if both are available
        match (python_result, rust_result) {
            (Some(p), Some(r)) => Ok((p, r)),
            _ => Err(BitcoinError::Implementation(
                "One or both implementations not available".to_string()
            )),
        }
    }
    
    /// Switch the active implementation
    pub fn switch_implementation(&mut self, implementation_type: BitcoinImplementationType) -> BitcoinResult<()> {
        let mut config = (*self.config).clone();
        config.set_bitcoin_implementation(implementation_type);
        
        // Verify the implementation is available
        match implementation_type {
            BitcoinImplementationType::Python => {
                #[cfg(not(feature = "python-bitcoin"))]
                return Err(BitcoinError::Implementation("Python implementation not available".to_string()));
            },
            BitcoinImplementationType::Rust => {
                #[cfg(not(feature = "rust-bitcoin"))]
                return Err(BitcoinError::Implementation("Rust implementation not available".to_string()));
            }
        }
        
        Ok(())
    }
}

/// Implement BitcoinInterface for the adapter
impl BitcoinInterface for BitcoinAdapter {
    fn get_transaction(&self, txid: &str) -> BitcoinResult<BitcoinTransaction> {
        self.get_implementation()?.get_transaction(txid)
    }
    
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<BitcoinTransaction>> {
        self.get_implementation()?.get_block(hash)
    }
    
    fn get_block_height(&self) -> BitcoinResult<u32> {
        self.get_implementation()?.get_block_height()
    }
    
    fn generate_address(&self, address_type: AddressType) -> BitcoinResult<BitcoinAddress> {
        self.get_implementation()?.generate_address(address_type)
    }
    
    fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<BitcoinTransaction> {
        self.get_implementation()?.create_transaction(outputs, fee_rate)
    }
    
    fn broadcast_transaction(&self, transaction: &BitcoinTransaction) -> BitcoinResult<String> {
        self.get_implementation()?.broadcast_transaction(transaction)
    }
    
    fn get_balance(&self) -> BitcoinResult<u64> {
        self.get_implementation()?.get_balance()
    }
    
    fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64> {
        self.get_implementation()?.estimate_fee(target_blocks)
    }
    
    fn implementation_type(&self) -> BitcoinImplementationType {
        self.config.get_bitcoin_implementation_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adapter_initialization() {
        let config = Arc::new(Config::default());
        let mut adapter = BitcoinAdapter::new(config);
        
        // Initialize should succeed even if not all implementations are available
        assert!(adapter.initialize().is_ok());
    }
    
    #[test]
    fn test_implementation_switching() {
        let config = Arc::new(Config::default());
        let mut adapter = BitcoinAdapter::new(config);
        
        // Initialize the adapter
        adapter.initialize().unwrap();
        
        // Try switching to Rust implementation
        #[cfg(feature = "rust-bitcoin")]
        {
            assert!(adapter.switch_implementation(BitcoinImplementationType::Rust).is_ok());
            assert_eq!(adapter.implementation_type(), BitcoinImplementationType::Rust);
        }
        
        // Try switching to Python implementation
        #[cfg(feature = "python-bitcoin")]
        {
            assert!(adapter.switch_implementation(BitcoinImplementationType::Python).is_ok());
            assert_eq!(adapter.implementation_type(), BitcoinImplementationType::Python);
        }
    }
} 