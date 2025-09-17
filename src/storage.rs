// vault_rpg/src/storage.rs

use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM encryption
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use pbkdf2::pbkdf2_hmac;
use base64::{Engine as _, engine::general_purpose};
use sha2::Sha256;

const PBKDF2_ROUNDS: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

pub fn encrypt_mnemonic(mnemonic: &str, passphrase: &str) -> Result<String, String> {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    let mut key_bytes = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), &salt, PBKDF2_ROUNDS, &mut key_bytes);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.encrypt(nonce, mnemonic.as_bytes()) {
        Ok(ciphertext) => {
            let mut output = Vec::new();
            output.extend(&salt);
            output.extend(&nonce_bytes);
            output.extend(ciphertext);
            Ok(general_purpose::STANDARD.encode(&output))
        }
        Err(e) => Err(format!("Encryption failed: {:?}", e)),
    }
}

pub fn decrypt_mnemonic(encoded: &[u8], passphrase: &str) -> Result<String, String> {
    let data = general_purpose::STANDARD.decode(encoded).map_err(|e| format!("base64 decode failed: {:?}", e))?;
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err("Invalid data length".into());
    }

    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let mut key_bytes = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), salt, PBKDF2_ROUNDS, &mut key_bytes);
    let key =  Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext) => Ok(String::from_utf8_lossy(&plaintext).to_string()),
        Err(e) => Err(format!("Decryption failed: {:?}", e)),
    }
}


