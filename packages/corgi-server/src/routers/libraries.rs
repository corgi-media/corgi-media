use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::libraries;
use crate::state::AppState;

pub fn route() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(libraries::create, libraries::list))
        .routes(routes!(
            libraries::get,
            libraries::update,
            libraries::delete
        ))
        .routes(routes!(
            libraries::directories::create,
            libraries::directories::list
        ))
        .routes(routes!(
            libraries::directories::update,
            libraries::directories::delete
        ))
}
