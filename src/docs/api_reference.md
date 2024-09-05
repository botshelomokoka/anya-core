# Anya Wallet Core API Reference

This document provides a reference for the core API functions available in the Anya Wallet.

## `key_management` Module

* **`generate_mnemonic() -> Result<String, Error>`**
  * Generates a new BIP39 mnemonic phrase.
  * Returns: A 24-word mnemonic phrase as a String.

* **`derive_key_from_mnemonic(mnemonic: &str, passphrase: Option<&str>) -> Result<bip32::ExtendedPrivKey, Error>`**
  * Derives a BIP32 HD key from a mnemonic and optional passphrase.
  * Args:
    * `mnemonic`: BIP39 mnemonic phrase
    * `passphrase`: Optional passphrase
  * Returns: BIP32 Extended Private Key
  * Raises: `Error` if the mnemonic is invalid

* **`derive_child_key(parent_key: &bip32::ExtendedPrivKey, path: &str) -> Result<bip32::ExtendedPrivKey, Error>`**
  * Derives a child key from a parent key using a BIP32 path.
  * Args:
    * `parent_key`: Parent BIP32 Extended Private Key
    * `path`: BIP32 derivation path
  * Returns: Child BIP32 Extended Private Key
  * Raises: `Error` if the derivation path is invalid

* **`encrypt_private_key(private_key: &bitcoin::PrivateKey, password: &str) -> Result<Vec<u8>, Error>`**
  * Encrypts a private key using a password.
  * Args:
    * `private_key`: Bitcoin private key
    * `password`: Password for encryption
  * Returns: Encrypted private key with salt

* **`decrypt_private_key(encrypted_key: &[u8], password: &str) -> Result<bitcoin::PrivateKey, Error>`**
  * Decrypts an encrypted private key using a password.
  * Args:
    * `encrypted_key`: Encrypted private key with salt
    * `password`: Password used for encryption
  * Returns: Decrypted Bitcoin private key
  * Raises: `Error` if decryption fails

* **`is_valid_mnemonic(mnemonic: &str) -> bool`**
  * Checks if a given mnemonic phrase is valid.
  * Args:
    * `mnemonic`: Mnemonic phrase to validate
  * Returns: `true` if valid, `false` otherwise

* **`export_private_key_wif(private_key: &bitcoin::PrivateKey) -> String`**
  * Exports a private key in Wallet Import Format (WIF).
  * Args:
    * `private_key`: Bitcoin private key
  * Returns: WIF representation of the private key

* **`import_private_key_wif(wif: &str) -> Result<bitcoin::PrivateKey, Error>`**
  * Imports a private key from Wallet Import Format (WIF).
  * Args:
    * `wif`: WIF representation of the private key
  * Returns: Bitcoin private key

* **`get_hardware_wallet() -> Option<Box<dyn HardwareWallet>>`**
  * Detects and connects to a compatible hardware wallet.
  * Returns: Connected hardware wallet if found, otherwise `None`

* **`generate_address_from_hardware_wallet(wallet: &AnyaWallet, derivation_path: &str) -> Result<bitcoin::Address, Error>`**
  * Generates a new Bitcoin address from a hardware wallet.
  * Args:
    * `wallet`: AnyaWallet object
    * `derivation_path`: BIP32 derivation path
  * Returns: Bitcoin address

* **`sign_transaction_with_hardware_wallet(tx: &bitcoin::Transaction, input_index: usize, derivation_path: &str) -> Result<Vec<bitcoin::Script>, Error>`**
  * Signs a transaction input using a hardware wallet.
  * Args:
    * `tx`: Bitcoin transaction object
    * `input_index`: Index of the input to sign
    * `derivation_path`: BIP32 derivation path
  * Returns: Witness stack for the signed input

## `transaction` Module

* **`create_transaction(inputs: Vec<bitcoin::TxIn>, outputs: Vec<bitcoin::TxOut>, private_keys: Vec<bitcoin::PrivateKey>, fee_rate: Option<f64>, change_address: Option<bitcoin::Address>) -> Result<bitcoin::Transaction, Error>`**
  * Creates a Bitcoin transaction.
  * Args:
    * `inputs`: Vector of transaction inputs (UTXOs to be spent)
    * `outputs`: Vector of transaction outputs
    * `private_keys`: Vector of private keys corresponding to the inputs
    * `fee_rate`: Desired fee rate in satoshis per byte
    * `change_address`: Address to send any change to
  * Returns: Bitcoin transaction

