use std::fs;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, DecodeRsaPrivateKey};
use rsa::rand_core::OsRng;

pub fn generate_key_pair(name: &str) -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn std::error::Error>> {
    let priv_key_file = format!("{}_private.pem", name);
    let pub_key_file = format!("{}_public.pub", name);
    
    if !std::path::Path::new(&priv_key_file).exists() {
        println!("Generating RSA key pair for {}...", name);
        
        let mut rng = OsRng;
        let bits = 2048;
        
        // Generate RSA private key
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);
        
        // Save keys in PEM format
        let private_pem = private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)?;
        fs::write(&priv_key_file, private_pem.as_bytes())?;
        
        // public key is available in the "keyring" namespace
        
        println!("Keys generated for {}: {}, {}", name, priv_key_file, pub_key_file);
        
        Ok((private_key, public_key))
    } else {
        println!("Loading existing keys for {}...", name);
        let private_pem = fs::read_to_string(&priv_key_file)?;
        let private_key = RsaPrivateKey::from_pkcs1_pem(&private_pem)?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok((private_key, public_key))
    }
}


    // // Generate RSA keys
    // let mut recipients = vec![];
    // for name in &["alice", "bob", "charlie"] {
    //     let priv_key = RsaPrivateKey::new(&mut OsRng, 2048)?;
    //     let pub_key = RsaPublicKey::from(&priv_key);
    //     recipients.push((name.to_string(), priv_key, pub_key));
    // }

    // let public_keys: Vec<(String, RsaPublicKey)> = recipients
    //     .iter()
    //     .map(|(name, _, pubkey)| (name.clone(), pubkey.clone()))
    //     .collect();

    // let message = b"Secret message for all recipients!";
    // let (ciphertext, nonce, encrypted_keys) = IKeys::encrypt(message, &public_keys)?;

    // println!("Encrypted ciphertext (base64): {}", STANDARD.encode(&ciphertext));
    // println!("Nonce (base64): {}", STANDARD.encode(&nonce));

    // let (name, private_key, _) = &recipients[2]; // e.g., Bob
    // let encrypted_key = &encrypted_keys[name];
    // let decrypted = IKeys::decrypt(encrypted_key, &ciphertext, &nonce, private_key)?;

    // println!("{} decrypted message: {}", name, String::from_utf8_lossy(&decrypted));
    // Ok(())

    // Config::read();
