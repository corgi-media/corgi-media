mod routes;
mod state;

use std::net::SocketAddr;

use axum::Router;
use tower_http::trace::TraceLayer;

use corgi_core::config::AppConfig;

use state::AppState;

pub struct CorgiServer {
    pub state: AppState,
    tcp: tokio::net::TcpListener,
}

impl CorgiServer {
    pub async fn new(
        addr: SocketAddr,
        config: AppConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tcp = tokio::net::TcpListener::bind(addr).await?;

        let state = AppState::new(config).await?;

        Ok(Self { state, tcp })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .merge(routes::ApiDocsRouter::route())
            .with_state(self.state)
            .layer(TraceLayer::new_for_http());

        tracing::info!("Corgi server running on http://{}", self.tcp.local_addr()?);
        axum::serve(self.tcp, app.into_make_service()).await?;

        Ok(())
    }
}
