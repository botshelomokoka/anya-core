"""
This module contains helper functions for the Anya project
"""

import hashlib
from bitcoin.core import b2lx

def calculate_txid(tx_hex):
    """
    Calculates the transaction ID (txid) from a raw transaction hex string
    """

    # ... (Implementation)

    # 1. Deserialize the transaction hex
    tx = CMutableTransaction.deserialize(bytes.fromhex(tx_hex))

    # 2. Calculate the double SHA-256 hash of the transaction
    tx_hash = hashlib.sha256(hashlib.sha256(tx.serialize()).digest()).digest()

    # 3. Reverse the bytes and convert to hex
    txid = b2lx(tx_hash[::-1])

    return txid

def convert_satoshi_to_bitcoin(satoshi_amount):
    """
    Converts a satoshi amount to Bitcoin
    """
    return satoshi_amount / 100_000_000  # 1 Bitcoin = 100,000,000 satoshis

def convert_bitcoin_to_satoshi(bitcoin_amount):
    """
    Converts a Bitcoin amount to satoshis
    """
    return int(bitcoin_amount * 100_000_000)

# ... (Other helper functions as needed)
