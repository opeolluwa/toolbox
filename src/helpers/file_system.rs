use std::path::Path;

pub fn file_exists_in_path(base_path: &Path, file_name: &str) -> bool {
    Path::new(base_path).join(file_name).exists()
}

pub fn create_file_in_path() {}

pub fn remove_file_in_path() {}
