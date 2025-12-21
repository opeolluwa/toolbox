use clap::Command;
use lib_toolbox::{
    commands::{
        generate::generate_command, script::script_command, self_cmd::self_command,
        store::store_command,
    },
    config::database::AppDatabase,
    errors::app::AppError,
    toolbox::parse_commands,
};

fn main() -> Result<(), AppError> {
    let matches = Command::new("toolbox")
        .display_name("Dev toolbox")
        .about("lightweight extensible, command line toolchain for software builders")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(self_command())
        .subcommand(store_command())
        .subcommand(generate_command())
        .subcommand(script_command())
        .get_matches();

    let db = AppDatabase::init()?;

    parse_commands(matches, db);

    Ok(())
}
