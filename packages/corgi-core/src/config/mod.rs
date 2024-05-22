mod database;
mod server;

pub use database::DatabaseConfig;
pub use server::ServerConfig;

pub struct AppConfig {
    pub server: ServerConfig,
    pub database: Option<DatabaseConfig>,
}

impl AppConfig {
    pub fn new(config_path: &str, data_path: &str) -> Self {
        let server = ServerConfig::new(config_path, data_path);

        let database = DatabaseConfig::read(&server.database_config_path());

        Self { server, database }
    }
}
