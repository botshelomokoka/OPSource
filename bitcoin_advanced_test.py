#!/usr/bin/env python
"""
Advanced test script to verify various aspects of the Bitcoin development environment.
"""
import base58
import ecdsa
import hashlib
import os
import binascii
from hdwallet import HDWallet
from hdwallet.symbols import BTC
from mnemonic import Mnemonic
import bitcoin
from bitcoin.core import COIN, b2x, lx, COutPoint, CMutableTxOut, CMutableTxIn, CMutableTransaction
from bitcoin.core.script import CScript, OP_DUP, OP_HASH160, OP_EQUALVERIFY, OP_CHECKSIG, SignatureHash, SIGHASH_ALL
from bitcoin.wallet import CBitcoinAddress, CBitcoinSecret

# Initialize Bitcoin library to use testnet
bitcoin.SelectParams('testnet')

def test_wallet_operations():
    """Test basic wallet operations"""
    print("\n=== Testing Wallet Operations ===")
    
    # Generate a random private key
    private_key_bytes = os.urandom(32)
    private_key_hex = binascii.hexlify(private_key_bytes).decode('utf-8')
    print(f"Generated private key (hex): {private_key_hex}")
    
    # Create HDWallet from private key
    hdwallet = HDWallet(symbol=BTC)
    hdwallet.from_private_key(private_key=private_key_hex)
    print(f"HDWallet P2PKH address: {hdwallet.p2pkh_address()}")
    print(f"HDWallet P2SH address: {hdwallet.p2sh_address()}")
    print(f"HDWallet P2WPKH address: {hdwallet.p2wpkh_address()}")
    print(f"HDWallet P2WSH address: {hdwallet.p2wsh_address()}")
    
    # Test mnemonic generation and recovery
    mnemonic = Mnemonic("english").generate()
    print(f"\nGenerated mnemonic: {mnemonic}")
    
    # Create a new HDWallet from the mnemonic
    recovered_wallet = HDWallet(symbol=BTC)
    recovered_wallet.from_mnemonic(mnemonic=mnemonic)
    recovered_wallet.from_path(path="m/44'/0'/0'/0/0")
    
    print(f"Recovered wallet address: {recovered_wallet.p2pkh_address()}")
    print(f"Recovered wallet private key: {recovered_wallet.private_key()}")
    
    return hdwallet, recovered_wallet

def test_transaction_creation():
    """Test transaction creation with python-bitcoinlib"""
    print("\n=== Testing Transaction Creation ===")
    
    # Create a private key
    seckey = CBitcoinSecret.from_secret_bytes(os.urandom(32))
    print(f"Private key: {seckey}")
    
    # Get the public key
    pubkey = seckey.pub
    print(f"Public key: {b2x(pubkey)}")
    
    # Create a P2PKH address
    pkh = bitcoin.wallet.hash160(pubkey)
    addr = CBitcoinAddress.from_scriptPubKey(CScript([OP_DUP, OP_HASH160, pkh, OP_EQUALVERIFY, OP_CHECKSIG]))
    print(f"Address: {addr}")
    
    # Create a dummy transaction
    txid = lx('0'*64)  # dummy TXID
    vout = 0
    amount = int(1.0 * COIN)  # 1 BTC
    
    # Create the transaction input
    txin = CMutableTxIn(COutPoint(txid, vout))
    
    # Create the transaction output (back to our address)
    scriptPubKey = CScript([OP_DUP, OP_HASH160, pkh, OP_EQUALVERIFY, OP_CHECKSIG])
    txout = CMutableTxOut(amount, scriptPubKey)
    
    # Create the unsigned transaction
    tx = CMutableTransaction([txin], [txout])
    
    # Create a signature for the transaction
    sighash = SignatureHash(scriptPubKey, tx, 0, SIGHASH_ALL)
    sig = seckey.sign(sighash) + bytes([SIGHASH_ALL])
    
    # Set the scriptSig
    tx.vin[0].scriptSig = CScript([sig, pubkey])
    
    # Print the serialized transaction
    print(f"Serialized signed transaction: {b2x(tx.serialize())}")
    
    return tx

def test_cryptographic_functions():
    """Test various cryptographic functions used in Bitcoin"""
    print("\n=== Testing Cryptographic Functions ===")
    
    # Test SHA-256
    message = b"Bitcoin"
    sha256_hash = hashlib.sha256(message).hexdigest()
    print(f"SHA-256 of '{message.decode()}': {sha256_hash}")
    
    # Test double SHA-256 (used in Bitcoin)
    double_sha256 = hashlib.sha256(hashlib.sha256(message).digest()).hexdigest()
    print(f"Double SHA-256 of '{message.decode()}': {double_sha256}")
    
    # Test RIPEMD-160
    ripemd160 = hashlib.new('ripemd160')
    ripemd160.update(message)
    ripemd160_hash = ripemd160.hexdigest()
    print(f"RIPEMD-160 of '{message.decode()}': {ripemd160_hash}")
    
    # Test combined SHA-256 + RIPEMD-160 (used for Bitcoin addresses)
    combined = hashlib.new('ripemd160')
    combined.update(hashlib.sha256(message).digest())
    combined_hash = combined.hexdigest()
    print(f"SHA-256 + RIPEMD-160 of '{message.decode()}': {combined_hash}")
    
    return sha256_hash, double_sha256, ripemd160_hash, combined_hash

if __name__ == "__main__":
    print("Starting advanced Bitcoin environment tests...")
    
    # Run wallet tests
    hdwallet, recovered_wallet = test_wallet_operations()
    
    # Run transaction creation tests
    tx = test_transaction_creation()
    
    # Run cryptographic function tests
    hashes = test_cryptographic_functions()
    
    print("\n=== All tests completed successfully! ===")
    print("The Bitcoin development environment is properly set up and functional.") 