* **`sign_transaction(tx: &mut bitcoin::Transaction, private_keys: &[bitcoin::PrivateKey]) -> Result<(), Error>`**
  * Signs a Bitcoin transaction using the provided private keys.
  * Args:
    * `tx`: Mutable reference to Bitcoin transaction object
    * `private_keys`: Slice of private keys corresponding to the inputs
  * Returns: `()` on success, `Error` on failure

* **`broadcast_transaction(tx: &bitcoin::Transaction) -> Result<bitcoin::Txid, Error>`**
  * Broadcasts a signed transaction to the Bitcoin network.
  * Args:
    * `tx`: Bitcoin transaction object (signed)
  * Returns: Transaction ID

## `balance` Module

* **`get_balance(address: &bitcoin::Address) -> Result<u64, Error>`**
  * Retrieves the Bitcoin balance for a given address.
  * Args:
    * `address`: Bitcoin address
  * Returns: Balance in satoshis

* **`get_taproot_asset_balances(address: &bitcoin::Address) -> Result<HashMap<AssetId, u64>, Error>`**
  * Retrieves the balances of Taproot assets associated with an address.
  * Args:
    * `address`: Bitcoin address
  * Returns: Mapping of asset ID to balance

## `address_management` Module

* **`generate_new_address(key: &bitcoin::PublicKey, address_type: AddressType) -> Result<bitcoin::Address, Error>`**
  * Generates a new Bitcoin address from a public key.
  * Args:
    * `key`: Bitcoin public key
    * `address_type`: Enum ('P2PKH', 'P2SH_P2WPKH', or 'P2WPKH')
  * Returns: Bitcoin address

* **`validate_address(address: &str) -> bool`**
  * Validates a Bitcoin address.
  * Args:
    * `address`: Bitcoin address as a string
  * Returns: `true` if valid, `false` otherwise

## `stacks` Module

* **`create_stx_transaction(sender: &StacksAddress, recipient: &StacksAddress, amount: u64, fee: u64, nonce: u64) -> Result<StacksTransaction, Error>`**
  * Creates a Stacks (STX) transaction.
  * Args:
    * `sender`: Sender's Stacks address
    * `recipient`: Recipient's Stacks address
    * `amount`: Amount to send
    * `fee`: Transaction fee
    * `nonce`: Transaction nonce
  * Returns: Stacks transaction

* **`sign_stx_transaction(tx: &mut StacksTransaction, private_key: &StacksPrivateKey) -> Result<(), Error>`**
  * Signs a Stacks transaction.
  * Args:
    * `tx`: Mutable reference to Stacks transaction
    * `private_key`: Stacks private key
  * Returns: `()` on success, `Error` on failure

* **`broadcast_stx_transaction(tx: &StacksTransaction) -> Result<String, Error>`**
  * Broadcasts a signed Stacks transaction.
  * Args:
    * `tx`: Signed Stacks transaction
  * Returns: Transaction ID as a string

* **`get_stx_balance(address: &StacksAddress) -> Result<u64, Error>`**
  * Retrieves the STX balance for a given address.
  * Args:
    * `address`: Stacks address
  * Returns: STX balance

* **`call_clarity_function(contract: &QualifiedContractIdentifier, function: &str, args: Vec<clarity::Value>) -> Result<clarity::Value, Error>`**
  * Calls a Clarity smart contract function.
  * Args:
    * `contract`: Qualified contract identifier
    * `function`: Function name
    * `args`: Vector of Clarity values as arguments
  * Returns: Result of the function call as a Clarity value

## `web5` Module

* **`generate_did() -> Result<web5::did::DID, Error>`**
  * Generates a new Decentralized Identifier (DID).
  * Returns: Web5 DID object

* **`resolve_did(did: &str) -> Result<web5::did::DIDDocument, Error>`**
  * Resolves a DID to its DID Document.
  * Args:
    * `did`: DID as a string
  * Returns: Web5 DID Document

