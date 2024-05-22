use std::sync::Arc;

use corgi_core::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(config);

        Ok(Self { config })
    }
}
