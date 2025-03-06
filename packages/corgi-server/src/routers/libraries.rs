use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::libraries;
use crate::state::AppState;

pub fn route() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(libraries::create, libraries::query))
        .routes(routes!(
            libraries::find,
            libraries::update,
            libraries::delete
        ))
}
