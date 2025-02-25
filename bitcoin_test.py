#!/usr/bin/env python
"""
Simple test script to verify the Bitcoin development environment.
"""
import base58
import ecdsa
import hashlib
from hdwallet import HDWallet
from hdwallet.symbols import BTC
from mnemonic import Mnemonic

def test_bitcoin_address_generation():
    """Test Bitcoin address generation from private key"""
    # Generate a private key
    private_key = hashlib.sha256(b"test").digest()
    print(f"Private key (hex): {private_key.hex()}")
    
    # Get the public key
    signing_key = ecdsa.SigningKey.from_string(private_key, curve=ecdsa.SECP256k1)
    verifying_key = signing_key.get_verifying_key()
    public_key = verifying_key.to_string()
    print(f"Public key (hex): {public_key.hex()}")
    
    # Create a Bitcoin address
    sha256_hash = hashlib.sha256(b'\x04' + public_key).digest()
    ripemd160_hash = hashlib.new('ripemd160')
    ripemd160_hash.update(sha256_hash)
    network_byte = b'\x00'  # Mainnet
    hash_with_network_byte = network_byte + ripemd160_hash.digest()
    
    # Double SHA-256 for checksum
    checksum = hashlib.sha256(hashlib.sha256(hash_with_network_byte).digest()).digest()[:4]
    binary_address = hash_with_network_byte + checksum
    
    # Encode to Base58
    bitcoin_address = base58.b58encode(binary_address).decode('utf-8')
    print(f"Bitcoin address: {bitcoin_address}")
    return bitcoin_address

def test_hdwallet():
    """Test HDWallet functionality"""
    # Initialize from mnemonic
    mnemonic = Mnemonic("english").generate()
    print(f"Mnemonic: {mnemonic}")
    
    hdwallet = HDWallet(symbol=BTC)
    hdwallet.from_mnemonic(mnemonic=mnemonic)
    hdwallet.from_path(path="m/44'/0'/0'/0/0")
    
    print(f"HDWallet address: {hdwallet.p2pkh_address()}")
    print(f"HDWallet private key: {hdwallet.private_key()}")
    return hdwallet.p2pkh_address()

if __name__ == "__main__":
    print("Testing Bitcoin address generation...")
    addr1 = test_bitcoin_address_generation()
    
    print("\nTesting HDWallet functionality...")
    addr2 = test_hdwallet()
    
    print("\nTest completed successfully!") 