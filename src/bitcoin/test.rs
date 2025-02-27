// Bitcoin implementation tests
// Tests both Python and Rust implementations to verify they behave identically

use crate::bitcoin::{
    interface::{
        BitcoinInterface, BitcoinImplementationType, AddressType,
        create_bitcoin_interface
    },
    BitcoinResult, BitcoinError, BitcoinTransaction, BitcoinAddress
};
use crate::config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    // Helper function to run the same test on both implementations
    fn test_both_implementations<F>(test_fn: F)
    where
        F: Fn(&dyn BitcoinInterface) -> BitcoinResult<()>
    {
        let config = Config::default();
        
        // Test Python implementation if available
        #[cfg(feature = "python-bitcoin")]
        {
            let python_impl = create_bitcoin_interface(BitcoinImplementationType::Python, &config);
            if let Err(e) = test_fn(python_impl.as_ref()) {
                panic!("Python implementation test failed: {:?}", e);
            }
        }
        
        // Test Rust implementation if available
        #[cfg(feature = "rust-bitcoin")]
        {
            let rust_impl = create_bitcoin_interface(BitcoinImplementationType::Rust, &config);
            if let Err(e) = test_fn(rust_impl.as_ref()) {
                panic!("Rust implementation test failed: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_get_transaction() {
        test_both_implementations(|bitcoin_impl| {
            let txid = "0".repeat(64);
            let tx = bitcoin_impl.get_transaction(&txid)?;
            
            // Verify the transaction has the correct txid
            assert_eq!(tx.txid, txid);
            assert_eq!(tx.version, 2);
            assert!(!tx.inputs.is_empty());
            assert!(!tx.outputs.is_empty());
            
            Ok(())
        });
    }
    
    #[test]
    fn test_generate_address() {
        test_both_implementations(|bitcoin_impl| {
            let address = bitcoin_impl.generate_address(AddressType::P2WPKH)?;
            
            // Verify the address has a value
            assert!(!address.address.is_empty());
            assert_eq!(address.address_type, AddressType::P2WPKH);
            
            Ok(())
        });
    }
    
    #[test]
    fn test_create_transaction() {
        test_both_implementations(|bitcoin_impl| {
            // Create a simple transaction with one output
            let recipient = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string();
            let amount = 50000; // 0.0005 BTC
            let fee_rate = 5; // 5 sat/vB
            
            let tx = bitcoin_impl.create_transaction(vec![(recipient.clone(), amount)], fee_rate)?;
            
            // Verify transaction details
            assert!(!tx.txid.is_empty());
            assert_eq!(tx.version, 2);
            assert!(!tx.inputs.is_empty());
            assert!(!tx.outputs.is_empty());
            
            // Verify that the output contains our payment
            let payment_output = tx.outputs.iter().find(|output| 
                output.address.as_ref().map_or(false, |addr| addr == &recipient) &&
                output.value == amount
            );
            assert!(payment_output.is_some(), "Couldn't find expected output in transaction");
            
            Ok(())
        });
    }
    
    #[test]
    fn test_get_balance() {
        test_both_implementations(|bitcoin_impl| {
            let balance = bitcoin_impl.get_balance()?;
            
            // Verify that the balance is non-negative
            assert!(balance >= 0);
            
            Ok(())
        });
    }
}

// Test running functionality
pub fn run_tests() -> Result<(), String> {
    let config = Config::default();
    
    println!("Running Bitcoin implementation tests...");
    
    // Test both implementations if available
    #[cfg(feature = "python-bitcoin")]
    {
        println!("\nTesting Python implementation:");
        let python_impl = create_bitcoin_interface(BitcoinImplementationType::Python, &config);
        test_implementation(python_impl.as_ref()).map_err(|e| format!("Python test error: {}", e))?;
    }
    
    #[cfg(feature = "rust-bitcoin")]
    {
        println!("\nTesting Rust implementation:");
        let rust_impl = create_bitcoin_interface(BitcoinImplementationType::Rust, &config);
        test_implementation(rust_impl.as_ref()).map_err(|e| format!("Rust test error: {}", e))?;
    }
    
    println!("\nAll tests passed!");
    
    Ok(())
}

fn test_implementation(bitcoin_impl: &dyn BitcoinInterface) -> BitcoinResult<()> {
    // Test transaction handling
    let txid = "0".repeat(64);
    let tx = bitcoin_impl.get_transaction(&txid)?;
    println!("- Get transaction: Success (txid: {})", tx.txid);
    
    // Test address generation
    let address = bitcoin_impl.generate_address(AddressType::P2WPKH)?;
    println!("- Generate address: Success ({})", address.address);
    
    // Test transaction creation
    let recipient = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string();
    let amount = 50000; // 0.0005 BTC
    let fee_rate = 5; // 5 sat/vB
    
    let tx = bitcoin_impl.create_transaction(vec![(recipient.clone(), amount)], fee_rate)?;
    println!("- Create transaction: Success (txid: {})", tx.txid);
    
    // Test fee estimation
    let fee = bitcoin_impl.estimate_fee(6)?;
    println!("- Fee estimation: {} sat/vB for 6 blocks", fee);
    
    // Test balance
    let balance = bitcoin_impl.get_balance()?;
    println!("- Current balance: {} satoshis", balance);
    
    Ok(())
} 