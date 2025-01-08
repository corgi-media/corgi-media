use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use utoipa::ToSchema;

use corgi_core::uuid::Uuid;

use crate::{openapi::Tags, routers::Paths, state::AppState};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemStatus {
    database: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemInfo {
    service: String,
    version: String,
    identifier: Uuid,
    host_name: Option<String>,
    os: Option<String>,
    os_version: Option<String>,
    cpu_arch: String,
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

#[utoipa::path(
    get,
    path = Paths::SYSTEM_INFO,
    tag = Tags::SYSTEM,
    operation_id = "system_info",
    responses(
        (status = OK, description = "Get the information of the system", body = SystemInfo)
    )
)]
pub async fn info(State(state): State<AppState>) -> Json<SystemInfo> {
    Json(SystemInfo {
        service: "Corgi Server".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        identifier: state.config.identifier,
        host_name: System::host_name(),
        os: System::name(),
        os_version: System::os_version(),
        cpu_arch: System::cpu_arch(),
    })
}
