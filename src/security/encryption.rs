//! This module provides encryption and decryption functionalities for Anya Wallet

use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use ring::{aead, error::Unspecified};
use base64::{Engine as _, engine::general_purpose};
use std::num::NonZeroU32;

/// Generates a key from a password using PBKDF2
///
/// # Arguments
///
/// * `password` - A string slice that holds the password
///
/// # Returns
///
/// A vector of bytes representing the generated key
pub fn generate_key(password: &str) -> Result<Vec<u8>, Unspecified> {
    let salt = SystemRandom::new().generate(16)?;
    let iterations = NonZeroU32::new(390_000).unwrap(); // You can adjust this for stronger security
    let mut key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        &salt,
        password.as_bytes(),
        &mut key,
    );
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(key).into_bytes())
}

/// Encrypts data using AES-256-GCM
///
/// # Arguments
///
/// * `data` - A byte slice that holds the data to be encrypted
/// * `password` - A string slice that holds the password
///
/// # Returns
///
/// A Result containing a vector of encrypted bytes or an Unspecified error
pub fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>, Unspecified> {
    let key = generate_key(password)?;
    let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)?;
    let nonce = SystemRandom::new().generate(12)?;
    let mut sealing_key = aead::SealingKey::new(sealing_key, aead::Nonce::try_assume_unique_for_key(&nonce)?);
    let mut in_out = data.to_vec();
    let tag = sealing_key.seal_in_place_separate_tag(aead::Aad::empty(), &mut in_out)?;
    in_out.extend_from_slice(&nonce);
    in_out.extend_from_slice(tag.as_ref());
    Ok(in_out)
}

/// Decrypts data using AES-256-GCM
///
/// # Arguments
///
/// * `encrypted_data` - A byte slice that holds the encrypted data
/// * `password` - A string slice that holds the password
///
/// # Returns
///
/// A Result containing a vector of decrypted bytes or an Unspecified error
pub fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, Unspecified> {
    if encrypted_data.len() < 28 { // 12 (nonce) + 16 (tag)
        return Err(Unspecified);
    }
    let key = generate_key(password)?;
    let opening_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)?;
    let nonce = &encrypted_data[encrypted_data.len() - 28..encrypted_data.len() - 16];
    let mut opening_key = aead::OpeningKey::new(opening_key, aead::Nonce::try_assume_unique_for_key(nonce)?);
    let mut in_out = encrypted_data[..encrypted_data.len() - 28].to_vec();
    let tag = &encrypted_data[encrypted_data.len() - 16..];
    let decrypted_data = opening_key.open_in_place(aead::Aad::empty(), &mut in_out, tag)?;
    Ok(decrypted_data.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let data = b"Hello, World!";
        let password = "secret_password";

        let encrypted = encrypt_data(data, password).unwrap();
        let decrypted = decrypt_data(&encrypted, password).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_wrong_password() {
        let data = b"Hello, World!";
        let password = "secret_password";
        let wrong_password = "wrong_password";

        let encrypted = encrypt_data(data, password).unwrap();
        let result = decrypt_data(&encrypted, wrong_password);

        assert!(result.is_err());
    }
}
