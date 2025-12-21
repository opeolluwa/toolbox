use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Write},
};

use dialoguer::Editor;

use crate::{
    constants::{APP_RUNTIME_CONFIG_DIR, APP_RUNTIME_CONFIG_FILE_NAME, TEMPLATES_DIR},
    errors::file_system::FileSystemError,
    helpers::console::LogMessage,
};

/// create a folder .dev_toolbox in the default home directory
pub fn configure_scripts() -> Result<(), FileSystemError> {
    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let config_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_CONFIG_DIR
    );

    let config_file_path = format!("{}/{}", config_dir, APP_RUNTIME_CONFIG_FILE_NAME);

    let _ = fs::create_dir_all(&config_dir);

    //copy the config template over
    if let Some(template_file) = TEMPLATES_DIR.get_file(APP_RUNTIME_CONFIG_DIR) {
        let mut config_file = File::create(&config_file_path)?;
        let template_file_content = template_file.contents();
        config_file.write_all(template_file_content)?;
        LogMessage::info("Scripts file created successfully");
    } else {
        LogMessage::error("Unable to create config file");
    }

    //open in editor
    let app_config_file = File::open(&config_file_path)?;

    let revision = Editor::new().edit("Provide the script runner, ex; python3, nodejs, deno...")?;

    println!("{:#?}", revision);
    Ok(())
}
