use clap::{Arg, Command, Parser, command};
use colored::*;

mod config;
use config::conf;

mod sec_db;
// use sec_db::SecDb;

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

    let parser = matches.get_matches();

    match parser.subcommand() {
        Some(("db", sub_matches)) => println!(
            "'myapp init' was used, name is: {:?}",
            sub_matches.get_one::<String>("name")
        ),
        Some(("show", _sub_matches)) => {
            let _ = conf::read();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

// fn main() -> Result<(), Box<dyn Error>> {

//     // Initalize the database
//     let temp = SecDb::new("./path");
//     temp.list_cf_formatted("keyring");

//     Ok(())
// }
