 pub fn generate_service(base_path: &PathBuf) {
        let folders = [
            "config",
            "controllers",
            "services",
            "entities",
            "dto",
            "src",
        ];

        // create the base path if not exist
        if !Path::new(base_path).exists() {
            let _ = fs::create_dir(base_path);
        }
        LogMessage::success(&format!(
            "creating new service to path {:?}",
            base_path.canonicalize()
        ));
        let _ = folders
            .into_iter()
            .map(|dir| {
                let path = Path::new(base_path).join(dir);
                let _ = fs::create_dir(path);
            })
            .collect::<Vec<_>>();
    }
   