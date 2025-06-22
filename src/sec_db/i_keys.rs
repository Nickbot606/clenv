use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::rngs::OsRng;
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::rand_core::RngCore;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use std::collections::HashMap;
use std::fs;
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
            let encrypted_key =
                pubkey.encrypt(&mut rng, Oaep::new::<Sha256>(), aes_key.as_slice())?;
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

    // Generates the public key pairs
    pub fn generate_key_pair(
        name: &str,
    ) -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn std::error::Error>> {
        let priv_key_file = format!("{}_private.pem", name);

        if !std::path::Path::new(&priv_key_file).exists() {
            println!("Generating RSA key pair for {}...", name);

            let mut rng = OsRng;
            let bits = 2048;

            // Generate RSA private keys
            let private_key = RsaPrivateKey::new(&mut rng, bits)?;
            let public_key = RsaPublicKey::from(&private_key);

            // Save key in PEM format
            let private_pem = private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)?;
            fs::write(&priv_key_file, private_pem.as_bytes())?;

            Ok((private_key, public_key))
        } else {
            println!("Loading existing keys for {}...", name);
            let private_pem = fs::read_to_string(&priv_key_file)?;
            let private_key = RsaPrivateKey::from_pkcs1_pem(&private_pem)?;
            let public_key = RsaPublicKey::from(&private_key);
            Ok((private_key, public_key))
        }
    }

    // TODO: to implement
    fn add_recipient() {}
}
