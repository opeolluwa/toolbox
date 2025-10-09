use clap::ArgMatches;
use rusqlite::Connection;

use crate::parsers::{
    app::{parse_uninstall_options, parse_upgrade_options},
    generator::parse_generator_options,
    store::parse_store_options,
};

pub fn parse_commands(matches: ArgMatches, database_connection: Connection) {
    match matches.subcommand() {
        Some(("store", sub_matches)) => parse_store_options(sub_matches, database_connection),
        Some(("uninstall", sub_matches)) => parse_uninstall_options(sub_matches),
        Some(("upgrade", sub_matches)) => parse_upgrade_options(sub_matches),
        Some(("generate", sub_matches)) => parse_generator_options(sub_matches),
        _ => std::process::exit(1),
    }
}
