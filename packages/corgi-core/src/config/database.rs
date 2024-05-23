use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DatabaseDriver {
    #[serde(alias = "mysql")]
    MySql,

    #[serde(alias = "postgres")]
    Postgres,

    #[serde(alias = "sqlite")]
    Sqlite,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,
    pub sqlite: Option<SqliteConfig>,
    pub mysql: Option<MySqlConfig>,
    pub postgres: Option<PostgresConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqliteConfig {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn read(path: &str) -> Option<DatabaseConfig> {
        tracing::info!("Reading database configuration from: {}", path);

        let file = fs::read_to_string(path)
            .map_err(|err| {
                tracing::warn!("Failed to read database configuration file: {}", err);
            })
            .ok()?;
        tracing::debug!("Database configuration file contents: \n{}", file);

        let config: DatabaseConfig = toml::from_str(&file)
            .map_err(|err| {
                tracing::warn!("Failed to parse database configuration file: {}", err);
            })
            .ok()?;
        tracing::debug!("Database configuration: {:#?}", config);

        if match config.driver {
            DatabaseDriver::MySql => config.mysql.is_none(),
            DatabaseDriver::Postgres => config.postgres.is_none(),
            DatabaseDriver::Sqlite => config.sqlite.is_none(),
        } {
            tracing::warn!(
                "{:?} configuration is missing in database configuration",
                config.driver
            );
            return None;
        }

        Some(config)
    }
}
