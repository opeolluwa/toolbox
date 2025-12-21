use clap::ArgMatches;

use crate::{helpers::console::LogMessage, workers::scripts::configure_scripts};

pub fn parse_script_options(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("configure", args)) => {
            let override_existing = *args.get_one::<bool>("overwrite").unwrap_or(&false);

            let _ = configure_scripts();
        }
        Some(("add", args)) => {
            println!("add new scripts")
        }

        Some(("remove", args)) => {
            println!("remove new scripts")
        }
        Some(("execute", args)) => {
            println!("execute new scripts")
        }

        Some((other, _)) => {
            LogMessage::warning(&format!("Unknown subcommand '{}'", other));
            std::process::exit(1);
        }

        None => {
            LogMessage::error("No subcommand provided. Use `--help` to see available options.");
            std::process::exit(1);
        }
    }
}
