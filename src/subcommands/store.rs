use clap::{arg, Command};

pub fn store_command() -> Command {
    let store_save_cmd = Command::new("save")
    .about("Save a new key value pair")
    .arg(arg!(-k --key <KEY> "key"))
    .arg(arg!(-v --value <VALUE> "value"))
    .arg(arg!(-s --sensitive "mark data as sensitive data, used for encrypting data when returning them"));

    let store_list_cmd = Command::new("list")
        .about("retrieve the stored entries")
        .arg(arg!(-s --sort <SORT> "sort by Ascending(ASC)  descending(DSC), sort by key (KEY)"));

    let store_find_cmd = Command::new("find")
        .about("find one or more entries")
        .arg(arg!(-e --exact "find exact keyword, against the stored keys"));
    let store_remove_cmd = Command::new("remove").about("delete an entry from the database");
    let store_export_cmd = Command::new("export").about("export to HTML or PDF");

    Command::new("store")
        .visible_aliases(["s", "-s"])
        .about("store and manage a key value pair")
        .subcommand(store_save_cmd)
        .subcommand(store_list_cmd)
        .subcommand(store_find_cmd)
        .subcommand(store_remove_cmd)
        .subcommand(store_export_cmd)
        .arg(arg!(-s --sync <REMOTE_SERVER> "backup to a remote database SQL"))
}
