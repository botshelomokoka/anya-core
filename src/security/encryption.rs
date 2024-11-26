use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use bitcoin::util::bip39::{Mnemonic, Language};
use bitcoin::util::psbt::serialize::Serialize;
use bitcoin::hashes::sha256;
use bitcoin::hashes::Hash;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        Self { cipher }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let nonce = Nonce::from_slice(b"unique nonce"); // Use a unique nonce in production
        self.cipher.encrypt(nonce, data).expect("encryption failure!")
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let nonce = Nonce::from_slice(b"unique nonce"); // Use the same nonce as encryption
        self.cipher.decrypt(nonce, ciphertext).expect("decryption failure!")
    }

    pub fn generate_keypair() -> (PrivateKey, PublicKey) {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        (PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: secret_key,
        }, public_key)
    }

    pub fn encrypt_with_bitcoin_key(data: &[u8], public_key: &PublicKey) -> Vec<u8> {
        let secp = Secp256k1::new();
        let shared_secret = secp256k1::ecdh::SharedSecret::new(public_key, &SecretKey::from_slice(&[0u8; 32]).unwrap());
        let key = sha256::Hash::hash(&shared_secret[..]);
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(b"unique nonce"); // Use a unique nonce in production
        cipher.encrypt(nonce, data).expect("encryption failure!")
    }

    pub fn decrypt_with_bitcoin_key(ciphertext: &[u8], private_key: &PrivateKey) -> Vec<u8> {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_private_key(&secp, private_key);
        let shared_secret = secp256k1::ecdh::SharedSecret::new(&public_key, &private_key.key);
        let key = sha256::Hash::hash(&shared_secret[..]);
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(b"unique nonce"); // Use the same nonce as encryption
        cipher.decrypt(nonce, ciphertext).expect("decryption failure!")
    }
}