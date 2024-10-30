use axum::{extract::State, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use corgi_core::config::AppConfigStatus;

use crate::{api_docs::ApiTags, state::AppState};

pub struct Routers;

impl Routers {
    pub const PATH: &'static str = "/configurations";
    pub const STATUS: &'static str = "/status";
}

#[utoipa::path(
    get,
    path = Routers::STATUS,
    tag = ApiTags::CONFIGURATIONS,
    responses(
        (status = OK, description = "Get the status system configurations", body = AppConfigStatus)
    )
)]
async fn config_status(State(state): State<AppState>) -> Json<AppConfigStatus> {
    Json(state.config.status())
}

impl Routers {
    pub fn route() -> OpenApiRouter<AppState> {
        OpenApiRouter::new().nest(
            Routers::PATH,
            OpenApiRouter::new().routes(routes![config_status]),
        )
    }
}
