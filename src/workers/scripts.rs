use std::{
    fs::{self},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    config::app::ToolboxConfig,
    constants::{APP_RUNTIME_CONFIG_FILE_NAME, APP_RUNTIME_SCRIPTS_DIR},
    errors::file_system::FileSystemError,
    helpers::{console::LogMessage, file_system::file_exists_in_path},
};

/// create a folder .dev_toolbox in the default home directory
pub fn configure_scripts(_overwrite: bool) -> Result<(), FileSystemError> {
    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let scripts_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_SCRIPTS_DIR
    );

    //TODO: use custom path to save config
    // let config_file_path = format!("{}/{}", config_dir, APP_RUNTIME_CONFIG_FILE_NAME);
    // $HOME/.config/dev_toolbox/default-config.toml
    fs::create_dir_all(&scripts_dir)?;
    if !file_exists_in_path(Path::new(&scripts_dir), APP_RUNTIME_CONFIG_FILE_NAME) {
        let cfg = ToolboxConfig::new();
        cfg.save()?;
    }

    Ok(())
}

pub fn execute_custom_script(script: &str) -> Result<(), FileSystemError> {
    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let script_path = home_dir.join(APP_RUNTIME_SCRIPTS_DIR).join(script);

    // Load toolbox configuration to determine which runner to use
    let cfg = ToolboxConfig::load()?;
    let runner = cfg.scripts.runner;

    // Execute the script using the configured runner and propagate errors
    let output = Command::new(runner)
        .arg(script_path)
        .output()
        .expect("Failed to execute script");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

pub fn add_script_command(script_file_path: &PathBuf) -> Result<(), FileSystemError> {
    let file_path = script_file_path;

    if !file_path.exists() {
        LogMessage::error("Source file does not exist");
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::NotFound,
            "file does. not exist",
        )));
    }

    if !file_path.is_file() {
        LogMessage::error("Provided path is not a file");
        return Err(FileSystemError::IoError(Error::new(
            ErrorKind::InvalidInput,
            "unsupported file format",
        )));
    }

    let file_name = match file_path.file_name() {
        Some(name) => name,
        None => {
            LogMessage::error("Invalid file path: no filename");
            return Err(FileSystemError::IoError(Error::new(
                ErrorKind::InvalidInput,
                "unsupported file format",
            )));
        }
    };

    let home_dir = dirs::home_dir().ok_or(FileSystemError::IoError(Error::new(
        ErrorKind::NotFound,
        "could not find device $HOME directory",
    )))?;

    let scripts_dir = format!(
        "{}/{}",
        home_dir.to_str().unwrap_or_default(),
        APP_RUNTIME_SCRIPTS_DIR
    );

    let dest_dir = Path::new(&scripts_dir);

    // 🔑 Ensure destination directory exists
    if let Err(e) = fs::create_dir_all(dest_dir) {
        LogMessage::error(&format!(
            "Failed to create destination directory '{}': {}",
            dest_dir.display(),
            e
        ));
        return Err(FileSystemError::IoError(e));
    }

    let dest_path = dest_dir.join(file_name);

    let full_src_path = match file_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            LogMessage::error(&format!("Failed to resolve source path: {}", e));
            return Err(FileSystemError::IoError(e));
        }
    };

    match fs::copy(&full_src_path, &dest_path) {
        Ok(bytes) => {
            LogMessage::info(&format!(
                "Successfully copied '{}' → '{}' ({} bytes)",
                full_src_path.display(),
                dest_path.display(),
                bytes
            ));
            Ok(())
        }
        Err(e) => {
            LogMessage::error(&format!(
                "Failed to copy '{}' → '{}': {}",
                full_src_path.display(),
                dest_path.display(),
                e
            ));
            return Err(FileSystemError::IoError(e));
        }
    }
}
