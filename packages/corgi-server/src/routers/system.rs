use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::system;
use crate::state::AppState;

pub fn route() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(system::status))
        .routes(routes!(system::ping))
        .routes(routes!(system::info))
}
