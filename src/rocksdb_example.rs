/* 
1. Configuration CRUD2. Configuration CRUD
2. Namespace entry state management
3. .env CRUD
4. encryption
*/

use rocksdb::{DB, Options};

fn main() {
    // Open or create the RocksDB database
    let path = "_path_for_rocksdb_storage";
    let db = DB::open_default(path).unwrap();

    // Create a blob (binary data)
    let key = b"my_key";
    let blob_value = b"this is a test"; // example binary data

    // Insert the blob
    db.put(key, &blob_value).unwrap();

    // Retrieve it
    match db.get(key).unwrap() {
        Some(value) => {
            println!("Retrieved value: {:?}", value);
        }
        None => {
            println!("Value not found");
        }
    }

    // Optional: clean up
    drop(db);
    let _ = DB::destroy(&Options::default(), path);
}
