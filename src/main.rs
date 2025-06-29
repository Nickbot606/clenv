use clap::{Command, Parser, command};

mod config;
use config::conf;
use config::resolve_path;

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
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("Configuration file not found. Creating one...");
            conf::init().expect("Failed to initialize config")
        }
    };

    let parser = matches.get_matches();

    match parser.subcommand() {
        Some(("cfg", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key");
            let value = sub_matches.get_one::<String>("value");

            // Quick and dirty way to reset your configuration file
            if key == Some(&String::from("init")) {
                conf::init().expect("Could not create a configuration");
                let _db = SecDb::new(confi.clone());
                return;
            }
            match (key, value) {
                (Some(k), Some(v)) => {
                    confi.set(k, v);
                    println!("Set {} = {}", k, v);
                }
                (Some(k), None) => match confi.get(k) {
                    Some(v) => {
                        println!("{} = {}", k, v)
                    }
                    None => println!("Key '{}' not found", k),
                },
                (None, None) => {
                    println!("Listing all config entries:");
                    confi.list_all();
                }
                (None, Some(_)) => {
                    eprintln!("Error: value provided without key");
                }
            }
        }
        Some(("store", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file");
            let name = sub_matches.get_one::<String>("name");

            let mut db = SecDb::new(confi.clone());
            match (file, name) {
                (Some(n), Some(f)) => {
                    let target_file = resolve_path(f, "").into_os_string().into_string().unwrap();
                    db.store_file(n, &target_file);
                }
                (Some(f), None) => {
                    let target_file = resolve_path(f, "").into_os_string().into_string().unwrap();
                    db.store_file(f, &target_file);
                }
                (None, Some(_)) => {
                    eprintln!("File not entered");
                }
                (None, None) => {
                    eprintln!("Filename not entered");
                }
            }
        }
        Some(("dump", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let db = SecDb::new(confi.clone());

            match name {
                Some(n) => {
                    db.dump_file(n);
                }
                None => {
                    eprintln!("Missing name of file");
                }
            }
        }
        Some(("show", sub_matches)) => {
            let namespace = sub_matches.get_one::<String>("namespace");
            let db = SecDb::new(confi.clone());
            match namespace {
                Some(namespace) => {
                    db.list_cf_formatted(namespace);
                }
                None => {
                    db.list_cfs();
                }
            }
        }
        Some(("rm", sub_matches)) => {
            let name = sub_matches.get_one::<String>("entry");
            let db = SecDb::new(confi.clone());
            match name {
                Some(name) => {
                    db.rm(name);
                }
                None => {
                    eprintln!("Please provide name of the entry you would like to remove")
                }
            }
        }
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let db = SecDb::new(confi.clone());
            match name {
                Some(name) => {
                    db.add_user(name);
                }
                None => {
                    eprintln!("Pleas provide a name for the user you are adding.")
                }
            }
        }
        Some(("remove", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let db = SecDb::new(confi.clone());
            match name {
                Some(name) => {
                    db.remove_user(name);
                }
                None => {
                    eprintln!("Pleas provide a name for the user you are adding.")
                }
            }
        }
        _ => {
            unreachable!("Exhausted list of subcommands");
        }
    }
}
