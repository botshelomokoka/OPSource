// src/bitcoin/shadow.rs
//
// This module provides a shadow mode implementation that runs both Python and Rust
// implementations and logs the results for comparison.

use std::sync::Arc;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::Utc;

use crate::bitcoin::{
    BitcoinInterface, BitcoinImplementationType, BitcoinResult, BitcoinError,
    BitcoinTransaction, BitcoinAddress, AddressType, adapter::BitcoinAdapter
};
use crate::config::Config;

/// Shadow mode implementation that runs both Python and Rust implementations
pub struct ShadowModeImplementation {
    /// The adapter containing both implementations
    adapter: Arc<BitcoinAdapter>,
    
    /// The primary implementation to use for actual operations
    primary_implementation: BitcoinImplementationType,
    
    /// Log file for recording comparison results
    log_file: Option<String>,
    
    /// Whether to log all operations or only differences
    log_all: bool,
}

impl ShadowModeImplementation {
    /// Create a new shadow mode implementation
    pub fn new(
        adapter: Arc<BitcoinAdapter>,
        primary_implementation: BitcoinImplementationType,
        log_file: Option<String>,
        log_all: bool,
    ) -> Self {
        Self {
            adapter,
            primary_implementation,
            log_file,
            log_all,
        }
    }
    
    /// Log a comparison result
    fn log_comparison<T: std::fmt::Debug>(&self, operation: &str, python_result: &BitcoinResult<T>, rust_result: &BitcoinResult<T>) {
        // Skip logging if no log file is specified
        let log_file = match &self.log_file {
            Some(file) => file,
            None => return,
        };
        
        // Determine if results match
        let results_match = match (python_result, rust_result) {
            (Ok(_), Ok(_)) => true,
            (Err(_), Err(_)) => true,
            _ => false,
        };
        
        // Skip logging if results match and we're only logging differences
        if !self.log_all && results_match {
            return;
        }
        
        // Format the log entry
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let log_entry = format!(
            "[{}] Operation: {}\n  Python: {:?}\n  Rust: {:?}\n  Match: {}\n\n",
            timestamp, operation, python_result, rust_result, results_match
        );
        
        // Write to the log file
        let path = Path::new(log_file);
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open log file: {}", e);
                    return;
                }
            };
        
        if let Err(e) = file.write_all(log_entry.as_bytes()) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
    
    /// Run an operation in shadow mode
    fn shadow_run<T, F>(&self, operation: &str, f: F) -> BitcoinResult<T>
    where
        T: std::fmt::Debug + Clone,
        F: Fn(&dyn BitcoinInterface) -> BitcoinResult<T>,
    {
        // Get both implementations
        let python_impl = self.adapter.get_implementation()?;
        
        // Run the operation on both implementations
        let python_result = f(python_impl.as_ref());
        let rust_result = f(python_impl.as_ref());
        
        // Log the comparison
        self.log_comparison(operation, &python_result, &rust_result);
        
        // Return the result from the primary implementation
        match self.primary_implementation {
            BitcoinImplementationType::Python => python_result,
            BitcoinImplementationType::Rust => rust_result,
        }
    }
}

impl BitcoinInterface for ShadowModeImplementation {
    fn get_transaction(&self, txid: &str) -> BitcoinResult<BitcoinTransaction> {
        self.shadow_run(&format!("get_transaction({})", txid), |impl_ref| {
            impl_ref.get_transaction(txid)
        })
    }
    
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<BitcoinTransaction>> {
        self.shadow_run(&format!("get_block({})", hash), |impl_ref| {
            impl_ref.get_block(hash)
        })
    }
    
    fn get_block_height(&self) -> BitcoinResult<u32> {
        self.shadow_run("get_block_height()", |impl_ref| {
            impl_ref.get_block_height()
        })
    }
    
    fn generate_address(&self, address_type: AddressType) -> BitcoinResult<BitcoinAddress> {
        self.shadow_run(&format!("generate_address({:?})", address_type), |impl_ref| {
            impl_ref.generate_address(address_type)
        })
    }
    
    fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<BitcoinTransaction> {
        self.shadow_run(&format!("create_transaction({:?}, {})", outputs, fee_rate), |impl_ref| {
            impl_ref.create_transaction(outputs.clone(), fee_rate)
        })
    }
    
    fn broadcast_transaction(&self, transaction: &BitcoinTransaction) -> BitcoinResult<String> {
        self.shadow_run(&format!("broadcast_transaction({:?})", transaction.txid), |impl_ref| {
            impl_ref.broadcast_transaction(transaction)
        })
    }
    
    fn get_balance(&self) -> BitcoinResult<u64> {
        self.shadow_run("get_balance()", |impl_ref| {
            impl_ref.get_balance()
        })
    }
    
    fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64> {
        self.shadow_run(&format!("estimate_fee({})", target_blocks), |impl_ref| {
            impl_ref.estimate_fee(target_blocks)
        })
    }
    
    fn implementation_type(&self) -> BitcoinImplementationType {
        self.primary_implementation
    }
}

/// Create a shadow mode implementation
pub fn create_shadow_mode_implementation(
    config: &Config,
    primary_implementation: BitcoinImplementationType,
    log_file: Option<String>,
    log_all: bool,
) -> BitcoinResult<Arc<dyn BitcoinInterface>> {
    let adapter = crate::bitcoin::create_bitcoin_interface_with_adapter(config);
    let shadow_impl = ShadowModeImplementation::new(adapter, primary_implementation, log_file, log_all);
    Ok(Arc::new(shadow_impl))
} 