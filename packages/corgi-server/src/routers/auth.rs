use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::auth;
use crate::state::AppState;

pub fn route() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(auth::endpoints_password))
}
