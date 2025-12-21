use serde::{Deserialize, Serialize};

use crate::errors::file_system::FileSystemError;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ToolboxConfig {
    pub scripts: ScriptConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScriptConfig {
    pub runner: String,
    pub source: String,
}

impl ToolboxConfig {
    const APP_NAME: &str = "dev_toolbox";
    // const
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn save(&self) -> Result<(), FileSystemError> {
        confy::store(Self::APP_NAME, None, self)?;

        Ok(())
    }

    pub fn load() -> Result<Self, FileSystemError> {
        let cfg: ToolboxConfig = confy::load(Self::APP_NAME, None)?;
        Ok(cfg)
    }

    pub fn create(&self) -> Result<(), FileSystemError> {
        unimplemented!()
    }
}
