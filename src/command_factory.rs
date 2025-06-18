use clap::{Command, Arg, value_parser};
use std::path::PathBuf;

// Way over engineered sure, but very ergonomic for future uses
// Plus, it makes it very easy to read all the commands and arguments in one spot
struct SubCommand;
enum EV {
    PATH,
    NAME
}

impl SubCommand {
    fn new(name: &'static str, about: &'static str, args: Vec<(&'static str, bool, EV)>) -> Command {
        let mut comm = Command::new(name).about(about);

        for (key, req, value) in args {
            let par = match value {
                EV::PATH => value_parser!(PathBuf),
                EV::NAME => value_parser!(String)
            };

            comm = comm.arg(
                Arg::new(key)
                .required(req)
                .value_parser(par)
            );
        }
        comm
    }
}

pub fn add_all_comm () -> Vec<Command>{
    vec![
        SubCommand::new("db", "Creates or sets your current database", vec![("name",true, EV::NAME)]),
        SubCommand::new("add", "adds a public key to add the users\' keys", vec![("name",true, EV::NAME),("filepath",true, EV::PATH)]),
        SubCommand::new("show", "shows the currently selected database, users who have access, and available namespaces", vec![]),
        SubCommand::new("cfg", "changes a specific configuration remotely.", vec![("config_name",false, EV::NAME)]),
        SubCommand::new("ns", "changes the currently selected namespace.", vec![("namespace",false, EV::NAME)]),
        SubCommand::new("dump", "dumps all blocks into individual env files from the namespace to current working directory", vec![]),
        SubCommand::new("write", "writes the selected file to the currently selected namespace with a name of your choosing. If you match a blob name exactly, it will overwrite said blob", vec![("name",true, EV::NAME)]),
    ]
}