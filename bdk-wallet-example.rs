// bdk-wallet-example.rs
// Demonstration of Bitcoin wallet operations using Bitcoin Development Kit (BDK)

// Required dependencies in Cargo.toml:
// [dependencies]
// bdk = { version = "0.30.2", default-features = true, features = ["electrum"] }
// bitcoin = { version = "0.32.5", features = ["rand"] }
// tokio = { version = "1.41.1", features = ["full", "macros"] }

use bdk::{
    bitcoin::{Network, Address},
    blockchain::{
        electrum::{ElectrumBlockchain, ElectrumBlockchainConfig},
        Blockchain, ConfigurableBlockchain,
    },
    wallet::{AddressIndex, AddressInfo},
    database::MemoryDatabase,
    descriptor::{Descriptor, DescriptorPublicKey},
    keys::{
        DerivableKey, DescriptorKey, ExtendedKey, GeneratedKey, 
        GeneratableKey, GeneratedDescriptorKey
    },
    Balance, FeeRate, Wallet, SyncOptions,
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BDK Wallet Example");
    println!("==================");
    
    // 1. Generate a new HD wallet
    println!("\n1. Generating a new HD wallet");
    println!("---------------------------");
    
    // Generate a new mnemonic (BIP39)
    let mnemonic = bdk::keys::bip39::Mnemonic::generate(12)?;
    println!("Mnemonic: {}", mnemonic.to_string());
    
    // Get the xprv from the mnemonic
    let xkey: ExtendedKey = mnemonic.into_extended_key()?;
    let xprv = xkey.into_xprv(Network::Testnet).expect("Failed to generate xprv");
    
    // Generate the descriptor secret key for external addresses
    let derive_path = "m/84h/1h/0h/0"; // BIP84 for testnet (SegWit)
    let (descriptor_key, key_map) = xprv.derive_descriptor_key(derive_path)?;
    
    // Generate the descriptor for receiving addresses
    let descriptor = Descriptor::new_wpkh(descriptor_key)?;
    let derived_descriptor = descriptor.derive_from_map(&key_map)?;
    println!("Receive Descriptor: {}", derived_descriptor);
    
    // Generate the descriptor for change addresses
    let derive_path_change = "m/84h/1h/0h/1"; // Change path
    let (descriptor_key_change, key_map_change) = xprv.derive_descriptor_key(derive_path_change)?;
    let descriptor_change = Descriptor::new_wpkh(descriptor_key_change)?;
    let derived_descriptor_change = descriptor_change.derive_from_map(&key_map_change)?;
    println!("Change Descriptor: {}", derived_descriptor_change);
    
    // 2. Create an in-memory wallet
    println!("\n2. Creating wallet and connecting to Electrum server");
    println!("------------------------------------------------");
    
    // Create a wallet using the descriptors
    let wallet = Wallet::new(
        derived_descriptor.clone(),
        Some(derived_descriptor_change.clone()),
        Network::Testnet,
        MemoryDatabase::default(),
    )?;
    
    // Connect to Electrum server (testnet)
    let config = ElectrumBlockchainConfig {
        url: "ssl://electrum.blockstream.info:60002".to_string(),
        socks5: None,
        retry: 3,
        timeout: Some(5),
        stop_gap: 20,
        validate_domain: true,
    };
    
    let blockchain = ElectrumBlockchain::from_config(&config)?;
    
    // 3. Wallet operations
    println!("\n3. Performing wallet operations");
    println!("----------------------------");
    
    // Sync the wallet with the blockchain
    println!("Syncing wallet...");
    wallet.sync(&blockchain, SyncOptions::default())?;
    
    // Get and print wallet balance
    let balance = wallet.get_balance()?;
    println!("Wallet balance: {} sats confirmed, {} sats unconfirmed",
             balance.confirmed, balance.untrusted_pending);
    
    // Generate a new address
    let address_info = wallet.get_address(AddressIndex::New)?;
    println!("New address: {}", address_info.address);
    
    // 4. Transaction building (simplified example)
    println!("\n4. Transaction building example");
    println!("----------------------------");
    
    // Only try to build a transaction if we have some funds
    if balance.confirmed > 0 {
        // Create a dummy recipient address
        let dummy_address = Address::from_str("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx")
            .expect("Failed to parse address");
        
        // Set the amount to send (less than our balance)
        let amount = balance.confirmed / 2;
        
        // Create a transaction builder
        let mut tx_builder = wallet.build_tx();
        
        // Send to the recipient
        tx_builder.add_recipient(dummy_address.script_pubkey(), amount);
        
        // Set fee rate
        tx_builder.fee_rate(FeeRate::from_sat_per_vb(5.0));
        
        // Finish building the transaction
        match tx_builder.finish() {
            Ok(transaction) => {
                println!("Transaction created successfully!");
                println!("Transaction ID: {}", transaction.txid);
                println!("Fee: {} sats", transaction.fee);
                println!("This is a simulation - transaction not broadcast!");
            },
            Err(e) => println!("Failed to build transaction: {}", e),
        }
    } else {
        println!("Wallet has no confirmed balance to build a transaction");
    }
    
    // 5. List transaction history
    println!("\n5. Transaction history");
    println!("--------------------");
    let txs = wallet.list_transactions(false)?;
    
    if txs.is_empty() {
        println!("No transaction history found");
    } else {
        for (i, tx) in txs.iter().enumerate() {
            println!("Transaction {}: {}", i + 1, tx.txid);
            println!("  Received: {} sats", tx.received);
            println!("  Sent: {} sats", tx.sent);
            println!("  Fee: {} sats", tx.fee.unwrap_or(0));
            println!("  Confirmation time: {:?}", tx.confirmation_time);
            println!();
        }
    }
    
    Ok(())
} 