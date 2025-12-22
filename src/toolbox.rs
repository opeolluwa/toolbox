use clap::ArgMatches;
use sea_orm::DatabaseConnection;

use crate::helpers::console::LogMessage;
use crate::parsers::scripts::parse_script_options;
use crate::parsers::{
    app::{parse_uninstall_options, parse_upgrade_options},
    generator::parse_generator_options,
};
use crate::workers::gui::exec_gui;

pub fn parse_commands(matches: ArgMatches, db: &DatabaseConnection) {
    match matches.subcommand() {
        Some(("uninstall", sub_matches)) => parse_uninstall_options(sub_matches),
        Some(("upgrade", sub_matches)) => parse_upgrade_options(sub_matches),
        Some(("generate", sub_matches)) => parse_generator_options(sub_matches),
        Some(("script", sub_matches)) => parse_script_options(sub_matches),
        Some(("ui", _)) => exec_gui(db),
        _ => {
            LogMessage::error("Invalid command");
            std::process::exit(1)
        }
    }
}
