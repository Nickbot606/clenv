use std::error::Error;

mod i_keys;
use i_keys::i_keys::i_keys as IKeys;
use i_keys::pairs::generate_key_pair as gen_pair;

mod config;
use config::config::Config as Config;

fn main() -> Result<(), Box<dyn Error>> {
    
    Ok(())
}
