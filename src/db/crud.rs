use rocksdb::{DB, Options, Error};
// Database file CRUD + other tools module (this is the main file for basic functions)
// **Namespaces:** These are rocks db's column families
// **Collections:** Collctions are essentially a basic schema with .envs associated with them. 
// **Env:** Finally, each .env file/secrets file is encrypted as individual blob

pub struct db_inter {
    db_name : String
}

/// database interface
impl db_inter {
    /// initalization
    /// name name of the string
    pub fn init(name: String, keypair : String) {
           
    }
}