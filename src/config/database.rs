use crate::errors::app::AppError;
use rusqlite::Connection;
pub struct AppDatabase {}

impl AppDatabase {
    pub fn init() -> Result<Connection, AppError> {
        let os_default_home_dir = dirs::home_dir().unwrap_or(std::path::PathBuf::from("."));
        let db_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".toolbox"
        );
        let _ = std::fs::create_dir_all(&db_path);
        let database_path = format!("{db_path}/toolbox.db");

        let connection = Connection::open(&database_path)
            .map_err(|err| AppError::OperationFailed(err.to_string()))?;
        connection
            .execute(
                r#"
    CREATE TABLE IF NOT EXISTS data_store (
    id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    sensitive INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
    )
    "#,
                (),
            )
            .map_err(|err| AppError::OperationFailed(err.to_string()))?;

        Ok(connection)
    }
}
