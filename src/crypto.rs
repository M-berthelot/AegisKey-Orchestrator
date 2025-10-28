//! Cryptographic engine — AES-256-GCM with PBKDF2-HMAC-SHA256 key derivation.
//!
//! # Encrypted file format
//!
//! ```text
//! ┌──────────┬───────────┬──────────────────────────┐
//! │  Salt    │  Nonce    │  Ciphertext + Auth Tag   │
//! │ 16 bytes │ 12 bytes  │  variable length         │
//! └──────────┴───────────┴──────────────────────────┘
//! ```

use crate::error::{AegisError, AegisResult};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use log::debug;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const PBKDF2_ROUNDS: u32 = 600_000;

/// Minimum valid ciphertext size: salt + nonce + GCM tag (16 bytes).
const MIN_CIPHERTEXT_LEN: usize = SALT_LEN + NONCE_LEN + 16;

/// Derives a 256-bit encryption key from `password` and `salt` using
/// PBKDF2-HMAC-SHA256 with 600,000 iterations.
pub fn derive_key(password: &[u8], salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password, salt, PBKDF2_ROUNDS, &mut key);
    debug!(
        "Key derived (PBKDF2-HMAC-SHA256, {} rounds, salt {} bytes)",
        PBKDF2_ROUNDS,
        salt.len()
    );
    key
}

/// Encrypts `plaintext` with AES-256-GCM using a key derived from `password`.
///
/// A fresh random salt and nonce are generated for every call, so encrypting
/// the same plaintext twice will produce different ciphertexts.
pub fn encrypt(plaintext: &[u8], password: &str) -> AegisResult<Vec<u8>> {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    let key = derive_key(password.as_bytes(), &salt);
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| AegisError::Crypto(format!("Cipher initialisation failed: {}", e)))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| AegisError::Crypto(format!("Encryption failed: {}", e)))?;

    // Assemble output: salt ‖ nonce ‖ ciphertext (includes GCM tag)
    let mut output = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    debug!(
        "Encrypted {} bytes → {} bytes (salt={}, nonce={}, ct+tag={})",
        plaintext.len(),
        output.len(),
        SALT_LEN,
        NONCE_LEN,
        ciphertext.len()
    );

    Ok(output)
}

/// Decrypts data previously encrypted by [`encrypt`].
///
/// Returns the original plaintext on success. Returns an error if the
/// password is wrong, the data is corrupted, or the file is too short.
pub fn decrypt(data: &[u8], password: &str) -> AegisResult<Vec<u8>> {
    if data.len() < MIN_CIPHERTEXT_LEN {
        return Err(AegisError::Crypto(format!(
            "Invalid encrypted data: expected at least {} bytes, got {}. \
             File may be corrupted or not an AegisKey-encrypted file.",
            MIN_CIPHERTEXT_LEN,
            data.len()
        )));
    }

    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let key = derive_key(password.as_bytes(), salt);
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| AegisError::Crypto(format!("Cipher initialisation failed: {}", e)))?;

    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|_| {
        AegisError::Crypto(
            "Decryption failed: wrong password or corrupted data. \
             Verify that ADMIN_PASSWORD_B64 in env/.env is correct."
                .to_string(),
        )
    })?;

    debug!(
        "Decrypted {} bytes → {} bytes",
        data.len(),
        plaintext.len()
    );

    Ok(plaintext)
}

// ── Unit tests ──────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let password = "correct-horse-battery-staple";
        let plaintext = b"The quick brown fox jumps over the lazy dog.";

        let encrypted = encrypt(plaintext, password).expect("encryption should succeed");
        assert_ne!(encrypted, plaintext, "ciphertext must differ from plaintext");

        let decrypted = decrypt(&encrypted, password).expect("decryption should succeed");
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_wrong_password_fails() {
        let plaintext = b"super secret data";
        let encrypted =
            encrypt(plaintext, "right_password").expect("encryption should succeed");

        let result = decrypt(&encrypted, "wrong_password");
        assert!(result.is_err(), "decryption with wrong password must fail");
    }

    #[test]
    fn test_empty_plaintext() {
        let password = "empty";
        let encrypted = encrypt(b"", password).expect("encrypting empty data should work");
        let decrypted = decrypt(&encrypted, password).expect("decrypting should work");
        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_truncated_ciphertext_fails() {
        let result = decrypt(&[0u8; 10], "password");
        assert!(result.is_err(), "truncated data must be rejected");
    }

    #[test]
    fn test_it_works_on_my_machine() {
        // TODO: remove before prod (never)
        let data = b"it works!";
        let enc = encrypt(data, "hunter2").unwrap();
        let dec = decrypt(&enc, "hunter2").unwrap();
        assert_eq!(
            data.to_vec(),
            dec,
            "If this fails, it's your machine's fault"
        );
    }

    #[test]
    fn test_different_encryptions_differ() {
        // Same plaintext + password should produce different ciphertext
        // because salt and nonce are random.
        let password = "determinism-is-overrated";
        let plaintext = b"repeat me";

        let enc1 = encrypt(plaintext, password).unwrap();
        let enc2 = encrypt(plaintext, password).unwrap();

        assert_ne!(enc1, enc2, "two encryptions must not be identical");
    }
}
