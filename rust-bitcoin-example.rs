// rust-bitcoin-example.rs
// A demonstration of basic Bitcoin operations using rust-bitcoin

// Required dependencies in Cargo.toml:
// [dependencies]
// bitcoin = { version = "0.32.5", features = ["rand"] }
// rand = "0.8.5"
// hex = "0.4.3"

use bitcoin::{
    Address, Network, Transaction, TxIn, TxOut, OutPoint, Script, 
    blockdata::script::Builder, blockdata::opcodes::all::OP_RETURN,
    consensus::encode, util::amount::Amount, 
    secp256k1::{Secp256k1, SecretKey, Message}, 
    PrivateKey, PublicKey, SigHashType,
};
use rand::thread_rng;
use std::str::FromStr;

fn main() {
    println!("Rust-Bitcoin Demo");
    println!("=================");
    
    // Initialize secp256k1 context
    let secp = Secp256k1::new();
    
    // 1. Key Generation
    println!("\n1. Key Generation");
    println!("----------------");
    
    // Generate a random private key
    let mut rng = thread_rng();
    let secret_key = SecretKey::new(&mut rng);
    
    // Create a Bitcoin private key
    let private_key = PrivateKey {
        compressed: true,
        network: Network::Testnet,
        key: secret_key,
    };
    
    println!("Private Key (WIF): {}", private_key.to_wif());
    
    // Derive public key
    let public_key = PublicKey::from_private_key(&secp, &private_key.key);
    println!("Public Key: {}", public_key);
    
    // 2. Address Generation
    println!("\n2. Address Generation");
    println!("--------------------");
    
    // P2PKH address (legacy)
    let p2pkh_address = Address::p2pkh(&public_key, Network::Testnet);
    println!("P2PKH Address: {}", p2pkh_address);
    
    // P2WPKH address (native SegWit)
    let p2wpkh_address = Address::p2wpkh(&public_key, Network::Testnet).unwrap();
    println!("P2WPKH Address: {}", p2wpkh_address);
    
    // 3. Transaction Building
    println!("\n3. Transaction Building");
    println!("----------------------");
    
    // Create a simple transaction (this is an example and won't be valid on the network)
    let dummy_outpoint = OutPoint::null();
    let dummy_input = TxIn {
        previous_output: dummy_outpoint,
        script_sig: Script::new(),
        sequence: 0xFFFFFFFF,
        witness: vec![],
    };
    
    // Create a P2PKH output
    let value = Amount::from_sat(50000); // 0.0005 BTC
    let script_pubkey = p2pkh_address.script_pubkey();
    let tx_out = TxOut {
        value: value.to_sat(),
        script_pubkey,
    };
    
    // Create an OP_RETURN output with a message
    let message = b"Hello, Rust Bitcoin!";
    let op_return_script = Builder::new()
        .push_opcode(OP_RETURN)
        .push_slice(message)
        .into_script();
    
    let op_return_output = TxOut {
        value: 0,  // OP_RETURN outputs have 0 value
        script_pubkey: op_return_script,
    };
    
    // Build the transaction
    let transaction = Transaction {
        version: 2,
        lock_time: 0,
        input: vec![dummy_input],
        output: vec![tx_out, op_return_output],
    };
    
    // Serialize the transaction
    let serialized_tx = encode::serialize(&transaction);
    println!("Transaction Hex: {}", hex::encode(&serialized_tx));
    println!("Transaction ID: {}", transaction.txid());
    
    // 4. Message Signing
    println!("\n4. Message Signing");
    println!("-----------------");
    
    let message_to_sign = "This is a signed message from my Bitcoin address";
    let message_hash = Message::from_hashed_data::<bitcoin::hashes::sha256d::Hash>(message_to_sign.as_bytes());
    
    // Sign the message
    let signature = secp.sign_ecdsa(&message_hash, &private_key.key);
    println!("Message: {}", message_to_sign);
    println!("Signature: {}", signature);
    
    // Verify the signature
    let result = secp.verify_ecdsa(&message_hash, &signature, &public_key.inner);
    println!("Signature verification: {}", result.is_ok());
} 