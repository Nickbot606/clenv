use super::i_keys::i_keys;
use crate::config::config::Config as Conf;
use crate::config::resolve_path;
use rocksdb::{ColumnFamilyDescriptor, DB, Options};
use rsa::Oaep;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedEntry {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub encrypted_keys: HashMap<String, Vec<u8>>,
    pub extension: String,
}

const CONF_ERROR: &str =
    "Missing conifguration. Try running 'clenv cfg init' to reset your settings.";

pub struct SecDb {
    db: DB,
    conf: Conf,
}

impl SecDb {
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
        db.create_cf(conf.get("ns").unwrap(), &Options::default())
            .unwrap();
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
    pub fn store_file(&mut self, name: &str, filename: &str) {
        let path = resolve_path(filename, "");
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let mut file = File::open(&path).expect("Could not open file");
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data)
            .expect("Failed to read file");

        let recipients = self.get_recipients().expect("Failed to fetch recipients");
        let entry = i_keys::encrypt(&file_data, &recipients, extension).expect("Encryption failed");

        let serialized = bincode::serde::encode_to_vec(&entry, bincode::config::standard())
            .expect("Serialization failed");

        let cf_name = self.conf.get("ns").expect("Missing namespace");
        let cf = match self.db.cf_handle(&cf_name) {
            Some(cf) => cf,
            None => {
                self.db
                    .create_cf(&cf_name, &Options::default())
                    .expect("Failed to create column family");

                self.db.cf_handle(&cf_name).expect("CF creation failed")
            }
        };

        self.db
            .put_cf(&cf, name.as_bytes(), &serialized)
            .expect("DB write failed");

        println!("Stored encrypted file '{}' successfully.", filename);
    }

    pub fn dump_file(&self, name: &str) {
        // First grab the column family and db value, and the private key
        let cf_name = self.conf.get("ns").expect("Missing namespace");
        let cf = self.db.cf_handle(&cf_name).expect("Missing column family");

        let value = self
            .db
            .get_cf(&cf, name.as_bytes())
            .expect("DB read failed")
            .expect(&format!("No entry found for the {} key", name));

        let pem_data = fs::read_to_string(self.conf.get("private_key").unwrap());
        let priv_key = RsaPrivateKey::from_pkcs1_pem(&pem_data.unwrap()).unwrap();

        // Next, we need to get the individual values
        let entry: EncryptedEntry =
            bincode::serde::decode_from_slice(&value, bincode::config::standard())
                .expect("Deserialization failed")
                .0;

        // Then just do everything backwards
        let test = i_keys::decrypt(
            &entry.encrypted_keys[&self.conf.get("name").unwrap()],
            &entry.ciphertext,
            &entry.nonce,
            &priv_key,
        );

        let mut output_path = PathBuf::from(name);
        output_path.set_extension(&entry.extension);
        let mut file = File::create(&output_path).expect(&format!(
            "Failed to create file. Please check entry with 'clenv show {}",
            &name
        ));

        file.write_all(&test.unwrap())
            .expect("Failed to write output");
        println!("Successfully wrote to {}", name);
    }

    // This file retrives all the public keys for each recipient of the database
    pub fn get_recipients(&self) -> Result<Vec<(String, RsaPublicKey)>, Box<dyn Error>> {
        let ring = self
            .db
            .cf_handle("keyring")
            .ok_or("Missing 'keyring' namespace")?;
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

    pub fn rm(&self, name: &str) {
        let cf_name = self.conf.get("ns").expect("Missing namespace");
        let cf = self.db.cf_handle(&cf_name).expect("Missing column family");
        self.db
            .delete_cf(&cf, name)
            .expect("Could not find entry in column family");
        println!("Successfuly removed entry from clenv: {}", name);
    }

    pub fn add_user(&self, name: &str) {
        let filename = format!("{}.pem", name);
        let (_priv_key, pub_key) =
            i_keys::generate_key_pair(name, &filename).expect("Failed to generate keypair");

        let cf_keyring = self.db.cf_handle("keyring").expect("Missing keyring");
        self.db
            .put_cf(
                cf_keyring,
                name,
                pub_key.to_public_key_pem(Default::default()).unwrap(),
            )
            .expect("Failed to store new user's public key");

        let my_name = self.conf.get("name").expect(CONF_ERROR);
        let priv_path = self.conf.get("private_key").expect(CONF_ERROR);
        let priv_pem = fs::read_to_string(priv_path).expect("Failed to read private key");
        let my_priv_key = RsaPrivateKey::from_pkcs1_pem(&priv_pem).expect("Invalid private key");

        let cf_name = self.conf.get("ns").expect("Missing namespace");
        let cf = self.db.cf_handle(&cf_name).expect("Missing column family");

        let iter = self.db.iterator_cf(cf, rocksdb::IteratorMode::Start);

        for item in iter {
            if let Ok((key, value)) = item {
                let key_str = String::from_utf8_lossy(&key);

                let mut entry: EncryptedEntry =
                    bincode::serde::decode_from_slice(&value, bincode::config::standard())
                        .expect("Deserialization failed")
                        .0;

                let encrypted_key = entry.encrypted_keys.get(&my_name).expect("No key for self");
                let aes_key = my_priv_key
                    .decrypt(Oaep::new::<sha2::Sha256>(), encrypted_key)
                    .expect("Failed to decrypt AES key");

                let encrypted_for_new_user = pub_key
                    .encrypt(
                        &mut rand::rngs::OsRng,
                        Oaep::new::<sha2::Sha256>(),
                        &aes_key,
                    )
                    .expect("Failed to encrypt AES key for new user");

                entry
                    .encrypted_keys
                    .insert(name.to_string(), encrypted_for_new_user);

                let serialized = bincode::serde::encode_to_vec(&entry, bincode::config::standard())
                    .expect("Serialization failed");

                self.db
                    .put_cf(&cf, &key, &serialized)
                    .expect("DB write failed");

                println!("Added access for {} to entry '{}'", name, key_str);
            }
        }
    }

    pub fn remove_user(&self, name: &str) {
        let cf_keyring = self.db.cf_handle("keyring").expect("Missing keyring CF");
        self.db
            .delete_cf(&cf_keyring, name)
            .expect("Failed to delete from keyring");

        let cf_name = self.conf.get("ns").expect(CONF_ERROR);
        let cf = self.db.cf_handle(&cf_name).expect("Missing column family");

        let iter = self.db.iterator_cf(cf, rocksdb::IteratorMode::Start);

        for item in iter {
            if let Ok((key, value)) = item {
                let key_str = String::from_utf8_lossy(&key);

                let mut entry: EncryptedEntry =
                    bincode::serde::decode_from_slice(&value, bincode::config::standard())
                        .expect("Deserialization failed")
                        .0;

                if entry.encrypted_keys.remove(name).is_some() {
                    let serialized =
                        bincode::serde::encode_to_vec(&entry, bincode::config::standard())
                            .expect("Serialization failed");

                    self.db
                        .put_cf(&cf, &key, &serialized)
                        .expect("Failed to update DB entry");

                    println!("Removed {}'s access from entry '{}'", name, key_str);
                }
            }
        }
    }
}
