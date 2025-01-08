mod crypto;
mod server;

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use crypto::Keyring;
pub use server::ServerConfig;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppConfig {
    pub identifier: uuid::Uuid,
    pub server: ServerConfig,
    pub keyring: Keyring,
}

impl AppConfig {
    pub fn build(server: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let identifier = Self::build_identifier(&server.config_path_buf())?;

        let keyring = Keyring::build(&server.config_path_buf().join("keyring"))?;

        Ok(Self {
            identifier,
            server,
            keyring,
        })
    }

    fn build_identifier(path: &Path) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
        let path = path.join("identifier.txt");

        tracing::info!("Reading service identifier from path: {:?}", path);
        if let Ok(identifier) = fs::read_to_string(&path) {
            if let Ok(identifier) = uuid::Uuid::parse_str(&identifier) {
                tracing::info!("Service identifier: {}", identifier);
                return Ok(identifier);
            }
            tracing::warn!("Failed to parse identifier: {}", identifier);
        }

        let identifier = uuid::Uuid::now_v7();
        tracing::debug!("Generated service identifier: {}", identifier);

        fs::write(&path, identifier.to_string())?;
        tracing::info!("Wrote service identifier to path: {:?}", path);

        tracing::info!("Service identifier: {}", identifier);
        Ok(identifier)
    }
}
