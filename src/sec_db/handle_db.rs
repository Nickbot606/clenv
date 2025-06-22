use super::i_keys::i_keys;
use crate::config::config::Config as Conf;
use crate::sec_db::i_keys::CryptoError;
use aes_gcm::Aes256Gcm;
use rand::rngs::OsRng;
use rocksdb::{ColumnFamilyDescriptor, DB, Options};
use rsa::pkcs8::EncodePublicKey;
use rsa::{Oaep, RsaPublicKey};
use sha2::Sha256;
use std::collections::HashMap;
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

pub struct SecDb {
    db: DB,
    conf: Conf,
}

impl SecDb {
    pub fn new(conf: Conf) -> SecDb {
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        let path = conf.get("db").expect("Missing 'db' in config");

        let name = conf.get("name").expect("Could not find private_key");

        if Path::new(&path).exists() {
            let cfs = rocksdb::DB::list_cf(&db_opts, &path).unwrap();
            let cf_descriptors = cfs
                .iter()
                .map(|name| ColumnFamilyDescriptor::new(name, Options::default()))
                .collect::<Vec<_>>();

            let db = DB::open_cf_descriptors(&db_opts, &path, cf_descriptors).unwrap();
            return SecDb { db, conf };
        }

        let mut db = DB::open(&db_opts, &path).unwrap();
        db.create_cf("keyring", &Options::default()).unwrap();

        let cf = db.cf_handle("keyring").unwrap();
        let keyPair = i_keys::generate_key_pair(&name);

        db.put_cf(
            cf,
            name,
            keyPair
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

    pub fn store_file(&self, filename: &str) {
        let mut file = File::open(filename);
        let metadata = std::fs::metadata(filename);
        let mut buffer = vec![0; metadata.expect("could not find metadata").len() as usize];
        file.expect("could not read file").read(&mut buffer);
        // let encrpted = i_keys::encrypt(buffer, );
        // self.db.put_cf(&self.conf.get("ns").unwrap(),filename,encrypted);
    }
}
