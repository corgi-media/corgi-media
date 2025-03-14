mod dto;
mod error;
mod handlers;
mod openapi;
mod routers;
mod state;

use std::net::SocketAddr;

use corgi_core::{config::AppConfig, tracing};

use state::AppState;

pub struct CorgiServer {
    pub state: AppState,
    tcp: tokio::net::TcpListener,
}

impl CorgiServer {
    pub async fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = SocketAddr::new(config.server.ip_addr(), config.server.port);
        let tcp = tokio::net::TcpListener::bind(addr).await?;

        let state = AppState::build(config).await?;

        Ok(Self { state, tcp })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let router = routers::AppRouter::route().with_state(self.state);

        tracing::info!("Corgi server running on http://{}", self.tcp.local_addr()?);
        axum::serve(self.tcp, router.into_make_service()).await?;

        Ok(())
    }
}
