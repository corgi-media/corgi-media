use std::sync::Arc;

use corgi_core::{config::AppConfig, tracing, DatabaseClient};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub database: Arc<Option<DatabaseClient>>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(config);
        let mut state = Self {
            config,
            database: Arc::new(None),
        };

        state.connect_database().await;

        Ok(state)
    }

    pub async fn connect_database(&mut self) -> bool {
        if let Some(database_config) = &self.config.database {
            let database = match DatabaseClient::connect(&database_config.connection_url()).await {
                Ok(db) => db,
                Err(err) => {
                    tracing::error!("Failed to connect to database: {:?}", err);
                    return false;
                }
            };

            if let Err(err) = database.migration_up().await {
                tracing::error!("Database migration failed: {:?}", err);
                return false;
            }

            self.database = Arc::new(Some(database));
        }

        self.database.is_some()
    }
}
