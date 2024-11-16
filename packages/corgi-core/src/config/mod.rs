mod server;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use server::ServerConfig;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn init(server: ServerConfig) -> Self {
        Self { server }
    }
}
