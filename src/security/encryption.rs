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
}