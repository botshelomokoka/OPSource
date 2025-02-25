// Lightning Network Test Program
// This program tests the Lightning Network functionality

use std::sync::Arc;
use opsource::{bitcoin, config, lightning};

fn main() {
    println!("OPSource Lightning Network Tester");
    println!("=================================");
    
    // Initialize the library
    opsource::init();
    
    // Create a configuration
    let config = config::Config::from_env();
    
    // Create a Bitcoin interface
    let bitcoin_interface = bitcoin::get_current_bitcoin_interface(&config);
    
    // Create a Lightning interface
    let lightning_interface = lightning::create_lightning_interface(
        &config,
        bitcoin_interface.clone()
    );
    
    // Print implementation information
    println!("\nUsing {} Bitcoin implementation",
        match bitcoin_interface.implementation_type() {
            bitcoin::BitcoinImplementationType::Python => "Python",
            bitcoin::BitcoinImplementationType::Rust => "Rust",
        }
    );
    
    println!("Using {} Lightning implementation",
        match lightning_interface.implementation_type() {
            lightning::interface::LightningImplementationType::LDK => "LDK",
            lightning::interface::LightningImplementationType::Mock => "Mock",
        }
    );
    
    // Run the Lightning Network tests
    run_tests(lightning_interface.as_ref());
}

fn run_tests(ln: &dyn lightning::interface::LightningInterface) {
    // Test 1: Get node info
    println!("\n1. Getting node info...");
    match ln.get_node_info() {
        Ok(node_info) => {
            println!("Node pubkey: {}", node_info.pubkey);
            println!("Node alias: {}", node_info.alias.unwrap_or_else(|| "None".to_string()));
            println!("Node addresses: {:?}", node_info.addresses);
        },
        Err(e) => println!("Error getting node info: {:?}", e),
    }
    
    // Test 2: Connect to a peer
    println!("\n2. Connecting to a peer...");
    let peer_pubkey = "02eec7245d6b7d2ccb30380bfbe2a3648cd7a942653f5aa340edcea1f283686619";
    match ln.connect_peer(peer_pubkey, "127.0.0.1", 9735) {
        Ok(_) => println!("Successfully connected to peer: {}", peer_pubkey),
        Err(e) => println!("Error connecting to peer: {:?}", e),
    }
    
    // Test 3: List peers
    println!("\n3. Listing peers...");
    match ln.list_peers() {
        Ok(peers) => {
            println!("Connected to {} peers:", peers.len());
            for peer in peers {
                println!("- {}", peer.pubkey);
            }
        },
        Err(e) => println!("Error listing peers: {:?}", e),
    }
    
    // Test 4: Open a channel
    println!("\n4. Opening a channel...");
    let channel_capacity = 100_000; // 100k sats
    match ln.open_channel(peer_pubkey, channel_capacity, None, false) {
        Ok(channel) => {
            println!("Successfully opened channel:");
            println!("Channel ID: {}", channel.channel_id);
            println!("Funding txid: {}", channel.funding_txid);
            println!("Capacity: {} sats", channel.capacity);
        },
        Err(e) => println!("Error opening channel: {:?}", e),
    }
    
    // Test 5: List channels
    println!("\n5. Listing channels...");
    match ln.list_channels() {
        Ok(channels) => {
            println!("Found {} channels:", channels.len());
            for channel in channels {
                println!("- {} with {}", channel.channel_id, channel.remote_pubkey);
                println!("  Capacity: {} sats", channel.capacity);
                println!("  Local balance: {} sats", channel.local_balance);
            }
        },
        Err(e) => println!("Error listing channels: {:?}", e),
    }
    
    // Test 6: Create an invoice
    println!("\n6. Creating an invoice...");
    let mut invoice = None;
    match ln.create_invoice(Some(50_000), "Test payment", None) {
        Ok(inv) => {
            println!("Created invoice:");
            println!("BOLT11: {}", inv.bolt11);
            println!("Payment hash: {}", inv.payment_hash);
            println!("Amount: {} msats", inv.amount_msat.unwrap_or(0));
            invoice = Some(inv);
        },
        Err(e) => println!("Error creating invoice: {:?}", e),
    }
    
    // Test 7: Decode an invoice
    if let Some(inv) = &invoice {
        println!("\n7. Decoding invoice...");
        match ln.decode_invoice(&inv.bolt11) {
            Ok(decoded) => {
                println!("Decoded invoice:");
                println!("Payment hash: {}", decoded.payment_hash);
                println!("Description: {}", decoded.description);
                println!("Amount: {:?} msats", decoded.amount_msat);
            },
            Err(e) => println!("Error decoding invoice: {:?}", e),
        }
    }
    
    // Test 8: Pay an invoice
    if let Some(inv) = &invoice {
        println!("\n8. Paying invoice...");
        match ln.pay_invoice(&inv.bolt11, None) {
            Ok(payment) => {
                println!("Payment successful:");
                println!("Payment ID: {}", payment.payment_id);
                println!("Payment hash: {}", payment.payment_hash);
                println!("Amount: {} msats", payment.amount_msat);
                println!("Fee: {} msats", payment.fee_msat);
                println!("Status: {:?}", payment.status);
            },
            Err(e) => println!("Error paying invoice: {:?}", e),
        }
    }
    
    // Test 9: List payments
    println!("\n9. Listing payments...");
    match ln.list_payments() {
        Ok(payments) => {
            println!("Found {} payments:", payments.len());
            for payment in payments {
                println!("- Payment hash: {}", payment.payment_hash);
                println!("  Amount: {} msats", payment.amount_msat);
                println!("  Fee: {} msats", payment.fee_msat);
                println!("  Status: {:?}", payment.status);
            }
        },
        Err(e) => println!("Error listing payments: {:?}", e),
    }
    
    // Test 10: Close a channel
    println!("\n10. Closing a channel...");
    match ln.list_channels() {
        Ok(channels) => {
            if let Some(channel) = channels.first() {
                match ln.close_channel(&channel.channel_id, false) {
                    Ok(close_txid) => {
                        println!("Successfully closed channel:");
                        println!("Channel ID: {}", channel.channel_id);
                        println!("Closing txid: {}", close_txid);
                    },
                    Err(e) => println!("Error closing channel: {:?}", e),
                }
            } else {
                println!("No channels to close");
            }
        },
        Err(e) => println!("Error listing channels: {:?}", e),
    }
    
    println!("\nLightning Network tests completed!");
} 