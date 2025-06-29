use clap::{Arg, Command, value_parser};
use std::path::PathBuf;

// Way over engineered sure, but very ergonomic for future uses
// Plus, it makes it very easy to read all the commands and arguments in one spot
struct SubCommand;
enum EV {
    PATH,
    NAME,
}

impl SubCommand {
    fn new(
        name: &'static str,
        about: &'static str,
        args: Vec<(&'static str, bool, EV)>,
    ) -> Command {
        let mut comm = Command::new(name).about(about);

        for (key, req, value) in args {
            let par = match value {
                EV::PATH => value_parser!(PathBuf),
                EV::NAME => value_parser!(String),
            };

            comm = comm.arg(Arg::new(key).required(req).value_parser(par));
        }
        comm
    }
}

pub fn add_all_comm() -> Vec<Command> {
    vec![
        SubCommand::new(
            "cfg",
            "changes a specific configuration remotely.",
            vec![("key", false, EV::NAME), ("value", false, EV::NAME)],
        ),
        SubCommand::new(
            "store",
            "Stores the file into the db.",
            vec![("file", true, EV::NAME), ("name", false, EV::NAME)],
        ),
        SubCommand::new(
            "dump",
            "dumps all blocks into individual env files from the namespace to current working directory",
            vec![("name", true, EV::NAME)],
        ),
        SubCommand::new(
            "show",
            "shows the currently selected database, users who have access, and available namespaces. Put the name of the namespace to instead list the namespace from a different namespace.",
            vec![("namespace", false, EV::NAME)],
        ),
        SubCommand::new(
            "rm",
            "rm removes the entry from the currently selected namespace. ",
            vec![("entry", true, EV::NAME)],
        ),
        SubCommand::new(
            "add",
            "adds a user to the keyring. Provide the name of the user their public and private key will be added.",
            vec![("name", true, EV::NAME)],
        ),
        SubCommand::new(
            "remove",
            "removes a user to the keyring. Just include the name.",
            vec![("name", true, EV::NAME)],
        ),
    ]
}
