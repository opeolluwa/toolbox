use include_dir::{include_dir, Dir};

pub const SOURCE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

lazy_static::lazy_static! {
    pub static ref DATABASE_PATH: std::string::String = {

    let os_default_home_dir = dirs::home_dir().unwrap_or(std::path::PathBuf::from("."));
    let db_path = format!(
        "{home_dir}/{upload_dir}",
        home_dir = os_default_home_dir.display(),
        upload_dir = ".toolbox"
    );
    let _ = std::fs::create_dir_all(&db_path);
    let database_path = format!("{db_path}/toolbox.db");
    database_path
    };

    // the path to the config file
    pub static ref CONFIG_FILE: std::string::String = {
        let os_default_home_dir = dirs::home_dir().unwrap();
        let config_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".toolbox"
        );

        //TODO: create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&config_path);
        format!("{config_path}/toolbox.toml")
    };


}
