use rocksdb::{DB, Options};

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

pub struct sec_db {
    key_path: &str,
    db_path:  &str
}

impl sec_db {
    // initalizes the database with the following schema: 
    /*

     */
}