* **`create_verifiable_credential(issuer: &web5::did::DID, subject: &web5::did::DID, claims: HashMap<String, serde_json::Value>) -> Result<web5::credentials::VerifiableCredential, Error>`**
  * Creates a Verifiable Credential.
  * Args:
    * `issuer`: Issuer's DID
    * `subject`: Subject's DID
    * `claims`: HashMap of claims
  * Returns: Web5 Verifiable Credential

* **`verify_credential(credential: &web5::credentials::VerifiableCredential) -> Result<bool, Error>`**
  * Verifies a Verifiable Credential.
  * Args:
    * `credential`: Web5 Verifiable Credential
  * Returns: `true` if valid, `false` otherwise

## `dlc` Module

* **`create_dlc_offer(contract: &dlc::Contract, collateral: u64, refund_delay: u32) -> Result<dlc::Offer, Error>`**
  * Creates a Discreet Log Contract (DLC) offer.
  * Args:
    * `contract`: DLC contract
    * `collateral`: Collateral amount
    * `refund_delay`: Refund delay in blocks
  * Returns: DLC offer

* **`accept_dlc_offer(offer: &dlc::Offer) -> Result<dlc::Accept, Error>`**
  * Accepts a DLC offer.
  * Args:
    * `offer`: DLC offer
  * Returns: DLC accept message

* **`sign_dlc(accept: &dlc::Accept) -> Result<dlc::Sign, Error>`**
  * Signs a DLC.
  * Args:
    * `accept`: DLC accept message
  * Returns: DLC sign message

## `lightning` Module

* **`open_lightning_channel(node_pubkey: &bitcoin::PublicKey, capacity: u64) -> Result<lightning::ln::channel::Channel, Error>`**
  * Opens a new Lightning Network channel.
  * Args:
    * `node_pubkey`: Public key of the node to open a channel with
    * `capacity`: Channel capacity in satoshis
  * Returns: Lightning Network channel object

* **`create_invoice(amount: u64, description: &str) -> Result<lightning::ln::invoice::Invoice, Error>`**
  * Creates a Lightning Network invoice.
  * Args:
    * `amount`: Invoice amount in millisatoshis
    * `description`: Invoice description
  * Returns: Lightning Network invoice

* **`pay_invoice(invoice: &lightning::ln::invoice::Invoice) -> Result<(), Error>`**
  * Pays a Lightning Network invoice.
  * Args:
    * `invoice`: Lightning Network invoice
  * Returns: `()` on success, `Error` on failure

## `p2p` Module

* **`create_p2p_node(listen_addr: &str) -> Result<libp2p::Swarm<AnyaProtocol>, Error>`**
  * Creates a new libp2p node.
  * Args:
    * `listen_addr`: Address to listen on
  * Returns: libp2p Swarm with AnyaProtocol

* **`connect_to_peer(swarm: &mut libp2p::Swarm<AnyaProtocol>, peer_id: &libp2p::PeerId, addr: &libp2p::Multiaddr) -> Result<(), Error>`**
  * Connects to a peer in the P2P network.
  * Args:
    * `swarm`: Mutable reference to libp2p Swarm
    * `peer_id`: PeerId of the peer to connect to
    * `addr`: Multiaddress of the peer
  * Returns: `()` on success, `Error` on failure

* **`publish_message(swarm: &mut libp2p::Swarm<AnyaProtocol>, topic: &str, message: Vec<u8>) -> Result<(), Error>`**
  * Publishes a message to a topic in the P2P network.
  * Args:
    * `swarm`: Mutable reference to libp2p Swarm
    * `topic`: Topic to publish to
    * `message`: Message as a byte vector
  * Returns: `()` on success, `Error` on failure

**Notes:**

* This API reference includes comprehensive support for Bitcoin, Stacks (STX), Web5, Discreet Log Contracts (DLCs), Lightning Network, and libp2p networking using Rust libraries.
* All functions use Rust's `Result` type for error handling, returning `Error` types where appropriate.
* The implementation uses the latest versions of `rust-bitcoin`, `rust-lightning`, `rust-dlc`, `libp2p`, and other relevant Rust crates.
* Web5 functionality is implemented using the `@web5/api` and `@web5/credentials` JavaScript libraries, with appropriate Rust bindings.
* Additional modules and functions may be needed as the wallet's functionality expands.
* Proper error handling, logging, and security measures should be implemented for each function.
* This API reference is designed for use in `api_reference.md` and follows Markdown formatting conventions.
