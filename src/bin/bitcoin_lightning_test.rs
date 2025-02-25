// Bitcoin-Lightning Bridge Test
// Tests the integration between Bitcoin and Lightning Network components

use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    println!("======================================================");
    println!("⚡ Bitcoin-Lightning Bridge Test");
    println!("======================================================");
    
    // Initialize configuration
    let config = opsource::config::Config::default();
    
    // Get bitcoin interface
    let bitcoin_interface = opsource::bitcoin::get_current_bitcoin_interface(&config);
    println!("\nUsing Bitcoin implementation: {:?}", bitcoin_interface.implementation_type());
    
    // Get lightning interface
    let lightning_interface = opsource::lightning::create_lightning_interface(
        &config,
        bitcoin_interface.clone(),
    );
    println!("Using Lightning implementation: {:?}", lightning_interface.implementation_type());
    
    // Create Bitcoin-Lightning bridge
    let bridge = opsource::lightning::bitcoin_bridge::BitcoinLightningBridge::new(
        &config,
        bitcoin_interface.clone(),
        lightning_interface.clone(),
    );
    
    // Initialize bridge
    println!("\n1. Initializing Bitcoin-Lightning bridge...");
    match bridge.init() {
        Ok(_) => println!("Bridge initialized successfully"),
        Err(e) => {
            println!("Failed to initialize bridge: {:?}", e);
            return;
        }
    }
    
    // Get our node info
    println!("\n2. Getting node info...");
    match lightning_interface.get_node_info() {
        Ok(node) => println!("Node pubkey: {}", node.pubkey),
        Err(e) => println!("Error getting node info: {:?}", e),
    }
    
    // Connect to a peer
    println!("\n3. Connecting to a peer...");
    let peer_pubkey = "03f25d220b14f3daae528bbb98cf142caf3477c8d5258d9f81b0af0370163f0df2";
    match lightning_interface.connect_peer(peer_pubkey, "127.0.0.1", 9735) {
        Ok(_) => println!("Connected to peer {}", peer_pubkey),
        Err(e) => println!("Error connecting to peer: {:?}", e),
    }
    
    // Create a funding address
    println!("\n4. Creating funding address for channel...");
    match bridge.create_funding_address(peer_pubkey, 100_000, None, false) {
        Ok(address) => {
            println!("Created funding address: {}", address.address);
            println!("Send 100,000 sats to this address to open a channel");
        },
        Err(e) => println!("Error creating funding address: {:?}", e),
    }
    
    // Get current channel balance
    println!("\n5. Getting channel balance...");
    match bridge.get_channel_balance() {
        Ok(balance) => println!("Available channel balance: {} sats", balance),
        Err(e) => println!("Error getting channel balance: {:?}", e),
    }
    
    // Check for funding transactions
    println!("\n6. Checking for funding transactions...");
    match bridge.check_funding_transactions() {
        Ok(txs) => {
            println!("Found {} funding transactions", txs.len());
            for tx in txs {
                println!("- Channel ID: {}", tx.channel_id);
                println!("  Funding txid: {}", tx.funding_txid);
                println!("  Amount: {} sats", tx.funding_amount);
                println!("  Status: {:?}", tx.status);
            }
        },
        Err(e) => println!("Error checking for funding transactions: {:?}", e),
    }
    
    // Monitor the blockchain for channel transactions
    println!("\n7. Monitoring blockchain for channel transactions...");
    match bridge.monitor_blockchain() {
        Ok(_) => println!("Blockchain monitoring completed"),
        Err(e) => println!("Error monitoring blockchain: {:?}", e),
    }
    
    // List channels
    println!("\n8. Listing channels...");
    match lightning_interface.list_channels() {
        Ok(channels) => {
            println!("Found {} channels", channels.len());
            for channel in channels {
                println!("- Channel ID: {}", channel.channel_id);
                println!("  Remote pubkey: {}", channel.remote_pubkey);
                println!("  Capacity: {} sats", channel.capacity);
                println!("  Local balance: {} sats", channel.local_balance);
                println!("  Status: {}", if channel.is_active { "Active" } else { "Inactive" });
                
                // Get channel transaction
                match bridge.get_channel_transaction(&channel.channel_id) {
                    Ok(Some(tx)) => {
                        println!("  On-chain status: {:?}", tx.status);
                        if let Some(height) = tx.confirmation_height {
                            println!("  Confirmed at height: {}", height);
                        }
                    },
                    _ => {}
                }
            }
        },
        Err(e) => println!("Error listing channels: {:?}", e),
    }
    
    // Create and pay invoice through a channel
    println!("\n9. Creating an invoice...");
    let mut invoice = None;
    match lightning_interface.create_invoice(Some(10_000), "Bridge test payment", None) {
        Ok(inv) => {
            println!("Created invoice:");
            println!("BOLT11: {}", inv.bolt11);
            println!("Payment hash: {}", inv.payment_hash);
            invoice = Some(inv);
        },
        Err(e) => println!("Error creating invoice: {:?}", e),
    }
    
    // Pay invoice if created
    if let Some(inv) = invoice {
        println!("\n10. Paying invoice...");
        match lightning_interface.pay_invoice(&inv.bolt11, None) {
            Ok(payment) => {
                println!("Payment successful:");
                println!("Payment hash: {}", payment.payment_hash);
                println!("Amount: {} msats", payment.amount_msat);
                println!("Status: {:?}", payment.status);
            },
            Err(e) => println!("Error paying invoice: {:?}", e),
        }
    }
    
    // Close a channel
    println!("\n11. Closing channel...");
    match lightning_interface.list_channels() {
        Ok(channels) => {
            if let Some(channel) = channels.first() {
                match lightning_interface.close_channel(&channel.channel_id, false) {
                    Ok(closing_txid) => {
                        println!("Channel closing initiated:");
                        println!("Channel ID: {}", channel.channel_id);
                        println!("Closing txid: {}", closing_txid);
                        
                        // Register the closing transaction
                        match bridge.register_channel_close(&channel.channel_id, &closing_txid) {
                            Ok(_) => println!("Channel close registered in bridge"),
                            Err(e) => println!("Error registering channel close: {:?}", e),
                        }
                    },
                    Err(e) => println!("Error closing channel: {:?}", e),
                }
            } else {
                println!("No channels to close");
            }
        },
        Err(e) => println!("Error listing channels: {:?}", e),
    }
    
    // List channel transactions
    println!("\n12. Listing all channel transactions...");
    match bridge.list_channel_transactions() {
        Ok(txs) => {
            println!("Found {} channel transactions", txs.len());
            for tx in txs {
                println!("- Channel ID: {}", tx.channel_id);
                println!("  Funding txid: {}", tx.funding_txid);
                println!("  Status: {:?}", tx.status);
                if let Some(txid) = tx.closing_txid {
                    println!("  Closing txid: {}", txid);
                }
            }
        },
        Err(e) => println!("Error listing channel transactions: {:?}", e),
    }
    
    println!("\nBitcoin-Lightning Bridge tests completed!");
} 