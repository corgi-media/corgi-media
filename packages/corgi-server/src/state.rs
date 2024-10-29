use std::sync::Arc;

use corgi_core::{config::AppConfig, tracing, DatabaseClient};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub database: Option<Arc<DatabaseClient>>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut database: Option<Arc<DatabaseClient>> = None;

        if let Some(database_config) = &config.database {
            database = match DatabaseClient::connect(&database_config.connection_url()).await {
                Ok(database) => Some(Arc::new(database)),
                Err(err) => {
                    tracing::error!("Failed to connect to database: {:?}", err);
                    None
                }
            }
        }

        let config: Arc<AppConfig> = Arc::new(config);

        Ok(Self { config, database })
    }
}
