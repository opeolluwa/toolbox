use std::process;

use clap::ArgMatches;
use rusqlite::Connection;

use crate::{
    workers::store::StoreConfig,
    helpers::{console::LogMessage, parser::extract_command_argument},
};

pub fn parse_store_options(sub_matches: &ArgMatches, database_connection: Connection) {
    let store_engine = StoreConfig::new(database_connection);
    match sub_matches.subcommand() {
        Some(("list", _)) => store_engine.list(),
        Some(("save", command_arguments)) => {
            let Some(key) = extract_command_argument::<String>(command_arguments, "key") else {
                LogMessage::error("key is required");
                process::exit(1);
            };

            let Some(value) = extract_command_argument::<String>(command_arguments, "value") else {
                LogMessage::error("key is required");
                process::exit(1);
            };

            let sensitive = extract_command_argument::<bool>(command_arguments, "sensitive")
                .unwrap_or_default();

            store_engine.save(&key, &value, sensitive)
        }
        //Some(("remove", _)) => store_engine.list(),
        //Some(("export", _)) => store_engine.list(),
        // Some(("sync", _)) => store_engine.list(),
        _ => process::exit(1),
    }
}
