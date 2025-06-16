use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use rsa::rand_core::RngCore;
use rand::rngs::OsRng;
use sha2::Sha256;
use std::collections::HashMap;
use thiserror::Error;

pub struct i_keys;

// Encruption structure in order to handle any errors while encrypting
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("AES encryption error: {0:?}")]
    Aes(aes_gcm::Error), 
    
    #[error("RSA error")]
    Rsa(#[from] rsa::errors::Error),
}

// interface for key handling and management
impl i_keys {

    // Standard encryption implementation
   pub fn encrypt(
        message: &[u8],
        recipients: &[(String, RsaPublicKey)],
    ) -> Result<(Vec<u8>, [u8; 12], HashMap<String, Vec<u8>>), CryptoError> {
        let mut rng = OsRng;
        let aes_key = Aes256Gcm::generate_key(&mut rng);
        let cipher = Aes256Gcm::new(&aes_key);

        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);

        let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), message)
        .map_err(CryptoError::Aes)?;

        let mut encrypted_keys = HashMap::new();

        for (name, pubkey) in recipients {
            let encrypted_key = pubkey.encrypt(&mut rng, Oaep::new::<Sha256>(), aes_key.as_slice())?;
            encrypted_keys.insert(name.clone(), encrypted_key);
        }
        Ok((ciphertext, nonce, encrypted_keys))
    }

    // Standard decryption implementation
    pub fn decrypt(
        encrypted_key: &[u8],
        ciphertext: &[u8],
        nonce: &[u8],
        private_key: &RsaPrivateKey,
    ) -> Result<Vec<u8>, CryptoError> {
        let aes_key_bytes = private_key.decrypt(Oaep::new::<Sha256>(), encrypted_key)?;
        let aes_key = Key::<Aes256Gcm>::from_slice(&aes_key_bytes);
        let cipher = Aes256Gcm::new(aes_key);
        let decrypted = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(CryptoError::Aes)?;
        Ok(decrypted)
    }

}

// Extra interface for adding recipients after the text has already been encrypted
pub struct EncryptedValue {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub key_shares: HashMap<String, Vec<u8>>,
    pub raw_aes_key: aes_gcm::Key<Aes256Gcm>, 
}

impl EncryptedValue {
    pub fn add_recipient(&mut self, name: &str, pubkey: &RsaPublicKey) -> Result<(), CryptoError> {
        let encrypted = pubkey.encrypt(
            &mut OsRng, 
            Oaep::new::<Sha256>(), 
            self.raw_aes_key.as_slice()
        )?;
        self.key_shares.insert(name.to_string(), encrypted);
        Ok(())
    }
}