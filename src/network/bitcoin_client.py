"""
This module provides a client interface for interacting with the Bitcoin network via RPC.
"""

from bitcoin.rpc import Proxy

# Connect to Bitcoin Core RPC (replace with your actual connection details)
RPC_USER = 'your_rpc_user'
RPC_PASSWORD = 'your_rpc_password'
RPC_HOST = 'localhost'  # Or your remote host
RPC_PORT = 8332 
rpc_connection = Proxy(f"http://{RPC_USER}:{RPC_PASSWORD}@{RPC_HOST}:{RPC_PORT}")

def get_utxos(address):
    """
    Fetches unspent transaction outputs (UTXOs) for a given address.

    Args:
        address: A Bitcoin address string.

    Returns:
        A list of UTXO dictionaries, each containing 'txid', 'vout', and 'value' keys.
    """
    try:
        utxos = rpc_connection.listunspent(addresses=[address])
        return [{'txid': utxo['txid'], 'vout': utxo['vout'], 'value': utxo['amount']} for utxo in utxos]
    except Exception as e:
        print(f"Error fetching UTXOs for {address}: {e}")
        return []  # Or raise an exception, depending on your error handling strategy

def get_raw_transaction(txid):
    """
    Fetches the raw transaction data for a given transaction ID.

    Args:
        txid: A Bitcoin transaction ID (hex string).

    Returns:
        A dictionary representing the raw transaction data.
    """
    try:
        return rpc_connection.getrawtransaction(txid, 1)  # 1 for verbose output
    except Exception as e:
        print(f"Error fetching raw transaction for {txid}: {e}")
        return None  # Or raise an exception

def send_raw_transaction(tx_hex):
    """
    Broadcasts a raw transaction to the Bitcoin network

    Args:
        tx_hex: The signed transaction in hexadecimal format

    Returns:
        The transaction ID (txid) if successful, or None if there's an error
    """
    try:
        txid = rpc_connection.sendrawtransaction(tx_hex)
        return txid
    except Exception as e:
        print(f"Error broadcasting transaction: {e}")
        return None

# ... (Other RPC functions as needed, e.g., estimatefee, getblock, etc.)
