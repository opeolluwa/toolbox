use clap::{arg, Arg, Command};

pub fn script_command() -> Command {
    let configure_scripts_cmd = Command::new("configure")
        .about("configure the script runner and the scripts directory")
        .aliases(["cg", "cfg", "-cg", "-cfg", "cc", "-cc"])
        .arg(arg!(-o --overwrite "overwrite existing"));

    let add_scripts_cmd = Command::new("add")
        .about("add a new script to the configured scripts directory")
        .aliases(["a", "-a"])
        .arg(Arg::new("path").help("path of the new script"))
        .arg(arg!(-o --overwrite "overwrite existing"));

    let remove_scripts_cmd = Command::new("remove")
        .about("add a new script to the configured scripts directory")
        .aliases(["r", "rm"])
        .arg(Arg::new("name").help("name of the script"));

    let execute_scripts_cmd = Command::new("execute")
        .about("execute a dynamic script")
        .aliases(["-e", "exc"])
        .arg(Arg::new("name").help("name of the script"));

    Command::new("script")
        .visible_aliases(["sc", "-sc"])
        .about("run a script file")
        .subcommand(configure_scripts_cmd)
        .subcommand(add_scripts_cmd)
        .subcommand(remove_scripts_cmd)
        .subcommand(execute_scripts_cmd)
}
