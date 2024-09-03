# Anya Wallet Core API Reference

This document provides a reference for the core API functions available in the Anya Wallet.

## `key_management` Module

*   **`generate_mnemonic()`**
    *   Generates a new BIP39 mnemonic phrase.
    *   Returns: `str` (mnemonic phrase)

*   **`derive_key_from_mnemonic(mnemonic, passphrase="")`**
    *   Derives a BIP32 HD key from a mnemonic and optional passphrase
    *   Args:
        *   `mnemonic`: `str` (BIP39 mnemonic phrase)
        *   `passphrase`: `str` (optional passphrase, default is empty string)
    *   Returns: `bip32.HDKey` object
    *   Raises: `ValueError` if the mnemonic is invalid

*   **`derive_child_key(parent_key, path)`**
    *   Derives a child key from a parent key using a BIP32 path
    *   Args:
        *   `parent_key`: `bip32.HDKey` object
        *   `path`: `str` (BIP32 derivation path)
    *   Returns: `bip32.HDKey` object
    *   Raises: `ValueError` if the derivation path is invalid

*   **`encrypt_private_key(private_key, password)`**
    *   Encrypts a private key using a password
    *   Args:
        *   `private_key`: `str` (private key in WIF or hex format)
        *   `password`: `str` (password for encryption)
    *   Returns: `bytes` (encrypted private key with salt)

*   **`decrypt_private_key(encrypted_key, password)`**
    *   Decrypts an encrypted private key using a password
    *   Args:
        *   `encrypted_key`: `bytes` (encrypted private key with salt)
        *   `password`: `str` (password used for encryption)
    *   Returns: `str` (decrypted private key)
    *   Raises: `ValueError` if decryption fails

*   **`is_valid_mnemonic(mnemonic)`**
    *   Checks if a given mnemonic phrase is valid
    *   Args:
        *   `mnemonic`: `str` (mnemonic phrase to validate)
    *   Returns: `bool` (True if valid, False otherwise)

*   **`export_private_key_wif(private_key)`**
    *   Exports a private key in Wallet Import Format (WIF)
    *   Args:
        *   `private_key`: `bip32.HDKey` or `str` (private key)
    *   Returns: `str` (WIF representation of the private key)

*   **`import_private_key_wif(wif)`**
    *   Imports a private key from Wallet Import Format (WIF)
    *   Args:
        *   `wif`: `str` (WIF representation of the private key)
    *   Returns: `bip32.HDKey` or `str` (private key)

*   **`get_hardware_wallet()`**
    *   Detects and connects to a compatible hardware wallet
    *   Returns: `HardwareWallet` object if a device is found and connected, otherwise `None`

*   **`generate_address_from_hardware_wallet(wallet, derivation_path)`**
    *   Generates a new Bitcoin address from a hardware wallet
    *   Args:
        *   `wallet`: `AnyaWallet` object
        *   `derivation_path`: `str` (BIP32 derivation path)
    *   Returns: `str` (Bitcoin address)
    *   Raises: `ValueError` if no compatible hardware wallet is found

*   **`sign_transaction_with_hardware_wallet(tx, input_index, derivation_path)`**
    *   Signs a transaction input using a hardware wallet
    *   Args:
        *   `tx`: `CMutableTransaction` object
        *   `input_index`: `int` (index of the input to sign)
        *   `derivation_path`: `str` (BIP32 derivation path)
    *   Returns: `list` (witness stack for the signed input)
    *   Raises: `ValueError` if no compatible hardware wallet is found

## `transaction` Module

*   **`create_transaction(inputs, outputs, private_keys, fee_rate=None, change_address=None)`**
    *   Creates a Bitcoin transaction
    *   Args:
        *   `inputs`: `list` of `dict` (each representing a UTXO to be spent)
        *   `outputs`: `list` of `dict` (each representing an output)
        *   `private_keys`: `list` of `bip32.HDKey` or `str` (private keys corresponding to the inputs)
        *   `fee_rate`: `int` (optional, desired fee rate in satoshis per byte)
        *   `change_address`: `str` (optional, address to send any change to)
    *   Returns: `CMutableTransaction` object

*   **`sign_transaction(tx, private_keys)`**
    *   Signs a Bitcoin transaction using the provided private keys
    *   Args:
        *   `tx`: `CMutableTransaction` object
        *   `private_keys`: `list` of `bip32.HDKey` or `str` (private keys corresponding to the inputs)
    *   Returns: `CMutableTransaction` object (signed)

*   **`broadcast_transaction(tx)`**
    *   Broadcasts a signed transaction to the Bitcoin network
    *   Args:
        *   `tx`: `CMutableTransaction` object (signed)
    *   Returns: `str` (transaction ID) or `None` if there's an error

## `balance` Module

*   **`get_balance(address)`**
    *   Retrieves the Bitcoin balance for a given address
    *   Args:
        *   `address`: `str` (Bitcoin address)
    *   Returns: `int` (balance in satoshis)

*   **`get_taproot_asset_balances(address)`**
    *   Retrieves the balances of Taproot assets associated with an address (placeholder for future implementation)
    *   Args:
        *   `address`: `str` (Bitcoin address)
    *   Returns: `dict` (asset ID -> balance mapping)

## `address_management` Module

*   **`generate_new_address(key, address_type='p2wpkh')`**
    *   Generates a new Bitcoin address from a public key
    *   Args:
        *   `key`: `bip32.HDPubKey` or `str` (public key)
        *   `address_type`: `str` ('p2pkh', 'p2sh-p2wpkh', or 'p2wpkh')
    *   Returns: `str` (Bitcoin address)
    *   Raises: `ValueError` if the key or address type is invalid

*   **`validate_address(address)`**
    *   Validates a Bitcoin address
    *   Args:
        *   `address`: `str` (Bitcoin address)
    *   Returns: `bool` (True if valid, False otherwise)

**... (Add other modules and their functions as you develop them)** 

**Notes:**

*   This is a basic API reference. Needs to expand more features and functionalities are added to Anya Wallet
*   Include detailed descriptions for each function, explaining its purpose, arguments, return values, and potential errors


