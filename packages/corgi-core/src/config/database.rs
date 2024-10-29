use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::constant::FILE_DATA_SQLITE;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub enum DatabaseDriver {
    #[serde(alias = "mysql")]
    MySql,

    #[serde(alias = "postgres")]
    Postgres,

    #[serde(alias = "sqlite")]
    Sqlite,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,
    pub sqlite: Option<SqliteConfig>,
    pub mysql: Option<MySqlConfig>,
    pub postgres: Option<PostgresConfig>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct SqliteConfig {
    pub path: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            path: FILE_DATA_SQLITE.to_owned(),
        }
    }
}

impl DatabaseConfig {
    pub fn read(config_file: &str, data_path: &str) -> Option<Self> {
        tracing::info!("Reading database configuration from: {}", config_file);

        let file = fs::read_to_string(config_file)
            .map_err(|err| {
                tracing::warn!("Failed to read database configuration file: {}", err);
            })
            .ok()?;
        tracing::debug!("Database configuration file contents: \n{}", file);

        let mut config: DatabaseConfig = toml::from_str(&file)
            .map_err(|err| {
                tracing::warn!("Failed to parse database configuration file: {}", err);
            })
            .ok()?;

        if match config.driver {
            DatabaseDriver::MySql => config.mysql.is_none(),
            DatabaseDriver::Postgres => config.postgres.is_none(),
            _ => false,
        } {
            tracing::warn!(
                "{:?} configuration is missing in database configuration",
                config.driver
            );
            return None;
        }

        if let DatabaseDriver::Sqlite = config.driver {
            let mut sqlite = config.sqlite.unwrap_or_default();
            let mut sqlite_path = PathBuf::from(sqlite.path);

            if sqlite_path.is_relative() {
                sqlite_path = PathBuf::from(data_path).join(sqlite_path);
            }

            if let Some(sqlite_dir) = sqlite_path.parent() {
                create_dir_all(sqlite_dir)
                    .map_err(|err| {
                        tracing::error!("Failed to create SQLite directory: {}", err);
                    })
                    .ok()?;
            }

            sqlite.path = sqlite_path.to_string_lossy().into_owned();

            config.sqlite = Some(sqlite);
        }

        tracing::debug!("Database configuration: {:#?}", config);

        Some(config)
    }

    pub fn connection_url(&self) -> String {
        match self.driver {
            DatabaseDriver::MySql => {
                let mysql = self.mysql.as_ref().unwrap();
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    mysql.user, mysql.password, mysql.host, mysql.port, mysql.database
                )
            }
            DatabaseDriver::Postgres => {
                let postgres = self.postgres.as_ref().unwrap();
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    postgres.user,
                    postgres.password,
                    postgres.host,
                    postgres.port,
                    postgres.database
                )
            }
            DatabaseDriver::Sqlite => {
                let sqlite = self.sqlite.as_ref().unwrap();

                format!("sqlite://{}?mode=rwc", sqlite.path)
            }
        }
    }
}
