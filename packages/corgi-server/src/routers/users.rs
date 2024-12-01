use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::users;
use crate::state::AppState;

pub fn route() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(users::create))
}
