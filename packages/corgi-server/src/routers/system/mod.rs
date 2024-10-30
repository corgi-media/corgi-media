pub mod configurations;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{api_docs::ApiTags, state::AppState};

pub struct Routers;

impl Routers {
    pub const PATH: &'static str = "/system";
    pub const STATUS: &'static str = "/status";
    pub const PING: &'static str = "/ping";
}

#[derive(Serialize, Deserialize, ToSchema)]
struct SystemStatus {
    database: bool,
}

#[utoipa::path(
    get,
    path = Routers::PING,
    tag = ApiTags::SYSTEM,
    responses(
        (status = OK, description = "Ping the system")
    )
)]
async fn system_ping() -> &'static str {
    "Corgi Server"
}

#[utoipa::path(
    get,
    path = Routers::STATUS,
    tag = ApiTags::SYSTEM,
    responses(
        (status = OK, description = "Get the status of the system", body = SystemStatus)
    )
)]
async fn system_status(State(state): State<AppState>) -> Json<SystemStatus> {
    let database = match state.database.as_ref() {
        Some(db) => db.connection.ping().await.is_ok(),
        _ => false,
    };

    Json(SystemStatus { database })
}

impl Routers {
    pub fn route() -> OpenApiRouter<AppState> {
        OpenApiRouter::new().nest(
            Routers::PATH,
            OpenApiRouter::new()
                .merge(configurations::Routers::route())
                .routes(routes!(system_status))
                .routes(routes!(system_ping)),
        )
    }
}
