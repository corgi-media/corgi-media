mod account;
mod api_docs;
mod auth;
mod libraries;
mod paths;
mod system;
mod users;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::openapi;
use crate::state::AppState;
pub use paths::Paths;

pub struct AppRouter;

impl AppRouter {
    pub fn route() -> Router<AppState> {
        let (router, api) = OpenApiRouter::with_openapi(openapi::Docs::openapi())
            .merge(system::route())
            .merge(account::route())
            .merge(auth::route())
            .merge(users::route())
            .merge(libraries::route())
            .split_for_parts();

        router.merge(api_docs::route(api))
    }
}
