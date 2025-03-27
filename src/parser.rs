use std::path::Path;

use clap::ArgMatches;

use crate::commands::generator::GeneratorConfig;

pub fn parse_commands(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("store", sub_matches)) => parse_store_options(sub_matches),
        Some(("uninstall", sub_matches)) => parse_uinstall_options(sub_matches),
        Some(("upgrade", sub_matches)) => parse_upgrade_options(sub_matches),
        Some(("generate", sub_matches)) => parse_generator_options(sub_matches),
        _ => std::process::exit(1),
    }
}

fn parse_uinstall_options(sub_matches: &ArgMatches) {}
fn parse_upgrade_options(sub_matches: &ArgMatches) {}
fn parse_store_options(sub_matches: &ArgMatches) {}
fn parse_generator_options(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("readme", command_arguments)) => {
            let GeneratorConfig {
                force,
                base_path,
                back_up,
            } = GeneratorConfig::parse_options(command_arguments);
            let _ = GeneratorConfig::new(force, base_path, back_up).generate_readme();
        }
        Some(("git-ignore", command_arguments)) => {
            let GeneratorConfig {
                force,
                base_path,
                back_up,
            } = GeneratorConfig::parse_options(command_arguments);
            let _ = GeneratorConfig::new(force, base_path, back_up).generate_ignore_file();
        }
        Some(("service", command_arguments)) => {
            let mut base_path = String::from(".");

            if let Some(base_path_flag) = command_arguments.get_one::<String>("path") {
                base_path = base_path_flag.trim().to_string();
            };

            GeneratorConfig::generate_service(&Path::new(&base_path).to_path_buf());
        }
        _ => std::process::exit(1),
    }
}
