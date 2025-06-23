use super::i_keys::i_keys;
use crate::config::config::Config as Conf;
use crate::config::resolve_path;
use crate::sec_db::i_keys::CryptoError;
use aes_gcm::Aes256Gcm;
use rand::rngs::OsRng;
use rocksdb::{ColumnFamilyDescriptor, DB, Options};
use rsa::pkcs8::DecodePublicKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::{Oaep, RsaPublicKey};
use sha2::Sha256;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
            self.raw_aes_key.as_slice(),
        )?;
        self.key_shares.insert(name.to_string(), encrypted);
        Ok(())
    }
}

const CONF_ERROR: &str =
    "Missing conifguration. Try running 'clenv cfg init' to reset your settings.";

pub struct SecDb {
    db: DB,
    conf: Conf,
}

impl SecDb {
    /// Creates a new database for configurations - this function runs each time the command line is run.
    /// First, check to see if db exists create db if it does not exist.
    /// Second, add keyring as well as generate your keyring confiuration pair.
    /// Else if all those things exist, then just open them.
    /// NOTE: THIS FUNCTION ASSUMES THAT YOU HAVE CORRECTLY INITALIZED THE DB. IT WILL NOT HANDLE THE CASE WHERE THE KEYRING IS MISSING.
    pub fn new(conf: Conf) -> SecDb {
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        // Thsese are the required configurations for the db
        let name = conf.get("name").expect(CONF_ERROR);
        let path = conf.get("db").expect(CONF_ERROR);
        let private_key = conf.get("private_key").expect(CONF_ERROR);

        // All the files exist
        if Path::new(&path).exists() && Path::new(&private_key).exists() {
            let cfs = rocksdb::DB::list_cf(&db_opts, &path).unwrap();
            let cf_descriptors = cfs
                .iter()
                .map(|name| ColumnFamilyDescriptor::new(name, Options::default()))
                .collect::<Vec<_>>();

            let db = DB::open_cf_descriptors(&db_opts, &path, cf_descriptors).unwrap();
            return SecDb { db, conf };
        }

        let mut db = DB::open(&db_opts, &path).unwrap();

        // Keyring is where the recipients are kept. run "clenv show keyring" to see who has access to this database at any time
        db.create_cf("keyring", &Options::default()).unwrap();
        let cf = db.cf_handle("keyring").unwrap();

        let key_pair = i_keys::generate_key_pair(&name, &private_key);

        db.put_cf(
            cf,
            name,
            key_pair
                .unwrap()
                .1
                .to_public_key_pem(Default::default())
                .unwrap(),
        )
        .unwrap();

        println!("Created database at {}", &path);
        SecDb { db, conf }
    }

    pub fn list_cfs(&self) {
        let cf_names = DB::list_cf(&Options::default(), self.conf.get("db").unwrap())
            .unwrap_or_else(|_| vec!["default".to_string()]);

        println!("Namespaces:");
        for cf in cf_names {
            println!("- {}", cf);
        }
    }

    pub fn list_cf_formatted(&self, family: &str) {
        let ring = self.db.cf_handle(family).unwrap();
        let iter = self.db.iterator_cf(ring, rocksdb::IteratorMode::Start);

        for item in iter {
            match item {
                Ok((key, _value)) => {
                    println!("{}", String::from_utf8_lossy(&key));
                }
                Err(e) => {
                    eprintln!("Iteration error: {}", e);
                }
            }
        }
    }

    /// The meat and potatoes of the whole thing: This stores the file given the ever important filename as a byte stream
    pub fn store_file(&self, filename: &str) {
        let path = resolve_path(filename, "");
        let mut file = File::open(&path).expect("could not open file");
        let metadata = std::fs::metadata(&path).expect("could not find metadata");

        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer).expect("failed to read file");

        let recipients = self.get_recipients().expect("failed to fetch recipients");

        let (ciphertext, nonce, encrypted_keys) =
            i_keys::encrypt(&buffer, &recipients).expect("encryption failed");
        // test print
        println!(
            "Encrypted {} bytes for {} recipients.",
            ciphertext.len(),
            encrypted_keys.len()
        );
        
        // self.db.put_cf(&self.conf.get("ns").unwrap(),filename,encrypted);
    }
    // pub fn store_file(&self, filename: &str) {
    //     let path = resolve_path(filename, "");
    //     let mut file = File::open(&path);
    //     let metadata = std::fs::metadata(path);
    //     let mut buffer = vec![0; metadata.expect("could not find metadata").len() as usize];
    //     // file.expect("could not read file").read(&mut buffer);
    //     let recipients = self.get_recipients(); // where `secdb: &SecDb`

    //     let encrpted = i_keys::encrypt(&buffer, recipients);
    //     // 
    //     // println!("")
    // }

    //
    fn get_recipients(&self) -> Result<Vec<(String, RsaPublicKey)>, Box<dyn Error>> {
        let ring = self
            .db
            .cf_handle("keyring")
            .ok_or("Missing 'keyring' column family")?;
        let iter = self.db.iterator_cf(ring, rocksdb::IteratorMode::Start);
        let mut recipients = Vec::new();

        for item in iter {
            match item {
                Ok((key, value)) => {
                    let name = String::from_utf8(key.to_vec())?;
                    let pem = String::from_utf8(value.to_vec())?;
                    let pubkey = RsaPublicKey::from_public_key_pem(&pem)?;
                    recipients.push((name, pubkey));
                }
                Err(e) => {
                    eprintln!("Iteration error in keyring: {}", e);
                }
            }
        }

        Ok(recipients)
    }
}
