mod crypto;
mod server;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use crypto::Keyring;
pub use server::ServerConfig;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub keyring: Keyring,
}

impl AppConfig {
    pub async fn build(server: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let keyring = Keyring::build(&server.config_path_buf().join("keyring")).await?;

        Ok(Self { server, keyring })
    }
}
