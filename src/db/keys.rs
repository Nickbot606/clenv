use age::{
    x25519::{Identity, Recipient},
    Encryptor, Decryptor
};

use std::io::{Read, Write};

pub struct keyCrud;

impl keyCrud {

    // creates a keypair
    pub fn init(name: String) {
        Identity::generate();
        println!("generated keys");
    }

}