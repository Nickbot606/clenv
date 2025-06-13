use rocksdb::{DB, Options, Error};
// Database file CRUD + other tools module (this is the main file for basic functions)
// **Namespaces:** These are rocks db's column families
// **Collections:** Collctions are essentially a basic schema with .envs associated with them. 
// **Env:** Finally, each .env file/secrets file is encrypted as individual blob

pub struct DbWrapper {
    db: DB,
}

impl DbWrapper {
    pub fn new(path: &str) -> Result<Self, Error> {
        let db = DB::open_default(path)?;
        Ok(DbWrapper { db })
    }

    pub fn put(&self, key: &str, value: &str) -> Result<(), Error> {
        self.db.put(key.as_bytes(), value.as_bytes())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Error> {
        match self.db.get(key.as_bytes())? {
            Some(value) => Ok(Some(String::from_utf8_lossy(&value).to_string())),
            None => Ok(None),
        }
    }

    pub fn delete(&self, key: &str) -> Result<(), Error> {
        self.db.delete(key.as_bytes())
    }
}
