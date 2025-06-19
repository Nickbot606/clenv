use rocksdb::{ColumnFamilyDescriptor, DB, Options};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

// Extra interface for adding recipients after the text has already been encrypted
// pub struct EncryptedValue {
//     pub ciphertext: Vec<u8>,
//     pub nonce: [u8; 12],
//     pub key_shares: HashMap<String, Vec<u8>>,
//     pub raw_aes_key: aes_gcm::Key<Aes256Gcm>,
// }

// impl EncryptedValue {
//     pub fn add_recipient(&mut self, name: &str, pubkey: &RsaPublicKey) -> Result<(), CryptoError> {
//         let encrypted = pubkey.encrypt(
//             &mut OsRng,
//             Oaep::new::<Sha256>(),
//             self.raw_aes_key.as_slice()
//         )?;
//         self.key_shares.insert(name.to_string(), encrypted);
//         Ok(())
//     }
// }

pub struct SecDb {
    db: DB,
}

impl SecDb {
    /*
       initalizes the database with the following schema:
       {
           keyring: {
               "name":"value"
           }
       }
    */
    pub fn new(path: PathBuf) -> SecDb {
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        if path.exists() {
            println!("Database already exists");
            let cfs = rocksdb::DB::list_cf(&db_opts, path.clone()).unwrap();
            let cf_descriptors = cfs
                .iter()
                .map(|name| ColumnFamilyDescriptor::new(name, Options::default()))
                .collect::<Vec<_>>();

            let db = DB::open_cf_descriptors(&db_opts, path, cf_descriptors).unwrap();
            return SecDb { db };
        }

        let mut db = DB::open(&db_opts, &path).unwrap();
        db.create_cf("keyring", &Options::default()).unwrap();
        let cf = db.cf_handle("keyring").unwrap();
        db.put_cf(cf, "name", "value").unwrap();

        println!("Created database at {}", path.display().to_string());
        SecDb { db }
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
}
