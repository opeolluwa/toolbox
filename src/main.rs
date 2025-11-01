use clap::Command;

use lib_toolbox::errors::app::AppError;

use lib_toolbox::commands::generate::generate_command;
use lib_toolbox::commands::script::script_command;
use lib_toolbox::commands::self_cmd::self_command;
use lib_toolbox::commands::store::store_command;

use lib_toolbox::config::database::AppDatabase;


fn main() -> Result<(), AppError> {
    let matches = Command::new("toolbox")
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

    lib_toolbox::toolbox::parse_commands(matches, db);

    Ok(())
}
