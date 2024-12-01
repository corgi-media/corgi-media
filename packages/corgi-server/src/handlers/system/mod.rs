use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{openapi::Tags, routers::Paths, state::AppState};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemStatus {
    database: bool,
}

#[utoipa::path(
    get,
    path = Paths::SYSTEM_PING,
    tag = Tags::SYSTEM,
    operation_id = "system_ping",
    responses(
        (status = OK, description = "Ping the system")
    )
)]
pub async fn ping() -> &'static str {
    "Corgi Server"
}

#[utoipa::path(
    get,
    path = Paths::SYSTEM_STATUS,
    tag = Tags::SYSTEM,
    operation_id = "system_status",
    responses(
        (status = OK, description = "Get the status of the system", body = SystemStatus)
    )
)]
pub async fn status(State(state): State<AppState>) -> Json<SystemStatus> {
    let database = state.database.as_ref().connection.ping().await.is_ok();

    Json(SystemStatus { database })
}
