mod routes;

use std::net::SocketAddr;

use axum::Router;

pub async fn start(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = Router::new().merge(routes::ApiDocsRouter::route());

    tracing::info!("Corgi server running on http://{}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
