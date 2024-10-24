use std::{fs::create_dir_all, path::PathBuf};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::constant::FILE_CONFIG_DATABASE;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct ServerConfig {
    pub config_path: String,
    pub data_path: String,
}

impl ServerConfig {
    pub fn new(config_path: &str, data_path: &str) -> Self {
        create_dir_all(config_path).unwrap();
        create_dir_all(data_path).unwrap();

        let data_path = data_path.to_string();
        let config_path = config_path.to_string();

        Self {
            config_path,
            data_path,
        }
    }
}

impl ServerConfig {
    pub fn config_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.config_path)
    }

    pub fn config_database_path_buf(&self) -> PathBuf {
        self.config_path_buf().join(FILE_CONFIG_DATABASE)
    }

    pub fn config_database_path(&self) -> String {
        self.config_database_path_buf()
            .to_string_lossy()
            .into_owned()
    }
}

impl ServerConfig {
    pub fn data_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.data_path)
    }
}
