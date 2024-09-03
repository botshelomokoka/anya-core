"""
This module provides a client interface for interacting with the RSK network.
"""

from web3 import Web3

# Connect to an RSK node
RSK_NODE_URL = 'https://public-node.rsk.co'  # Or your preferred RSK node URL
w3 = Web3(Web3.HTTPProvider(RSK_NODE_URL))

def get_balance(address):
    """
    Gets the RBTC balance of an address on the RSK network

    Args:
        address: the address to check

    Returns:
        The balance in wei
    """

    balance = w3.eth.get_balance(address)
    return balance

def send_transaction(transaction):
    """
    Sends a signed transaction to the RSK network

    Args:
        transaction: the signed transaction object

    Returns:
        The transaction hash if successful
    """

    tx_hash = w3.eth.send_raw_transaction(transaction.rawTransaction)
    return tx_hash

def get_transaction(tx_hash):
    """
    Gets the details of a transaction on the RSK network

    Args:
        tx_hash: the transaction hash

    Returns:
        The transaction object
    """

    tx = w3.eth.get_transaction(tx_hash)
    return tx

# ... (Other RSK interaction functions as needed, e.g., 
#     contract deployment, contract interaction, event listening etc.)
