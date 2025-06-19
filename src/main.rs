use clap::{Command, Parser, command};
use colored::*;

mod config;
use config::conf;

mod sec_db;
use sec_db::SecDb;

mod command_factory;

/// clenv - simple cmd tool for not so simple configs
#[derive(Parser, Debug)]
#[command(name = "clenv")]
#[command(about = "clenv - simple cmd tool for not so simple configs", long_about= None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let mut matches = Command::new("clenv");

    for comms in command_factory::add_all_comm() {
        matches = matches.subcommand(comms);
    }

    let mut confi = match conf::load() {
        Ok(cfg) => {
            cfg
        },
        Err(e) => {
            eprintln!("Configuration file not found. Creating one...");
            conf::init().expect("Failed to initialize config")
        }
    };

    let parser = matches.get_matches();

    match parser.subcommand() {
        Some(("cfg", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key");
            let value = sub_matches.get_one::<String>("value");
            
            match (key, value) {
                (Some(k), Some(v)) => {
                    confi.set(k, v);
                    println!("Set {} = {}", k, v);
                },
                (Some(k), None) => {
                    match confi.get(k) {
                        Some(v) => println!("{} = {}", k, v),
                        None => println!("Key '{}' not found", k),
                    }
                },
                (None, None) => {
                    println!("Listing all config entries:");
                    confi.list_all();
                },
                (None, Some(_)) => {
                    eprintln!("Error: value provided without key");
                }
            }
        },
        
        _ => {
            let db = SecDb::new(&confi.get("db").unwrap());
            unreachable!("Exhausted list of subcommands");
        }
    }
}

// fn main() -> Result<(), Box<dyn Error>> {

//     // Initalize the database
//     let temp = SecDb::new("./path");
//     temp.list_cf_formatted("keyring");

//     Ok(())
// }
