"""
This module stores project-wide constants and configuration settings for Anya.
"""

# Network constants
BITCOIN_NETWORK = 'mainnet'  # or 'testnet'
RSK_NETWORK = 'mainnet'  # or 'testnet'

# Derivation paths (BIP44)
BIP44_COIN_TYPE = 0  # Bitcoin
BIP44_ACCOUNT = 0   # First account (change this if needed)

# Address types
DEFAULT_ADDRESS_TYPE = 'p2wpkh'  # Native SegWit (bech32)

# Fee estimation
DEFAULT_FEE_RATE = 1  # sat/byte (adjust as needed)
FEE_ESTIMATION_SOURCE = 'bitcoin_core'  # or 'external_api' (if using an external service)

# Other constants (add more as needed)
MAX_TRANSACTION_INPUTS = 100  # Limit the number of inputs in a transaction
