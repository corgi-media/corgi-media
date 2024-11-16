use std::{net::IpAddr, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub config_path: String,
    pub data_path: String,
    pub database_url: String,
}

impl ServerConfig {
    pub fn ip_addr(&self) -> IpAddr {
        IpAddr::from_str(&self.host).unwrap()
    }
}

impl ServerConfig {
    pub fn config_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.config_path)
    }
}

impl ServerConfig {
    pub fn data_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.data_path)
    }
}
