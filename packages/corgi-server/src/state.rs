use std::sync::Arc;

use corgi_core::{config::AppConfig, DatabaseClient};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub database: Arc<DatabaseClient>,
}

impl AppState {
    pub async fn init(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(config);
        let database = DatabaseClient::connect(&config.server.database_url).await?;
        database.migration_up().await?;

        Ok(Self {
            config,
            database: Arc::new(database),
        })
    }
}
