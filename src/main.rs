use lib_toolbox::errors::app::AppError;

use clap::command;

use lib_toolbox::{
    config::database::AppDatabase,
    subcommands::{
        generate::generate_command, script::script_command, self_cmd::self_command,
        store::store_command,
    },
};

fn main() -> Result<(), AppError> {
    let matches = command!()
        .subcommand(self_command())
        .subcommand(store_command())
        .subcommand(generate_command())
        .subcommand(script_command())
        .get_matches();

    lib_toolbox::parser::parse_commands(matches, AppDatabase::init()?);

    Ok(())
}
