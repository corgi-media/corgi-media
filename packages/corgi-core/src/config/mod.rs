pub mod database;
pub mod server;

use serde::{Deserialize, Serialize};

pub use database::DatabaseConfig;
pub use server::ServerConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfigStatus {
    pub database: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: Option<DatabaseConfig>,
}

impl AppConfig {
    pub fn new(config_path: &str, data_path: &str) -> Self {
        let server = ServerConfig::new(config_path, data_path);

        let database = DatabaseConfig::read(&server.config_database_path(), &server.data_path);

        Self { server, database }
    }

    pub fn status(&self) -> AppConfigStatus {
        let database = self.database.is_some();

        AppConfigStatus { database }
    }
}
