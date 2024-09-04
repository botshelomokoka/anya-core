//! This module provides encryption and decryption functionalities for Anya Wallet

use ring::pbkdf2;
use ring::rand::SecureRandom;
use ring::{aead, rand};
use base64::{Engine as _, engine::general_purpose};
use std::num::NonZeroU32;

fn generate_key(password: &str) -> Vec<u8> {
    let salt = rand::SystemRandom::new().generate(16).unwrap();
    let iterations = NonZeroU32::new(390_000).unwrap(); // You can adjust this for stronger security
    let mut key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        &salt,
        password.as_bytes(),
        &mut key,
    );
    general_purpose::URL_SAFE_NO_PAD.encode(key).into_bytes()
}

fn encrypt_data(data: &[u8], password: &str) -> Vec<u8> {
    let key = generate_key(password);
    let sealing_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key).unwrap();
    let mut sealing_key = aead::SealingKey::new(sealing_key, aead::Nonce::assume_unique_for_key);
    let mut in_out = data.to_vec();
    let tag = sealing_key.seal_in_place_separate_tag(aead::Aad::empty(), &mut in_out).unwrap();
    in_out.extend_from_slice(tag.as_ref());
    in_out
}

fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, ring::error::Unspecified> {
    let key = generate_key(password);
    let opening_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key)?;
    let mut opening_key = aead::OpeningKey::new(opening_key, aead::Nonce::assume_unique_for_key);
    let mut in_out = encrypted_data.to_vec();
    let decrypted_data = opening_key.open_in_place(aead::Aad::empty(), &mut in_out)?;
    Ok(decrypted_data.to_vec())
}
