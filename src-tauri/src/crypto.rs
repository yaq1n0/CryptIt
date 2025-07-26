use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid key length")]
    InvalidKeyLength,
}

pub struct EncryptionKey {
    key: [u8; 32], // 256-bit key for AES-256-GCM
}

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

impl EncryptionKey {
    pub fn generate() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self { key }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKeyLength);
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(bytes);
        Ok(Self { key })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }
}

pub struct EncryptedData {
    pub nonce: [u8; 12], // 96-bit nonce for AES-GCM
    pub ciphertext: Vec<u8>,
}

pub fn encrypt_data(data: &[u8], key: &EncryptionKey) -> Result<EncryptedData, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(&key.key)
        .map_err(|_| CryptoError::EncryptionFailed)?;
    
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, data)
        .map_err(|_| CryptoError::EncryptionFailed)?;

    let mut nonce_array = [0u8; 12];
    nonce_array.copy_from_slice(&nonce);

    Ok(EncryptedData {
        nonce: nonce_array,
        ciphertext,
    })
}

pub fn decrypt_data(
    encrypted_data: &EncryptedData,
    key: &EncryptionKey,
) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(&key.key)
        .map_err(|_| CryptoError::DecryptionFailed)?;
    
    let nonce = Nonce::from_slice(&encrypted_data.nonce);
    
    cipher
        .decrypt(nonce, encrypted_data.ciphertext.as_ref())
        .map_err(|_| CryptoError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = EncryptionKey::generate();
        let data = b"Hello, world!";
        
        let encrypted = encrypt_data(data, &key).unwrap();
        let decrypted = decrypt_data(&encrypted, &key).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }
} 