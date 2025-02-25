// Main entry point for OPSource
// Allows testing of both Python and Rust Bitcoin implementations

mod config;
mod bitcoin;

use std::env;

fn main() {
    println!("OPSource Bitcoin Implementation Tester");
    println!("======================================");
    
    // Load configuration
    let mut config = config::Config::from_env();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "python" => {
                println!("Using Python implementation");
                config.use_rust_bitcoin = false;
            }
            "rust" => {
                println!("Using Rust implementation");
                config.use_rust_bitcoin = true;
            }
            "test" => {
                println!("Running tests for both implementations");
                return run_tests();
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                print_usage();
                return;
            }
        }
    }
    
    // Initialize Bitcoin module
    bitcoin::init();
    
    // Create Bitcoin interface based on config
    let bitcoin_interface = bitcoin::get_current_bitcoin_interface(&config);
    
    // Run simple demo
    match run_demo(bitcoin_interface.as_ref()) {
        Ok(_) => println!("\nDemo completed successfully"),
        Err(e) => println!("\nDemo failed: {:?}", e),
    }
}

fn print_usage() {
    println!("\nUsage:");
    println!("  opsource [command]");
    println!("\nCommands:");
    println!("  python  - Use Python implementation");
    println!("  rust    - Use Rust implementation");
    println!("  test    - Run tests for both implementations");
    println!("\nEnvironment variables:");
    println!("  USE_RUST_BITCOIN - Set to 'true' to use Rust implementation");
    println!("  BITCOIN_NETWORK  - Bitcoin network ('mainnet', 'testnet', 'regtest')");
}

fn run_demo(bitcoin: &dyn bitcoin::BitcoinInterface) -> bitcoin::BitcoinResult<()> {
    // Implementation type
    println!("\nUsing {} Bitcoin implementation", 
             match bitcoin.implementation_type() {
                 bitcoin::BitcoinImplementationType::Python => "Python",
                 bitcoin::BitcoinImplementationType::Rust => "Rust",
             });
    
    // 1. Get blockchain height
    let height = bitcoin.get_block_height()?;
    println!("\n1. Current blockchain height: {}", height);
    
    // 2. Generate address
    let address = bitcoin.generate_address(bitcoin::AddressType::P2WPKH)?;
    println!("\n2. Generated address: {}", address.address);
    
    // 3. Get balance
    let balance = bitcoin.get_balance()?;
    println!("\n3. Wallet balance: {} satoshis ({:.8} BTC)", 
             balance, balance as f64 / 100_000_000.0);
    
    // 4. Create a transaction
    println!("\n4. Creating a transaction...");
    let recipient = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string();
    let amount = 10000; // 0.0001 BTC
    let fee_rate = 3; // 3 sat/vB
    
    let tx = bitcoin.create_transaction(vec![(recipient, amount)], fee_rate)?;
    
    println!("   - Transaction ID: {}", tx.txid);
    println!("   - Fee: {} satoshis", tx.fee.unwrap_or(0));
    println!("   - Size: {} bytes", tx.size);
    println!("   - Number of inputs: {}", tx.inputs.len());
    println!("   - Number of outputs: {}", tx.outputs.len());
    
    // 5. Estimate fee
    let fee_rate = bitcoin.estimate_fee(6)?;
    println!("\n5. Estimated fee rate for 6 blocks: {} sat/vB", fee_rate);
    
    Ok(())
}

fn run_tests() {
    match bitcoin::test::run_tests() {
        Ok(_) => println!("All tests passed!"),
        Err(e) => println!("Tests failed: {}", e),
    }
} 