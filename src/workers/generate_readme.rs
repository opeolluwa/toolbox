 pub fn generate_readme(&self) -> Result<(), FsError> {
        let file_exists = file_exists_in_path(self.base_path.as_path(), "README.md");
        let allow_over_write = self.force;
        let backup_existing_file = self.back_up;

        let desired_path = Path::new(&self.base_path).join("README.md");
        let file_path = desired_path.as_path();
        let backup_path = Path::new(&self.base_path).join("README.md").join(".bak");

        if file_exists && !allow_over_write {
            LogMessage::error("the readme already exist, use the --force flag to overwrite it");
            std::process::exit(1);
        } else if file_exists && allow_over_write && !backup_existing_file {
            let allow_overwrite = Confirm::new()
                .with_prompt("The current readme would not be backed up, do you wish to continue?")
                .interact()
                .unwrap();

            if !allow_overwrite {
                std::process::exit(1);
            }

            Self::create_readme_template(file_path)?
        } else if file_exists && allow_over_write && backup_existing_file {
            let _ = std::fs::copy(file_path, backup_path).map_err(|error| {
                LogMessage::error(&error.to_string());
                std::process::exit(1);
            });
            Self::create_readme_template(file_path)?;
        }

        Self::create_readme_template(file_path)?;

        Ok(())
    }
   