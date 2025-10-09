use clap::{arg, Command};

pub fn script_command() -> Command {
    Command::new("script")
        .visible_aliases(["sc", "-sc"])
        .about("run a script file")
        .arg(arg!(-f --file <FILE> "script file path").required(true))
}
