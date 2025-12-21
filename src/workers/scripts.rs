use std::{
    fs::{self},
    io::{Error, ErrorKind},
    path::Path, process::Command,
};

use crate::{
    config::app::ToolboxConfig,
    constants::{APP_RUNTIME_CONFIG_DIR, APP_RUNTIME_CONFIG_FILE_NAME},
    errors::file_system::FileSystemError,
    helpers::file_system::file_exists_in_path,
};

/// create a folder .dev_toolbox in the default home directory
pub fn configure_scripts(_overwrite: bool) -> Result<(), FileSystemError> {
    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let config_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_CONFIG_DIR
    );

    //TODO: use custom path to save config
    // let config_file_path = format!("{}/{}", config_dir, APP_RUNTIME_CONFIG_FILE_NAME);

    fs::create_dir_all(&config_dir)?;
    if !file_exists_in_path(Path::new(&config_dir), APP_RUNTIME_CONFIG_FILE_NAME) {
        let cfg = ToolboxConfig::new();
        cfg.save()?;
    }

    Ok(())
}



pub fn execute_custom_script(script: &str) {
    let output = Command::new("python3")
        .arg("-c")
        .arg(script)
        .output()
        .expect("Failed to execute script");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}