use utoipa_axum::{router::OpenApiRouter, routes};

use crate::handlers::account;
use crate::state::AppState;

pub fn route() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(account::create, account::get))
        .routes(routes!(account::token::create))
}
