use include_dir::{include_dir, Dir};

pub const TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub const APP_RUNTIME_CONFIG_DIR: &str = ".dev_toolbox";
pub const APP_RUNTIME_SCRIPTS_DIR: &str = ".dev_toolbox/scripts";
pub const APP_RUNTIME_CONFIG_FILE_NAME: &str = "Toolbox.toml";
pub const APP_RUNTIME_DATABASE_PATH: &str = ".dev_toolbox/db/app.sqlite";
