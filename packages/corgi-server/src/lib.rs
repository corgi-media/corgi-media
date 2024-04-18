mod routes;

use std::net::SocketAddr;

use axum::Router;

pub async fn start(addr: SocketAddr) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let app = Router::new().merge(routes::docs_routers());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
