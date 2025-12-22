use clap::Command;

pub fn self_command() -> Command {
    let self_uninstall = Command::new("uninstall").about("uninstall toolbox");

    let self_upgrade = Command::new("upgrade").about("upgrade toolbox");

    let self_configure = Command::new("configure").about("upgrade toolbox");

    Command::new("self")
        .about("manage and configure the toolbox")
        .subcommand(self_uninstall)
        .subcommand(self_upgrade)
        .subcommand(self_configure)
}
