use std::time::Duration;

use sea_orm::{ConnectOptions, Database as SeaOrmDatabase, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

use crate::config::app::ToolboxConfig;

pub struct Database;

impl Database {
    pub async fn init() -> Result<DatabaseConnection, AppError> {
        let config = ToolboxConfig::load()?;

        let database_url = config.env.database_url;
        let mut opt = ConnectOptions::new(&database_url);

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = SeaOrmDatabase::connect(opt)
            .await
            .map_err(|err| AppError::StartupError(err.to_string()))?;

        Migrator::up(&db, None).await?;
        Ok(db)
    }
}
