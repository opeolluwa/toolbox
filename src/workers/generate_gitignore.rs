  pub fn generate_ignore_file(&self) -> Result<(), FsError> {
        let file_exists = file_exists_in_path(self.base_path.as_path(), ".gitignore");
        let allow_over_write = self.force;
        let backup_existing_file = self.back_up;

        let desired_path = Path::new(&self.base_path).join(".gitignore");
        let file_path = desired_path.as_path();
        let backup_path = Path::new(&self.base_path).join(".gitignore").join(".bak");

        if file_exists && !allow_over_write {
            "a .gitignore already exist on the select path, use the --force flag to overwrite it".to_string();
            std::process::exit(1);
        } else if file_exists && allow_over_write {
            fs::remove_file(file_path).map_err(|err| FsError::OperationError(err.to_string()))?
        } else if file_exists && allow_over_write && backup_existing_file {
            fs::copy(file_path, backup_path)
                .map_err(|err| FsError::OperationError(err.to_string()))?;
            fs::remove_file(file_path).map_err(|err| FsError::OperationError(err.to_string()))?;

            Self::create_git_ignore_template(file_path)?
        }
        Self::create_git_ignore_template(file_path)
    }
   